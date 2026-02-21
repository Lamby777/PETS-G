# To use this, define GRCC_GODOT_RUST_LIB_NAME then include it.
# Example:
#
# ---8<----------------------
# GRCC_GAME_PKG_NAME=cctoy
# include grcc.mk
# ---8<----------------------
#
# This will build cctoy.dll, libcctoy.dylib, libcctoy.so
#
# More info on:
# https://github.com/ufoot/godot-rust-cross-compiler
#
# Updated for Godot 4 and GDExtension

.PHONY: grcc-all
.PHONY: grcc-test
.PHONY: grcc-debug
.PHONY: grcc-release
.PHONY: grcc-clean
.PHONY: grcc-doc
.PHONY: grcc-clean-prepare
.PHONY: grcc-lib-all
.PHONY: grcc-lib-windows
.PHONY: grcc-lib-windows-x64
.PHONY: grcc-lib-windows-arm64
.PHONY: grcc-lib-android
.PHONY: grcc-lib-android-arm64
.PHONY: grcc-lib-android-arm32
.PHONY: grcc-lib-android-x64
.PHONY: grcc-lib-android-x32
.PHONY: grcc-lib-macosx
.PHONY: grcc-lib-macosx-x64
.PHONY: grcc-lib-macosx-arm64
.PHONY: grcc-lib-linux
.PHONY: grcc-lib-linux-x64
.PHONY: grcc-lib-linux-arm64
.PHONY: grcc-lib-wasm
.PHONY: grcc-native
.PHONY: grcc-cross
.PHONY: grcc-copy-local
.PHONY: grcc-copy-if-exists
.PHONY: grcc-copy-all
.PHONY: grcc-copy-windows
.PHONY: grcc-copy-windows-x64
.PHONY: grcc-copy-windows-arm64
.PHONY: grcc-copy-android
.PHONY: grcc-copy-macosx
.PHONY: grcc-copy-linux
.PHONY: grcc-copy-linux-x64
.PHONY: grcc-copy-linux-arm64
.PHONY: grcc-copy-wasm
.PHONY: grcc-pkg-all
.PHONY: grcc-pkg-windows
.PHONY: grcc-pkg-windows-x64
.PHONY: grcc-pkg-windows-arm64
.PHONY: grcc-installer-windows
.PHONY: grcc-installer-windows-x64
.PHONY: grcc-installer-windows-arm64
.PHONY: grcc-pkg-android
.PHONY: grcc-pkg-macosx
.PHONY: grcc-pkg-linux
.PHONY: grcc-pkg-linux-x64
.PHONY: grcc-pkg-linux-arm64
.PHONY: grcc-pkg-wasm
.PHONY: grcc-pkg-source
.PHONY: grcc-dmg-macosx

grcc-all: grcc-native

grcc-native: grcc-test grcc-debug grcc-copy-local

grcc-cross: grcc-test grcc-lib-all grcc-copy-if-exists

grcc-export: grcc-test grcc-pkg-all grcc-installer-windows grcc-dmg-macosx

grcc-lib-all: grcc-lib-windows grcc-lib-android grcc-lib-macosx grcc-lib-linux grcc-lib-wasm

GRCC_WINDOWS_X64_TARGET=x86_64-pc-windows-gnullvm
GRCC_WINDOWS_ARM64_TARGET=aarch64-pc-windows-gnullvm
GRCC_ANDROID_ARM64_TARGET=aarch64-linux-android
GRCC_ANDROID_ARM32_TARGET=armv7-linux-androideabi
GRCC_ANDROID_X64_TARGET=x86_64-linux-android
GRCC_ANDROID_X32_TARGET=i686-linux-android
GRCC_MACOSX_X64_TARGET=x86_64-apple-darwin
GRCC_MACOSX_ARM64_TARGET=aarch64-apple-darwin
GRCC_LINUX_X64_TARGET=x86_64-unknown-linux-gnu
GRCC_LINUX_ARM64_TARGET=aarch64-unknown-linux-gnu
GRCC_WASM_TARGET=wasm32-unknown-unknown

# This must be defined
ifeq (,$(GRCC_GAME_PKG_NAME))
GRCC_GAME_PKG_NAME=please-define-GRCC_GAME_PKG_NAME
endif
# This should be defined, while not strictly mandatory, version matters.
ifeq (,$(GRCC_GAME_PKG_VERSION))
GRCC_GAME_PKG_VERSION=0.0.1
endif

# Default values provided for the following, based on package info.
ifeq (,$(GRCC_GODOT_RUST_LIB_NAME))
GRCC_GODOT_RUST_LIB_NAME=$(GRCC_GAME_PKG_NAME)
endif
ifeq (,$(GRCC_GAME_REPO_NAME))
GRCC_GAME_REPO_NAME=$(GRCC_GAME_PKG_NAME)
endif
ifeq (,$(GRCC_GAME_REPO_VERSION))
GRCC_GAME_REPO_VERSION=$(GRCC_GAME_PKG_VERSION)
endif

