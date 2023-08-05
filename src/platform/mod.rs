use crate::{FilterState, Todos};

#[cfg(feature = "desktop")]
pub mod desktop;
#[cfg(feature = "web")]
pub mod web;

pub trait Store {
    fn get(&self) -> Todos;
    fn set(&self, item: &Todos);
}

#[cfg(feature = "web")]
pub use web::get_store;

#[cfg(feature = "desktop")]
pub use desktop::get_store;

#[cfg(feature = "web")]
impl Default for FilterState {
    fn default() -> Self {
        let url = web_sys::window().unwrap().location().hash().unwrap();
        match url.as_str() {
            "#/active" => FilterState::Active,
            "#/completed" => FilterState::Completed,
            _ => FilterState::All,
        }
    }
}

#[cfg(feature = "desktop")]
impl Default for FilterState {
    fn default() -> Self {
        FilterState::All
    }
}

