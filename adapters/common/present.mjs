/**
 * Shared present helper: shell out to beavermath present (machine-safe JSON).
 * Used by OMP extension and OpenCode plugin.
 */
import { spawnSync } from "node:child_process";

/**
 * @param {string} source exact UTF-8 assistant source (immutable)
 * @param {{ bin?: string, mode?: string }} [opts]
 * @returns {{ ok: true, presentation: object } | { ok: false, error: string, exact_source: string }}
 */
export function presentSource(source, opts = {}) {
  const bin = opts.bin || process.env.BEAVERMATH_BIN || "beavermath";
  const mode = opts.mode || "semantic";
  if (typeof source !== "string") {
    return { ok: false, error: "source must be string", exact_source: String(source ?? "") };
  }
  const r = spawnSync(bin, ["present", "--mode", mode], {
    input: source,
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
    env: process.env,
  });
  if (r.error) {
    return { ok: false, error: r.error.message, exact_source: source };
  }
  if (r.status !== 0) {
    return {
      ok: false,
      error: (r.stderr || r.stdout || `exit ${r.status}`).trim(),
      exact_source: source,
    };
  }
  try {
    const presentation = JSON.parse(r.stdout);
    if (presentation.exact_source !== source) {
      // Never accept a present response that rewrote source
      return { ok: false, error: "present mutated exact_source", exact_source: source };
    }
    if (typeof presentation.display_text !== "string") {
      return { ok: false, error: "missing display_text", exact_source: source };
    }
    // Machine purity
    if (r.stdout.includes("\u001b")) {
      return { ok: false, error: "ESC in present stdout", exact_source: source };
    }
    return { ok: true, presentation };
  } catch (e) {
    return { ok: false, error: `json: ${e.message}`, exact_source: source };
  }
}

/**
 * Extract plain text from OMP AgentMessage-like content.
 * @param {unknown} content
 * @returns {string}
 */
export function extractOmpMessageText(content) {
  if (typeof content === "string") return content;
  if (!Array.isArray(content)) return "";
  const parts = [];
  for (const block of content) {
    if (!block || typeof block !== "object") continue;
    const t = block.type;
    if (t === "text" && typeof block.text === "string") parts.push(block.text);
    else if (typeof block.text === "string" && !t) parts.push(block.text);
    // skip thinking / toolCall / images for math presentation of main text only
  }
  return parts.join("");
}

/**
 * Extract text parts from OpenCode Part[] array.
 * @param {unknown} parts
 * @returns {string}
 */
export function extractOpenCodePartsText(parts) {
  if (!Array.isArray(parts)) return "";
  const out = [];
  for (const p of parts) {
    if (!p || typeof p !== "object") continue;
    if (p.type === "text" && typeof p.text === "string") out.push(p.text);
    else if (typeof p.text === "string") out.push(p.text);
  }
  return out.join("");
}
