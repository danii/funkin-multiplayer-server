Friday Night Funkin Multiplayer Protocol
========================================
This document is for, like, uber nerds, so if you aren't like ten thousand IQ, this document will probably confuse the heck out of you. Sorry if that sounds like some weird flex. Any way uh, I uh, let's get professional I guess?

This protocol is meant for each player, to be referred to as clients, to connect to a central server, discover rooms, connect, vote, play, and display game results. Each server is allowed to either host a single "room", or a "house", which consists of many "rooms".

Due to the nature of the game this protocol is for, it doesn't make much effort to combat cheating, and instead relies on the server or clients to spot cheaters. Most attempts to thwart cheating on a protocol level would be futile, as the protocol is server based, and the protocol and code is free.

Each room can either be one of three variations; public, private, and password protected. Public rooms are not created by anyone, and instead are instantiated, managed, and merged all under the server's will. Players in public rooms vote for the song of their choosing, and the most voted song is chosen by the server to be played. Private rooms are managed by an individual owner, which can pick a song to play. Private rooms are invite only, but may be converted into public servers. Password protected rooms are very similar to private rooms, but players must enter a password to enter. Password protected rooms are primarily intended for server's that are only hosting one room.

The protocol is based on different states of opperation, where a different set of messages are provided based on the state. Certain messages can change the protocol state under certain conditions.

<!-- todo
To prevent protocol errors when a client sends one message, right before it receives a message from the server that changes the protocol state, but was sent after the server initially sent that message, 
-->

Flavors
-------
This protocol contains two flavors, two ways of effectively using it. It may even be thought of four flavors, a pair of the two main flavors, encrypted, and a pair of the two main flavors, unencrypted.

### WebSocket Flavor
The WebSocket flavor of this protocol is used primarily within web browsers. The WebSocket is always initiated with HTTP or HTTPS first, and the protocol is switched over to WS or WSS via the HTTP 101 Switching Protocols status code.

Every packet must use the binary WebSocket frame. Each identified packet starts the frame with one byte of it's packet identifier, followed by the packet. Unidentified packets just contain itself with no other metadata. The length of each packet is conveyed by the WebSocket protocol.

### Raw Flavor
Work in progress! For now, pretend this doesn't exist?

Types
-----
This is a list of the types used within the packets within the protocol.

Work in progress!

### Lobby Event Enum
Based on a 1 Byte Numeric.

| Name        | Value |
| ----------- | ----- |
| Ready Start | 0x00  |
| Ready End   | 0x01  |
| Song Reveal | 0x02  |
| Game Start  | 0x03  |

#### Ready Start
This enum is only usable in the [Events Packet] if the lobby is public. All clients should show a count down in seconds to the event. Clients should be unable to ready before this point in time. Attempting to send a [Ready Packet] before this point in time is a protocol error.

#### Ready End
This sets a deadline for readying. All clients should show a count down in seconds to the event. If any client is not ready before the event, they **MUST** be disconnected from the room by the server. This enum may be used in any type of lobby.

#### Song Reveal
This sets a deadline for the song to be revealed. All clients should show some animation of a song being picked.
TODO

#### Game Start
This sets a deadline for the game to start. All clients should show a count down in seconds to the event. TODO

[Ready Packet]: #ready-packet
[Events Packet]: #events-packet

Packets
-------
### Packets Available Across Any State
#### Server Abilities Packet
This packet is not bound to any protocol state, and neither is it bound to any packet identifier; it is instead identified by being the first packet sent from a server.

##### Properties
| Property      | Value  |
| ------------- | ------ |
| Identifier    | *N/A*  |
| Bound To      | Client |
| Useable State | All    |
| Target State  | None   |

##### Definition
<table>
	<tr>
		<th colspan=3>Field Name</th>
		<th>Field Type</th>
		<th>Description</th>
	</tr>
	<tr>
		<td colspan=3>House</td>
		<td>1 Bit Boolean</td>
		<td>If set, this means the server is a house, otherwise this server is a room.</td>
	</tr>
	<tr>
		<td rowspan=4>Specialized Data</td>
		<th>House Value</th>
		<th>Field Name</th>
		<th colspan=2></th>
	</tr>
	<tr>
		<!---->
		<td>True</td>
		<td><i>Padding</i></td>
		<td>7 Bit Unused</td>
		<td></td>
	</tr>
	<tr>
		<!---->
		<td>False</td>
		<td>Room Type</td>
		<td>2 Bit Room Type</td>
		<td>The type of the room hosted on the server.</td>
	</tr>
	<tr>
		<!---->
		<td>False</td>
		<td><i>Padding</i></td>
		<td>5 Bit Unused</td>
		<td></td>
	</tr>
	<tr>
		<td colspan=3>Message Of The Day</td>
		<td>0 Bytes to 64 Bytes Raw String</td>
		<td>The server's message of the day. Length is determined by the packet's length.</td>
	</tr>
</table>

