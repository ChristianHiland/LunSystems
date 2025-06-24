-include .config

# The Kconfig tool for the menu interface
KCONFIG_MCONF ?= kconfig-mconf
# The Kconfig file
KCONFIG_FILE = Kconfig
# Other Vars
CURRENT_PATH := $(CURDIR)
# Define a variable to hold extra, conditional targets.
EXTRA_TARGETS :=

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

ifeq ($(LTT_COMPILE_ENABLE), y)
	EXTRA_TARGETS += LTT
endif

EXTRA_TARGETS += LunSystems

# Default target: build the Rust project
all: $(EXTRA_TARGETS)
	@mkdir -p output/lib && mkdir -p output/bin
	@echo "Cleaning up build files, and extra compile cache..."
	@rm -rf output/lib/*.d
	
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
	@cd LunSystems && cargo clean
	@rm -f .config .config.old
	@rm -rf output
	@rm -rf src/lib/LunTool/target/
	@rm -rf src/bin/LunSystems/target/
	@rm -rf src/bin/HowlingInstall/target/
	@rm -rf src/bin/Howling/target/
	@rm -rf src/lib/Howling/target/

.PHONY: all clean menuconfig