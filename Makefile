-include .config

# The Kconfig tool for the menu interface.
KCONFIG_MCONF ?= kconfig-mconf
# THe Kconfig file
KCONFIG_FILE = Kconfig
# Other Vars
CURRENT_PATH := $(CURDIR)

#
# Config Condidtions 
#

# Define a variable to hold extra, conditional targets, and targets.
EXTRA_TARGETS :=
OUTPUT_TARGET :=
LUNPACK_DEPND := DOWLOAD_IMPORTS UNPACK_IMPORTS

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

# Default target: make -j 4
all: $(EXTRA_TARGETS) $(OUTPUT_TARGET)
	@echo "cleaning up build files, and any compile time made files..."
	@sleep 3
	@rm -rf output/LunSystems/lib/*.d
	@echo "All files have been copied to 'output' folder."
	@echo "Done!"


# LunSystem Targets
Howling:
	@echo "Compiling Howling"
	@cd src/bin/Howling && cargo build && cp $(CURRENT_PATH)/src/bin/Howling/target/debug/Howling $(CURRENT_PATH)/output/LunSystems/bin/
LunTool:
	@echo "Compiling LunTool (Libary)"
	@cd src/lib/LunTool && cargo build && cp $(CURRENT_PATH)/src/lib/LunTool/target/debug/libLunTool.* $(CURRENT_PATH)/output/LunSystems/lib/

LibHowling:
	@echo "Compiling Howling (Libary)"
	@cd src/lib/Howling && cargo build && cp $(CURRENT_PATH)/src/lib/Howling/target/debug/libHowling.* $(CURRENT_PATH)/output/LunSystems/lib/

LunSystems:
	@echo "Compiling LunSystems"
	@cd src/bin/LunSystems && cargo build && cp $(CURRENT_PATH)/src/bin/LunSystems/target/debug/LunSystems $(CURRENT_PATH)/output/LunSystems/bin/

HowlingInstall:
	@echo "Compiling Howling Install"
	@cd src/bin/HowlingInstall && cargo build && cp $(CURRENT_PATH)/src/bin/HowlingInstall/target/debug/HowlingInstall $(CURRENT_PATH)/output/LunSystems/bin/
LTT:
	@echo "Compiling LTT"
	@cd src/bin/LTT && cargo build && cp $(CURRENT_PATH)/src/bin/LTT/target/debug/LTT $(CURRENT_PATH)/output/LunSystems/bin/

# Lunpack Funcs

DOWLOAD_IMPORTS:
	@mkdir imports
	@echo "Downloading software for Lunpack..."
	@sleep 3
	@echo "[INFO] Downloading Gzip..."
	@cd imports && wget -v https://ftp.gnu.org/gnu/gzip/gzip-1.14.zip
	@echo "[INFO] Downloading Cpio..."
	@cd imports && wget -v https://ftp.gnu.org/gnu/cpio/cpio-latest.tar.gz
UNPACK_IMPORTS:
	@echo "[INFO] Unzipping Gzip..."
	@cd imports && unzip gzip-1.14.zip && cd gzip-1.14 && ./configure
	@cd imports/gzip-1.14/ && make -j 4
	@cd imports && gzip -dv cpio-latest.tar.gz && tar -xf cpio-latest.tar && cd cpio-2.15 && ./configure
	@cd imports/cpio-2.15/ && make -j 4

LUNPACK_COMPILE: $(LUNPACK_DEPND)
	@echo "Compacting into a Lunpack..."
	@cd output/ && tar -czvf final.tar.gz LunSystems/
	@cd output/ && mv final.tar.gz final.lunpack
	@mkdir -p output/lunpack/tools/
	@cp imports/gzip-1.14/gzip $(CURRENT_PATH)/output/lunpack/tools/
	@cd output/ && mv final.lunpack lunpack/ && cd ../src/bin/extract/ && cargo build && cp $(CURRENT_PATH)/src/bin/extract/target/debug/extract $(CURRENT_PATH)/output/lunpack/
	@echo "Compacted into 'output/lunpack'"


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
	@rm final.cpio.gz
	@rm -rf output/lunpack

.PHONY: all clean menuconfig