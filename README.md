To flash to the Micro:bit v2 you need to

install dependencies as seen here
https://docs.rust-embedded.org/discovery/microbit/03-setup/linux.html

rustup target add thumbv7em-none-eabihf
once. this just adds the architecture to the available targets

then just flash it with

cargo embed --target thumbv7em-none-eabihf 


