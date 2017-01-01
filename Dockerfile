FROM ubuntu
MAINTAINER syu_cream
WORKDIR /opt

#
# Prepare build environment
#
RUN apt-get dist-upgrade
RUN apt-get -y update
RUN apt-get install -y       \
            git              \
            build-essential  \
            autoconf         \
            automake         \
            autotools-dev    \
            libtool          \
            pkg-config       \
            libssl-dev

#
# Prepare build environment for rust
#
RUN apt-get install -y       \
            curl             \
            sudo

RUN ldconfig

RUN curl -sf -L https://static.rust-lang.org/rustup.sh | sh

#
# Build and install latest fullock and k2hash
#
RUN git clone https://github.com/yahoojapan/k2hash.git
RUN cd k2hash &&                               \
    git submodule update --init --recursive && \
    cd fullock &&                              \
    ./autogen.sh &&                            \
    ./configure &&                             \
    make &&                                    \
    make install &&                            \
    cd ../ &&                                  \
    ./autogen.sh &&                            \
    ./configure &&                             \
    make &&                                    \
    make install

RUN ldconfig

#
# For development
#
RUN apt-get install -y      \
            gdb             \
            vim
