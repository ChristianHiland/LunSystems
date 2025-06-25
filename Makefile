-include .config

# Vars

# Kconfig

# The Kconfig tool for the menu interface.
KCONFIG_MCONF ?= kconfig-mconf
# THe Kconfig file
KCONFIG_FILE = Kconfig

# Other Vars
CURRENT_PATH := $(CURDIR)

#
# Config Conditions 
#

# Define a variable to hold extra, conditional targets, and targets.
EXTRA_TARGETS := folders Howling LunTool LibHowling LunSystems HowlingInstall LTT

# Default target: make -j 4
all: $(EXTRA_TARGETS)
ifeq ($(CONFIG_COMPILE_INTO_LUNPACK), y)
# Cleaning up compiled project...
	@echo "Cleaning up build files, and any compile time made files..."
	@sleep 3
	@rm -rf output/LunSystems/lib/*.d
# Download Imports
	@mkdir imports
	@echo "Downloading software for Lunpack..."
	@sleep 3
	@echo "[INFO] Downloading Gzip..."
	@cd imports && wget -v https://ftp.gnu.org/gnu/gzip/gzip-1.14.zip
	@echo "[INFO] Downloading Cpio..."
	@cd imports && wget -v https://ftp.gnu.org/gnu/cpio/cpio-latest.tar.gz
# Unpack imports
	@echo "[INFO] Unzipping Gzip..."
	@cd imports && unzip gzip-1.14.zip && cd gzip-1.14 && ./configure
	@cd imports/gzip-1.14/ && make -j 4
	@cd imports && gzip -dv cpio-latest.tar.gz && tar -xf cpio-latest.tar && cd cpio-2.15 && ./configure
	@cd imports/cpio-2.15/ && make -j 4
# Compacking and copying into the output/lunpack
	@echo "Compacting into a Lunpack..."
	@cd output/ && tar -czvf final.tar.gz LunSystems/
	@cd output/ && mv final.tar.gz final.lunpack
# Copying gzip into the Lunpack/tools folder
	@mkdir -p output/lunpack/tools/
	@cp imports/gzip-1.14/gzip $(CURRENT_PATH)/output/lunpack/tools/
	@cd output/ && mv final.lunpack lunpack/
	@cd src/bin/extract/ && cargo build && cp $(CURRENT_PATH)/src/bin/extract/target/debug/extract $(CURRENT_PATH)/output/lunpack/
	@echo "All files have been copied to the 'output/lunpack/' folder"
else
# Leave all files in the output folder without any Lunpack folder.
	@echo "Cleaning up build files, and any compile time made files..."
	@sleep 3
	@rm -rf output/LunSystems/lib/*.d
	@echo "All files have been copied to the 'output/' folder."
	@echo "Done!"
endif

# LunSystem Targets
folders:
	@mkdir -p output/bin && @mkdir -p output/lib
ifeq ($(CONFIG_HOWLINGOS_ENABLE), y)
	@mkdir -p output/LunSystems/HowlingOS/
endif
Howling:
ifeq ($(CONFIG_HOWLING_COMPILE), y)
	@echo "Compiling Howling"
	@cd src/bin/Howling && cargo build && cp $(CURRENT_PATH)/src/bin/Howling/target/debug/Howling $(CURRENT_PATH)/output/LunSystems/HowlingOS/
endif
LunTool:
ifeq ($(CONFIG_LIB_LUNTOOL_COMPILE), y)
	@echo "Compiling LunTool (Libary)"
	@cd src/lib/LunTool && cargo build && cp $(CURRENT_PATH)/src/lib/LunTool/target/debug/libLunTool.* $(CURRENT_PATH)/output/LunSystems/lib/
endif
LibHowling:
ifeq ($(CONFIG_LIB_HOWLING_COMPILE), y)
	@echo "Compiling Howling (Libary)"
	@cd src/lib/Howling && cargo build && cp $(CURRENT_PATH)/src/lib/Howling/target/debug/libHowling.* $(CURRENT_PATH)/output/LunSystems/HowlingOS/
endif
LunSystems:
	@echo "Compiling LunSystems"
	@cd src/bin/LunSystems && cargo build && cp $(CURRENT_PATH)/src/bin/LunSystems/target/debug/LunSystems $(CURRENT_PATH)/output/LunSystems/bin/
HowlingInstall:
ifeq ($(CONFIG_HOWLING_INSTALL_COMPILE), y)
	@echo "Compiling Howling Install"
	@cd src/bin/HowlingInstall && cargo build && cp $(CURRENT_PATH)/src/bin/HowlingInstall/target/debug/HowlingInstall $(CURRENT_PATH)/output/LunSystems/bin/
endif
LTT:
ifeq ($(CONFIG_LTT_COMPILE_ENABLE), y)
	@echo "Compiling LTT"
	@cd src/bin/LTT && cargo build && cp $(CURRENT_PATH)/src/bin/LTT/target/debug/LTT $(CURRENT_PATH)/output/LunSystems/bin/
endif

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
	@rm -rf output/lunpack

.PHONY: all clean menuconfig