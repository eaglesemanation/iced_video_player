//! # Iced Video Player
//!
//! A convenient video player widget for Iced.
//!
//! To get started, load a video from a URI (e.g., a file path prefixed with `file:///`) using [`Video::new`](crate::Video::new),
//!     then use it like any other Iced widget in your `view` function by creating a [`VideoPlayer`].
//!
//! Example:
//! ```rust
//! use iced_video_player::{Video, VideoPlayer};
//! use iced::{Sandbox, Element};
//!
//! # #![allow(clippy::needless_doctest_main)]
//! fn main() {
//!     App::run(Default::default());
//! }
//!
//! struct App {
//!     video: Video,
//! }
//!
//! impl Sandbox for App {
//!     type Message = ();
//!
//!     fn new() -> Self {
//!         App {
//!             video: Video::new(&url::Url::parse("file:///C:/my_video.mp4").unwrap()).unwrap(),
//!         }
//!     }
//!
//!     fn title(&self) -> String {
//!         String::from("Video Player")
//!     }
//!
//!     fn update(&mut self, _message: ()) {}
//!
//!     fn view(&mut self) -> Element<()> {
//!         VideoPlayer::new(&self.video).into()
//!     }
//! }
//! ```
//!
//! You can programmatically control the video (e.g., seek, pause, loop, grab thumbnails) by accessing various methods on [`Video`].

mod pipeline;
mod video;
mod video_player;

use gstreamer as gst;
use thiserror::Error;

pub use video::Position;
pub use video::Video;
pub use video_player::VideoPlayer;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Glib(#[from] glib::Error),
    #[error("{0}")]
    Bool(#[from] glib::BoolError),
    #[error("failed to get the gstreamer bus")]
    Bus,
    #[error("failed to get AppSink element with name='{0}' from gstreamer pipeline")]
    AppSink(String),
    #[error("{0}")]
    StateChange(#[from] gst::StateChangeError),
    #[error("failed to cast gstreamer element")]
    Cast,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("invalid URI")]
    Uri,
    #[error("failed to get media capabilities")]
    Caps,
    #[error("failed to query media duration or position")]
    Duration,
    #[error("failed to sync with playback")]
    Sync,
    #[error("failed to lock internal sync primitive")]
    Lock,
}