ifeq (,$(wildcard /opt/godot-rust-cross-compiler.txt))
GRCC_USE_DOCKER=yes
GRCC_INVOKE_DOCKER_RUST=install -d $(GRCC_CROSS_COMPILER_CACHE_DIR)/git && install -d $(GRCC_CROSS_COMPILER_CACHE_DIR)/registry && docker run -v $$(pwd):/build -v$$(realpath $(GRCC_CROSS_COMPILER_CACHE_DIR)/git):/root/.cargo/git -v$$(realpath $(GRCC_CROSS_COMPILER_CACHE_DIR)/registry):/root/.cargo/registry
GRCC_INVOKE_DOCKER_GODOT_EXPORT=docker run -v $$(pwd):/build ufoot/godot-rust-cross-compiler
else
GRCC_USE_DOCKER=no
GRCC_INVOKE_DOCKER_GODOT_EXPORT=
endif

# Detect host architecture for Android NDK limitation
# Google does not provide Android NDK toolchains for arm64 Linux hosts.
# Android builds require an x86_64 host (or x86_64 Docker image under Rosetta).
GRCC_HOST_ARCH := $(shell uname -m)
GRCC_HOST_IS_ARM64 := $(filter aarch64 arm64,$(GRCC_HOST_ARCH))

define GRCC_ANDROID_ARM64_HOST_ERROR

ERROR: Android builds are not supported on arm64 Linux hosts.

The Android NDK only provides toolchains for x86_64 Linux hosts.
Google does not ship arm64 Linux host toolchains.

Workarounds:
  1. Use an x86_64 Docker image: docker build --platform linux/amd64 ...
  2. Build on an x86_64 machine
  3. Use native macOS (NDK includes arm64 macOS support)

See: https://github.com/android/ndk/issues/1440

endef
export GRCC_ANDROID_ARM64_HOST_ERROR

.PHONY: grcc-check-android-host
grcc-check-android-host:
ifeq (yes,$(GRCC_USE_DOCKER))
ifneq (,$(GRCC_HOST_IS_ARM64))
	@echo "$$GRCC_ANDROID_ARM64_HOST_ERROR"
	@exit 1
endif
endif

GRCC_NATIVE_DEBUG_WINDOWS_SRC=./rust/target/debug/$(GRCC_GODOT_RUST_LIB_NAME).dll
GRCC_NATIVE_DEBUG_MACOSX_SRC=./rust/target/debug/lib$(GRCC_GODOT_RUST_LIB_NAME).dylib
GRCC_NATIVE_DEBUG_LINUX_SRC=./rust/target/debug/lib$(GRCC_GODOT_RUST_LIB_NAME).so

GRCC_GODOT_GDNATIVE_DIR=./pets-gd/gdnative
GRCC_WINDOWS_X64_SRC=./rust/target/$(GRCC_WINDOWS_X64_TARGET)/release/$(GRCC_GODOT_RUST_LIB_NAME).dll
GRCC_WINDOWS_X64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/windows/$(GRCC_WINDOWS_X64_TARGET)/
GRCC_WINDOWS_ARM64_SRC=./rust/target/$(GRCC_WINDOWS_ARM64_TARGET)/release/$(GRCC_GODOT_RUST_LIB_NAME).dll
GRCC_WINDOWS_ARM64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/windows/$(GRCC_WINDOWS_ARM64_TARGET)/
GRCC_ANDROID_ARM64_SRC=./rust/target/$(GRCC_ANDROID_ARM64_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).so
GRCC_ANDROID_ARM64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/android/$(GRCC_ANDROID_ARM64_TARGET)/
GRCC_ANDROID_ARM32_SRC=./rust/target/$(GRCC_ANDROID_ARM32_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).so
GRCC_ANDROID_ARM32_DST=$(GRCC_GODOT_GDNATIVE_DIR)/android/$(GRCC_ANDROID_ARM32_TARGET)/
GRCC_ANDROID_X64_SRC=./rust/target/$(GRCC_ANDROID_X64_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).so
GRCC_ANDROID_X64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/android/$(GRCC_ANDROID_X64_TARGET)/
GRCC_ANDROID_X32_SRC=./rust/target/$(GRCC_ANDROID_X32_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).so
GRCC_ANDROID_X32_DST=$(GRCC_GODOT_GDNATIVE_DIR)/android/$(GRCC_ANDROID_X32_TARGET)/
GRCC_MACOSX_X64_SRC=./rust/target/$(GRCC_MACOSX_X64_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).dylib
GRCC_MACOSX_X64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/macosx/$(GRCC_MACOSX_X64_TARGET)/
GRCC_MACOSX_ARM64_SRC=./rust/target/$(GRCC_MACOSX_ARM64_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).dylib
GRCC_MACOSX_ARM64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/macosx/$(GRCC_MACOSX_ARM64_TARGET)/
GRCC_LINUX_X64_SRC=./rust/target/$(GRCC_LINUX_X64_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).so
GRCC_LINUX_X64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/linux/$(GRCC_LINUX_X64_TARGET)/
GRCC_LINUX_ARM64_SRC=./rust/target/$(GRCC_LINUX_ARM64_TARGET)/release/lib$(GRCC_GODOT_RUST_LIB_NAME).so
GRCC_LINUX_ARM64_DST=$(GRCC_GODOT_GDNATIVE_DIR)/linux/$(GRCC_LINUX_ARM64_TARGET)/
GRCC_WASM_SRC=./rust/target/$(GRCC_WASM_TARGET)/release/$(GRCC_GODOT_RUST_LIB_NAME).wasm
GRCC_WASM_DST=$(GRCC_GODOT_GDNATIVE_DIR)/wasm/$(GRCC_WASM_TARGET)/

