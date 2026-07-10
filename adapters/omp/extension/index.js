/**
 * BeaverMath OMP extension (O1–O2) — delivery fixed for live UX
 *
 * CustomMessage.display is boolean (show/hide). Readable math lives in `content`.
 * MessageRenderer returns { render(width): string[] }.
 *
 * Delivery (O2):
 * 1. Always paint immediately via ctx.ui.setWidget(string[], { placement: "belowEditor" })
 *    — works during message_end while streaming is still winding down.
 * 2. On turn_end / agent_end when idle: also sendMessage without deliverAs:"nextTurn"
 *    so a visible custom transcript entry is appended (not hidden next-turn queue).
 *
 * Never mutates original assistant message source.
 */

import { presentSource, extractOmpMessageText } from "../../common/present.mjs";

export const CUSTOM_TYPE = "beavermath.math-preview";
const WIDGET_KEY = "beavermath";

/**
 * Minimal pi-tui Component.
 * @param {string} text
 */
export function textComponent(text) {
  const raw = String(text ?? "");
  return {
    render(width) {
      const w = Math.max(20, Math.min(width || 80, 200));
      const out = [];
      for (const para of raw.split("\n")) {
        if (para.length === 0) {
          out.push("");
          continue;
        }
        let line = "";
        for (const word of para.split(/(\s+)/)) {
          if (line.length + word.length <= w) {
            line += word;
          } else {
            if (line.trim().length) out.push(line.replace(/\s+$/, ""));
            if (word.length > w) {
              let rest = word.trimStart();
              while (rest.length > w) {
                out.push(rest.slice(0, w));
                rest = rest.slice(w);
              }
              line = rest;
            } else {
              line = word.trimStart();
            }
          }
        }
        if (line.length) out.push(line.replace(/\s+$/, ""));
      }
      if (out.length === 0) out.push("");
      return out;
    },
  };
}

/**
 * @param {object} p PresentationV1
 * @param {string} messageId
 */
export function buildPreviewPayload(p, messageId) {
  return {
    customType: CUSTOM_TYPE,
    display: true,
    content: p.display_text,
    details: {
      beavermath: true,
      sourceMessageId: messageId,
      content_sha256: p.content_sha256,
      exact_source: p.exact_source,
      spanCount: p.spans?.length ?? 0,
    },
  };
}

/**
 * Widget lines for setWidget (host caps string arrays at ~10 lines).
 * @param {object} p PresentationV1
 */
export function buildWidgetLines(p) {
  const header = `BeaverMath · ${p.spans?.length ?? 0} expr`;
  const body = String(p.display_text || "").split("\n");
  const lines = [header, ...body];
  return lines.slice(0, 10);
}

/**
 * Pure delivery planner: what UX surfaces to hit and with which options.
 * Testable without a live OMP session.
 *
 * @param {{ idle: boolean, phase: "message_end" | "turn_end" | "agent_end" }} ctx
 * @param {object} p PresentationV1
 * @param {string} messageId
 * @returns {{ widget: { key: string, lines: string[], options: object }, transcript: null | { payload: object, options: object } }}
 */
export function planO2Delivery(ctx, p, messageId) {
  const payload = buildPreviewPayload(p, messageId);
  const widget = {
    key: WIDGET_KEY,
    lines: buildWidgetLines(p),
    options: { placement: "belowEditor" },
  };
  // Transcript custom message only when idle (turn/agent end). Never nextTurn (hidden queue).
  let transcript = null;
  if (ctx.idle && (ctx.phase === "turn_end" || ctx.phase === "agent_end")) {
    transcript = {
      payload,
      // Omit deliverAs entirely — do not use nextTurn (hidden from UI).
      options: { triggerTurn: false },
    };
  }
  return { widget, transcript };
}

/**
 * Apply plan against host surfaces (pi + ctx.ui).
 * @returns {{ widgetCalled: boolean, sendMessageCalled: boolean, sendMessageOptions: object | null }}
 */
