[![Build status](https://ci.appveyor.com/api/projects/status/4oxkxfu76oft3qkk?svg=true)](https://ci.appveyor.com/project/greaka/arcdps-bhud)
# Arcdps BHUD Integration

This is a plugin that uses the [Arcdps](https://www.deltaconnected.com/arcdps/) [Combat API](https://www.deltaconnected.com/arcdps/api/) and exposes
some of the data to [Blish HUD](https://github.com/dlamkins/BlishHUD).

# Installing

Download the binary from the [releases](https://github.com/greaka/arcdps-bhud/releases) page and extract the dll into your `bin64` folder.

If you are unsure which version you want to download, take the one with `msvc` in the name.

# Clone the repo

```powershell
git clone https://github.com/greaka/arcdps-bhud
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

# Limitations

Currently, these limitations are in place:

- It will start working 30s after detecting BHUD, otherwise bhud could bug out and stop accepting anything
- Due to a limit of messages per second (depending on PC, on mine it is around 250), this plugin cannot share everything with BHUD
