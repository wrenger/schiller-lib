FROM rustembedded/cross:x86_64-pc-windows-gnu

RUN apt-get update && \
    apt-get install --assume-yes libclang-9-dev

# Include directory for bindgen
ENV CPATH=/usr/lib/gcc/x86_64-w64-mingw32/7.3-posix/include/
