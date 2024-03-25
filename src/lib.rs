// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2024 Jaxydog
//
// This file is part of Amethyst Colorizer.
//
// Amethyst Colorizer is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General
// Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// Amethyst Colorizer is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License along with Amethyst Colorizer. If not, see
// <https://www.gnu.org/licenses/>.

//! A small utility website that automatically converts amethyst into their dyeable variants.
#![deny(clippy::expect_used, clippy::panic, clippy::unwrap_used, unsafe_code)]
#![warn(clippy::nursery, clippy::todo, clippy::pedantic, missing_docs)]
#![allow(clippy::module_name_repetitions)]

use config::{DyeColorConfig, Filter, FilterOperation, FilterTarget, FilterType};
use image::{
    imageops::colorops::{brighten_in_place, contrast_in_place, huerotate_in_place},
    Pixel, RgbaImage,
};
use palette::{FromColor, GetHue, Hsv, Hsva, IntoColor, SaturateAssign, SetHue, ShiftHueAssign, Srgb, Srgba};

/// Defines the library's configuration file.
pub mod config;

/// A result type returned by the library.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A possible error returned by the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A filter was given an invalid type and operator combination.
    #[error("invalid operation '{0:?}', '{1:?}', '{2:?}'")]
    InvalidFilter(FilterType, FilterTarget, FilterOperation),
}

impl Error {
    /// Creates an invalid filter error.
    #[must_use]
    pub const fn invalid_filter(filter: Filter) -> Self {
        Self::InvalidFilter(filter.kind, filter.target, filter.operation)
    }
}

/// Iterates over each pixel within an image, applying the given closure to its HSVA value.
///
/// # Errors
///
/// This function may return an error if the given closure returns an error.
fn walk_pixels(
    image: &mut RgbaImage,
    mut f: impl FnMut(&mut Hsva<palette::encoding::Srgb>) -> Result<()>,
) -> Result<()> {
    for pixel in image.pixels_mut() {
        // black magic
        let mut hsva = Hsva::from_color(Srgba::from_components(pixel.to_rgba().0.into()).into_format());

        f(&mut hsva)?;

        let rgba: Srgba<f32> = hsva.into_color();

        pixel.0 = rgba.into_format().into_components().into();
    }

    Ok(())
}

/// Applies transformations to an image to convert it into a 'dyed' variant.
///
/// # Errors
///
/// This function may return an error if a given filter has an invalid target/operator combination.
pub fn transform_image(config: &DyeColorConfig, image: &mut RgbaImage) -> Result<()> {
    let target = Hsv::from_color(Srgb::from_components(config.rgb.into()).into_format());

    self::walk_pixels(image, |hsva| {
        hsva.set_hue(target.hue);

        for filter in config.filters.iter().filter(|f| f.kind == FilterType::Pixel).copied() {
            self::apply_pixel_filter(filter, hsva)?;
        }

        Ok(())
    })?;

    for filter in config.filters.iter().filter(|f| f.kind == FilterType::Image).copied() {
        self::apply_image_filter(filter, image)?;
    }

    Ok(())
}

/// Applies pixel-specific filters.
///
/// # Errors
///
/// This function may return an error if a given filter has an invalid target/operator combination.
pub fn apply_pixel_filter(filter: Filter, hsva: &mut Hsva<palette::encoding::Srgb>) -> Result<()> {
    match filter.target {
        FilterTarget::Contrast => return Err(Error::invalid_filter(filter)),
        FilterTarget::Hue => match filter.operation {
            FilterOperation::Add => hsva.shift_hue_assign(filter.value),
            FilterOperation::Multiply => hsva.set_hue(hsva.get_hue().into_inner() * filter.value),
            FilterOperation::Set => hsva.set_hue(filter.value),
        },
        FilterTarget::Saturation => match filter.operation {
            FilterOperation::Add => hsva.saturate_fixed_assign(filter.value),
            FilterOperation::Multiply => hsva.saturate_assign(filter.value),
            FilterOperation::Set => hsva.saturation = filter.value.clamp(0.0, 1.0),
        },
        FilterTarget::Brightness => match filter.operation {
            FilterOperation::Add => hsva.value = (hsva.value + filter.value).clamp(0.0, 1.0),
            FilterOperation::Multiply => hsva.value = (hsva.value * filter.value).clamp(0.0, 1.0),
            FilterOperation::Set => hsva.value = filter.value.clamp(0.0, 1.0),
        },
    };

    Ok(())
}

/// Applies image-specific filters.
///
/// # Errors
///
/// This function may return an error if a given filter has an invalid target/operator combination.
#[allow(clippy::cast_possible_truncation)]
pub fn apply_image_filter(filter: Filter, image: &mut RgbaImage) -> Result<()> {
    match filter.target {
        FilterTarget::Contrast => match filter.operation {
            FilterOperation::Add => contrast_in_place(image, filter.value),
            FilterOperation::Multiply => contrast_in_place(image, filter.value - 1.0),
            FilterOperation::Set => return Err(Error::invalid_filter(filter)),
        },
        FilterTarget::Hue => match filter.operation {
            FilterOperation::Add => huerotate_in_place(image, filter.value.round() as i32),
            FilterOperation::Multiply | FilterOperation::Set => return Err(Error::invalid_filter(filter)),
        },
        FilterTarget::Saturation => self::walk_pixels(image, |hsva| self::apply_pixel_filter(filter, hsva))?,
        FilterTarget::Brightness => match filter.operation {
            FilterOperation::Add => brighten_in_place(image, filter.value.round() as i32),
            FilterOperation::Multiply | FilterOperation::Set => return Err(Error::invalid_filter(filter)),
        },
    };

    Ok(())
}
