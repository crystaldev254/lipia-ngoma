#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Enums for different features

#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum RequestStatus {
    #[default]
    Pending,
    Played,
}

#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum TipStatus {
    #[default]
    Pending,
    Completed,
}

#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserStatus {
    #[default]
    Active,
    Deactivated,
}

// Define roles for users
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserRole {
    #[default]
    RegularUser,
    Admin,
    DJ,
}

// User struct with roles
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    name: String,
    contact: String,
    created_at: u64,
    status: UserStatus,
    role: UserRole, // User role field
    points: u64,
}

// Song Request struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SongRequest {
    id: u64,
    user_id: u64,
    song_name: String,
    request_status: RequestStatus,
    created_at: u64,
}

// Tip struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Tip {
    id: u64,
    user_id: u64,
    dj_name: String,
    amount: u64,
    tip_status: TipStatus,
    created_at: u64,
}

// Event struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Event {
    id: u64,
    event_name: String,
    dj_name: String,
    venue: String,
    capacity: u64,
    scheduled_at: u64,
    created_at: u64,
}

// Playlist struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Playlist {
    id: u64,
    dj_name: String,
    event_id: u64,
    song_list: Vec<String>,
    created_at: u64,
}

// Rating struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Rating {
    id: u64,
    user_id: u64,
    dj_name: String,
    rating: u8, // Rating out of 5
    review: String,
    created_at: u64,
}

// Leaderboard struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct LeaderboardEntry {
    dj_name: String,
    total_tips: u64,
    total_ratings: u8,
    avg_rating: f64,
}

// Implement `Storable` and `BoundedStorable` for each struct

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, User).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 256;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for SongRequest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, SongRequest).unwrap()
    }
}

impl BoundedStorable for SongRequest {
    const MAX_SIZE: u32 = 256;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Tip {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Tip).unwrap()
    }
}

impl BoundedStorable for Tip {
    const MAX_SIZE: u32 = 256;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Event {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Event).unwrap()
    }
}

impl BoundedStorable for Event {
    const MAX_SIZE: u32 = 256;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Rating {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Rating).unwrap()
    }
}

impl BoundedStorable for Rating {
    const MAX_SIZE: u32 = 256;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Playlist {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, Playlist).unwrap()
    }
}

impl BoundedStorable for Playlist {
    const MAX_SIZE: u32 = 256;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for LeaderboardEntry {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes, LeaderboardEntry).unwrap()
    }
}

impl BoundedStorable for LeaderboardEntry {
    const MAX_SIZE: u32 = 256;
    const IS_FIXED_SIZE: bool = false;
}

// Initialize memory and storage

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static SONG_REQUESTS_STORAGE: RefCell<StableBTreeMap<u64, SongRequest, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static TIPS_STORAGE: RefCell<StableBTreeMap<u64, Tip, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static EVENTS_STORAGE: RefCell<StableBTreeMap<u64, Event, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static RATINGS_STORAGE: RefCell<StableBTreeMap<u64, Rating, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static PLAYLISTS_STORAGE: RefCell<StableBTreeMap<u64, Playlist, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static LEADERBOARD_STORAGE: RefCell<StableBTreeMap<u64, LeaderboardEntry, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
}

// Payloads
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    name: String,
    contact: String,
    role: UserRole, // Include role in payload
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct SongRequestPayload {
    user_id: u64,
    song_name: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct TipPayload {
    user_id: u64,
    dj_name: String,
    amount: u64,
}

// Event Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct EventPayload {
    event_name: String,
    dj_name: String,
    venue: String,
    capacity: u64,
    scheduled_at: u64,
}

// Rating Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct RatingPayload {
    user_id: u64,
    dj_name: String,
    rating: u8,
    review: String,
}

// Playlist Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct PlaylistPayload {
    dj_name: String,
    event_id: u64,
    song_list: Vec<String>,
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
    AlreadyExists { msg: String },
    Unauthorized { msg: String },
}

// CRUD Operations

#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, Error> {
    // Validate all the input fields
    if payload.name.is_empty()
        && payload.contact.is_empty()
        && payload.role == UserRole::RegularUser
    {
        return Err(Error::InvalidInput {
            msg: "Invalid input fields".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let user = User {
        id,
        name: payload.name,
        contact: payload.contact,
        created_at: time(),
        status: UserStatus::Active,
        role: payload.role,
        points: 0, // Initialize points to 0
    };

    USERS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(user.id, user.clone());
    });

    Ok(user)
}

// Create song request function
#[ic_cdk::update]
fn create_song_request(payload: SongRequestPayload) -> Result<SongRequest, Error> {
    // Validate all the input fields
    if payload.song_name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Invalid input fields".to_string(),
        });
    }

    // Validate the user ID
    if USERS_STORAGE.with(|storage| !storage.borrow().contains_key(&payload.user_id)) {
        return Err(Error::NotFound {
            msg: "User not found".to_string(),
        });
    }

    // Increment the ID counter
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let song_request = SongRequest {
        id,
        user_id: payload.user_id,
        song_name: payload.song_name,
        request_status: RequestStatus::Pending,
        created_at: time(),
    };

    SONG_REQUESTS_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(song_request.id, song_request.clone());
    });

    Ok(song_request)
}

