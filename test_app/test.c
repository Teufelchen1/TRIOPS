#define IO_ADDR (0x10000000)
#define STRING "Hello world!\n"

void main(void);
void print(char *str, unsigned int len);

__asm__(
    ".globl _start\n"
    "_start:\n"
    "lui sp, 0x80004\n"
    "addi sp, sp, 0x0000\n"
    "call main\n"
);

void main(void) {
    const char * str = STRING;
    unsigned int k = 20;
    for(unsigned int i = 0; i < sizeof(STRING); i++) {
        k += 2;
        *(char *)IO_ADDR = str[i];
    }
    print("WoW!\n", 5);
    if(k > 40)
        __asm__("EBREAK");
    else
        __asm__("ECALL");
}

void print(char *str, unsigned int len) {
    for(unsigned int i = 0; i < len; i++) {
        *(char *)IO_ADDR = str[i];
    }
}