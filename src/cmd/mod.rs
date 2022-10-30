use serenity::framework::standard::macros::{group};

pub mod verify_ticket;

use crate::cmd::verify_ticket::{VERIFY_COMMAND};

#[group]
#[commands(verify)]
pub struct General;
