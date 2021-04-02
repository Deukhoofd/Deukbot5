use serenity::client::Context;
use serenity::model::id::UserId;
use serenity::model::user::User;

static mut DEUKBOT_ID: UserId = UserId(0);
static mut SELF_USER: Option<User> = None;

pub fn set_deukbot_id(id: UserId) {
    unsafe {
        DEUKBOT_ID = id;
    }
}

pub fn deukbot_id() -> UserId {
    unsafe { DEUKBOT_ID }
}

pub async fn set_self_user(ctx: &Context) {
    unsafe {
        SELF_USER = Some(deukbot_id().to_user(ctx).await.unwrap());
    }
}

pub fn self_user<'a>() -> &'a Option<User> {
    unsafe { &SELF_USER }
}