export function applyO2Delivery(pi, ui, plan) {
  const result = {
    widgetCalled: false,
    sendMessageCalled: false,
    sendMessageOptions: null,
  };
  if (plan.widget && ui && typeof ui.setWidget === "function") {
    ui.setWidget(plan.widget.key, plan.widget.lines, plan.widget.options);
    result.widgetCalled = true;
  }
  if (plan.transcript && pi && typeof pi.sendMessage === "function") {
    pi.sendMessage(plan.transcript.payload, plan.transcript.options);
    result.sendMessageCalled = true;
    result.sendMessageOptions = plan.transcript.options;
  }
  return result;
}

/** @param {import("@oh-my-pi/pi-coding-agent").ExtensionAPI} pi */
export default function beavermathExtension(pi) {
  pi.setLabel("BeaverMath");

  /** @type {{ p: object, messageId: string } | null} */
  let pending = null;

  pi.registerMessageRenderer(CUSTOM_TYPE, (message, _options, _theme) => {
    let text = "";
    if (typeof message.content === "string") {
      text = message.content;
    } else if (Array.isArray(message.content)) {
      text = message.content
        .map((b) => (b && typeof b.text === "string" ? b.text : ""))
        .join("");
    }
    if (message.details && typeof message.details.display_text === "string") {
      text = message.details.display_text;
    }
    if (!text) return undefined;
    const body = textComponent(`BeaverMath\n${text}`);
    return {
      render(width) {
        return body.render(width);
      },
    };
  });

  function tryPresentFromMessage(msg) {
    if (!msg) return null;
    const role = msg.role || msg.type;
    if (role && role !== "assistant" && role !== "model") return null;
    const source = extractOmpMessageText(msg.content);
    if (!source.trim()) return null;
    if (!/[\$\\]/.test(source) && !/```(?:math|latex|tex)/i.test(source)) return null;
    const result = presentSource(source);
    if (!result.ok) return { error: result.error };
    const p = result.presentation;
    if (!p.spans || p.spans.length === 0) return null;
    if (p.display_text === p.exact_source) return null;
    return {
      p,
      messageId: String(msg.id || msg.messageId || "unknown"),
    };
  }

  pi.on("message_end", async (event, ctx) => {
    try {
      const got = tryPresentFromMessage(event?.message);
      if (!got) return;
      if (got.error) {
        try {
          ctx?.ui?.notify?.(`BeaverMath: ${got.error}`, "warning");
        } catch {
          /* fail-open */
        }
        return;
      }

      pending = { p: got.p, messageId: got.messageId };

      try {
        pi.appendEntry("beavermath.presentation", {
          sourceMessageId: got.messageId,
          content_sha256: got.p.content_sha256,
          spanCount: got.p.spans.length,
        });
      } catch {
        /* optional */
      }

      // Immediate UX: widget always (works while turn is still unwinding)
      const plan = planO2Delivery(
        { idle: false, phase: "message_end" },
        got.p,
        got.messageId,
      );
      applyO2Delivery(pi, ctx?.ui, plan);
    } catch {
      /* fail-open */
    }
  });

  function flushTranscriptIfIdle(phase, ctx) {
    if (!pending) return;
    try {
      const idle =
        typeof pi.isIdle === "function" ? !!pi.isIdle() : phase === "agent_end";
      const plan = planO2Delivery(
        { idle, phase },
        pending.p,
        pending.messageId,
      );
      // Re-assert widget; add transcript when idle
      applyO2Delivery(pi, ctx?.ui, plan);
      if (plan.transcript) {
        pending = null;
      }
    } catch {
      /* fail-open */
    }
  }

  pi.on("turn_end", async (_event, ctx) => {
    flushTranscriptIfIdle("turn_end", ctx);
  });

  pi.on("agent_end", async (_event, ctx) => {
    flushTranscriptIfIdle("agent_end", ctx);
  });

  pi.registerCommand("beavermath-status", {
    description: "Show BeaverMath extension status",
    handler: async (_args, ctx) => {
      ctx.ui?.notify?.(
        "BeaverMath O2: widget on message_end + visible custom message on idle turn/agent end (never nextTurn hidden queue).",
        "info",
      );
    },
  });
}
