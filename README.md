Extract Samsung Firmware
========================

[![release](https://img.shields.io/github/release/johnmave126/extract-samsung-firmware?style=flat-square&logo=github)](https://github.com/johnmave126/extract-samsung-firmware/releases/latest)
[![build status](https://img.shields.io/github/actions/workflow/status/johnmave126/extract-samsung-firmware/release.yml?style=flat-square&logo=github)](https://github.com/johnmave126/extract-samsung-firmware/actions)

A command line utility to extract Samsung SSD firmware updator from official iso images. Samsung is infamous for the awful Linux boot environment used by their firmware updators (read [this blog post](https://blog.quindorian.org/2021/05/firmware-update-samsung-ssd-in-linux.html/)). This utility automates the procedure in the blog post.

# Usage
```
Extract Samsung SSD firmware from the iso file

Usage: extract-samsung-firmware.exe [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>       Firmware iso file location
  -o, --output-dir <DIR>  Target directory to extract to [default: .]
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version
```

On Linux, an additional option `-e` is added to allow running the firmware updator directly (although not recommended).

# License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.