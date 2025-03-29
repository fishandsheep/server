# server
a simple httpserver

# cross 
https://github.com/orgs/cross-rs/packages?repo_name=cross

# linux-arm 32
cross build --target=armv7-unknown-linux-gnueabihf --release
# linux-arm 64
cross build --target=aarch64-unknown-linux-gnu --release

# window 64 x86
cross build --target x86_64-pc-windows-gnu --release