GRCC_CROSS_COMPILER_CACHE_DIR=target/cross-compiler-cache

GRCC_WINDOWS_MINGW_HEADERS=/opt/llvm-mingw/x86_64-w64-mingw32/include
GRCC_MACOSX_SDK_HEADERS=/opt/macosx-build-tools/cross-compiler/SDK/MacOSX14.5.sdk/usr/include
GRCC_MACOSX_SDK_CC_X64=/opt/macosx-build-tools/cross-compiler/bin/x86_64-apple-darwin23.5-clang
GRCC_MACOSX_SDK_CC_ARM64=/opt/macosx-build-tools/cross-compiler/bin/aarch64-apple-darwin23.5-clang

GRCC_EXPORT_DIR=export
GRCC_EXPORT_WINDOWS_X64_PKG=$(GRCC_GAME_PKG_NAME)-windows-x64-v$(GRCC_GAME_PKG_VERSION)
GRCC_EXPORT_WINDOWS_ARM64_PKG=$(GRCC_GAME_PKG_NAME)-windows-arm64-v$(GRCC_GAME_PKG_VERSION)
GRCC_EXPORT_ANDROID_PKG=$(GRCC_GAME_PKG_NAME)-android-v$(GRCC_GAME_PKG_VERSION)
GRCC_EXPORT_MACOSX_PKG=$(GRCC_GAME_PKG_NAME)-macosx-v$(GRCC_GAME_PKG_VERSION)
GRCC_EXPORT_LINUX_X64_PKG=$(GRCC_GAME_PKG_NAME)-linux-x64-v$(GRCC_GAME_PKG_VERSION)
GRCC_EXPORT_LINUX_ARM64_PKG=$(GRCC_GAME_PKG_NAME)-linux-arm64-v$(GRCC_GAME_PKG_VERSION)
GRCC_EXPORT_WASM_PKG=$(GRCC_GAME_PKG_NAME)-web-v$(GRCC_GAME_PKG_VERSION)

# Windows installer settings
GRCC_INSTALLER_TEMPLATE=/opt/grcc/installer.nsi.template
GRCC_INSTALLER_WINDOWS_X64=$(GRCC_GAME_PKG_NAME)-windows-x64-v$(GRCC_GAME_PKG_VERSION)-setup.exe
GRCC_INSTALLER_WINDOWS_ARM64=$(GRCC_GAME_PKG_NAME)-windows-arm64-v$(GRCC_GAME_PKG_VERSION)-setup.exe
GRCC_GAME_PUBLISHER?=Unknown Publisher

# macOS DMG settings
GRCC_DMG_MACOSX=$(GRCC_GAME_PKG_NAME)-macosx-v$(GRCC_GAME_PKG_VERSION).dmg
GRCC_DMG_VOLUME_NAME=$(GRCC_GAME_PKG_NAME)

# Godot 4 uses --headless instead of separate headless binary
GRCC_GODOT_HEADLESS=godot --headless

grcc-test:
	cd pets-lib && cargo test

grcc-debug:
	cd pets-lib && cargo build

grcc-release:
	cd pets-lib && cargo build --release

grcc-clean: grcc-clean-prepare
	rm -rf export

grcc-doc:
	cd pets-lib && cargo doc --workspace --offline

grcc-clean-prepare:
	rm -rf $(GRCC_GODOT_GDNATIVE_DIR)
	cd pets-lib && (cargo clean || rm -rf ./target || sudo rm -rf ./target)

grcc-lib-all: grcc-lib-windows grcc-lib-android grcc-lib-macosx grcc-lib-linux

grcc-lib-windows: grcc-lib-windows-x64 grcc-lib-windows-arm64

grcc-lib-windows-x64:
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) -e C_INCLUDE_PATH=$(GRCC_WINDOWS_MINGW_HEADERS) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_WINDOWS_X64_TARGET)
else
	export C_INCLUDE_PATH=$(GRCC_WINDOWS_MINGW_HEADERS) && cd pets-lib && cargo build --release --target $(GRCC_WINDOWS_X64_TARGET)
