# Chaaya: An NDS Emulator Built in Zig

## Build
This project uses [gyro](https://github.com/mattnite/gyro) for zig package management. NixOS
users can install from my custom derivation
[here](https://github.com/devins2518/dotfiles/blob/surface/overlays/gyro.nix). There is an AUR
package for Arch users as well.

Build:
```sh
gyro build
```
Run:
```sh
gyro build run
```

### Why Zig?
I originally wrote this in Rust since it was the language which I was most comfortable writing
stuff in that. However, the memory management model of Rust made it a bit difficult to me to
properly implement the relationships between the different but interconnected parts of the
hardware. I've only dipped my toes in Zig, but so far, it seems like a very good language to write
an emulator in. Language constructs like suspend/resume will make writing cycle accurate functions
trivial compared to Rust. This all is extremely experimental, and I don't even know if this is a
good idea, but this seems to be a fun experiment.

## Website: https://taruntapan.github.io/
