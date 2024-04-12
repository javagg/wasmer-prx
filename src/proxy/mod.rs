use async_trait::async_trait;
use crate::{
    OutboundDatagram,
    AnyStream
};
use crate::session::Session;

pub mod direct;
pub mod chain;

/// An outbound handler for outgoing TCP conections.
#[async_trait]
pub trait OutboundStreamHandler<S = AnyStream>: Send + Sync + Unpin {
    /// Returns the address which the underlying transport should
    /// communicate with.
    // fn connect_addr(&self) -> OutboundConnect;

    /// Handles a session with the given stream. On success, returns a
    /// stream wraps the incoming stream.
    async fn handle<'a>(
        &'a self,
        sess: &'a Session,
        lhs: Option<&mut S>,
        stream: Option<S>,
    ) -> std::io::Result<S>;
}

#[async_trait]
pub trait InboundDatagramHandler<S = AnyStream/*, D = AnyInboundDatagram*/>:
    Send + Sync + Unpin
{
    // async fn handle<'a>(&'a self, socket: D) -> std::io::Result<InboundTransport<S, D>>;
}

/// An outbound handler for outgoing UDP connections.
#[async_trait]
pub trait OutboundDatagramHandler<S = AnyStream, D = Box<dyn OutboundDatagram>>:
    Send + Sync + Unpin
{
    /// Returns the address which the underlying transport should
    /// communicate with.
    // fn connect_addr(&self) -> OutboundConnect;

    // /// Returns the transport type of this handler.
    // fn transport_type(&self) -> DatagramTransportType;

    /// Handles a session with the transport. On success, returns an outbound
    /// datagram wraps the incoming transport.
    async fn handle<'a>(
        &'a self,
        sess: &'a Session,
        // transport: Option<OutboundTransport<S, D>>,
    ) -> std::io::Result<D>;
}
