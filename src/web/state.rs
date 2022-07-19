use std::sync::Arc;

use serenity::CacheAndHttp;

#[derive(Clone)]
pub struct State {
    pub bot: Arc<CacheAndHttp>,
}
