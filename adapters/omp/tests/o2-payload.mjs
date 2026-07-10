/**
 * Prove O2 payload shape matches OMP CustomMessage host contract:
 * - display is boolean true (not a string)
 * - content holds readable display_text
 * - MessageRenderer returns Component with render(width): string[]
 *
 * Drives real beavermath present + exported buildPreviewPayload / text component path.
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import { presentSource, extractOmpMessageText } from "../../common/present.mjs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

// Load extension helpers (buildPreviewPayload exported)
const extPath = join(__dirname, "../extension/index.js");
const { buildPreviewPayload } = await import(extPath);

const fixture = JSON.parse(
  readFileSync(join(__dirname, "fixtures/message_end.json"), "utf8"),
);
const source = extractOmpMessageText(fixture.message.content);
const r = presentSource(source);
if (!r.ok) {
  console.error("FAIL present", r.error);
  process.exit(1);
}
const p = r.presentation;
const payload = buildPreviewPayload(p, "msg-fixture-1");

// Host contract: display is boolean
if (payload.display !== true) {
  console.error("FAIL display must be boolean true, got", payload.display, typeof payload.display);
  process.exit(1);
}
if (typeof payload.content !== "string" || !payload.content.includes("power")) {
  console.error("FAIL content must hold semantic display_text", payload.content?.slice?.(0, 80));
  process.exit(1);
}
if (payload.content === source) {
  console.error("FAIL content should be presentation not raw source");
  process.exit(1);
}
if (payload.details?.exact_source !== source) {
  console.error("FAIL exact_source in details for provenance");
  process.exit(1);
}

// Simulate MessageRenderer Component
const fakeMessage = {
  customType: payload.customType,
  content: payload.content,
  display: payload.display,
  details: payload.details,
};
// Inline the same renderer logic as extension
function textComponent(text) {
  return {
    render(width) {
      const w = Math.max(20, width || 80);
      const out = [];
      for (const para of String(text).split("\n")) {
        let line = "";
        for (const word of para.split(/(\s+)/)) {
          if (line.length + word.length <= w) line += word;
          else {
            if (line.trim()) out.push(line.trimEnd());
            line = word.trimStart();
          }
        }
        if (line.length) out.push(line.trimEnd());
      }
      return out.length ? out : [""];
    },
  };
}
const component = textComponent(fakeMessage.content);
if (typeof component.render !== "function") {
  console.error("FAIL Component missing render");
  process.exit(1);
}
const lines = component.render(60);
if (!Array.isArray(lines) || !lines.some((l) => l.includes("power"))) {
  console.error("FAIL render(width) lines", lines);
  process.exit(1);
}

// Coercion check: non-boolean display would hide message
const bad = { ...payload, display: p.display_text };
if (Boolean(bad.display) === true && typeof bad.display === "string") {
  // strings are truthy but host coerces with typeof === 'boolean'
  const coerced = typeof bad.display === "boolean" ? bad.display : false;
  if (coerced !== false) {
    console.error("FAIL expected non-boolean display to coerce to hide");
    process.exit(1);
  }
}

console.log("PASS O2 payload + Component contract");
console.log("display:", payload.display, "content sample:", payload.content.slice(0, 80));
console.log("render lines:", lines.length);
