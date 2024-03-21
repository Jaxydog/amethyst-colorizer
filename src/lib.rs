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
    imageops::colorops::{contrast_in_place, huerotate_in_place},
    Pixel, RgbaImage,
};
use palette::{
    encoding::Linear, FromColor, GetHue, Hsv, Hsva, IntoColor, LinSrgb, LinSrgba, SetHue, ShiftHueAssign, Srgb, Srgba,
};

#[cfg(all(feature = "cli", feature = "shuttle"))]
compile_error!("only one of 'cli' or 'shuttle' may be enabled");

/// Defines the library's configuration file.
pub mod config;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// Applies transformations to an image to convert it into a 'dyed' variant.
pub fn transform_image(config: &DyeColorConfig, image: &mut RgbaImage) {
    let target: LinSrgb<f32> = Srgb::from_components(config.rgb.into()).into_linear();
    let target = Hsv::from_color(target);

    for pixel in image.pixels_mut() {
        let rgba = pixel.to_rgba().0.into();
        let rgba: LinSrgba<f32> = Srgba::from_components(rgba).into_linear();
        let mut hsva = Hsva::from_color(rgba);

        hsva.set_hue(target.get_hue());

        for filter in config.filters.iter().filter(|f| f.kind == FilterType::Pixel).copied() {
            self::apply_pixel_filter(filter, &mut hsva);
        }

        let rgba: LinSrgba<f32> = hsva.into_color();
        let rgba: LinSrgba<u8> = rgba.into_format();

        pixel.0 = rgba.into_components().into();
    }

    for filter in config.filters.iter().filter(|f| f.kind == FilterType::Image).copied() {
        self::apply_image_filter(filter, image);
    }
}

/// Applies pixel-specific filters.
pub fn apply_pixel_filter(filter: Filter, hsva: &mut Hsva<Linear<palette::encoding::Srgb>>) {
    match filter.target {
        FilterTarget::Contrast => { /* intentionally ignored */ }
        FilterTarget::Hue => match filter.operation {
            FilterOperation::Add => hsva.shift_hue_assign(filter.value),
            FilterOperation::Multiply => hsva.set_hue(hsva.get_hue().into_inner() * filter.value),
            FilterOperation::Set => hsva.set_hue(filter.value),
        },
        FilterTarget::Saturation => match filter.operation {
            FilterOperation::Add => hsva.saturation = (hsva.saturation + filter.value).clamp(0.0, 1.0),
            FilterOperation::Multiply => hsva.saturation = (hsva.saturation * filter.value).clamp(0.0, 1.0),
            FilterOperation::Set => hsva.saturation = filter.value.clamp(0.0, 1.0),
        },
        FilterTarget::Brightness => match filter.operation {
            FilterOperation::Add => hsva.value = (hsva.value + filter.value).clamp(0.0, 1.0),
            FilterOperation::Multiply => hsva.value = (hsva.value * filter.value).clamp(0.0, 1.0),
            FilterOperation::Set => hsva.value = filter.value.clamp(0.0, 1.0),
        },
    }
}

/// Applies image-specific filters.
#[allow(clippy::cast_possible_truncation)]
pub fn apply_image_filter(filter: Filter, image: &mut RgbaImage) {
    match filter.target {
        FilterTarget::Contrast => match filter.operation {
            FilterOperation::Add => contrast_in_place(image, filter.value),
            FilterOperation::Multiply | FilterOperation::Set => contrast_in_place(image, filter.value - 1.0),
        },
        FilterTarget::Hue => match filter.operation {
            FilterOperation::Add => huerotate_in_place(image, filter.value.round() as i32),
            FilterOperation::Multiply => huerotate_in_place(image, value),
            FilterOperation::Set => todo!(),
        },
        FilterTarget::Saturation => todo!(),
        FilterTarget::Brightness => todo!(),
    };
}
