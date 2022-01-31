use super::registered;
use super::registered::Registry;
use super::profile::UserProfile;
use super::profile;
use super::input;

pub fn getinput() -> String {
let mut re = String::new();
std::io::stdin().read_line(&mut re).unwrap();
re
}

pub fn returning(game_registry: Registry, re: String) -> Result<UserProfile, String> {
        match re.as_str() {
            // need an exit from this
            "y" => { loop {
                    println!("Enter username:");
                    let name = input::getinput();
                    let user = registered::Registry::retrieve(&game_registry, name);
                    if let Some(user) = user {
                    break Ok(user);
                    }
                    else { println!("try again") }
                    }
                    
 
                }
            "n" => {
                let user = profile::newuser(game_registry);
                println!("Registered!");
                println!("Your goal is to escape the room.");
                Ok(user)
                }
            _ => Err("invalid response".to_string())
}
}
