cargo bootimage

qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rustos.bin
