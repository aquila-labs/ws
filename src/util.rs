//! The util module rexports some tools from mio in order to facilitate handling timeouts.
use slab;

/// A handle to a specific timeout.
pub use mio::timer::Timeout;
/// Used to identify some timed-out event.
pub use mio::Token;
/// A Slab allocator for associating tokens to data.
pub type Slab<T> = slab::Slab<T, Token>;
#[cfg(feature = "ssl")]
/// TcpStream underlying the WebSocket
pub use mio::tcp::TcpStream;
