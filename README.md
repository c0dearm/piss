# PISS

[![Rust](https://github.com/c0dearm/piss/workflows/Rust/badge.svg?branch=master)](https://github.com/c0dearm/piss/actions)
[![Crates](https://img.shields.io/crates/v/piss.svg)](https://crates.io/crates/piss)
[![Docs](https://docs.rs/piss/badge.svg)](https://docs.rs/piss)
[![Codecov](https://codecov.io/gh/c0dearm/piss/branch/master/graph/badge.svg)](https://codecov.io/gh/c0dearm/piss)
[![License](https://camo.githubusercontent.com/47069b7e06b64b608c692a8a7f40bc6915cf629c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f6c6963656e73652d417061636865322e302532464d49542d626c75652e737667)](https://github.com/c0dearm/piss/blob/master/COPYRIGHT)

**PISS (Picture Secret Steganography)** is a command line tool to encode/decode secrets into/from image files using LSB steganography.

Run `piss --help` for usage.

## Installation
```sh
cargo install piss
```

## Usage examples:
### Encode a secret into an image
```sh
piss encode samples/the-matrix.jpg samples/secret.txt samples/the-matrix-reloaded.png
```

Original image:

![alt text](https://github.com/c0dearm/piss/raw/master/samples/the-matrix.jpg "Original image")

Image with secret:

![alt text](https://github.com/c0dearm/piss/raw/master/samples/the-matrix-reloaded.png "Image with secret")

### Recover secret from image:
```sh
piss decode samples/the-matrix-reloaded.png samples/secret-reloaded.txt
```

### Miscelaneous
By default, PISS uses 2 bits per image byte to encode the secret, you can change this value if desired:
```sh
piss -b 4 encode samples/the-matrix.jpg samples/secret.txt samples/the-matrix-reloaded.png
```

Just remember to decode using the same number of bits, otherwise the output will be garbage:
```sh
piss -b 4 decode samples/the-matrix-reloaded.png samples/secret-reloaded.txt
```

## Important note
It is not recommended to encode secrets and save the output as `.jpg` as compression is performed and the secret is lost.

## Features

### Supports from 1 to 8 bits secret encoding
The user can specify the number of bits of the secret to use per image byte

### Change between image formats
It is possible to convert the image format during encoding, just set a different extension for the output.

# Contributing

If you find a vulnerability, bug or would like a new feature, [open a new issue](https://github.com/c0dearm/piss/issues/new).

To introduce your changes into the codebase, submit a Pull Request.

Many thanks!

# License

PISS is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
