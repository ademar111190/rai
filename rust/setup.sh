#!/usr/bin/env bash
platforms=(
    # mac os
    aarch64-apple-darwin
    x86_64-apple-darwin

    # linux
    aarch64-unknown-linux-gnu
    i686-unknown-linux-gnu
    x86_64-unknown-linux-gnu

    # android
    aarch64-linux-android
    armv7-linux-androideabi
    i686-linux-android
    x86_64-linux-android
);
for platform in ${platforms[@]}; do
    rustup target add $platform
done

touch .cargo/config.toml
echo "[target.aarch64-linux-android]" > .cargo/config.toml
echo "linker = \"${ANDROID_CARGO_HOME}/aarch64-linux-android21-clang++\"" >> .cargo/config.toml
echo "[target.armv7-linux-androideabi]" >> .cargo/config.toml
echo "linker = \"${ANDROID_CARGO_HOME}/armv7a-linux-androideabi21-clang++\"" >> .cargo/config.toml
echo "[target.i686-linux-android]" >> .cargo/config.toml
echo "linker = \"${ANDROID_CARGO_HOME}/i686-linux-android21-clang++\"" >> .cargo/config.toml
echo "[target.x86_64-linux-android]" >> .cargo/config.toml
echo "linker = \"${ANDROID_CARGO_HOME}/x86_64-linux-android21-clang++\"" >> .cargo/config.toml
echo "Done"
