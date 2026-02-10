// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

//! Views for the widgets which are built-in to Masonry. These are the primitives your Xilem app's view tree will generally be constructed from.

#[cfg(feature = "text")]
mod button;
mod canvas;
#[cfg(feature = "text")]
mod checkbox;
mod flex;
mod grid;
mod image;
mod indexed_stack;
#[cfg(feature = "text")]
mod label;
mod portal;
#[cfg(feature = "text")]
mod progress_bar;
mod prop;
#[cfg(feature = "text")]
mod prose;
mod resize_observer;
mod sized_box;
mod slider;
mod spinner;
mod split;
mod switch;
mod task;
#[cfg(feature = "text")]
mod text_input;
mod transform;
#[cfg(feature = "text")]
mod variable_label;
mod virtual_scroll;
mod worker;
mod zstack;

#[cfg(feature = "text")]
pub use self::button::*;
pub use self::canvas::*;
#[cfg(feature = "text")]
pub use self::checkbox::*;
pub use self::flex::*;
pub use self::grid::*;
pub use self::image::*;
pub use self::indexed_stack::*;
#[cfg(feature = "text")]
pub use self::label::*;
pub use self::portal::*;
#[cfg(feature = "text")]
pub use self::progress_bar::*;
pub use self::prop::*;
#[cfg(feature = "text")]
pub use self::prose::*;
pub use self::resize_observer::*;
pub use self::sized_box::*;
pub use self::slider::*;
pub use self::spinner::*;
pub use self::split::*;
pub use self::switch::*;
pub use self::task::*;
#[cfg(feature = "text")]
pub use self::text_input::*;
pub use self::transform::*;
#[cfg(feature = "text")]
pub use self::variable_label::*;
pub use self::virtual_scroll::*;
pub use self::worker::*;
pub use self::zstack::*;
