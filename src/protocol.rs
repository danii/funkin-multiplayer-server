use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "opcode")]
pub enum RoomLogin<'i> {
	#[serde(rename = "0")]
	ClientInformation {
		username: &'i str
	},
	#[serde(rename = "1")]
	RoomInformation {
		users: Vec<&'i str>
	}
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "opcode")]
pub enum Lobby<'i> {
	#[serde(rename = "0")]
	ClientReady,
	#[serde(rename = "1")]
	UsersReadied {
		#[serde(borrow)]
		users: Vec<&'i str>
	},
	#[serde(rename = "2")]
	GameStart,
	#[serde(rename = "3")]
	UserJoin {
		user: &'i str
	},
	#[serde(rename = "4")]
	UserLeft {
		user: &'i str
	}
}