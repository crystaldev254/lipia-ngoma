
# DJ Event and Tip Management System

This project is a decentralized platform built for managing DJ events, song requests, tips, and user interactions. It includes features for user registration, event creation, playlist management, and rating systems. The system utilizes a stable data structure for efficient storage and retrieval of data.

## Description

The DJ Event and Tip Management System is designed to streamline the management of DJ events, song requests, and user tipping. Users can register, request songs, tip DJs, and rate their performance. DJs can manage their playlists and events, while administrators have control over user roles and the platform's functionality.

## Key Features

- **User Roles and Management**: Users can be assigned roles such as RegularUser, DJ, or Admin. Admins can manage all users, events, and system settings.
- **Song Requests**: Users can request songs to be played at specific events. DJs can view and manage song requests.
- **Tipping System**: Users can tip DJs, and tips are tracked for performance evaluation.
- **Event Management**: DJs can create and manage events, including details like venue, capacity, and schedule.
- **Playlists**: DJs can create and manage playlists associated with specific events.
- **Rating System**: Users can rate DJs after events, and ratings are used to generate a leaderboard of top-performing DJs.
- **Leaderboard**: A leaderboard showcases DJs based on their total tips and average ratings, helping users discover top-rated performers.

## Structs and Sample Payloads

### User Struct

```rust
struct User {
    id: u64,
    name: String,
    contact: String,
    created_at: u64,
    status: UserStatus,
    role: UserRole,
    points: u64,
}
```

#### Sample User Payload

```json
{
  "name": "John Doe",
  "contact": "123-456-7890",
  "role": "DJ"
}
```

### Song Request Struct

```rust
struct SongRequest {
    id: u64,
    user_id: u64,
    song_name: String,
    request_status: RequestStatus,
    created_at: u64,
}
```

#### Sample Song Request Payload

```json
{
  "user_id": 1,
  "song_name": "Song Title"
}
```

### Tip Struct

```rust
struct Tip {
    id: u64,
    user_id: u64,
    dj_name: String,
    amount: u64,
    tip_status: TipStatus,
    created_at: u64,
}
```

#### Sample Tip Payload

```json
{
  "user_id": 1,
  "dj_name": "DJ Example",
  "amount": 500
}
```

### Event Struct

```rust
struct Event {
    id: u64,
    event_name: String,
    dj_name: String,
    venue: String,
    capacity: u64,
    scheduled_at: u64,
    created_at: u64,
}
```

#### Sample Event Payload

```json
{
  "event_name": "New Year Party",
  "dj_name": "DJ Example",
  "venue": "City Hall",
  "capacity": 500,
  "scheduled_at": 17280000
}
```

### Playlist Struct

```rust
struct Playlist {
    id: u64,
    dj_name: String,
    event_id: u64,
    song_list: Vec<String>,
    created_at: u64,
}
```

#### Sample Playlist Payload

```json
{
  "dj_name": "DJ Example",
  "event_id": 1,
  "song_list": ["Song A", "Song B", "Song C"]
}
```

### Rating Struct

```rust
struct Rating {
    id: u64,
    user_id: u64,
    dj_name: String,
    rating: u8, // Rating out of 5
    review: String,
    created_at: u64,
}
```

#### Sample Rating Payload

```json
{
  "user_id": 1,
  "dj_name": "DJ Example",
  "rating": 5,
  "review": "Amazing performance!"
}
```

### Leaderboard Entry Struct

```rust
struct LeaderboardEntry {
    dj_name: String,
    total_tips: u64,
    total_ratings: u8,
    avg_rating: f64,
}
```

### Error Struct

```rust
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
    AlreadyExists { msg: String },
    Unauthorized { msg: String },
}

## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```


This project leverages stable memory structures to efficiently store and retrieve large amounts of data in a decentralized way. Each feature and struct ensures that data is properly stored and can be queried by various users, DJs, and administrators.