endif

grcc-lib-windows-arm64:
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_WINDOWS_ARM64_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_WINDOWS_ARM64_TARGET)
endif

grcc-lib-android: grcc-check-android-host grcc-lib-android-arm64 grcc-lib-android-arm32 grcc-lib-android-x64 grcc-lib-android-x32

grcc-lib-android-arm64: grcc-check-android-host
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_ANDROID_ARM64_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_ANDROID_ARM64_TARGET)
endif

grcc-lib-android-arm32: grcc-check-android-host
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_ANDROID_ARM32_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_ANDROID_ARM32_TARGET)
endif

grcc-lib-android-x64: grcc-check-android-host
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_ANDROID_X64_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_ANDROID_X64_TARGET)
endif

grcc-lib-android-x32: grcc-check-android-host
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_ANDROID_X32_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_ANDROID_X32_TARGET)
endif

grcc-lib-macosx: grcc-lib-macosx-x64 grcc-lib-macosx-arm64

grcc-lib-macosx-x64:
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) -e CC=$(GRCC_MACOSX_SDK_CC_X64) -e C_INCLUDE_PATH=$(GRCC_MACOSX_SDK_HEADERS) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_MACOSX_X64_TARGET)
else
	export CC=$(GRCC_MACOSX_SDK_CC_X64) C_INCLUDE_PATH=$(GRCC_MACOSX_SDK_HEADERS) && cd pets-lib && cargo build --release --target $(GRCC_MACOSX_X64_TARGET)
endif

grcc-lib-macosx-arm64:
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) -e CC=$(GRCC_MACOSX_SDK_CC_ARM64) -e C_INCLUDE_PATH=$(GRCC_MACOSX_SDK_HEADERS) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_MACOSX_ARM64_TARGET)
else
	export CC=$(GRCC_MACOSX_SDK_CC_ARM64) C_INCLUDE_PATH=$(GRCC_MACOSX_SDK_HEADERS) && cd pets-lib && cargo build --release --target $(GRCC_MACOSX_ARM64_TARGET)
endif

grcc-lib-linux: grcc-lib-linux-x64 grcc-lib-linux-arm64

grcc-lib-linux-x64:
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_LINUX_X64_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_LINUX_X64_TARGET)
endif

grcc-lib-linux-arm64:
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_LINUX_ARM64_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_LINUX_ARM64_TARGET)
endif

