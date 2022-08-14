# Cinderella Terminal
Bridge the terminal platform support like Cinderella!!

## Why?
I wanted to use the tui crate for [Cinderella](https://github.com/lovelyyfiaaa/cinderella). Well, unfortunately, Termion doesn't work on Windows, it was made primarily for Unix-like systems. Crossterm fills in that gap, by having a Windows backend.

There is a common practice to not blindly pull in dependencies, as it can be a supply chain attack vector!

So I personally would prefer Termion to Crossterm, since it was made by the Redux folks~!! 

I'm pretty sure everyone knows about Jeremy Soller, the maintainer for Termion. He's even a System76 employee, soo there are more trust to his works than say, someone that I have never met yet.

This is nothing against Crossterm  and its developers <3

Crossterm will still be used on non-Unix platforms~!!

Additionally, Termion is in a stable release!! ^^

## Installation
By default, Cinderella Terminal uses Crossterm, since there isn't a great way to set default features based on the `cfg` target!!

So if you would like to use this crate fully, you will have to enable the `termion` feature on `cfg(unix)` platforms!!

```toml
[target.'cfg(windows)'.dependencies]
cinderella-terminal = { git = "https://github.com/lovelyyfiaaa/cinderella-terminal.git" }

[target.'cfg(unix)'.dependencies]
cinderella-terminal = { git = "https://github.com/lovelyyfiaaa/cinderella-terminal.git", default-features = false, features = ["termion"]}
...
```

## Usage

cinderella-terminal uses a backend pattern~!! There are three backends to choose: crossterm, termion, and auto, the latter is just a reexport of the formers!! If you wish to implement your own backend, there's a Backend trait in [`interface.rs`](src/interface.rs).

```rs
use cinderella_terminal::backend::auto::*;

fn main(){
    let terminal = Auto::init_terminal();

    
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
