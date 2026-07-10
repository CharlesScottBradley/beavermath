/**
 * Fixture: OpenCode text.complete payload → present (real binary).
 */
import { readFileSync, mkdtempSync, writeFileSync, readdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { presentSource } from "../../common/present.mjs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(
  readFileSync(join(__dirname, "fixtures/text_complete.json"), "utf8"),
);
const source = fixture.output.text;

const r = presentSource(source);
if (!r.ok) {
  console.error("FAIL", r.error);
  process.exit(1);
}
const p = r.presentation;
if (p.exact_source !== source) {
  console.error("FAIL source mutated");
  process.exit(1);
}
if (!p.display_text.includes("https://x.com/$user$/ok")) {
  console.error("FAIL URL");
  process.exit(1);
}
if (!p.display_text.includes("`$x$`")) {
  console.error("FAIL code");
  process.exit(1);
}
if (p.spans.length < 1) {
  console.error("FAIL no spans");
  process.exit(1);
}

// Simulate plugin write without rewrite
const dir = mkdtempSync(join(tmpdir(), "bm-oc-"));
writeFileSync(
  join(dir, "present.json"),
  JSON.stringify({ exact_source: p.exact_source, display_text: p.display_text }),
);
const written = JSON.parse(readFileSync(join(dir, "present.json"), "utf8"));
if (written.exact_source !== source) {
  console.error("FAIL stored source");
  process.exit(1);
}

// Bare ASCII strict
const bare = presentSource("plain x^2+y^2=z^2 without dollars");
if (!bare.ok) {
  console.error("FAIL bare present", bare.error);
  process.exit(1);
}
if ((bare.presentation.spans || []).length !== 0) {
  console.error("FAIL bare ASCII selected as math", bare.presentation.spans);
  process.exit(1);
}

console.log("PASS opencode fixture→present");
console.log("display:", p.display_text.slice(0, 100));
console.log("bare spans:", bare.presentation.spans?.length ?? 0);
