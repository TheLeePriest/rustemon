mod pokemon_structs;

use reqwest;
use std::io;
use pokemon_structs::Pokemon;

fn get_pokemon_name() -> String {
    println!("Enter the name of the Pokemon you want to learn about:");
    let mut pokemon_name = String::new();
    io::stdin().read_line(&mut pokemon_name).unwrap();
    pokemon_name.trim().to_string()
}


async fn get_pokemon_data(pokemon_name: &String) -> Result<Pokemon, reqwest::Error> {
    println!("Getting data for: {}", pokemon_name);
    let base_api_url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name.to_lowercase());
    let pokemon_data = reqwest::get(&base_api_url).await?;
    let pokemon: Pokemon = pokemon_data.json().await?;

    Ok(pokemon)
}

#[tokio::main]
async fn main() {
    let pokemon_name = get_pokemon_name();
    println!("You entered: {}", pokemon_name);
    match get_pokemon_data(&pokemon_name).await {
        Ok(pokemon) => {
            println!("Pokemon data: {:#?}", pokemon);
        }
        Err(e) => {
            eprintln!("Error fetching Pokemon data: {}", e);
        }
    }
}
