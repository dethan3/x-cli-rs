# ChatGPT Image real-browser debugging checklist

Use this checklist when validating `x chatgpt-image generate` against a real Chrome session through `kimi-webbridge`.

## Prerequisites

- `kimi-webbridge` daemon is running.
- Chrome WebBridge extension is installed and connected.
- Chrome is already signed in to `https://chatgpt.com`.
- The ChatGPT Images page is accessible in the same Chrome profile.
- The installed `x` binary is on `PATH`.

## Smoke test

```bash
x --verbose chatgpt-image generate "a red apple on a wooden table" -o ./images --timeout 180
```

The command should print flow logs to stderr and a JSON object to stdout.

## Expected verbose steps

```text
status -> navigate -> input -> submit -> wait_url -> wait_image -> read_image_meta -> download_image -> write_file
```

## Stage-by-stage checks

### 1. status

Expected:

- WebBridge daemon responds at `http://127.0.0.1:10086/status`.
- `running` is `true`.
- `extension_connected` is `true`.

Common failures:

- `daemon_unreachable`: daemon is not running or port is different.
- `daemon_not_running`: status endpoint is reachable but daemon reports not running.
- `extension_not_connected`: Chrome extension is missing, disabled, or not connected.

Useful checks:

```bash
curl -fsSL http://127.0.0.1:10086/status
XCLI_WEBBRIDGE_URL=http://127.0.0.1:10086 x --verbose chatgpt-image generate "hello"
```

### 2. navigate

Expected:

- Chrome opens `https://chatgpt.com/images/`.
- The active session is signed in.

Common failures:

- Login page appears instead of ChatGPT Images.
- Captcha or interstitial blocks the page.
- Browser profile differs from the one where the user is signed in.

### 3. input

Expected:

- Prompt appears inside `#prompt-textarea`.
- The submit button becomes enabled.

Current selector:

```text
#prompt-textarea
```

Common failures:

- ChatGPT changed the selector.
- The field is inside a different container or iframe.
- Input event was not accepted by the app.

Manual browser console checks:

```js
document.querySelector('#prompt-textarea')
document.querySelector('#prompt-textarea')?.textContent
```

### 4. submit

Expected:

- Clicking `#composer-submit-button` submits the prompt.
- URL eventually becomes a conversation URL containing `/c/`.

Current selector:

```text
#composer-submit-button
```

Common failures:

- Submit button selector changed.
- Button remains disabled because input was not accepted.
- ChatGPT is rate-limited or blocked by an interstitial.

Manual browser console check:

```js
document.querySelector('#composer-submit-button')
```

### 5. wait_url

Expected:

- `location.href.includes('/c/')` becomes `true`.

Common failures:

- ChatGPT changed conversation URL shape.
- Page stays on `/images/` because submit did not fire.
- Network or account issue prevents conversation creation.

Manual browser console check:

```js
location.href
location.href.includes('/c/')
```

### 6. wait_image

Expected:

- A generated image appears under `<main>`.

Current selector:

```text
main img[src*='/backend-api/estuary/content']
```

Common failures:

- Image URL shape changed.
- Image is rendered as a blob URL or background image.
- Generation timed out or failed in the UI.

Manual browser console checks:

```js
document.querySelector("main img[src*='/backend-api/estuary/content']")
Array.from(document.querySelectorAll('main img')).map(img => img.src)
```

### 7. read_image_meta

Expected:

- Image metadata returns `src`, `alt`, and current conversation URL.

Common failures:

- Selector matched the wrong image.
- Image has no usable `src`.

### 8. download_image

Expected:

- The browser context can `fetch()` the image URL.
- Returned bytes decode from base64.

Common failures:

- Fetch blocked by auth/session mismatch.
- Image URL expires.
- Browser returns an HTML error page instead of image bytes.

### 9. write_file

Expected:

- Output directory exists or can be created.
- PNG is written as `chatgpt-YYYYMMDD-HHMMSS.png`.

Common failures:

- Output directory has no write permission.
- Disk is full.

## Useful commands

Run with custom bridge URL:

```bash
XCLI_WEBBRIDGE_URL=http://127.0.0.1:10086 x --verbose chatgpt-image generate "hello"
```

Run with longer timeout:

```bash
x --verbose chatgpt-image generate "hello" --timeout 300
```

Capture stdout and stderr separately:

```bash
x --verbose chatgpt-image generate "hello" > out.json 2> debug.log
```

Inspect output:

```bash
cat out.json
cat debug.log
```

## Reporting a bug

Include:

- OS and architecture.
- x-cli-rs version or commit SHA.
- `kimi-webbridge` version if available.
- Chrome version.
- Command used.
- Full stderr from `--verbose`.
- JSON stdout.
- The first failing step from the verbose log.
- Screenshots or selector checks when the failure is page-related.
