use std::convert::TryFrom;
use std::io;

use async_trait::async_trait;

use crate::{
    proxy::*,
    session::{Session},
};

pub struct Handler {
    // pub actors: Vec<AnyOutboundHandler>,
}

impl Handler {
}

#[async_trait]
impl OutboundStreamHandler for Handler {
    // fn connect_addr(&self) -> OutboundConnect {
    //     self.next_connect_addr(0)
    // }

    async fn handle<'a>(
        &'a self,
        sess: &'a Session,
        mut lhs: Option<&mut AnyStream>,
        mut stream: Option<AnyStream>,
    ) -> io::Result<AnyStream> {
        unimplemented!()    
    }
}
