use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use std::collections::BTreeMap;

use profile;
use profile::profile::UserProfile;

pub fn login(pub_key: AgentPubKey, pass: &str, registry: BTreeMap<ID, UserProfile>) -> Result<UserProfile> {
        let pass = registry.get(&pub_key).password?;
        pass
}
