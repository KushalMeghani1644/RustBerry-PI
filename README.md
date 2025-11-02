# RustBerry

An open, hackable, Pi‑like virtual single‑board computer and educational OS, built in Rust and running on QEMU (RISC‑V).

## Overview
RustBerry lets you explore bare‑metal programming, CPU architecture, and OS development without physical hardware. It combines a custom no_std boot path with a minimal kernel that targets QEMU’s `virt` machine.

## Current Status
- Bootloader built
- UART output working
- Next: heap, interrupts, timer, scheduler, syscalls

Example boot log:
```
[BOOT] RustBerry PI starting...
[UART] Hello from RustBerry PI
```

## Quick Start
### Prerequisites
- Rust toolchain (via rustup)
- QEMU with RISC‑V: `qemu-system-riscv64`

### Build
```
rustup target add riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf
```

### Run under QEMU
```
qemu-system-riscv64 \
  -machine virt \
  -nographic \
  -bios none \
  -kernel target/riscv64imac-unknown-none-elf/debug/bootloader
```
Quit QEMU with Ctrl+A, then X (in `-nographic` mode).

## Goals and Roadmap
1) PI‑like Board Simulator (Virtual Hardware)
- Define memory map (UART, GPIO, RAM, MMIO)
- Add MMIO device drivers (UART, timer, LED, etc.)
- Provide a minimal HAL crate (`rustberry-hal`)
- Offer a device‑tree–like configuration or static layout

2) Educational OS (RustBerry OS)
- Boots on the RustBerry virtual board
- `#![no_std]` + `alloc`, custom kernel
- Console I/O, heap allocator, basic task scheduler
- Evolve toward interrupts, timer ticks, syscalls
- Optional: shell, filesystem, additional drivers, simple graphics

## Repository Layout
- `bootloader/` – no_std boot stage and early bring‑up
- Future: `rustberry-hal/`, `kernel/`

## Acknowledgments
Built with Rust and QEMU; inspired by educational OS projects and the broader embedded Rust community.
