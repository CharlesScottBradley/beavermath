/**
 * Assert O2 delivery plan hits live UX surfaces correctly.
 * Drives shipped planO2Delivery / applyO2Delivery / buildPreviewPayload + real present.
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { presentSource, extractOmpMessageText } from "../../common/present.mjs";
import {
  planO2Delivery,
  applyO2Delivery,
  buildPreviewPayload,
  CUSTOM_TYPE,
} from "../extension/index.js";

const __dirname = dirname(fileURLToPath(import.meta.url));
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
const mid = "msg-1";

// --- message_end (streaming / not idle): widget only, no nextTurn sendMessage
const planEnd = planO2Delivery({ idle: false, phase: "message_end" }, p, mid);
if (!planEnd.widget || !Array.isArray(planEnd.widget.lines)) {
  console.error("FAIL expected widget lines on message_end", planEnd);
  process.exit(1);
}
if (planEnd.widget.lines.length === 0 || planEnd.widget.lines.length > 10) {
  console.error("FAIL widget line count", planEnd.widget.lines.length);
  process.exit(1);
}
if (planEnd.transcript !== null) {
  console.error("FAIL must not queue transcript on non-idle message_end", planEnd.transcript);
  process.exit(1);
}

// --- idle turn_end: widget + sendMessage without deliverAs nextTurn
const planIdle = planO2Delivery({ idle: true, phase: "turn_end" }, p, mid);
if (!planIdle.transcript) {
  console.error("FAIL expected transcript delivery when idle turn_end");
  process.exit(1);
}
if (planIdle.transcript.options?.deliverAs === "nextTurn") {
  console.error("FAIL must not use deliverAs nextTurn (hidden queue)");
  process.exit(1);
}
if (planIdle.transcript.options?.deliverAs) {
  console.error("FAIL deliverAs should be omitted", planIdle.transcript.options);
  process.exit(1);
}
const payload = planIdle.transcript.payload;
if (payload.display !== true || typeof payload.display !== "boolean") {
  console.error("FAIL display boolean", payload.display);
  process.exit(1);
}
if (payload.content !== p.display_text) {
  console.error("FAIL content must be display_text");
  process.exit(1);
}
if (payload.customType !== CUSTOM_TYPE) {
  console.error("FAIL customType");
  process.exit(1);
}

// --- applyO2Delivery mocks: prove surfaces are actually invoked
const calls = { widgets: [], messages: [] };
const mockUi = {
  setWidget(key, lines, options) {
    calls.widgets.push({ key, lines, options });
  },
};
const mockPi = {
  sendMessage(payload, options) {
    calls.messages.push({ payload, options });
  },
};

const appliedEnd = applyO2Delivery(mockPi, mockUi, planEnd);
if (!appliedEnd.widgetCalled || appliedEnd.sendMessageCalled) {
  console.error("FAIL message_end apply", appliedEnd, calls);
  process.exit(1);
}
if (calls.widgets.length !== 1 || calls.widgets[0].key !== "beavermath") {
  console.error("FAIL setWidget not called correctly", calls.widgets);
  process.exit(1);
}

const appliedIdle = applyO2Delivery(mockPi, mockUi, planIdle);
if (!appliedIdle.widgetCalled || !appliedIdle.sendMessageCalled) {
  console.error("FAIL idle apply", appliedIdle);
  process.exit(1);
}
const lastMsg = calls.messages[calls.messages.length - 1];
if (lastMsg.options?.deliverAs === "nextTurn") {
  console.error("FAIL applied sendMessage used nextTurn");
  process.exit(1);
}
if (lastMsg.payload.display !== true) {
  console.error("FAIL applied payload display");
  process.exit(1);
}

// buildPreviewPayload still consistent
const bp = buildPreviewPayload(p, mid);
if (bp.display !== true || bp.content !== p.display_text) {
  console.error("FAIL buildPreviewPayload");
  process.exit(1);
}

console.log("PASS O2 delivery plan + apply (widget immediate, transcript on idle, never nextTurn)");
console.log("widget lines:", planEnd.widget.lines.length, "sample:", planEnd.widget.lines[0]);
console.log("idle sendMessage options:", JSON.stringify(planIdle.transcript.options));
