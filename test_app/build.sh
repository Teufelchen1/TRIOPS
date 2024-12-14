riscv64-elf-gcc test.c -O1 -fPIE -ffreestanding -nostdlib -fno-builtin -march=rv32i -mabi=ilp32 -T link.ld -o test.elf
riscv64-elf-objcopy -O binary test.elf test.bin
