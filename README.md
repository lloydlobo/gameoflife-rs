# Game Of Life - rust

## Dev

-   Enable Bevy's Dynamic Linking Feature: This is the most impactful compilation time decrease! If bevy
    is a dependency you can compile the binary with the "dynamic" feature flag (enables dynamic
    linking). Note that right now, this doesn't work on Windows. [Source](https://bevyengine.org/learn/book/getting-started/setup/)

    ```bash
    cargo run --features bevy/dynamic
    ```

-   LLD linker: The Rust compiler spends a lot of time in the "link" step. LLD is much faster at linking than the default Rust linker. To install LLD, find your OS below and run the given command:

    ```bash
    # Ubuntu:
        sudo apt-get install lld
    # Arch:
        sudo pacman -S lld
    # Windows: Ensure you have the latest cargo-binutils
        cargo install -f cargo-binutils
        rustup component add llvm-tools-preview
    # MacOS: Modern LLD does not yet support MacOS, but we can use zld instead:
        brew install michaeleisel/zld/zld
    ```

### Bevy Dependencies

-   [Linux](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
-   [More information](https://bevyengine.org/learn/book/getting-started/setup/)

#### Fedora Setup

[Source: github](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

```bash
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
```

## Credits

Thanks to Jon Škoberne [jonchisko](https://github.com/jonchisko)

-   [Github](https://github.com/jonchisko/GameOfLife)
-   [Youtube](https://www.youtube.com/watch?v=spUWzEbfhHY)
