# Universal oneshot (H1)

Any host that can print model stdout to a pipe:

```bash
# Filter document (semantic replacement of explicit math only)
some-agent --print '...' | beavermath filter

# Or a file
beavermath filter --file message.txt

# Structured artifacts
printf '%s' 'Answer $x^2$.' | beavermath render --format jsonl
```

## Exit status

`beavermath filter` returns 0 on success. Wrappers that chain a producer should preserve the **producer** exit code when filtering succeeds (see `codex/oneshot.sh`).

## Machine purity

Filter/json stdout contain no terminal graphics protocol bytes. Adversarial ESC in input is visibly escaped in text sinks.
