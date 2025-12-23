# RIOT for TRIOPS

The [RIOT](https://github.com/RIOT-OS/RIOT) OS supports the Hifive1b board, which is emulated by TRIOPS.
All you need to do is building a RIOT app for that board and load the resulting binary file into TRIOPS.
To setup your system for RIOT development, please refer to the [Getting Started](https://guide.riot-os.org/getting-started/installing/) guide of RIOT.
The tl;dr is that you will need GCC (`riscv64-unknown-elf`), binutils, and newlib.

Because it is easy to use RIOT with external apps, TRIOPS comes with an example app provided in the `app/` folder.
The `Makefile` in the example configures RIOT to:

* target the Hifive1b board
* set the app name to `triops_app`
* use the emulator for running the app
* which RIOT modules to include

While the `main.c` prints a hello-world and starts the RIOT shell.

## Building the RIOT example app

1. Clone RIOT. Because it is a git submodule, you can fetch it right here:
```sh
git submodule update
```

2. Enter the `app/` directory and run gnu `make`
```sh
cd app/ && make
```

3. You will see RIOTs build process and the result binary `triops_app.elf`. It should look something like this:
```txt
Building application "triops_app" for "hifive1b" with CPU "fe310".

make -C /foo/triops/RIOT_App/RIOT/pkg/mpaland-printf/ 
make -C /foo/triops/RIOT_App/RIOT/build/pkg/mpaland-printf -f /foo/triops/RIOT_App/RIOT/pkg/mpaland-printf/mpaland-printf.mk
make -C /foo/triops/RIOT_App/RIOT/boards
make -C /foo/triops/RIOT_App/RIOT/boards/common/init
make -C /foo/triops/RIOT_App/RIOT/boards/hifive1b
make -C /foo/triops/RIOT_App/RIOT/core
make -C /foo/triops/RIOT_App/RIOT/core/lib
make -C /foo/triops/RIOT_App/RIOT/cpu/fe310
make -C /foo/triops/RIOT_App/RIOT/cpu/fe310/periph
make -C /foo/triops/RIOT_App/RIOT/cpu/fe310/vendor
make -C /foo/triops/RIOT_App/RIOT/cpu/riscv_common
make -C /foo/triops/RIOT_App/RIOT/cpu/riscv_common/periph
make -C /foo/triops/RIOT_App/RIOT/drivers
make -C /foo/triops/RIOT_App/RIOT/drivers/periph_common
make -C /foo/triops/RIOT_App/RIOT/sys
make -C /foo/triops/RIOT_App/RIOT/sys/auto_init
make -C /foo/triops/RIOT_App/RIOT/sys/div
make -C /foo/triops/RIOT_App/RIOT/sys/isrpipe
make -C /foo/triops/RIOT_App/RIOT/sys/libc
make -C /foo/triops/RIOT_App/RIOT/sys/malloc_thread_safe
make -C /foo/triops/RIOT_App/RIOT/sys/newlib_syscalls_default
make -C /foo/triops/RIOT_App/RIOT/sys/preprocessor
make -C /foo/triops/RIOT_App/RIOT/sys/ps
make -C /foo/triops/RIOT_App/RIOT/sys/shell
make -C /foo/triops/RIOT_App/RIOT/sys/shell/cmds
make -C /foo/triops/RIOT_App/RIOT/sys/stdio
make -C /foo/triops/RIOT_App/RIOT/sys/stdio_uart
make -C /foo/triops/RIOT_App/RIOT/sys/tsrb
   text    data     bss     dec     hex filename
   8850    1193    2628   12671    317f /foo/triops/RIOT_App/app/bin/hifive1b/triops_app.elf
```

4. Run RIOT within TRIOPS. This assumes that you already built TRIOPS using `cargo build`!
```sh
make term
```
