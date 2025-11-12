APPLICATION = triops_app
PROJECT_BASE ?= $(CURDIR)/../
RIOTBASE ?= $(PROJECT_BASE)/RIOT

BOARD=hifive1b
TERMPROG = ../../target/debug/rv
TERMFLAGS = $(ELFFILE)

USEMODULE += shell
USEMODULE += shell_cmds_default
USEMODULE += ps

include $(RIOTBASE)/Makefile.include
