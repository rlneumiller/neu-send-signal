# neu-send-signal

A tiny Rust CLI for sending a Signal message through the CallMeBot API.  
It supports strict quoting rules by default, with optional modes for shell‑safe and raw message handling.

## Installation

### Install locally from this directory

If you’ve cloned the repository:

```
cargo install --path .
```

This builds the release binary and installs it into your Cargo bin directory (usually `~/.cargo/bin`).

### Or build manually

```
cargo build --release
```

Binary will be located at:

```
target/release/neu-send-signal
```

## Required environment variables

Before running the tool, set:

```
export CALLMEBOT_MYPHONE="your-callmebot-phone-id"
export CALLMEBOT_APIKEY="your-callmebot-api-key"
```

Example:

```
export CALLMEBOT_MYPHONE="12345678-1234-1234-1234-1234567890ab"
export CALLMEBOT_APIKEY="123456"
```

## Usage

```
neu-send-signal [OPTIONS] [MESSAGE]
```

### Default (strict) mode

Strict mode requires the message to be passed as a **single quoted string**:

```
neu-send-signal "Hello from neu-send-signal"
```

If you forget the quotes:

```
neu-send-signal Hello world
```

You’ll get:

```
Error: Message text must be provided as a single quoted string.
Hint: Use --shell-safe for auto-joining or --raw for literal input.
```

If no message is provided, a default message is used:

```
neu-send-signal
```

Sends:

```
This is the default message from neu-send-signal
```

---

## Shell‑safe mode (`--shell-safe`)

Allows unquoted multi‑word messages. Arguments are auto‑joined with spaces.

```
neu-send-signal --shell-safe Hello world here's an apostrophe
```

Useful when you want convenience but still want safe joining.

---

## Raw mode (`--raw`)

Accepts **anything** the shell passes, with no validation and no quoting rules.

```
neu-send-signal --raw Hello world here's $PATH `weird stuff`
```

This is ideal for scripting, pipelines, or when you want literal behavior.

---

## Examples

### Strict mode (quoted)
```
neu-send-signal "Hello from neu-send-signal"
```

### Shell‑safe mode
```
neu-send-signal --shell-safe Hello from neu-send-signal built with Rust
```

### Raw mode
```
neu-send-signal --raw Hello from neu-send-signal built with Rust here's an apostrophe
```

### Default message
```
neu-send-signal
```

---

## Exit behavior

- Prints `✓ Message Sent` on success  
- Prints an error message if:
  - environment variables are missing  
  - quoting rules are violated  
  - the HTTP request fails  

---

## Notes

- This tool uses the CallMeBot Signal API endpoint:

  ```
  https://signal.callmebot.com/signal/send.php
  ```

- Message text is URL‑encoded before sending.

