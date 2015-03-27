
pub use self::structs::Line;
pub use self::structs::Message;

pub use self::functions::connect_and_listen;
pub use self::functions::is_valid_channel_name;
pub use self::functions::make_reply;

pub type Subscriber = fn(&Message) -> Vec<String>;

mod structs;
mod functions;
