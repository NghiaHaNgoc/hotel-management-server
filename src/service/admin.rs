mod amenity;
mod type_room;
mod users;

pub use amenity::add_amenity::add_amenity;
pub use amenity::delete_amenity::delete_amenity;
pub use amenity::list_amenity::list_amenity;
pub use type_room::add_type_room::add_type_room;
pub use type_room::delete_type_room::delete_type_room;
pub use type_room::list_type_room::list_type_room;
pub use type_room::update_type_room::update_type_room;
pub use users::add_user::add_user;
pub use users::list_user::list_user;
pub use users::update_user::update_user;
