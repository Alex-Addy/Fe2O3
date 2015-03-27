mod id;
mod ping;
mod random;
mod fortune;
pub use self::id::id_module as id;
pub use self::ping::ping_module as ping;
pub use self::random::random_module as random;
pub use self::fortune::fortune_module as fortune;
