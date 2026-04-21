# Command Parsing

## What we're building
Minimal interactive shell: read line from keyboard → parse → dispatch → print result.

## Input pipeline
```
keyboard IRQ → scancode → ASCII → line buffer → parse → execute
```

## Parsing approach
- Split on whitespace
- First token = command name
- Rest = args
- No pipes, no env vars (yet)

## Commands to implement
- `help` — list commands
- `clear` — clear VGA buffer
- `echo <args>` — print args
- `halt` — halt CPU

## Related
- [[05-Keyboard/PS2-Protocol]]
- [[02-VGA/VGA-Buffer]]

## Status
> Not started