#[ic_cdk::update]
fn create_tip(payload: TipPayload) -> Result<Tip, Error> {
    // Validate all the input fields
    if payload.amount == 0 && payload.dj_name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Invalid input fields".to_string(),
        });
    }

    // Validate the user ID
    if USERS_STORAGE.with(|storage| !storage.borrow().contains_key(&payload.user_id)) {
        return Err(Error::NotFound {
            msg: "User not found".to_string(),
        });
    }

    // Increment the ID counter
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let tip = Tip {
        id,
        user_id: payload.user_id,
        dj_name: payload.dj_name,
        amount: payload.amount,
        tip_status: TipStatus::Pending,
        created_at: time(),
    };

    TIPS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(tip.id, tip.clone());
    });

    Ok(tip)
}

// Earn points for users based on actions
#[ic_cdk::update]
fn update_user_points(user_id: u64, points: u64) -> bool {
    USERS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut user) = storage.get(&user_id) {
            user.points += points;
            storage.insert(user_id, user);
            return true;
        }
        false
    })
}

// Create event function
#[ic_cdk::update]
fn create_event(payload: EventPayload) -> Result<Event, Error> {
    // Validate all the input fields
    if payload.event_name.is_empty()
        && payload.dj_name.is_empty()
        && payload.venue.is_empty()
        && payload.capacity == 0
    {
        return Err(Error::InvalidInput {
            msg: "Invalid input fields".to_string(),
        });
    }

    // Increment the ID counter
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let event = Event {
        id,
        event_name: payload.event_name,
        dj_name: payload.dj_name,
        venue: payload.venue,
        capacity: payload.capacity,
        scheduled_at: payload.scheduled_at,
        created_at: time(),
    };

    EVENTS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(event.id, event.clone());
    });

    Ok(event)
}

// Retrieve all events and throw error incase there are no events
#[ic_cdk::query]
fn get_all_events() -> Result<Vec<Event>, Error> {
    EVENTS_STORAGE.with(|storage| {
        let storage = storage.borrow();
        let events: Vec<Event> = storage.iter().map(|(_, event)| event.clone()).collect();
        if events.is_empty() {
            Err(Error::NotFound {
                msg: "No events found".to_string(),
            })
        } else {
            Ok(events)
        }
    })
}

// Retrieve events by event name
#[ic_cdk::query]
fn get_event_by_name(event_name: String) -> Result<Event, Error> {
    EVENTS_STORAGE.with(|storage| {
        let storage = storage.borrow();
        for (_, event) in storage.iter() {
            if event.event_name == event_name {
                return Ok(event.clone());
            }
        }
        Err(Error::NotFound {
            msg: "Event not found".to_string(),
        })
    })
}

// Create rating function
#[ic_cdk::update]
fn create_rating(payload: RatingPayload) -> Result<Rating, Error> {
    // Validate all the input fields
    if payload.dj_name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Invalid input fields".to_string(),
        });
    }

    // Validate the user ID
    if USERS_STORAGE.with(|storage| !storage.borrow().contains_key(&payload.user_id)) {
        return Err(Error::NotFound {
            msg: "User not found".to_string(),
        });
    }

    // Increment the ID counter
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let rating = Rating {
        id,
        user_id: payload.user_id,
        dj_name: payload.dj_name,
        rating: payload.rating,
        review: payload.review,
        created_at: time(),
    };

    RATINGS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(rating.id, rating.clone());
    });

    Ok(rating)
}

// Create playlist function
#[ic_cdk::update]
fn create_playlist(payload: PlaylistPayload) -> Result<Playlist, Error> {
    // Validate all the input fields
    if payload.dj_name.is_empty() && payload.song_list.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Invalid input fields".to_string(),
        });
    }

    // Validate the event ID
    if EVENTS_STORAGE.with(|storage| !storage.borrow().contains_key(&payload.event_id)) {
        return Err(Error::NotFound {
            msg: "Event not found".to_string(),
        });
    }

    // Increment the ID counter
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let playlist = Playlist {
        id,
        dj_name: payload.dj_name,
        event_id: payload.event_id,
        song_list: payload.song_list,
        created_at: time(),
    };

    PLAYLISTS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(playlist.id, playlist.clone());
    });

    Ok(playlist)
}

// Function to retrieve playlists by DJ name
#[ic_cdk::query]
fn get_playlist_by_dj_name(dj_name: String) -> Result<Vec<Playlist>, String> {
    PLAYLISTS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let playlists: Vec<Playlist> = stable_btree_map
            .iter()
            .filter(|(_, playlist)| playlist.dj_name == dj_name)
            .map(|(_, playlist)| playlist.clone())
            .collect();
        if playlists.is_empty() {
            Err(format!("No playlists found for DJ: {}", dj_name))
        } else {
            Ok(playlists)
        }
    })
}

// Function to retrieve playlists by event ID
#[ic_cdk::query]
fn get_playlist_by_event_id(event_id: u64) -> Result<Vec<Playlist>, String> {
    PLAYLISTS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let playlists: Vec<Playlist> = stable_btree_map
            .iter()
            .filter(|(_, playlist)| playlist.event_id == event_id)
            .map(|(_, playlist)| playlist.clone())
            .collect();
        if playlists.is_empty() {
            Err(format!("No playlists found for event ID: {}", event_id))
        } else {
            Ok(playlists)
        }
    })
}

// Search DJs by genre, rating, or location
#[ic_cdk::query]
fn search_djs(_genre: String, rating: u8, _location: String) -> Vec<LeaderboardEntry> {
    LEADERBOARD_STORAGE.with(|storage| {
        let storage = storage.borrow();
        let mut djs = Vec::new();
        for (_, dj) in storage.iter() {
            if dj.avg_rating >= rating as f64 {
                djs.push(dj.clone());
            }
        }
        djs
    })
}

// need this to generate candid
ic_cdk::export_candid!();
