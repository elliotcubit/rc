# rc

rc is a command line tool for converting text between encodings and applying transformations to them.

## Installation

```bash
$ git clone https://github.com/elliotcubit/rc.git
$ cd rc
$ cargo install --path .
```

## Usage

rc converts between encodings

```
$ rc --from hex --to utf8 68656c6c6f20726321
hello rc!
```

rc can guess what encoding your argument is, and will tell you its guess

```
$ rc --to utf8 68656c6c6f20726321
	[hex (inferred) ~> utf8]

hello rc!
```

rc outputs common formats when you don't tell it what to do

```
$ rc 68656c6c6f20726321
	[hex (inferred) ~> utf8, hex, base 64]

utf8: "hello rc!"
hex: "68656c6c6f20726321"
base 64: "aGVsbG8gcmMh"
```

rc outputs multiple specific formats

```
$ rc -t utf8 -t base64 68656c6c6f20726321
	[hex (inferred) ~> utf8, base 64]

utf8: "hello rc!"
base 64: "aGVsbG8gcmMh"
```

rc accepts data from stdin

```
$ nc 127.0.0.1 | rc
	[utf8 (inferred) ~> utf8, hex, base 64]

utf8: "hello network"
hex: "68656c6c6f206e6574776f726b"
base 64: "aGVsbG8gbmV0d29yaw=="
```

## Features

### Encodings

- [x] raw bytes
	- should only be an input format
- [x] hex
- [x] base64
- [x] utf8
- [ ] base32
- [x] binary
- [ ] ascii85
- [ ] url
- [ ] spelling alphabet

### Number bases

- [ ] 2
- [ ] 8
- [ ] 10
- [ ] 16

### Basic transforms

- [ ] reverse
- [ ] uppercase
- [ ] camelcase
- [ ] snakecase

### Ciphers

- [ ] caeser
- [ ] vigenère
- [ ] rot13
- [ ] substitution

### Crypto

- [ ] md5
- [ ] sha256

## License

MIT