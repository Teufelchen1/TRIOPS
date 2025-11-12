# name of your application
APPLICATION = triops_app

# If no BOARD is found in the environment, use this default:
BOARD = hifive1b

# Comment this out to disable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
DEVELHELP ?= 1

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# Modules to include:
USEMODULE += shell
USEMODULE += shell_cmds_default
USEMODULE += ps

TERMPROG = ../../target/debug/triops
TERMFLAGS = $(ELFFILE)

PROJECT_BASE ?= $(CURDIR)/../
# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(PROJECT_BASE)/RIOT

include $(RIOTBASE)/Makefile.include
