![Build status](https://travis-ci.com/lilith645/Maat-Editor3D.svg?token=nw7eyDYfjBcSaxj1G3h7&branch=master)
# 3D editor for maat engine

### No longer maintained, a better option will be built later with new Maat-Graphics Library.

### Windows Specific Setup
### MacOs Specific Setup
### Linux Specific Setup

Use your package manager to install the required dev-tools and vulkan drivers

For example on ubuntu:
```
sudo apt-get install build-essential git python cmake libvulkan-dev vulkan-utils
```

### Notes for cross-compiling from Linux to windows

$ cargo build --target=x86_64-pc-windows-gnu --release

Need to include libstdc++-6.dll libgcc_s_seh-1.dll
from
/usr/lib/gcc/x86_64-w64-mingw32/8.3-win32

$ RUSTFLAGS='-C link-args=-static-libstdc++' cargo build --target=x86_64-pc-windows-gnu -v

## Donate

Lilith645 (Original and Current maintainer) [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/AoAkuma)

## Contributing

Contributions are welcome! Feel free to submit pull requests.

## License

Licensed under
 * MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
