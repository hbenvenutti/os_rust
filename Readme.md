# Rust Operational System


## Compilando
```shell
cargo build --target thumbv7em-none-eabihf
```
By passing a --target argument we cross compile our executable for a bare metal target system. Since the target system has no operating system, the linker does not try to link the C runtime and our build succeeds without any linker errors.

<br>

## Mudando o target do rust
```shell
rustup target add thumbv7em-none-eabihf
```
This downloads a copy of the standard (and core) library for the system. Now we can build our freestanding executable for this target:
