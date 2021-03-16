FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.2.1

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes \
        libusb-1.0-0-dev:armhf \
        libudev-dev:armhf \
        libhidapi-dev:armhf

# https://github.com/rust-embedded/cross/issues/404
# https://github.com/rust-lang/pkg-config-rs
ENV PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig

# The pkg-config of libusb-1.0 points /usr/lib/arm-linux-gnueabihf/ but
# the pkg-config of libudeev points /lib/arm-linux-gnueabihf/ and this
# may cause ld failure like
#
#   warning: libudev.so.1, needed by /usr/lib/arm-linux-gnueabihf/libusb-1.0.so,
#   not found (try using -rpath or -rpath-link)
#
# To fix this issue, use -rpath-link to directly specify the directory
ENV RUSTFLAGS="-C link-arg=-Wl,-rpath-link,/lib/arm-linux-gnueabihf"
