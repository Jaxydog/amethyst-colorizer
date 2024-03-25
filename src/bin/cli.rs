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

#![deny(clippy::expect_used, clippy::panic, clippy::unwrap_used, unsafe_code)]
#![warn(clippy::nursery, clippy::todo, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::path::Path;

use amethyst_colorizer::config::{Config, DyeColor};
use anyhow::{bail, Result};
use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// The path of the image to convert.
    pub path: Box<Path>,
    /// The path of the color configuration to load.
    #[arg(short = 'c', long = "config", value_name = "PATH", default_value = "./res/default.json")]
    pub config: Box<Path>,
    /// The expected dye color. If absent, all colors will be generated.
    #[arg(short = 't', long = "target-color")]
    pub color: Option<DyeColor>,
    /// The directory to output the converted files into.
    #[arg(short = 'o', long = "output-dir", value_name = "DIR", default_value = "./out/")]
    pub output: Box<Path>,
}

#[macro_export]
macro_rules! assert {
    ($test:expr $(,)?) => {
        if !$test {
            ::anyhow::bail!("assertion failed - '{}'", ::std::stringify!($test));
        }
    };
    ($test:expr, $($message:tt)+) => {
        if !$test {
            ::anyhow::bail!($($message)*);
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        $crate::assert!($left == $right);
    };
    ($left:expr, $right:expr, $($message:tt)+) => {
        $crate::assert!($left == $right, $($message)+);
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr $(,)?) => {
        $crate::assert!($left != $right);
    };
    ($left:expr, $right:expr, $($message:tt)+) => {
        $crate::assert!($left != $right, $($message)+);
    };
}

#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $value {
            $pattern $(if $guard)? => {}
            _ => bail!("assertion failed - unable to match for '{:?}'", $value);
        }
    };
    ($value:expr, $pattern:pat $(if $guard:expr)?, $($message:tt)+) => {
        match $value {
            $pattern $(if $guard)? => {}
            _ => bail!($($message)+),
        }
    };
}

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    assert!(arguments.config.try_exists()?, "unable to find the target file at {:?}", arguments.config);
    assert!(arguments.path.try_exists()?, "unable to find the configuration file at {:?}", arguments.path);

    let mut file_extension = None;

    if let Some(extension) = arguments.path.extension().and_then(|s| s.to_str()) {
        assert_matches!(extension, "png" | "zip", "the specified file must be either a .zip or .png file");

        file_extension = Some(extension);
    }

    if arguments.output.try_exists()? {
        assert!(arguments.output.is_dir(), "the specified output path is not a directory");
    } else {
        std::fs::create_dir_all(&arguments.output)?;
    }

    let config: Config = serde_json::from_slice(&std::fs::read(&arguments.config)?)?;

    if let Some(ref color) = arguments.color {
        assert!(config.colors.contains_key(color), "the given color is missing from the configuration file");
    }

    match file_extension {
        Some("png") => self::main_png(&arguments, &config),
        Some("zip") | None => self::main_zip(arguments, config),
        Some(extension) => bail!("unknown extension '{extension}'"),
    }
}

fn main_png(arguments: &Arguments, config: &Config) -> Result<()> {
    let image = image::open(&arguments.path)?;

    if let Some(ref color) = arguments.color {
        let output = arguments.output.join(format!("{color}_amethyst.png"));
        let mut buffer = image.to_rgba8();

        let Some(config) = config.colors.get(color) else {
            bail!("the given color is missing from the configuration file");
        };

        amethyst_colorizer::transform_image(config, &mut buffer)?;
        image::save_buffer(output, &buffer, buffer.width(), buffer.height(), image.color())?;

        return Ok(());
    }

    for (color, config) in &config.colors {
        let output = arguments.output.join(format!("{color}_amethyst.png"));
        let mut buffer = image.to_rgba8();

        amethyst_colorizer::transform_image(config, &mut buffer)?;
        image::save_buffer(output, &buffer, buffer.width(), buffer.height(), image.color())?;
    }

    Ok(())
}

fn main_zip(arguments: Arguments, config: Config) -> Result<()> {
    Ok(())
}
