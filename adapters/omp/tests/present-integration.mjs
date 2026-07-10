/**
 * Fixture: OMP message_end → present. Drives real beavermath binary.
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { presentSource, extractOmpMessageText } from "../../common/present.mjs";

const __dirname = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(
  readFileSync(join(__dirname, "fixtures/message_end.json"), "utf8"),
);

const source = extractOmpMessageText(fixture.message.content);
if (!source.includes("$x^2+y^2=z^2$")) {
  console.error("FAIL: fixture source missing math");
  process.exit(1);
}

const r1 = presentSource(source);
const r2 = presentSource(source);
if (!r1.ok || !r2.ok) {
  console.error("FAIL present", r1, r2);
  process.exit(1);
}
const p1 = r1.presentation;
const p2 = r2.presentation;
if (p1.exact_source !== source) {
  console.error("FAIL source mutated");
  process.exit(1);
}
if (p1.content_sha256 !== p2.content_sha256) {
  console.error("FAIL digest unstable");
  process.exit(1);
}
if (p1.display_text === p1.exact_source) {
  console.error("FAIL display equals source");
  process.exit(1);
}
if (!p1.display_text.includes("power") && !p1.display_text.includes("equals")) {
  console.error("FAIL no readable presentation", p1.display_text);
  process.exit(1);
}
if (!p1.display_text.includes("https://x.com/$user$/p")) {
  console.error("FAIL URL not protected in display");
  process.exit(1);
}
if (!p1.display_text.includes("`$x$`")) {
  console.error("FAIL code not protected");
  process.exit(1);
}
if (JSON.stringify(p1).includes("\u001b")) {
  console.error("FAIL ESC in JSON");
  process.exit(1);
}
console.log("PASS omp fixture→present");
console.log("display:", p1.display_text.slice(0, 120));
console.log("spans:", p1.spans?.length);
