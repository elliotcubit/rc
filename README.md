### rc

Use `rc` to perform encoding changes and text transforms easily.

```
# Specify encodings to convert from and to
$ rc --from hex --to ascii 68656c6c6f20726321
hello rc!

# Infer the existing encoding
$ rc -t ascii 68656c6c6f20726321
	[hex (inferred) ~> ascii]

hello rc!

# When not specified, output common encodings
$ rc 68656c6c6f20726321
	[hex (inferred) ~> ascii, hex, base 64]

ascii: "hello rc!"
hex: "68656c6c6f20726321"
base 64: "aGVsbG8gcmMh"

# Specify multiple output formats
$ rc -t ascii -t base64 68656c6c6f20726321
	[hex (inferred) ~> ascii, base 64]

ascii: "hello rc!"
base 64: "aGVsbG8gcmMh"
```

`rc` is in early stages and may change dramatically. It's missing _many_ features:

- Planned encodings are missing
- Performance improvements
- Number base conversions
- Crypto things
- Actual text transforms instead of encoding changes
- Array output
- Supporting escaped bytes in input (`echo -e`'s behavior)

```
# TODO
$ rc --array -e "\x05\x06"
{ 0x05, 0x06 }

# Different formats
$ rc --array --container "[]" --delimiter ""
[ 0x05 0x06 ]
```

- Separating output every n bytes / characters
- Reading from stdin
- Converting a stream instead of as a batch

Since this is mostly a learning project for Rust, performance will likely be _not great_.

### Supported formats

#### Encodings

- [x] raw bytes
- [x] 7-bit ascii
- [ ] integer
	- `414243 -> 65 66 67`
- [x] hex
- [x] base64
- [ ] base32
- [ ] binary
- [ ] ascii85
- [ ] utf8
- [ ] url
	- `like*this -> like%2athis`
- [ ] spelling alphabet
	- `LikeThis -> Lima India Kilo Echo Tango Hotel India Sierra`

#### Number bases

- [ ] 2
- [ ] 8
- [ ] 10
- [ ] 16

#### Basic transforms

- [ ] reverse
- [ ] uppercase
- [ ] camelcase
- [ ] snakecase

#### Ciphers

- [ ] caeser
- [ ] vigenère
- [ ] rot13
- [ ] substitution

#### Crypto

- [ ] md5
- [ ] sha256