use std::env;
use std::process;

use tmdb::movies::credits::CastMember;
use tmdb::Tmdb;
use dotenv;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().expect("No .env file was found.");
    let tmdb_api_key =
        dotenv::var("TMDB_API_KEY").expect("Environment variable TMDB_API_KEY was not found");
    let tmdb = Tmdb::create(&tmdb_api_key);

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "\ncompare-movies requires two parameters.\n\n\tUsage: compare-movies <id-one> <id-two>"
        );
        process::exit(1);
    }

    let id_a = String::from(&args[1]);
    let id_b = String::from(&args[2]);

    let movie_a = tmdb
        .movies
        .get_movie(id_a.parse().unwrap())
        .await
        .expect(&format!("Movie with ID {id_a} not found"));
    let movie_b = tmdb
        .movies
        .get_movie(id_b.parse().unwrap())
        .await
        .expect(&format!("Movie with ID {id_b} not found"));

    let movie_a_cast = movie_a.get_credits(&tmdb).await.unwrap();
    let movie_b_cast = movie_b.get_credits(&tmdb).await.unwrap();

    let mutual_cast = movie_a_cast
        .cast
        .iter()
        .filter(|c| {
            !!&movie_b_cast
                .cast
                .binary_search_by(|f| f.id.cmp(&c.id))
                .is_ok()
        })
        .collect::<Vec<&CastMember>>();

    println!(
        "Two films selected are {} and {}",
        movie_a.title, movie_b.title
    );

    dbg!(tmdb.movies.get_images(550).await.ok());

    if mutual_cast.len() > 0 {
        println!("Actors in both films:");
        mutual_cast
            .into_iter()
            .for_each(|person| println!("{}", person.name));
    } else {
        println!("There is no cast in common.");
    }
}
