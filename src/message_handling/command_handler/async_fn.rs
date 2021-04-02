use crate::message_handling::command_handler::command_data::CommandData;
use serenity::Error;
use std::future::Future;
use std::pin::Pin;

pub trait AsyncFn {
    fn call(
        &self,
        args: CommandData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'static>>;
}

impl<T, F> AsyncFn for T
where
    T: Fn(CommandData) -> F,
    F: Future<Output = Result<(), Error>> + 'static + Send,
{
    fn call(
        &self,
        args: CommandData,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'static>> {
        Box::pin(self(args))
    }
}
