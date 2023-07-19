#define IO_ADDR (0x60000000)
#define STRING "Hello world!\n"

void _start(void) __attribute__ ((section (".entry")));
void main(void);
void print(char *str, unsigned int len);

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

void _start() {
    main();
}

void print(char *str, unsigned int len) {
    for(unsigned int i = 0; i < len; i++) {
        *(char *)IO_ADDR = str[i];
    }
}