grcc-lib-wasm:
ifeq (yes,$(GRCC_USE_DOCKER))
	cd pets-lib && $(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler cargo build --release --target $(GRCC_WASM_TARGET)
else
	cd pets-lib && cargo build --release --target $(GRCC_WASM_TARGET)
endif
	# Optimize WASM with wasm-opt
ifeq (yes,$(GRCC_USE_DOCKER))
	$(GRCC_INVOKE_DOCKER_RUST) ufoot/godot-rust-cross-compiler wasm-opt -Os $(GRCC_WASM_SRC) -o $(GRCC_WASM_SRC)
else
	wasm-opt -Os $(GRCC_WASM_SRC) -o $(GRCC_WASM_SRC)
endif

grcc-copy-local:
	if (uname -a | grep -i windows) ; then install -d $(GRCC_WINDOWS_X64_DST) && cp $(GRCC_NATIVE_DEBUG_WINDOWS_SRC) $(GRCC_WINDOWS_X64_DST) ; fi
	if (uname -a | grep -i darwin) ; then \
		if (uname -m | grep -i arm64) ; then \
			install -d $(GRCC_MACOSX_ARM64_DST) && cp $(GRCC_NATIVE_DEBUG_MACOSX_SRC) $(GRCC_MACOSX_ARM64_DST) ; \
		else \
			install -d $(GRCC_MACOSX_X64_DST) && cp $(GRCC_NATIVE_DEBUG_MACOSX_SRC) $(GRCC_MACOSX_X64_DST) ; \
		fi \
	fi
	if (uname -a | grep -i linux) ; then install -d $(GRCC_LINUX_X64_DST) && cp $(GRCC_NATIVE_DEBUG_LINUX_SRC) $(GRCC_LINUX_X64_DST) ; fi

grcc-copy-if-exists:
	if test -f $(GRCC_WINDOWS_X64_SRC) ; then install -d $(GRCC_WINDOWS_X64_DST) && cp $(GRCC_WINDOWS_X64_SRC) $(GRCC_WINDOWS_X64_DST) ; fi
	if test -f $(GRCC_WINDOWS_ARM64_SRC) ; then install -d $(GRCC_WINDOWS_ARM64_DST) && cp $(GRCC_WINDOWS_ARM64_SRC) $(GRCC_WINDOWS_ARM64_DST) ; fi
	if test -f $(GRCC_ANDROID_ARM64_SRC) ; then install -d $(GRCC_ANDROID_ARM64_DST) && cp $(GRCC_ANDROID_ARM64_SRC) $(GRCC_ANDROID_ARM64_DST) ; fi
	if test -f $(GRCC_ANDROID_ARM32_SRC) ; then install -d $(GRCC_ANDROID_ARM32_DST) && cp $(GRCC_ANDROID_ARM32_SRC) $(GRCC_ANDROID_ARM32_DST) ; fi
	if test -f $(GRCC_ANDROID_X64_SRC) ; then install -d $(GRCC_ANDROID_X64_DST) && cp $(GRCC_ANDROID_X64_SRC) $(GRCC_ANDROID_X64_DST) ; fi
	if test -f $(GRCC_ANDROID_X32_SRC) ; then install -d $(GRCC_ANDROID_X32_DST) && cp $(GRCC_ANDROID_X32_SRC) $(GRCC_ANDROID_X32_DST) ; fi
	if test -f $(GRCC_MACOSX_X64_SRC) ; then install -d $(GRCC_MACOSX_X64_DST) && cp $(GRCC_MACOSX_X64_SRC) $(GRCC_MACOSX_X64_DST) ; fi
	if test -f $(GRCC_MACOSX_ARM64_SRC) ; then install -d $(GRCC_MACOSX_ARM64_DST) && cp $(GRCC_MACOSX_ARM64_SRC) $(GRCC_MACOSX_ARM64_DST) ; fi
	if test -f $(GRCC_LINUX_X64_SRC) ; then install -d $(GRCC_LINUX_X64_DST) && cp $(GRCC_LINUX_X64_SRC) $(GRCC_LINUX_X64_DST) ; fi
	if test -f $(GRCC_LINUX_ARM64_SRC) ; then install -d $(GRCC_LINUX_ARM64_DST) && cp $(GRCC_LINUX_ARM64_SRC) $(GRCC_LINUX_ARM64_DST) ; fi
	if test -f $(GRCC_WASM_SRC) ; then install -d $(GRCC_WASM_DST) && cp $(GRCC_WASM_SRC) $(GRCC_WASM_DST) ; fi

grcc-copy-all: grcc-copy-windows grcc-copy-android grcc-copy-macosx grcc-copy-linux grcc-copy-wasm

grcc-copy-windows: grcc-copy-windows-x64 grcc-copy-windows-arm64

grcc-copy-windows-x64: grcc-lib-windows-x64
	install -d $(GRCC_WINDOWS_X64_DST) && cp $(GRCC_WINDOWS_X64_SRC) $(GRCC_WINDOWS_X64_DST)

grcc-copy-windows-arm64: grcc-lib-windows-arm64
	install -d $(GRCC_WINDOWS_ARM64_DST) && cp $(GRCC_WINDOWS_ARM64_SRC) $(GRCC_WINDOWS_ARM64_DST)

grcc-copy-android: grcc-check-android-host grcc-lib-android-arm64 grcc-lib-android-arm32 grcc-lib-android-x64 grcc-lib-android-x32
	install -d $(GRCC_ANDROID_ARM64_DST) && cp $(GRCC_ANDROID_ARM64_SRC) $(GRCC_ANDROID_ARM64_DST)
	install -d $(GRCC_ANDROID_ARM32_DST) && cp $(GRCC_ANDROID_ARM32_SRC) $(GRCC_ANDROID_ARM32_DST)
	install -d $(GRCC_ANDROID_X64_DST) && cp $(GRCC_ANDROID_X64_SRC) $(GRCC_ANDROID_X64_DST)
	install -d $(GRCC_ANDROID_X32_DST) && cp $(GRCC_ANDROID_X32_SRC) $(GRCC_ANDROID_X32_DST)

grcc-copy-macosx: grcc-lib-macosx-x64 grcc-lib-macosx-arm64
	install -d $(GRCC_MACOSX_X64_DST) && cp $(GRCC_MACOSX_X64_SRC) $(GRCC_MACOSX_X64_DST)
	install -d $(GRCC_MACOSX_ARM64_DST) && cp $(GRCC_MACOSX_ARM64_SRC) $(GRCC_MACOSX_ARM64_DST)

grcc-copy-linux: grcc-copy-linux-x64 grcc-copy-linux-arm64

grcc-copy-linux-x64: grcc-lib-linux-x64
	install -d $(GRCC_LINUX_X64_DST) && cp $(GRCC_LINUX_X64_SRC) $(GRCC_LINUX_X64_DST)

grcc-copy-linux-arm64: grcc-lib-linux-arm64
	install -d $(GRCC_LINUX_ARM64_DST) && cp $(GRCC_LINUX_ARM64_SRC) $(GRCC_LINUX_ARM64_DST)

grcc-copy-wasm: grcc-lib-wasm
	install -d $(GRCC_WASM_DST) && cp $(GRCC_WASM_SRC) $(GRCC_WASM_DST)

grcc-pkg-all: grcc-pkg-windows grcc-pkg-android grcc-pkg-macosx grcc-pkg-linux grcc-pkg-wasm grcc-pkg-source

# [TODO] report this bug, need to launch the export twice for it to work, else complains about missing lib
GRCC_PKG_BUILDX2=pets-gd/buildx2.sh

# Godot 4 export preset names (architecture-specific presets must be defined in export_presets.cfg)
GRCC_EXPORT_PRESET_WINDOWS_X64=Windows Desktop x64
GRCC_EXPORT_PRESET_WINDOWS_ARM64=Windows Desktop arm64
GRCC_EXPORT_PRESET_ANDROID=Android
GRCC_EXPORT_PRESET_MACOSX=macOS
GRCC_EXPORT_PRESET_LINUX_X64=Linux x64
GRCC_EXPORT_PRESET_LINUX_ARM64=Linux arm64
GRCC_EXPORT_PRESET_WEB=Web

grcc-pkg-windows: grcc-pkg-windows-x64 grcc-pkg-windows-arm64

grcc-pkg-windows-x64: grcc-copy-windows-x64
	rm -f $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_X64_PKG).zip pets-gd/$(GRCC_GAME_PKG_NAME).exe pets-gd/$(GRCC_GODOT_RUST_LIB_NAME).dll
	echo 'for i in warmup real ; do $(GRCC_GODOT_HEADLESS) --path pets-gd --export-release "$(GRCC_EXPORT_PRESET_WINDOWS_X64)" $(GRCC_GAME_PKG_NAME).exe ; done' > $(GRCC_PKG_BUILDX2) && chmod a+x $(GRCC_PKG_BUILDX2) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_PKG_BUILDX2) && rm $(GRCC_PKG_BUILDX2)
	install -d $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_X64_PKG)
	mv pets-gd/$(GRCC_GAME_PKG_NAME).exe pets-gd/$(GRCC_GODOT_RUST_LIB_NAME).dll $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_X64_PKG)
	cd $(GRCC_EXPORT_DIR) && zip -r $(GRCC_EXPORT_WINDOWS_X64_PKG).zip $(GRCC_EXPORT_WINDOWS_X64_PKG) && rm -rf $(GRCC_EXPORT_WINDOWS_X64_PKG)

