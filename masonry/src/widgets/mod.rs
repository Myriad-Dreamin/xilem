// Copyright 2018 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! Common widgets.

mod align;
#[cfg(any(feature = "text", test))]
mod button;
mod canvas;
#[cfg(any(feature = "text", test))]
mod checkbox;
#[cfg(any(feature = "text", test))]
mod divider;
mod flex;
mod grid;
mod image;
mod indexed_stack;
#[cfg(any(feature = "text", test))]
mod label;
mod passthrough;
mod portal;
#[cfg(any(feature = "text", test))]
mod progress_bar;
#[cfg(any(feature = "text", test))]
mod prose;
mod resize_observer;
mod scroll_bar;
mod sized_box;
mod slider;
mod spinner;
mod split;
mod switch;
#[cfg(any(feature = "text", test))]
mod text_area;
#[cfg(any(feature = "text", test))]
mod text_input;
#[cfg(any(feature = "text", test))]
mod variable_label;
mod virtual_scroll;
mod zstack;

// TODO - Split off widgets and other exports?
// (e.g. actions, param types)

pub use self::align::*;
#[cfg(any(feature = "text", test))]
pub use self::button::*;
pub use self::canvas::*;
#[cfg(any(feature = "text", test))]
pub use self::checkbox::*;
#[cfg(any(feature = "text", test))]
pub use self::divider::*;
pub use self::flex::*;
pub use self::grid::*;
pub use self::image::*;
pub use self::indexed_stack::*;
#[cfg(any(feature = "text", test))]
pub use self::label::*;
pub use self::passthrough::*;
pub use self::portal::*;
#[cfg(any(feature = "text", test))]
pub use self::progress_bar::*;
#[cfg(any(feature = "text", test))]
pub use self::prose::*;
pub use self::resize_observer::*;
pub use self::scroll_bar::*;
pub use self::sized_box::*;
pub use self::slider::*;
pub use self::spinner::*;
pub use self::split::*;
pub use self::switch::*;
#[cfg(any(feature = "text", test))]
pub use self::text_area::*;
#[cfg(any(feature = "text", test))]
pub use self::text_input::*;
#[cfg(any(feature = "text", test))]
pub use self::variable_label::*;
pub use self::virtual_scroll::*;
pub use self::zstack::*;
