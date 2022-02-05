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
cp ./target/release/violet /path/to/other/dir
```

**show messages**
```bash
./violet
```

### Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (git checkout -b feature/AmazingFeature)
3. Commit your Changes (git commit -m 'Add some AmazingFeature')
4. Push to the Branch (git push origin feature/AmazingFeature)
5. Open a Pull Request

### LICENSE
MIT LICENSE
