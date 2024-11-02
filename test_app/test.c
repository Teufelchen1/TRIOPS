#define IO_ADDR (0x10013000)

void main(void);
void print(char *str);

__asm__(
    ".globl _start\n"
    "_start:\n"
    "lui sp, 0x80004\n"
    "addi sp, sp, 0x0000\n"
    "call main\n"
);

void main(void) {
    print("Hello world!\n");
    print("WoW!\n");

    /* Signal termination */
    __asm__("EBREAK");
}

void print(char *str) {
    for(unsigned int i = 0; str[i] != 0; i++) {
        *(char *)IO_ADDR = str[i];
    }
}
