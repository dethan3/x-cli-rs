# Release Checklist

Use this checklist before publishing the first public release of `x-cli-rs`.

## 1. Repository health

- [ ] GitHub Actions is enabled for the repository.
- [ ] `CI` workflow runs on push and pull requests.
- [ ] `Release` workflow is visible in the Actions tab.
- [ ] Branch protection rules are configured, if desired.

## 2. Local Rust checks

Run from the repository root:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --release -p xcli -p chatgpt-image-cli
```

Expected result:

- [ ] formatting passes
- [ ] clippy passes with `-D warnings`
- [ ] all tests pass
- [ ] release binaries build locally

## 3. WebBridge compatibility

Before release, verify against a real `kimi-webbridge` daemon.

Prerequisites:

- [ ] daemon is running at `http://127.0.0.1:10086`
- [ ] Chrome extension is connected
- [ ] Chrome is signed in to `chatgpt.com`
- [ ] ChatGPT Images page is available in the logged-in account

Run:

```bash
cargo run -p xcli -- --verbose chatgpt-image generate "a cute panda riding a bicycle" -o ./images
cargo run -p chatgpt-image-cli -- --verbose generate "a cute panda riding a bicycle" -o ./images
```

Verify:

- [ ] verbose logs show `status`
- [ ] verbose logs show `navigate`
- [ ] verbose logs show `input`
- [ ] verbose logs show `submit`
- [ ] verbose logs show `wait_url`
- [ ] verbose logs show `wait_image`
- [ ] verbose logs show `read_image_meta`
- [ ] verbose logs show `download_image`
- [ ] verbose logs show `write_file`
- [ ] generated PNG exists
- [ ] generated PNG can be opened

## 4. JSON output contract

Successful output must be valid JSON on stdout only:

```json
{
  "ok": true,
  "data": {
    "prompt": "...",
    "path": "...",
    "bytes": 123,
    "caption": "...",
    "conversation_url": "https://chatgpt.com/c/...",
    "elapsed_ms": 12345
  }
}
```

Error output must be valid JSON on stdout only:

```json
{
  "ok": false,
  "error": {
    "code": "invalid_args",
    "message": "..."
  }
}
```

Verify:

- [ ] success exits with code `0`
- [ ] failure exits with code `1`
- [ ] stdout is JSON only
- [ ] verbose logs are written to stderr
- [ ] error codes are stable

Recommended checks:

```bash
cargo run -p xcli -- chatgpt-image generate "" ; echo $?
cargo run -p xcli -- --verbose chatgpt-image generate "hello" >/tmp/xcli-out.json 2>/tmp/xcli-err.log
python -m json.tool /tmp/xcli-out.json >/dev/null
```

## 5. Release workflow dry run

Use manual dispatch before tagging, if possible:

- [ ] Run `Release` workflow with `workflow_dispatch`.
- [ ] Linux artifact is produced.
- [ ] macOS arm64 artifact is produced.
- [ ] macOS x86_64 artifact is produced.
- [ ] Windows artifact is produced.
- [ ] Each artifact contains `x` and `chatgpt-image-cli`.
- [ ] Windows artifact contains `x.exe` and `chatgpt-image-cli.exe`.
- [ ] Each artifact has a matching `.sha256` file.

## 6. Install scripts

After a release exists, verify install scripts.

macOS / Linux:

```bash
XCLI_RS_VERSION=v0.1.0 sh ./install.sh
XCLI_RS_VERSION=v0.1.0 XCLI_RS_INSTALL_DIR=/tmp/x-cli-rs-bin sh ./install.sh
```

Windows PowerShell:

```powershell
$env:XCLI_RS_VERSION="v0.1.0"
./install.ps1
```

Verify:

- [ ] correct target triple is detected
- [ ] release zip downloads
- [ ] checksum downloads
- [ ] checksum verification passes
- [ ] binaries are installed
- [ ] installed `x --help` works
- [ ] installed `chatgpt-image-cli --help` works

## 7. Publish v0.1.0

Create and push the tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Verify:

- [ ] Release workflow starts automatically.
- [ ] Release workflow succeeds.
- [ ] GitHub Release is created.
- [ ] Release notes are generated.
- [ ] All zip files are attached.
- [ ] All checksum files are attached.

## 8. Post-release smoke test

Install from the public release:

```bash
curl -fsSL https://raw.githubusercontent.com/hu-qi/x-cli-rs/main/install.sh | sh
x --help
chatgpt-image-cli --help
```

Run a real generation:

```bash
x --verbose chatgpt-image generate "a cute panda riding a bicycle" -o ./images
```

Verify:

- [ ] command succeeds
- [ ] image file exists
- [ ] stdout JSON is valid
- [ ] stderr logs are useful

## 9. Rollback plan

If release is broken:

- [ ] Delete or mark the GitHub Release as prerelease.
- [ ] Create a patch branch.
- [ ] Fix the issue.
- [ ] Tag `v0.1.1`.
- [ ] Update README if install instructions changed.
- [ ] Add a release note explaining the fix.
