use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use std::collections::BTreeMap;



#[derive(Debug, Clone)]    
pub struct Registry {
    pub users: BTreeMap<String,UserProfile>
}

impl Registry {

    pub  fn init() -> Registry {
        let registry = Registry {
            users: BTreeMap::new()
        };
        registry
        }

    pub fn register(mut self, name: String, user: UserProfile) {
     self.users.insert(name, user);
    }

    pub fn retrieve(&self, username: String) -> Option<UserProfile> {
        if self.users.contains_key(&username) {
            let fetch = self.users[&username].clone();
            Some(fetch)
        }
        else {
            println!("No profile found");
            None
        }
    }
}

