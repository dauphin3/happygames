use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use crate::profile;
pub use crate::interface;



#[hdk_entry(id = "User", visibility = "public", required_validations = 10)] //10 is arbitray, high number is more distributed "secure"
#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
pub struct UserProfile {
    pub pub_key: AgentPubKey,
    // &str makes handling easier
    pub username: &str, // for now, can't change name or pass, will need to convert to String
    password: &str, 
    pub stats: UserStats,
}

impl UserAction for UserProfile {
    fn new(name: &str, pass: &str) -> UserProfile {
        UserProfile {
            pub_key: AgentPubKey::from(agent_info.agent_initial_pubkey),
            username: name,
            password: pass,
            stats: UserStats {games: 0, clues: 0, wins: 0, leader: Leader {score:0}}
        }
    }
}
pub struct UserStats {  // OPPORTUNITY: add game complexity with more stats
    pub games: u64,
    pub clues: u64,
    pub wins: u64,
    pub leader: Leader
}

pub struct Leader {
    score: u64,     // for now, score is total clues found. TODO: create matrix 
    //rank: Vec<UserProfile>,   // TODO: rank
    //top_ten: bool,
}

//store UserProfile in DHT, return it and associated entry hash
#[hdk_extern] 
fn new_profile(name: &str, pass: &str) -> ExternResult<(EntryHash, UserProfile), WasmError> {
    let agent_info = agent_info()?;

    let profile = UserProfile::new(name, pass);
    create_entry(&profile.clone())?;
    let address = hash_entry(&profile.clone())?;
    Ok((address, profile))
}

