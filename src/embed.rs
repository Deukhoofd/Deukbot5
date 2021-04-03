use serenity::builder::CreateEmbed;
use serenity::utils::Color;

pub fn setup_embed(
    e: &'a mut serenity::builder::CreateEmbed,
    title: &str,
    description: &str,
) -> &'a mut CreateEmbed {
    e.title(title);
    e.description(description);
    set_default_style(e)
}

pub fn set_default_style(e: &mut CreateEmbed) -> &mut CreateEmbed {
    let user = crate::global::self_user().as_ref().unwrap();

    e.author(|a| {
        a.name(user.name.to_string());
        a.icon_url(user.static_avatar_url().unwrap());

        a
    });
    e.color(Color::from(15844367));
    e.timestamp(&chrono::Utc::now());
    e
}
