pub mod context;
pub mod error;
pub mod prelude;
pub mod types;
pub mod websocket;
pub mod client;

mod discord_protos {
    pub(crate) mod discord_users {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/discord_protos.discord_users.v1.preloaded_user_settings.rs"));            
        }
    }
}

pub use discord_protos::discord_users::v1 as discord_proto;
