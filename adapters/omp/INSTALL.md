# Install BeaverMath OMP extension

From a clone of this repository:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
REPO_ROOT="$(git rev-parse --show-toplevel)"

# Interactive with extension for this launch
omp --extension="$REPO_ROOT/adapters/omp/extension"

# Print mode smoke (if model/auth configured)
omp --extension="$REPO_ROOT/adapters/omp/extension" -p 'Say only: The identity is $x^2+y^2=z^2$.'
```

Or after `cargo install --path crates/beavermath-cli --locked`, keep using the extension path from your clone (OMP loads JS from disk; it is not inside the CLI binary).

Expect: assistant source contains `$...$`; a BeaverMath custom preview may follow with semantic display.
