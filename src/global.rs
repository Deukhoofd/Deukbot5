use serenity::model::id::UserId;

static mut OWNER_ID: UserId = UserId(0);

pub fn set_owner_id(id: UserId) {
    unsafe {
        OWNER_ID = id;
    }
}
pub fn owner_id() -> UserId {
    unsafe { OWNER_ID }
}
