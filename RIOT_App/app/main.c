#include <stdio.h>
#include "shell.h"

int main(void)
{
    puts("Hello TRIOPS!");

    printf("You are running RIOT on a(n) %s (emulated) board.\n", RIOT_BOARD);
    printf("This board features a(n) %s CPU.\n", RIOT_CPU);

    printf("You are in the RIOT shell now, type 'help' for...help.\n");

    char line_buf[SHELL_DEFAULT_BUFSIZE];
    shell_run(NULL, line_buf, SHELL_DEFAULT_BUFSIZE);

    return 0;
}
