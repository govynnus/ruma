//! Matrix room name.

/// The name of a room.
///
/// It can't exceed 255 bytes or be empty.
#[repr(transparent)]
pub struct RoomName(str);

opaque_identifier_validated!(RoomName, ruma_identifiers_validation::room_name::validate);
