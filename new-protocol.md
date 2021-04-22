Friday Night Funkin Multiplayer Protocol
========================================
This document is for, like, uber nerds, so if you aren't like ten thousand IQ, this document will probably confuse the heck out of you. Sorry if that sounds like some weird flex. Any way uh, I uh, let's get professional I guess?

The key words "**MUST**", "**MUST NOT**", "**REQUIRED**", "**SHALL**", "**SHALL NOT**", "**SHOULD**", "**SHOULD NOT**", "**RECOMMENDED**", "**MAY**", and "**OPTIONAL**" in this document are to be interpreted as described in [RFC 2119].

This protocol is meant for each player, to be referred to as clients, to connect to a central server, discover rooms, connect, vote, play, and display game results. Each server is allowed to either host a single "room", or a "house", which consists of many "rooms".

Due to the nature of the game this protocol is for, it doesn't make much effort to combat cheating, and instead relies on the server or clients to spot cheaters. Most attempts to thwart cheating on a protocol level would be futile, as the protocol is server based, and the protocol and code is free.

Each room can either be one of three variations; public, private, and password protected. Public rooms are not created by anyone, and instead are instantiated, managed, and merged all under the server's will. Players in public rooms vote for the song of their choosing, and the most voted song is chosen by the server to be played. Private rooms are managed by an individual owner, which can pick a song to play. Private rooms are invite only, but may be converted into public servers. Password protected rooms are very similar to private rooms, but players must enter a password to enter. Password protected rooms are primarily intended for server's that are only hosting one room.

The protocol is based on different states of opperation, where a different set of messages are provided based on the state. Certain messages can change the protocol state under certain conditions.

[RFC 2119]: https://tools.ietf.org/html/rfc2119

<!-- todo
To prevent protocol errors when a client sends one message, right before it receives a message from the server that changes the protocol state, but was sent after the server initially sent that message, 
-->

Flavors
-------
This protocol contains two flavors, two ways of effectively using it. It may even be thought of four flavors, a pair of the two main flavors, encrypted, and a pair of the two main flavors, unencrypted.

### WebSocket Flavor
The WebSocket flavor of this protocol is used primarily within web browsers. The WebSocket **MUST** only be initiated with HTTP or HTTPS first, and the protocol be switched over to WS or WSS via the HTTP 101 Switching Protocols status code.

