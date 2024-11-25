#define INT_UART0_BASE (0x10013000)
/* Register offsets */
#define UART_REG_TXFIFO         0x00
#define UART_REG_RXFIFO         0x04
#define UART_REG_TXCTRL         0x08
#define UART_REG_RXCTRL         0x0c
#define UART_REG_IE             0x10
#define UART_REG_IP             0x14
#define UART_REG_DIV            0x18

/* TXFIFO register */
#define UART_TXFIFO_FULL        (1 << 31)
#define UART_RXFIFO_EMPTY       (1 << 31)

/* TXCTRL register */
#define UART_TXEN               0x1
#define UART_TXWM(x)            (((x) & 0xffff) << 16)

/* RXCTRL register */
#define UART_RXEN               0x1
#define UART_RXWM(x)            (((x) & 0xffff) << 16)

/* IP register */
#define UART_IP_TXWM            0x1
#define UART_IP_RXWM            0x2

#define uint32_t unsigned int
#define _REG32(p, i) (*(volatile uint32_t *)((p) + (i)))

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


static inline int uart_read(char * byte)
{
    uint32_t data = _REG32(INT_UART0_BASE, UART_REG_RXFIFO);
    if ((data & UART_RXFIFO_EMPTY) != (uint32_t)UART_RXFIFO_EMPTY) {
        *byte = (data & 0xff);
        return 1;
    }
    return 0;
}

static void _drain()
{
    uint32_t data = _REG32(INT_UART0_BASE, UART_REG_RXFIFO);

    /* Intr cleared automatically when data is read */
    // while ((data & UART_RXFIFO_EMPTY) != (uint32_t)UART_RXFIFO_EMPTY) {
    //     data = _REG32(INT_UART0_BASE, UART_REG_RXFIFO);
    // }
}

void uart_init()
{
    /* Enable TX */
    _REG32(INT_UART0_BASE, UART_REG_TXCTRL) = UART_TXEN;

    /* avoid trap by emptying RX FIFO */
    _drain();

    /* enable RX interrupt */
    _REG32(INT_UART0_BASE, UART_REG_IE) = UART_IP_RXWM;

    /* Enable RX */
    _REG32(INT_UART0_BASE, UART_REG_RXCTRL) = UART_RXEN;
}

void uart_write(const char *data, int len)
{
    for (int i = 0; i < len; i++) {
        /* Wait for FIFO to empty */
        while ((_REG32(INT_UART0_BASE,
                       UART_REG_TXFIFO) & UART_TXFIFO_FULL)
               == (uint32_t)UART_TXFIFO_FULL) {}

        /* Write a byte */
        _REG32(INT_UART0_BASE, UART_REG_TXFIFO) = data[i];
    }
}


void main(void) {
    uart_init();
    print("Hello world!\n");
    print("Type CAT for a fun time!\n");
    print("Type $ to exit.\n");
    int exit = 0;
    int last_was_C = 0;
    int last_was_A = 0;
    do {
        char i = read();
        if (i != 0)
            puts(i);
        if(i == '$') exit = 1;
        if (i == 'C') {
            last_was_C = 1;
            last_was_A = 0;
        } else if (last_was_C) {
            last_was_C = 0;
            if (i == 'A') {
                last_was_A = 1;
            }
        }
        else if (last_was_A) {
            last_was_A = 0;
            if (i == 'T') {
                print("\n _._     _,-'\"\"`-._\n(,-.`._,'(       |\\`-/|\n    `-.-' \\ )-`( , o o)\n          `-    \\`_`\"'-\n");
            }
        }
    } while(exit == 0);
    puts('\n');

    /* Signal termination */
    __asm__("EBREAK");
}

void puts(char chr) {
    uart_write(&chr, 1);
}

void print(char *str) {
    for(unsigned int i = 0; str[i] != 0; i++) {
        uart_write(&str[i], 1);
    }
}

char read() {
    char byte = 0;
    uart_read(&byte);
    return byte;
}
