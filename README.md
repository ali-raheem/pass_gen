# pass_gen

Simple alphanumeric password generator in rust. Now at version 1.0 and stable.

### Building
Uses the cargo build system
```
$ cargo build --release
```
The resultant binary will be in ./target/release/ called pass_gen this should then be copied to a secure location, and root only writeable (to prevent tampering).

### Running
```
$ pass_gen 16
```
Will generate a 16 char long password, you may call it without a argument or a non-numeric one and it will default to 8 chars.
