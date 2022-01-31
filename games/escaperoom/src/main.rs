
mod input;
mod profile;
mod registered;
mod actions;

use crate::input::*;
use profile::UserProfile;
use registered::Registry;
use actions::Round;

use std::collections::BTreeMap;

fn main() {
let game_registry: Registry = registered::Registry::init();  
let mut user_list: BTreeMap<String, UserProfile> = BTreeMap::new();
let outcome = vec!["positive", "negative", "neutral",];

println!("Welcome to the Escape Room");
println!("Returning? y/n");
let mut re = input::getinput();
let result = returning(game_registry.clone(), re.clone());
let current_user: UserProfile = match result {
    Ok(result) => result,
    Err(result) => loop {
        let result = returning(game_registry.clone(), re.clone());
        match result {
            Ok(result) => { result; }
            Err(result) => { continue; }
        }
    }
};

println!("The room looks empty. You can see the walls but no obvious doors or windows./n
What do you do first?/n
a: yell for help
b: walk to a wall
c: examine the ceiling
d: stop and think/n");
let choice = input::getinput();
let select = actions::Round::gen(outcome);
    match choice.as_str() {
        "a" => println!("no one hears you."),
        "b" => actions::Round::search(select),
        "c" => actions::Round::examine(current_user, select),
        "d" => actions::Round::ponder(select),
        _ => println!("invalid response")
    }


}
