Friday Night Funkin Competitive Multiplayer Mod
===============================================
> This is outdated.

This modification requires a websocket server to manage rooms.

Websocket Protocol Specification
--------------------------------
Default state is Room Login State.

### Room Login State
This protocol state is for getting the client's username.

#### Client Login Packet
> Server <= Client

Tells the server the client's username for use in results screen.

##### Definition
```ts
interface ClientLogin {
	opcode: "0";
	username: string; // [1, 32] characters.
}
```

##### Response
After the client sends this, the server then sends a Room Information Packet.

#### Room Information Packet
> Server => Client

Tells the client about all current room settings and users within the room.

##### Definition
```ts
interface RoomInformation {
	opcode: "1";

	/**
		All mentioned usernames within this list are the other users within the lobby.
	*/
	users: string[];
}
```

##### Response
After the server sends this, the protocol switches into the Lobby State.

#### Name Taken Packet
> Server => Client

Tells the client that the name they choose was taken. The client is allowed to send another Client Login Packet with a different name,

##### Definition
```ts
interface NameTakenPacket {
	opcode: "2";
}
```

### Lobby State
This protocol state is active while no game is active.

#### Client Ready Packet
> Server <= Client

Tells the server the client is ready.

##### Definition
```ts
interface ClientReady {
	opcode: "0";
}
```

##### Response
The typical response from the server to this packet is a Users Readied Update Packet, or a Game Start Packet.

#### Users Readied Update Packet
> Server => Client

Tells clients who is ready or not.

##### Definition
```ts
interface UsersReadiedUpdate {
	opcode: "1";

	/**
		All mentioned usernames within this list have readied.
	*/
	users: string[];
}
```

#### Game Start Packet
> Server => Client

Starts the game.

##### Defintion
```ts
interface GameStart {
	opcode: "2";
}
```

#### User Join Packet
> Server => Client

Notifies that a user has joined.

##### Definition
```ts
interface UserJoin {
	opcode: "3";

	/**
		The user that has joined.
	*/
	user: string;
}
```

#### User Leave Packet
> Server => Client

Notifies that a user has left.

##### Definition
```ts
interface UserLeave {
	opcode: "4";

	/**
		The user that has left.
	*/
	user: string;
}
```

### Play State
This protocol is active when the game is in the playing.

#### Client Score Update Packet
> Server <= Client

Updates the client's score.

##### Definition
```ts
interface ClientScoreUpdate {
	opcode: "0";

	/**
		The client's new score.
	*/
	score: number;

	/**
		The client's new health, between 0 and 2.
	*/
	health: number;
}
```

#### User Score Update Packet
> Server => Client

Tells the clients about the scores of another user.

##### Definition
```ts
interface UserScoreUpdate {
	opcode: "1";

	/**
		The user in question.
	*/
	user: string;

	/**
		The user's score.
	*/
	score: number;

	/**
		The user's health.
	*/
	health: number;
}
```

#### User Leave Packet
> Server => Client

Notifies that a user has left.

##### Definition
```ts
interface UserLeave {
	opcode: "2";

	/**
		The user that has left.
	*/
	user: string;
}
```
