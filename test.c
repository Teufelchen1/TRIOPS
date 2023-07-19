#define OUT_MEMORY_ADDR 0x20000000
#define STRING "Hello world!\n"

volatile char * out = (char *) OUT_MEMORY_ADDR;

void _start(void) __attribute__ ((section (".entry")));
void main(void);
void print(char *str, unsigned int len);

void main(void) {
    const char * str = STRING;
    unsigned int k = 20;
    for(unsigned int i = 0; i < sizeof(STRING); i++) {
        k += 2;
        *out = str[i];
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
        *out = str[i];
    }
}