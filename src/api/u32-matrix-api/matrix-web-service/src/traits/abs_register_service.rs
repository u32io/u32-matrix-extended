use std::pin::Pin;
use std::future::Future;

pub trait AbsRegisterService {
    fn register_user(self) -> Pin<Box<dyn Future<Output=Result<(),()>>>>;
}