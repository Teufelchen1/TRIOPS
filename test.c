// riscv32-unknown-elf-gcc -c test.c -ffreestanding -fno-builtin -march=rv32i -o test.elf
// riscv32-unknown-elf-objcopy --strip-debug --only-section=.text -O binary test.elf test.hex

int main(void) {
    unsigned int k = 20;
    for(int i = 0; i < 10; i++) {
        k += 2;
    }
    if(k == 40)
        __asm__("EBREAK");
    else
        __asm__("ECALL");
}