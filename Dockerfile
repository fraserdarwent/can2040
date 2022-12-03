FROM rust:1.65.0-slim
RUN apt-get update
RUN apt-get install -y qemu-system-arm qemu-efi libclang-dev
RUN apt-get install -y git wget
RUN rustup component add rustfmt
ENV GCC_ARM_VERSION 10.3-2021.10
RUN wget "https://developer.arm.com/-/media/Files/downloads/gnu-rm/${GCC_ARM_VERSION}/gcc-arm-none-eabi-${GCC_ARM_VERSION}-x86_64-linux.tar.bz2"
RUN apt-get install -y bzip2
RUN tar -xf gcc-arm-none-eabi-${GCC_ARM_VERSION}-x86_64-linux.tar.bz2 -C /usr/share/
RUN ln -s /usr/share/gcc-arm-none-eabi-${GCC_ARM_VERSION}/bin/* /usr/bin/
RUN apt-get install -y build-essential cmake python3 # Required for building pico-sdk
