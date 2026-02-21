# Ignore this makefile, it only exists for CI purposes. (cross-compilation)
GRCC_GAME_PKG_NAME=PETS_2037
GRCC_GAME_PKG_VERSION=0.0.2
GRCC_GODOT_RUST_LIB_NAME=pets-lib
GRCC_GAME_REPO_NAME=PETS-G

include grcc.mk

all: grcc-all
test: grcc-test
clean: grcc-clean
native: grcc-native
cross: grcc-cross
export: grcc-export

