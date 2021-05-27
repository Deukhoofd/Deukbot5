use serenity::builder::CreateEmbed;
use serenity::model::user::CurrentUser;
use serenity::utils::Color;

pub fn setup_embed(
    u: &CurrentUser,
    e: &'a mut serenity::builder::CreateEmbed,
    title: &str,
    description: &str,
) -> &'a mut CreateEmbed {
    e.title(title);
    e.description(description);
    set_default_embed_style(u, e)
}

pub fn set_default_embed_style<'a>(u: &CurrentUser, e: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
    e.author(|a| {
        a.name(u.name.to_string());
        a.icon_url(u.static_avatar_url().unwrap_or_else(|| "".to_string()));

        a
    });
    e.color(Color::from(15844367));
    e.timestamp(&chrono::Utc::now());
    e
}
