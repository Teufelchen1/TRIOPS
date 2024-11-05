#define IO_ADDR (0x10013000)

void main(void);
void puts(char chr);
void print(char *str);
char read();

__asm__(
    ".globl _start\n"
    "_start:\n"
    "lui sp, 0x80004\n"
    "addi sp, sp, 0x0000\n"
    "call main\n"
);

void main(void) {
    print("Hello world!\n");
    print("Type CAT for a fun time!\n");
    print("Type $ to exit.\n");
    int exit = 1;
    int last_was_C = 0;
    int last_was_A = 0;
    int last_was_T = 0;
    do {
        char i = read();
        if (i != 0)
            puts(i);
        if(i == '$') exit = 0;
        if (i == 'C') {
            last_was_C = 1;
            last_was_A = 0;
            last_was_T = 0;
        } else if (last_was_C) {
            last_was_C = 0;
            if (i == 'A') {
                last_was_A = 1;
                last_was_T = 0;
            }
        }
        else if (last_was_A) {
            last_was_A = 0;
            if (i == 'T') {
                last_was_T = 1;
            }
        }
        else if (last_was_T) {
            last_was_T = 0;
            print(" _._     _,-'\"\"`-._\n(,-.`._,'(       |\\`-/|\n    `-.-' \\ )-`( , o o)\n          `-    \\`_`\"'-\n");
        }
    } while(exit != 0);
    puts('\n');

    /* Signal termination */
    __asm__("EBREAK");
}

void puts(char chr) {
    *(volatile char *)IO_ADDR = chr;
}

void print(char *str) {
    for(unsigned int i = 0; str[i] != 0; i++) {
        puts(str[i]);
    }
}

char read() {
    return *(char *)IO_ADDR;
}
