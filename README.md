# Ctermderella
Bridge the terminal platform support like Cinderella!!

## Why?
I wanted to use the tui crate for [Cinderella](https://github.com/lovelyyfiaaa/cinderella). Well, unfortunately, Termion doesn't work on Windows, it was made primarily for Unix-like systems. Crossterm fills in that gap, by having a Windows backend.

But for me personally, Termion can be trusted more than Crossterm, since it was made by the Redux folks~!! I'm pretty sure everyone knows about Jeremy Soller, the maintainer for Termion. He's even a System76 employee, soo there are more trust to his works than say, someone that I have never met yet.

Additionally, Termion is in a stable release!! ^^

## Installa
Crossterm, since there isn't a great way to set default features based on the target!!

So if you would like to use this crate fully, you will have to enable the `termion` feature.

```toml
[target.'cfg(windows)'.dependencies]
ctermderella = { git = "https://github.com/lovelyyfiaaa/ctermderella.git" }

[target.'cfg(unix)'.dependencies]
ctermderella = { git = "https://github.com/lovelyyfiaaa/ctermderella.git", default-features = false, features = ["termion"]}
...
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
