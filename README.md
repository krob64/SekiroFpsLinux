# Sekiro FPS Unlocker for Linux

A small cli tool that will unlock the framerate of sekiro on linux.

## Prerequisites

This tool requires [libmem 4](https://github.com/rdbo/libmem) to be installed on your system.

## Compiling and Running

1. Clone the repo
```
$ git clone https://github.com/krob64/sekirofpsunlocker
```

2. Compile the binary
```
$ cd sekirotool
$ cargo build --release
```

3. Start sekiro

4. Start the fps unlocker
```
$ cd target/release
$ ./sekirotool -mf 120
```
You can start it like that everytime, you can put the binary in your $PATH or you can do some steam launchoption magic. Whatever floats your boat.

# Credits

[uberhalit/SekiroFpsUnlockAndMore](https://github.com/uberhalit/SekiroFpsUnlockAndMore) for the signatures\
[rdbo/libmem](https://github.com/rdbo/libmem) for an excellent memory manipulation library
