//! Application lifecycle and state management

pub mod application;
pub mod lifecycle;
pub mod state;

pub use application::Application;
pub use lifecycle::Lifecycle;
pub use state::AppState;
