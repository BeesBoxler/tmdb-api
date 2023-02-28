use dotenv;
use tmdb::Tmdb;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // tmdb uses Rust's async/await. You don't have to use tokio,
    // but you will need an async runtime.

    dotenv::dotenv().ok();

    let tmdb = Tmdb::create(&dotenv::var("TMDB_API_KEY").unwrap());

    // Let's get a film
    if let Ok(little_women) = tmdb.movies.get_movie(331482).await {
        let tagline = &little_women.tagline.as_ref().unwrap();
        let title = &little_women.title;
        let year = &little_women.release_date;

        println!("{title} (released: {year}) - {tagline}");
        println!("----------------------------------------------------");

        if let Ok(credits) = &little_women.get_credits(&tmdb).await {
            credits.cast.iter().for_each(|actor| {
                let actor_name = &actor.name;
                let character = &actor.character;
                println!("{actor_name} | {character}");
            })
        }
    };
}
