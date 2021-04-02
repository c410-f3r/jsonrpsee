use crate::jsonrpc;
use alloc::{boxed::Box, string::String};
use core::fmt;

/// Convenience type for displaying errors.
#[derive(Clone, Debug, PartialEq)]
pub struct Mismatch<T> {
	/// Expected value.
	pub expected: T,
	/// Actual value.
	pub got: T,
}

impl<T: fmt::Display> fmt::Display for Mismatch<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("Expected: {}, Got: {}", self.expected, self.got))
	}
}

/// Error type.
pub enum Error {
	/// Networking error or error on the low-level protocol layer.
	TransportError(Box<dyn std::error::Error + Send + Sync>),
	/// JSON-RPC request error.
	Request(jsonrpc::Error),
	/// Subscription error.
	Subscription(String, String),
	/// Frontend/backend channel error.
	Internal(futures::channel::mpsc::SendError),
	/// Invalid response,
	InvalidResponse(Mismatch<String>),
	/// The background task has been terminated.
	RestartNeeded(String),
	/// Failed to parse the data that the server sent back to us.
	ParseError(jsonrpc::ParseError),
	/// Invalid subscription ID.
	InvalidSubscriptionId,
	/// Invalid request ID.
	InvalidRequestId,
	/// A request with the same request ID has already been registered.
	DuplicateRequestId,
	/// Method was already registered.
	MethodAlreadyRegistered(String),
	/// Subscribe and unsubscribe method names are the same.
	SubscriptionNameConflict(String),
	/// Websocket request timeout
	WsRequestTimeout,
	/// Configured max number of request slots exceeded.
	MaxSlotsExceeded,
	/// Custom error.
	Custom(String),
}

impl fmt::Debug for Error {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::TransportError(ref elem) => write!(f, "Networking or low-level protocol error: {}", elem),
			Self::Request(ref elem) => write!(f, "JSON-RPC request error: {}", elem),
			Self::Subscription(ref elem0, ref elem1) => {
				write!(f, "Subscription failed, subscribe_method: {} unsubscribe_method: {}", elem0, elem1)
			}
			Self::Internal(ref elem) => write!(f, "Frontend/backend channel error: {}", elem),
			Self::InvalidResponse(ref elem) => write!(f, "Invalid response: {}", elem),
			Self::RestartNeeded(ref elem) => {
				write!(f, "The background task been terminated because: {}; restart required", elem)
			}
			Self::ParseError(ref elem) => write!(f, "Parse error: {}", elem),
			Self::InvalidSubscriptionId => write!(f, "Invalid subscription ID"),
			Self::InvalidRequestId => write!(f, "Invalid request ID"),
			Self::DuplicateRequestId => write!(f, " request with the same request ID has already been registered"),
			Self::MethodAlreadyRegistered(ref elem) => write!(f, "Method: {} was already registered", elem),
			Self::SubscriptionNameConflict(ref elem) => {
				write!(f, "Cannot use the same method name for subscribe and unsubscribe, used: {}", elem)
			}
			Self::WsRequestTimeout => write!(f, "Websocket request timeout"),
			Self::MaxSlotsExceeded => write!(f, "Configured max number of request slots exceeded"),
			Self::Custom(ref elem) => write!(f, "Custom error: {}", elem),
		}
	}
}

impl fmt::Display for Error {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Generic transport error.
pub enum GenericTransportError<T> {
	/// Request was too large.
	TooLarge,
	/// Concrete transport error.
	Inner(T),
}

impl<T> fmt::Debug for GenericTransportError<T>
where
	T: fmt::Debug,
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::TooLarge => write!(f, "The request was too big"),
			Self::Inner(ref inner) => write!(f, "Transport error: {:?}", inner),
		}
	}
}

impl<T> fmt::Display for GenericTransportError<T>
where
	T: fmt::Debug,
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[cfg(feature = "std")]
impl<T> std::error::Error for GenericTransportError<T> where T: fmt::Debug {}