Every packet **MUST** use the binary WebSocket frame, except the [Ping](#ping-packet) & [Pong](#pong-packet) packets, which may be expressed through [Ping](https://tools.ietf.org/html/rfc6455#section-5.5.2) & [Pong](https://tools.ietf.org/html/rfc6455#section-5.5.3) frames.

Each packet **MUST** start with a [1 Byte Integer], representing it's identifier, unless that packet is unidentified. The data that follows after the identifier is the packet data itself.

Ping frames **MUST** only be responded by Pong frames, not Pong packets, and Ping packets **MUST** only be responded by Pong packets, not Pong frames.

[1 Byte Integer]: #integers

### Raw Flavor
Work in progress! For now, pretend this doesn't exist?

Types
-----
This is a list of the types used within the packets within the protocol. Each type may be prefixed by an additional length constraint, if the type supports it. Below is a list of valid length constraints.
- *N* Bit - A type **MUST** be exactly *N* bits in length.
- *N* Byte - A type **MUST** be exactly *N* octets in length.
- *X* *XUnit* To *Y* *YUnit* - A type **MUST** be anywhere from *X* *xunits* to *Y* *yunits* in length. *XUnit* and *YUnit* must be either *Bit* or *Byte*, where *Bit* references bits in length, and *Byte* represents octets in length.
- Any Length - A type may be any size in length, but it's size **MUST** be in units of octets. This bound is effectively equivalent to the bound "1 Byte To ∞ Byte".

### Integers
Integers are any numeric value, represented in big endian. Integers must have a length constraint, but their length constraint may be anything.

### Booleans
Booleans are either true, or false. Booleans are represented in big endian, and their lowest bit is the boolean value; a 1 means true, a 0 means false. The meaning of the other bits of the boolean are undefined, as they are never read, and may be any value, but they **SHOULD** all be 0. The only valid length constraints on booleans are "1 Byte" or "1 Bit". If there is no length constraint, they are to be 1 octet in length.

### Raw Strings
Raw Strings are a set of UTF-8 data. Unlike normal strings, they are not prefixed. Raw Strings may have any length constraint. If there is no length constraint, they are to have the same length constraint as "Any Length".

### Unuseds
Unused are just that, unused. Their values are undefined, and may be any value, but all of their bits **SHOULD** all be 0. Unuseds must have a length constraint, but their length constraint may be anything, except a range of sizes.

### Room Type Enum
This enum is derived from a [2 Bit Integer].
| Name               | Value |
| ------------------ | ----- |
| Public             | 0b00  |
| Private            | 0b01  |
| Password Protected | 0b11  |

[2 Bit Integer]: #integers

### Song Difficulty Enum
This enum is derived from a [1 Byte Integer].
| Name   | Value |
| ------ | ----- |
| Easy   | 0x00  |
| Normal | 0x01  |
| Hard   | 0x02  |

[1 Byte Integer]: #integers

### Lobby Event Enum
This enum is derived from a [1 Byte Integer].
| Name        | Value |
| ----------- | ----- |
| Ready Start | 0x00  |
| Ready End   | 0x01  |
| Song Reveal | 0x02  |
| Game Start  | 0x03  |

[1 Byte Integer]: #integers

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

### Protocol Time
Protocol Times are a [64 Bit Integer], that represents a time, in milliseconds, since the start of the connection.

[64 Bit Integer]: #integers

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
		<td><a href="#booleans">1 Bit Boolean</a></td>
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
		<td><a href="#unuseds">7 Bit Unused</a></td>
		<td></td>
	</tr>
	<tr>
		<!---->
		<td rowspan=2>False</td>
		<td>Room Type</td>
		<td><a href="#room-type-enum">Room Type Enum</a></td>
		<td>The type of the room hosted on the server.</td>
	</tr>
	<tr>
		<!---->
		<!---->
		<td><i>Padding</i></td>
		<td><a href="#unuseds">5 Bit Unused</a></td>
		<td></td>
	</tr>
	<tr>
		<td colspan=3>Message Of The Day</td>
		<td><a href="#raw-strings">0 Byte To 64 Byte Raw String</a></td>
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
| Field Name | Field Type           | Description                                                |
| ---------- | -------------------- | ---------------------------------------------------------- |
| Data       | [Any Length Integer] | Random data to identify this ping when received as a pong. |

[Any Length Integer]: #integers

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
| Field Name | Field Type           | Description                                                |
| ---------- | -------------------- | ---------------------------------------------------------- |
| Data       | [Any Length Integer] | Random data to identify this ping when received as a pong. |

[Any Length Integer]: #integers

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
		<td><a href="#booleans">1 Byte Boolean</a></td>
		<td>Whether the client is ready or not.</td>
	</tr>
	<tr>
		<td rowspan=3>Optional Data<br><i>Must not be specified if Ready is False.</i></td>
		<th>Field Name</th>
		<th colspan=2></th>
	</tr>
	<tr>
		<td>Song Difficulty</td>
		<td><a href="#song-difficulty-enum">Song Difficulty Enum</a></td>
		<td>The difficulty of the song.</td>
	</tr>
	<tr>
		<td>Song Name</td>
		<td><a href="#raw-strings">Any Length Raw String</a></td>
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
		<td><a href="#protocol-time">Protocol Time</a></td>
		<td>The time of the event.</td>
	</tr>
</table>

<!--
ready start
ready end
song reveal
game start
-->

