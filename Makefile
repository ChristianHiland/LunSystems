-include .config

# The Kconfig tool for the menu interface
KCONFIG_MCONF ?= kconfig-mconf
# The Kconfig file
KCONFIG_FILE = Kconfig
# Other Vars
CURRENT_PATH := $(CURDIR)

#
# Config Condidtions 
#

# Define a variable to hold extra, conditional targets.
EXTRA_TARGETS :=
OUTPUT_TARGET :=

ifeq ($(CONFIG_HOWLING_INSTALL_COMPILE), y)
	EXTRA_TARGETS += HowlingInstall
endif

ifeq ($(CONFIG_HOWLING_COMPILE), y)
	EXTRA_TARGETS += Howling
endif

ifeq ($(CONFIG_LIB_HOWLING_COMPILE), y)
	EXTRA_TARGETS += LibHowling
endif

ifeq ($(CONFIG_LIB_LUNTOOL_COMPILE), y)
	EXTRA_TARGETS += LunTool
endif

ifeq ($(CONFIG_LTT_COMPILE_ENABLE), y)
	EXTRA_TARGETS += LTT
endif

ifeq ($(CONFIG_COMPILE_INTO_LUNPACK), y)
	OUTPUT_TARGET += LUNPACK_COMPILE
endif


EXTRA_TARGETS += LunSystems

# Default target: build the Rust project
all: $(EXTRA_TARGETS) $(OUTPUT_TARGET)
	@mkdir -p output/lib && mkdir -p output/bin
	@echo "Cleaning up build files, and extra compile cache..."
	@rm -rf output/lib/*.d
	@echo "Done! All files have been copied to the 'output' folder"
	
# LunSystem Targets
Howling:
	@echo "Compiling Howling"
	@cd src/bin/Howling && cargo build && cp $(CURRENT_PATH)/src/bin/Howling/target/debug/Howling $(CURRENT_PATH)/output/bin/
LunTool:
	@echo "Compiling LunTool (Libary)"
	@cd src/lib/LunTool && cargo build && cp $(CURRENT_PATH)/src/lib/LunTool/target/debug/libLunTool.* $(CURRENT_PATH)/output/lib/

LibHowling:
	@echo "Compiling Howling (Libary)"
	@cd src/lib/Howling && cargo build && cp $(CURRENT_PATH)/src/lib/Howling/target/debug/libHowling.* $(CURRENT_PATH)/output/lib/

LunSystems:
	@echo "Compiling LunSystems"
	@cd src/bin/LunSystems && cargo build && cp $(CURRENT_PATH)/src/bin/LunSystems/target/debug/LunSystems $(CURRENT_PATH)/output/bin/

HowlingInstall:
	@echo "Compiling Howling Install"
	@cd src/bin/HowlingInstall && cargo build && cp $(CURRENT_PATH)/src/bin/HowlingInstall/target/debug/HowlingInstall $(CURRENT_PATH)/output/bin/
LTT:
	@echo "Compiling LTT"
	@cd src/bin/LTT && cargo build && cp $(CURRENT_PATH)/src/bin/LTT/target/debug/LTT $(CURRENT_PATH)/output/bin/


# Target to run the configuration menu
menuconfig:
	@$(KCONFIG_MCONF) $(KCONFIG_FILE)
	@echo "\nConfiguration saved in .config. Run 'make'"

# Target to clean up Cargo and Kconfig artifacts
clean:
	@rm -f .config .config.old
	@rm -rf output
	@cd src/lib/LunTool/ && cargo clean
	@cd src/bin/LunSystems/ && cargo clean
	@cd src/bin/HowlingInstall/ && cargo clean
	@cd src/bin/Howling/ && cargo clean
	@cd src/lib/Howling/ && cargo clean

.PHONY: all clean menuconfig