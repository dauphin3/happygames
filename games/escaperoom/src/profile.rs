
use crate::registered;
use crate::registered::Registry;
use crate::input;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserProfile {
    pub username: String,
    password: String,
    pub stats: UserStats,

}

impl UserProfile {
     pub fn new(name: String, pass: String) -> UserProfile {
        UserProfile { username: name, password: pass, stats: UserStats {games: 0, clues: 0, wins: 0} }
        }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserStats {
    pub games: i8,
    pub clues: i8,
    pub wins: i8
}

impl UserStats {
    fn update(user: &mut UserStats, field: &str) {
        if field == "games" {
            user.games += 1;
        }
        if field == "clues" {
            user.clues += 1;
        }
        if field == "wins" {
            user.wins += 1;
        }
    }
}

pub fn newuser(game_registry: Registry) -> UserProfile {
    println!("What is your name?");
    let name = input::getinput();
    println!("Choose password:");
    let pass = input::getinput();
    let user = UserProfile::new(name.clone(), pass);
    registered::Registry::register(game_registry, name, user.clone());
    user
    }
   