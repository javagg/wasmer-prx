use async_trait::async_trait;

use crate::proxy::InboundDatagramHandler;

pub struct Handler {

}


#[async_trait]
impl InboundDatagramHandler for Handler {

}