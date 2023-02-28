# TMDb API Wrapper&nbsp; ![Latest Version]
[Latest Version]: https://img.shields.io/badge/version-v0.1.0-important.svg

Rust wrapper for communicating with The Movie Database API.

Currently uses [tokio](https://docs.rs/tokio/latest/tokio/) and [reqwest](https://docs.rs/reqwest/latest/reqwest/), but that is likely to change. I don't see any reason this package won't work in WASM. I haven't tested it though.

## Usage
```rust
use tmdb::Tmdb;

const TMDB_API_KEY: &'static str = "mysupersecretapikey";

#[tokio::main]
async fn main() {
    // tmdb uses Rust's async/await. You don't have to use tokio,
    // but you will need an async runtime.

    let tmdb = Tmdb::create(TMDB_API_KEY);

    // Let's get a film and print its cast
    if let Ok(little_women) = tmdb.movies.get_movie(331482).await {
        let tagline = &little_women.tagline.as_ref().unwrap();
        let title = &little_women.title;
        let year = &little_women.release_date;

        println!("{title} (released: {year}) - {tagline}");
        println!("--------------------------------");

        if let Ok(credits) = &little_women.get_credits(&tmdb).await {
            credits.cast.iter().for_each(|actor| {
                let actor_name = &actor.name;
                let character = &actor.character;
                println!("{actor_name} | {character}");
            })
        }
    };
}
```

```
Little Women (released: 2019-12-25) - Own your story
--------------------------------
Saoirse Ronan | Josephine 'Jo' March
Florence Pugh | Amy March
Emma Watson | Margaret 'Meg' March
Eliza Scanlen | Elizabeth 'Beth' March
...
```
---
## License
Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.