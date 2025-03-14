# Pico reverse proxy

A very small HTTP [reverse proxy](https://www.cloudflare.com/learning/cdn/glossary/reverse-proxy/).

> [!WARNING]
> - This does not support HTTPS
> - You should not use this in production

I created it because sometimes
I need a reverse proxy in trusted networks and I don't want to run
[caddy](https://caddyserver.com), [nginx](https://nginx.org/en/),
or [haproxy](https://www.haproxy.com). Often this will be in some
kind of embedded, resource-constrained environment.

## Building

You'll need a [Rust](https://www.rust-lang.org) tool chain. Then
you can merely `cargo build --release` to get a binary.

## Running

Run like `pico-reverse-proxy --source localhost:8080 --destination remotehost:5432`

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org/>
