use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::{CurrentUser, Interaction};
use serenity::prelude::Context;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

pub struct SlashCommand {
    pub build_function: Box<
        dyn Fn(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand
            + Send
            + Sync
            + 'static,
    >,
    pub call_function: Box<dyn InteractionFn + Send + Sync + 'static>,
}

pub struct InteractionData {
    pub ctx: Box<Context>,
    pub req: Box<Interaction>,
    pub user: Box<CurrentUser>,
}

pub trait InteractionFn {
    fn call(
        &self,
        args: InteractionData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'static>>;
}

impl<T, F> InteractionFn for T
where
    T: Fn(InteractionData) -> F,
    F: Future<Output = Result<(), Box<dyn Error>>> + 'static + Send,
{
    fn call(
        &self,
        args: InteractionData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'static>> {
        Box::pin(self(args))
    }
}