#### Ping Packet
The purpose of this packet is three fold.
1. This packet is used within the Raw flavor of the protocol the same way a ping frame is used within the WebSocket flavor of the protocol
1. Within browsers, there is no standard way to use WebSocket ping frames, so this comes as a backup
1. Finally, this packet is also used, in conjunction with the Pong Packet, as a basic keep alive mechanism
	- Note that WebSocket ping & pong frames can also serve this purpose

Therefore, it is important this packet be usable in any state.

This packet MUST be responded to by a pong packet, **NOT** a WebSocket pong frame. For more information on WebSocket ping & pong frames, read about the WebSocket flavor of this protocol.

##### Properties
| Property      | Value |
| ------------- | ----- |
| Identifier    | 0xFE  |
| Bound To      | Both  |
| Useable State | All   |
| Target State  | *N/A* |

##### Definition
| Field Name | Field Type         | Description                                                |
| ---------- | ------------------ | ---------------------------------------------------------- |
| Data       | Any Length Integer | Random data to identify this ping when received as a pong. |

#### Pong Packet
The purpose of this packet is three fold.
1. This packet is used within the Raw flavor of the protocol the same way a pong frame is used within the WebSocket flavor of the protocol
1. Within browsers, there is no standard way to use WebSocket pong frames, so this comes as a backup
1. Finally, this packet is also used, in conjunction with the Ping Packet, as a basic keep alive mechanism
	- Note that WebSocket ping & pong frames can also serve this purpose

Therefore, it is important this packet be usable in any state.

This packet MAY be in response to a ping packet, but **MUST NOT** be in response to a WebSocket ping frame. For more information on WebSocket ping & pong frames, read about the WebSocket flavor of this protocol.

##### Properties
| Property      | Value |
| ------------- | ----- |
| Identifier    | 0xFF  |
| Bound To      | Both  |
| Useable State | All   |
| Target State  | *N/A* |

##### Definition
| Field Name | Field Type         | Description                                                |
| ---------- | ------------------ | ---------------------------------------------------------- |
| Data       | Any Length Untyped | Random data to identify this ping when received as a pong. |

### Initial State
This is the initial state of the protocol, the state that is used after connecting. There isn't much to do within this state, other than to wait and retrieve the Server Abilities Packet, send or receive ping frames, pong frames, Ping Packets, and Pong Packets, to determine the delay between the client and server.

The protocol may be in this state for up to thirty seconds, afterwards the server MAY disconnect at it's own discretion. The client MAY disconnect at any time.

#### Next State Packet
This packet changes the protocol state depending on the server's Server Abilities Packet.

The following rules dictates the next state.
- If the server reports to host a house
	- and it reports to only host public rooms, then the next state is TODO,
	- otherwise, the next state is TODO.
- If the server reports to host a room
	- and that room is password protected, then the next state is TODO,
	- otherwise, the next state is the [Lobby State].

[Lobby State]: #lobby-state

##### Properties
| Property     | Value  |
| ------------ | ------ |
| Identifier   | 0x00   |
| Bound To     | Server |
| Target State | TODO   |

##### Definition
*This packet contains no data.*

### Lobby State
TODO

#### Ready Packet
Specifies whether or not the client is ready, and the song they pick, if the song is to be decided by voting.

##### Properties
| Property     | Value  |
| ------------ | ------ |
| Identifier   | 0x00   |
| Bound To     | Server |
| Target State | None   |

##### Definition
<table>
	<tr>
		<th colspan=2>Field Name</th>
		<th>Field Type</th>
		<th>Description</th>
	</tr>
	<tr>
		<td colspan=2>Ready</td>
		<td>1 Byte Boolean</td>
		<td>Whether the client is ready or not.</td>
	</tr>
	<tr>
		<td rowspan=3>Optional Data<br><i>Must not be specified if Ready is False.</i></td>
		<th>Field Name</th>
		<th colspan=2></th>
	</tr>
	<tr>
		<td>Difficulty</td>
		<td>1 Byte Difficulty</td>
		<td>The difficulty of the song.</td>
	</tr>
	<tr>
		<td>Name</td>
		<td>Any Length Raw String</td>
		<td>The name of the song.</td>
	</tr>
</table>

#### Events Packet
Notifies of an upcoming event in the lobby. A client should act as if the event is coming up, but the server is not required to start that event at the specified time.

##### Properties
| Property     | Value  |
| ------------ | ------ |
| Identifier   | 0x00   |
| Bound To     | Client |
| Target State | None   |

##### Definition
<table>
	<tr>
		<th colspan=2>Field Name</th>
		<th>Field Type</th>
		<th>Description</th>
	</tr>
	<tr>
		<td rowspan=3>Array</td>
		<th>Field Name</th>
		<th colspan=2></th>
	</tr>
	<tr>
		<td>Event Type</td>
		<td><a href="#lobby-event-enum">Lobby Event Enum</a></td>
		<td>The type of event to be notified for.</td>
	</tr>
	<tr>
		<td>Event Time</td>
		<td>Protocol Time</td>
		<td>The time of the event.</td>
	</tr>
</table>

<!--
ready start
ready end
song reveal
game start
-->

