# Bitforge

bitforge is a low level sandbox for unix-like systems.

## Key Features
* Partial emulation (syscalls, jumps and memory reads are emulated, everything else should run natively).
* Intercepting syscalls on the instruction layer.

## Goals
- [ ] Configuration through both toml and command line arguments
- [ ] Proxy capalities by overwriting network syscalls (similar to [torsocks](https://linux.die.net/man/8/torsocks) except on the instruction layer)

## License
This project is licensed under the MIT License.


