#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@raspberrypi
readonly TARGET_PATH=/home/pi/pi-world
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/pi-world

readonly PACKAGE=out

cargo build --release

rm -rf ${PACKAGE}
mkdir ${PACKAGE}
mv ${SOURCE_PATH} ./${PACKAGE}
cp -r configs ./${PACKAGE}
rsync -r ${PACKAGE}/ ${TARGET_HOST}:${TARGET_PATH}
