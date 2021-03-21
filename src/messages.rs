use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "opcode")]
pub enum LoginState<'i> {
	#[serde(rename = "0")]
	ClientLogin {
		username: &'i str
	},
	#[serde(rename = "1")]
	RoomInformation {
		users: Vec<&'i str>
	}
}
