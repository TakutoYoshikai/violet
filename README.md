# violet
violet is a program to show different messages every year.

### Usage
**encrypt message**
```bash
cargo run <PATH TO TEXT MESSAGE FILE> <KEY 32 ascii characters>
# It outputs the encrypted message. You can copy to clipboard.
```
**register the messages**
```rust
const MESSAGES: &'static [&'static str] = &[]; # You can paste the encrypted messages to here.
const START_TIME: &str = "2021-12-13-00-00-00-+0900"; 
const KEY: &str = ""; # the KEY 32 ascii characters.
```
**compile**
```bash
cargo build --release
```

**show messages**
```bash
./violet
```
### LICENSE
MIT LICENSE