grcc-pkg-windows-arm64: grcc-copy-windows-arm64
	rm -f $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_ARM64_PKG).zip pets-gd/$(GRCC_GAME_PKG_NAME).exe pets-gd/$(GRCC_GODOT_RUST_LIB_NAME).dll
	echo 'for i in warmup real ; do $(GRCC_GODOT_HEADLESS) --path pets-gd --export-release "$(GRCC_EXPORT_PRESET_WINDOWS_ARM64)" $(GRCC_GAME_PKG_NAME).exe ; done' > $(GRCC_PKG_BUILDX2) && chmod a+x $(GRCC_PKG_BUILDX2) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_PKG_BUILDX2) && rm $(GRCC_PKG_BUILDX2)
	install -d $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_ARM64_PKG)
	mv pets-gd/$(GRCC_GAME_PKG_NAME).exe pets-gd/$(GRCC_GODOT_RUST_LIB_NAME).dll $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_ARM64_PKG)
	cd $(GRCC_EXPORT_DIR) && zip -r $(GRCC_EXPORT_WINDOWS_ARM64_PKG).zip $(GRCC_EXPORT_WINDOWS_ARM64_PKG) && rm -rf $(GRCC_EXPORT_WINDOWS_ARM64_PKG)

grcc-pkg-android: grcc-check-android-host grcc-copy-android
	rm -f $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_ANDROID_PKG).apk pets-gd/$(GRCC_EXPORT_ANDROID_PKG).apk
	echo 'for i in warmup real ; do $(GRCC_GODOT_HEADLESS) --path pets-gd --export-release "$(GRCC_EXPORT_PRESET_ANDROID)" $(GRCC_EXPORT_ANDROID_PKG).apk ; done' > $(GRCC_PKG_BUILDX2) && chmod a+x $(GRCC_PKG_BUILDX2) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_PKG_BUILDX2) && rm $(GRCC_PKG_BUILDX2)
	install -d $(GRCC_EXPORT_DIR) && mv pets-gd/$(GRCC_EXPORT_ANDROID_PKG).apk $(GRCC_EXPORT_DIR)

grcc-pkg-macosx: grcc-copy-macosx
	rm -f $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_MACOSX_PKG).zip pets-gd/$(GRCC_EXPORT_MACOSX_PKG).zip
	echo 'for i in warmup real ; do $(GRCC_GODOT_HEADLESS) --path pets-gd --export-release "$(GRCC_EXPORT_PRESET_MACOSX)" $(GRCC_EXPORT_MACOSX_PKG).zip ; done' > $(GRCC_PKG_BUILDX2) && chmod a+x $(GRCC_PKG_BUILDX2) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_PKG_BUILDX2) && rm $(GRCC_PKG_BUILDX2)
	install -d $(GRCC_EXPORT_DIR) && mv pets-gd/$(GRCC_EXPORT_MACOSX_PKG).zip $(GRCC_EXPORT_DIR)

