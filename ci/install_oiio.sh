#!/bin/bash

set -eux

OIIO_VERSION="1.8.17"

# Avoid downloading if our cache included the untarred directory
if [[ ! -d "oiio-Release-${OIIO_VERSION}" ]]; then
  wget --continue "https://github.com/OpenImageIO/oiio/archive/Release-${OIIO_VERSION}.tar.gz"
  tar xf "Release-${OIIO_VERSION}.tar.gz"
fi
cd "oiio-Release-${OIIO_VERSION}"
mkdir -p build
cd build
cmake -DCMAKE_INSTALL_PREFIX=/usr ..
sudo make install

CI_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

sudo cp -f "${CI_DIR}"/openimageio.pc /usr/lib/pkgconfig/openimageio.pc
