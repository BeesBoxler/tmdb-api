use std::{collections::HashSet, process};
use std::env;

use api_demo::tmdb::{Person, Tmdb};
use dotenv;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().expect("No .env file was found.");
    let tmdb_api_key =
        dotenv::var("TMDB_API_KEY").expect("Environment variable TMDB_API_KEY was not found");
    let tmdb = Tmdb::create(&tmdb_api_key);

    let args:Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("\ncompare-movies requires two parameters.\n\tUsage: compare-movies <id-one> <id-two>");
        process::exit(1);
    }

    let id_a = String::from(&args[1]);
    let id_b = String::from(&args[2]);

    let movie_a = tmdb
        .get_movie(&id_a)
        .await
        .expect(&format!("Movie with ID {id_a} not found"));
    let movie_b = tmdb
        .get_movie(&id_b)
        .await
        .expect(&format!("Movie with ID {id_b} not found"));

    let movie_a_cast: HashSet<Person> = tmdb
        .get_credits(&id_a)
        .await
        .expect(&format!("Credits for movie {} not found.", movie_a.title))
        .cast
        .into_iter()
        .collect();
    let movie_b_cast: HashSet<Person> = tmdb
        .get_credits(&id_b)
        .await
        .expect(&format!("Credits for movie {} not found.", movie_b.title))
        .cast
        .into_iter()
        .collect();

    let intersect:Vec<&Person> = movie_a_cast.intersection(&movie_b_cast).collect();

    println!(
        "Two films selected are {} and {}",
        movie_a.title, movie_b.title
    );
    if intersect.len() > 0 {
        println!("Actors in both films:");
        intersect.into_iter().for_each(|person| println!("{}", person.name));
    } else {
        println!("There is no cast in common.");
    }
}
