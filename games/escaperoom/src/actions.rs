extern crate rand;

use crate::profile::UserProfile;

use rand::prelude::*;

pub enum Round {
    Turn
}

impl Round {
    pub fn gen(outcome: Vec<&str>) -> &str {
        let rng: usize = rand::thread_rng().gen_range(0..=2);
        let output: &str = outcome[rng];
        output
        }
    pub fn search(gen: &str) {
        if gen == "positive" { println!("You find a lever.") };
        if gen == "negative" { println!("A monster approaches!") };
        if gen == "neutral" { println!("You don't see anything..") };
    }
    pub fn examine(mut plyr: UserProfile, gen: &str) {
        if gen == "positive" { println!("You find a clue!"); plyr.stats.clues += 1; }
        if gen == "negative" { println!("You get dizzy") }; 
        if gen == "neutral" { println!("Nothing happens.") };

    }
    pub fn ponder(gen: &str) {
        if gen == "positive" { println!("You get a brilliant idea!") };
        if gen == "negative" { println!("You fall asleep") };
        if gen == "neutral" { println!("The clock is ticking..") };
    } 

}
