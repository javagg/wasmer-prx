use tokio::io::{AsyncRead, AsyncWrite};
use async_trait::async_trait;

pub mod proxy;
pub mod session;
pub mod address;
mod error;

use address::Address;

pub type Result<T> = std::result::Result<T, error::Error>;

/// A reliable transport for both inbound and outbound handlers.
pub trait ProxyStream: AsyncRead + AsyncWrite + Send + Sync + Unpin {}

impl<S> ProxyStream for S where S: AsyncRead + AsyncWrite + Send + Sync + Unpin {}

pub type AnyStream = Box<dyn ProxyStream>;

pub type StreamId = u64;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct DatagramSource {
    pub address: Address,
    pub stream_id: Option<StreamId>,
}

/// The receive half.
#[async_trait]
pub trait OutboundDatagramRecvHalf: Sync + Send + Unpin {
    /// Receives a message on the socket. On success, returns the number of
    /// bytes read and the origin of the message.
    async fn recv_from(&mut self, buf: &mut [u8]) -> std::io::Result<(usize, Address)>;
}

/// The send half.
#[async_trait]
pub trait OutboundDatagramSendHalf: Sync + Send + Unpin {
    /// Sends a message on the socket to `dst_addr`. On success, returns the
    /// number of bytes sent.
    async fn send_to(&mut self, buf: &[u8], dst_addr: &Address) -> std::io::Result<usize>;

    /// Close the soccket gracefully.
    async fn close(&mut self) -> std::io::Result<()>;
}

pub trait OutboundDatagram: Send + Unpin {
    /// Splits the datagram.
    fn split(
        self: Box<Self>,
    ) -> (
        Box<dyn OutboundDatagramRecvHalf>,
        Box<dyn OutboundDatagramSendHalf>,
    );
}


/// An unreliable transport for inbound handlers.
pub trait InboundDatagram: Send + Sync + Unpin {
    /// Splits the datagram.
    fn split(
        self: Box<Self>,
    ) -> (
        Box<dyn InboundDatagramRecvHalf>,
        Box<dyn InboundDatagramSendHalf>,
    );

    /// Turns the datagram into a [`std::net::UdpSocket`].
    fn into_std(self: Box<Self>) -> std::io::Result<std::net::UdpSocket>;
}

// pub type AnyInboundDatagram = Box<dyn InboundDatagram>;

/// The receive half.
#[async_trait]
pub trait InboundDatagramRecvHalf: Sync + Send + Unpin {
    /// Receives a single datagram message on the socket. On success, returns
    /// the number of bytes read, the source where this message
    /// originated and the destination this message shall be sent to.
    async fn recv_from(
        &mut self,
        buf: &mut [u8],
    ) -> Result<(usize, DatagramSource, Address)>;
}

/// The send half.
#[async_trait]
pub trait InboundDatagramSendHalf: Sync + Send + Unpin {
    /// Sends a datagram message on the socket to `dst_addr`, the `src_addr`
    /// specifies the origin of the message. On success, returns the number
    /// of bytes sent.
    async fn send_to(
        &mut self,
        buf: &[u8],
        src_addr: &Address,
        dst_addr: &Address,
    ) -> std::io::Result<usize>;

    /// Close the socket gracefully.
    async fn close(&mut self) -> std::io::Result<()>;
}
