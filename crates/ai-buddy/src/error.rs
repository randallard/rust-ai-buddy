use crate::event;
use async_openai::error::OpenAIError;
use derive_more::From;
use std::io;
use tokio::sync::broadcast;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	#[from]
	Custom(String),

	// -- Event
	#[from]
	BoadcastSend(broadcast::error::SendError<event::Event>),

	// -- Std
	#[from]
	IO(io::Error),

	// -- Externals
	#[from]
	Toml(toml::de::Error),
	#[from]
	SerdeJson(serde_json::Error),
	#[from]
	GlobSet(globset::Error),
	#[from]
	OpenAI(OpenAIError),
}

impl From<&str> for Error {
	fn from(val: &str) -> Self {
		Error::Custom(val.to_string())
	}
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
