#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! A `Cow`-like data structure where owned data is stored inside an `Arc`.
//!
//! [Bevy]: https://bevyengine.org/
//!

mod cow_arc;

pub use cow_arc::*;