grcc-pkg-linux: grcc-pkg-linux-x64 grcc-pkg-linux-arm64

grcc-pkg-linux-x64: grcc-copy-linux-x64
	rm -f $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_LINUX_X64_PKG).tar.gz pets-gd/$(GRCC_GAME_PKG_NAME) pets-gd/lib$(GRCC_GODOT_RUST_LIB_NAME).so
	echo 'for i in warmup real ; do $(GRCC_GODOT_HEADLESS) --path pets-gd --export-release "$(GRCC_EXPORT_PRESET_LINUX_X64)" $(GRCC_GAME_PKG_NAME) ; done' > $(GRCC_PKG_BUILDX2) && chmod a+x $(GRCC_PKG_BUILDX2) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_PKG_BUILDX2) && rm $(GRCC_PKG_BUILDX2)
	install -d $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_LINUX_X64_PKG)
	mv pets-gd/$(GRCC_GAME_PKG_NAME) pets-gd/lib$(GRCC_GODOT_RUST_LIB_NAME).so $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_LINUX_X64_PKG)
	cd $(GRCC_EXPORT_DIR) && tar czf $(GRCC_EXPORT_LINUX_X64_PKG).tar.gz $(GRCC_EXPORT_LINUX_X64_PKG) && rm -rf $(GRCC_EXPORT_LINUX_X64_PKG)

grcc-pkg-linux-arm64: grcc-copy-linux-arm64
	rm -f $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_LINUX_ARM64_PKG).tar.gz pets-gd/$(GRCC_GAME_PKG_NAME) pets-gd/lib$(GRCC_GODOT_RUST_LIB_NAME).so
	echo 'for i in warmup real ; do $(GRCC_GODOT_HEADLESS) --path pets-gd --export-release "$(GRCC_EXPORT_PRESET_LINUX_ARM64)" $(GRCC_GAME_PKG_NAME) ; done' > $(GRCC_PKG_BUILDX2) && chmod a+x $(GRCC_PKG_BUILDX2) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_PKG_BUILDX2) && rm $(GRCC_PKG_BUILDX2)
	install -d $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_LINUX_ARM64_PKG)
	mv pets-gd/$(GRCC_GAME_PKG_NAME) pets-gd/lib$(GRCC_GODOT_RUST_LIB_NAME).so $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_LINUX_ARM64_PKG)
	cd $(GRCC_EXPORT_DIR) && tar czf $(GRCC_EXPORT_LINUX_ARM64_PKG).tar.gz $(GRCC_EXPORT_LINUX_ARM64_PKG) && rm -rf $(GRCC_EXPORT_LINUX_ARM64_PKG)

grcc-pkg-wasm: grcc-copy-wasm
	rm -rf $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WASM_PKG) $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WASM_PKG).zip
	install -d $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WASM_PKG)
	echo 'for i in warmup real ; do $(GRCC_GODOT_HEADLESS) --path pets-gd --export-release "$(GRCC_EXPORT_PRESET_WEB)" $(GRCC_EXPORT_WASM_PKG)/index.html ; done' > $(GRCC_PKG_BUILDX2) && chmod a+x $(GRCC_PKG_BUILDX2) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_PKG_BUILDX2) && rm $(GRCC_PKG_BUILDX2)
	mv pets-gd/$(GRCC_EXPORT_WASM_PKG) $(GRCC_EXPORT_DIR)/
	cd $(GRCC_EXPORT_DIR) && zip -r $(GRCC_EXPORT_WASM_PKG).zip $(GRCC_EXPORT_WASM_PKG) && rm -rf $(GRCC_EXPORT_WASM_PKG)

grcc-pkg-source: .git/config grcc-clean-prepare
	export REPO="$$(grep url .git/config | head -n 1 | cut -d = -f 2)" && install -d $(GRCC_EXPORT_DIR) && rm -f $(GRCC_EXPORT_DIR)/$(GRCC_GAME_REPO_NAME).tar && tar cf $(GRCC_EXPORT_DIR)/$(GRCC_GAME_REPO_NAME).tar --exclude=.git --exclude=export --exclude=rust/target --exclude=pets-gd/.godot . && cd $(GRCC_EXPORT_DIR) && rm -rf $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION) && rm -f $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION).tar.gz $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION).zip && mkdir $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION) && cd $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION) && tar xf ../$(GRCC_GAME_REPO_NAME).tar && cd .. && rm $(GRCC_GAME_REPO_NAME).tar && tar czf $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION).tar.gz $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION) && zip -r $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION).zip $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION) && rm -rf $(GRCC_GAME_REPO_NAME)-$(GRCC_GAME_REPO_VERSION)

# Windows installers (NSIS)
# -------------------------

