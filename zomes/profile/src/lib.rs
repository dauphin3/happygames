use hdk::prelude::*;

pub use interface;

pub mod profile;

pub use profile::*;

entry_defs![
    Anchor::entry_def(),
    Path::entry_def(),
    profile::UserProfile::entry_def()
];