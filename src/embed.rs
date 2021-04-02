use serenity::builder::CreateEmbed;
use serenity::utils::Color;

pub fn set_default_style(e: &mut CreateEmbed) {
    let user = crate::global::self_user().as_ref().unwrap();

    e.author(|a| {
        a.name(user.name.to_string());
        a.icon_url(user.static_avatar_url().unwrap());

        a
    });
    e.color(Color::from(15844367));
    e.timestamp(&chrono::Utc::now());
}
