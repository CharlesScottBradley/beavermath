#!/usr/bin/env python3
"""Map Grok/Cursor-compat Stop JSON → HookRequestV1. Exit 0 with empty stdout if no content."""
from __future__ import annotations

import json
import sys


def extract_content(d: dict) -> str:
    for key in (
        "content",
        "assistantMessage",
        "lastAssistantMessage",
        "text",
        "message",
        "agentMessage",
        "response",
    ):
        v = d.get(key)
        if isinstance(v, str) and v.strip():
            return v
    # Nested event payload
    for nest in ("payload", "data", "event", "body"):
        inner = d.get(nest)
        if isinstance(inner, dict):
            c = extract_content(inner)
            if c:
                return c
    msgs = d.get("messages")
    if isinstance(msgs, list):
        for m in reversed(msgs):
            if not isinstance(m, dict):
                continue
            role = (m.get("role") or m.get("type") or "").lower()
            if role in ("assistant", "model", "agent"):
                c = m.get("content") or m.get("text")
                if isinstance(c, str) and c.strip():
                    return c
                if isinstance(c, list):
                    parts = []
                    for block in c:
                        if isinstance(block, dict) and block.get("type") in ("text", "output_text"):
                            parts.append(str(block.get("text") or block.get("content") or ""))
                        elif isinstance(block, str):
                            parts.append(block)
                    joined = "".join(parts)
                    if joined.strip():
                        return joined
    return ""


def main() -> int:
    try:
        d = json.load(sys.stdin)
    except Exception:
        return 0
    if not isinstance(d, dict):
        return 0
    content = extract_content(d)
    if not content.strip():
        return 0
    mid = (
        d.get("message_id")
        or d.get("messageId")
        or d.get("turnId")
        or d.get("sessionId")
        or d.get("session_id")
        or "turn"
    )
    req = {
        "schema_version": "1",
        "kind": "message_end",
        "message_id": str(mid),
        "revision": int(d.get("revision") or d.get("rev") or 1),
        "content": content,
        "session_id": d.get("sessionId") or d.get("session_id"),
        "write_preview": True,
    }
    json.dump(req, sys.stdout, ensure_ascii=False)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