grcc-installer-windows: grcc-installer-windows-x64 grcc-installer-windows-arm64

GRCC_INSTALLER_BUILDSCRIPT=pets-gd/installer-build.sh

grcc-installer-windows-x64: grcc-pkg-windows-x64
	install -d $(GRCC_EXPORT_DIR)
	cd $(GRCC_EXPORT_DIR) && unzip -o $(GRCC_EXPORT_WINDOWS_X64_PKG).zip
	echo 'makensis -DGAME_NAME="$(GRCC_GAME_PKG_NAME)" \
		-DGAME_VERSION="$(GRCC_GAME_PKG_VERSION)" \
		-DGAME_PUBLISHER="$(GRCC_GAME_PUBLISHER)" \
		-DEXE_FILE="$(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_X64_PKG)/$(GRCC_GAME_PKG_NAME).exe" \
		-DEXE_NAME="$(GRCC_GAME_PKG_NAME).exe" \
		-DDLL_FILE="$(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_X64_PKG)/$(GRCC_GODOT_RUST_LIB_NAME).dll" \
		-DDLL_NAME="$(GRCC_GODOT_RUST_LIB_NAME).dll" \
		-DOUTPUT_FILE="$(GRCC_EXPORT_DIR)/$(GRCC_INSTALLER_WINDOWS_X64)" \
		$(GRCC_INSTALLER_TEMPLATE)' > $(GRCC_INSTALLER_BUILDSCRIPT) && chmod a+x $(GRCC_INSTALLER_BUILDSCRIPT) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_INSTALLER_BUILDSCRIPT) && rm $(GRCC_INSTALLER_BUILDSCRIPT)
	rm -rf $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_X64_PKG)

grcc-installer-windows-arm64: grcc-pkg-windows-arm64
	install -d $(GRCC_EXPORT_DIR)
	cd $(GRCC_EXPORT_DIR) && unzip -o $(GRCC_EXPORT_WINDOWS_ARM64_PKG).zip
	echo 'makensis -DGAME_NAME="$(GRCC_GAME_PKG_NAME)" \
		-DGAME_VERSION="$(GRCC_GAME_PKG_VERSION)" \
		-DGAME_PUBLISHER="$(GRCC_GAME_PUBLISHER)" \
		-DEXE_FILE="$(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_ARM64_PKG)/$(GRCC_GAME_PKG_NAME).exe" \
		-DEXE_NAME="$(GRCC_GAME_PKG_NAME).exe" \
		-DDLL_FILE="$(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_ARM64_PKG)/$(GRCC_GODOT_RUST_LIB_NAME).dll" \
		-DDLL_NAME="$(GRCC_GODOT_RUST_LIB_NAME).dll" \
		-DOUTPUT_FILE="$(GRCC_EXPORT_DIR)/$(GRCC_INSTALLER_WINDOWS_ARM64)" \
		$(GRCC_INSTALLER_TEMPLATE)' > $(GRCC_INSTALLER_BUILDSCRIPT) && chmod a+x $(GRCC_INSTALLER_BUILDSCRIPT) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_INSTALLER_BUILDSCRIPT) && rm $(GRCC_INSTALLER_BUILDSCRIPT)
	rm -rf $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_WINDOWS_ARM64_PKG)

# macOS DMG (disk image)
# ----------------------
# Creates a DMG from the macOS zip export
# Uses genisoimage to create hybrid ISO and libdmg-hfsplus to convert to DMG

GRCC_DMG_BUILDSCRIPT=pets-gd/dmg-build.sh

grcc-dmg-macosx: grcc-pkg-macosx
	install -d $(GRCC_EXPORT_DIR)
	rm -rf $(GRCC_EXPORT_DIR)/dmg-staging $(GRCC_EXPORT_DIR)/$(GRCC_DMG_MACOSX)
	mkdir -p $(GRCC_EXPORT_DIR)/dmg-staging
	cd $(GRCC_EXPORT_DIR)/dmg-staging && unzip -q ../$(GRCC_EXPORT_MACOSX_PKG).zip
	echo 'genisoimage -V "$(GRCC_DMG_VOLUME_NAME)" -D -R -apple -no-pad -o $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_MACOSX_PKG).cdr $(GRCC_EXPORT_DIR)/dmg-staging && \
		dmg $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_MACOSX_PKG).cdr $(GRCC_EXPORT_DIR)/$(GRCC_DMG_MACOSX)' > $(GRCC_DMG_BUILDSCRIPT) && chmod a+x $(GRCC_DMG_BUILDSCRIPT) && $(GRCC_INVOKE_DOCKER_GODOT_EXPORT) sh $(GRCC_DMG_BUILDSCRIPT) && rm $(GRCC_DMG_BUILDSCRIPT)
	rm -rf $(GRCC_EXPORT_DIR)/dmg-staging $(GRCC_EXPORT_DIR)/$(GRCC_EXPORT_MACOSX_PKG).cdr
