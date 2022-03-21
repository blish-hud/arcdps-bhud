# Arcdps BHUD Integration

This is a plugin that uses the [Arcdps](https://www.deltaconnected.com/arcdps/) [Combat API](https://www.deltaconnected.com/arcdps/api/) and exposes
some of the data to [Blish HUD](https://github.com/blish-hud/BlishHUD).

This fork has been modified to:

1. Make an effort to parse the events on the server 
2. Return the data as a protobuf object, for easier consumption
3. Use a static TCP port, which is less secure but a lot easier to work with

Note: I say "make an effort" becuase the structure is somewhat complicated and not well
documented. Some of the parsing is guesswork, and some of the values mean different things at
different times. I've tried normalizing it and create new properties for the special values, but no guarantee. 

If you find a wrong parsing and figure out what it should be, please open an issue or PR.

# Installing

Download the binary from the [releases](https://github.com/theterrasque/arcdps-bhud/releases) page and extract the dll into your `bin64` folder.

# Reading realtime data

When loaded in GW2 this will open a TCP port at `localhost:12112`

When it's sending data the first 4 bytes (Little endian encoded!) will be the length of the data, then the rest will
be a [protobuf](https://developers.google.com/protocol-buffers/) encoded Event object. Please look at protobuf/eventdata.proto for details

## Python example code

You can find a very basic python client in the examples folder

# Clone the repo

```powershell
git clone https://github.com/blish-hud/arcdps-bhud
```

# Build it

You need a somewhat recent `rust` version. I didn't check the minimum version. arcdps-bhud is built against the latest `stable` channel.

1. Install `rustc`. For example via [rustup](https://rustup.rs/).
2. Install `cargo`. This is not necessary if you used [rustup](https://rustup.rs/).
3. Build it:
```powershell
cargo build --release
```
4. Copy it into your bin64 folder of Guild Wars 2:
```powershell
Copy-Item "C:\Git\arcdps-bhud\target\release\arcdps_bhud.dll" -Destination "A:\Guild Wars 2\bin64"
```
5. Start Gw2

# Contributing

Open an issue or a PR!

Please check first if you agree with the license.

