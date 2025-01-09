
build-cross-release version:
    mkdir -p tmp
    rm -f tmp/*
    cross build --target arm-unknown-linux-musleabihf --release
    mv target/arm-unknown-linux-musleabihf/release/pico-reverse-proxy tmp/pico-reverse-proxy-arm-unknown-linux-musleabihf-{{version}}
    cross build --target aarch64-unknown-linux-musl --release
    mv target/aarch64-unknown-linux-musl/release/pico-reverse-proxy tmp/pico-reverse-proxy-aarch64-unknown-linux-musl-{{version}}
    cross build --target x86_64-unknown-linux-musl --release
    mv target/x86_64-unknown-linux-musl/release/pico-reverse-proxy tmp/pico-reverse-proxy-x86_64-unknown-linux-musl-{{version}}
