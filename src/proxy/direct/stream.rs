use async_trait::async_trait;
use anyhow::{anyhow, Result};
use thiserror::{self, Error};

use crate::proxy::OutboundStreamHandler;
use crate::{
    session::Session,
    AnyStream
};

pub struct Handler;

#[async_trait]
impl OutboundStreamHandler for Handler {
    // fn connect_addr(&self) -> OutboundConnect {
    //     OutboundConnect::Direct
    // }

    async fn handle<'a>(
        &'a self,
        _sess: &'a Session,
        _lhs: Option<&mut AnyStream>,
        stream: Option<AnyStream>,
    ) -> std::io::Result<AnyStream> {
        stream.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "invalid input"))
    }
}
