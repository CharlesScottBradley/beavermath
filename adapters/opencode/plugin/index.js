/**
 * BeaverMath OpenCode plugin (C1)
 *
 * On experimental.text.complete: run beavermath present, cache presentation.
 * Default: do NOT rewrite output.text (source immutability).
 * Optional: BEAVERMATH_OPENCODE_REWRITE_TEXT=1 rewrites display (experimental; may poison store).
 *
 * C2 bubble paint: API-blocked — no display-only paint hook without mutating text.
 */

import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { presentSource } from "../../common/present.mjs";

/** @type {import("@opencode-ai/plugin").Plugin} */
export async function BeaverMathPlugin(input, _options) {
  const dataDir =
    process.env.BEAVERMATH_OPENCODE_DATA ||
    join(input.directory || process.cwd(), ".beavermath-opencode");
  try {
    mkdirSync(dataDir, { recursive: true });
  } catch {
    /* ignore */
  }

  const cache = new Map();

  return {
    "experimental.text.complete": async (inp, output) => {
      try {
        const source = output?.text;
        if (typeof source !== "string" || !source.trim()) return;
        if (!/[\$\\]/.test(source) && !/```(?:math|latex|tex)/i.test(source)) {
          return;
        }

        const key = `${inp.sessionID}:${inp.messageID}:${inp.partID}`;
        const result = presentSource(source);
        if (!result.ok) {
          // Fail-open: leave output.text unchanged
          writeFileSync(
            join(dataDir, "last-error.json"),
            JSON.stringify({ key, error: result.error }, null, 2),
          );
          return;
        }

        const p = result.presentation;
        cache.set(key, p);

        // Display projection file (not model store)
        const path = join(
          dataDir,
          `present-${(inp.messageID || "msg").slice(0, 12)}-${(inp.partID || "part").slice(0, 8)}.json`,
        );
        writeFileSync(
          path,
          JSON.stringify(
            {
              sessionID: inp.sessionID,
              messageID: inp.messageID,
              partID: inp.partID,
              content_sha256: p.content_sha256,
              exact_source: p.exact_source,
              display_text: p.display_text,
              spanCount: p.spans?.length ?? 0,
            },
            null,
            2,
          ),
        );

        // Optional experimental rewrite — OFF by default
        if (process.env.BEAVERMATH_OPENCODE_REWRITE_TEXT === "1") {
          if (p.spans?.length > 0 && p.display_text !== p.exact_source) {
            output.text = p.display_text;
          }
        }
      } catch {
        // Fail-open
      }
    },
  };
}

// Default export for opencode plugin loaders that expect default
export default BeaverMathPlugin;
