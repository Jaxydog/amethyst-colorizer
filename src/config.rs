use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use serde::{Deserialize, Serialize};

/// The configuration file's format.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// A list of dye colors and their set values.
    pub colors: HashMap<DyeColor, DyeColorConfig>,
}

/// All possible dye colors.
#[allow(missing_docs)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DyeColor {
    White,
    LightGray,
    Gray,
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    LightBlue,
    Blue,
    Purple,
    Magenta,
    Pink,
}

impl Display for DyeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = serde_json::to_string(self).unwrap_or_else(|_| "dyed".to_string());

        f.write_str(&color.replace('"', ""))
    }
}

/// Configuration for a single dye color.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DyeColorConfig {
    /// The RBG components.
    pub rgb: [u8; 3],
    /// Whether to allow alpha transparency.
    #[serde(default = "get_bool::<true>", skip_serializing_if = "check_bool::<true>")]
    pub allow_alpha: bool,
    /// The color's filters.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub filters: Box<[Filter]>,
}

/// A color filter.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Filter {
    /// The type of filter.
    #[serde(rename = "type")]
    pub kind: FilterType,
    /// The target of the filter.
    pub target: FilterTarget,
    /// The operation applied to the value.
    pub operation: FilterOperation,
    /// The color filter value.
    pub value: f32,
}

/// The type of a filter.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterType {
    /// Applied to individual pixels.
    Pixel,
    /// Applied to the whole image.
    Image,
}

/// The target value of a filter.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterTarget {
    /// The hue of the color or image.
    Hue,
    /// The saturation of the color or image.
    Saturation,
    /// The lightness of the color or image.
    Brightness,
    /// The contrast of the image. Does nothing for pixels.
    Contrast,
}

/// Describes how to apply a filter's value.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperation {
    /// The value is added to the base value.
    Add,
    /// The value is multiplied by the base value.
    Multiply,
    /// The value is set.
    Set,
}

macro_rules! simple_const_get {
    ($($name:ident as $type:ty),* $(,)?) => {$(
        #[allow(unused)]
        const fn $name<const V: $type>() -> $type { V }
    )*};
}

macro_rules! simple_const_check {
    ($($name:ident as $type:ty),* $(,)?) => {$(
        #[allow(unused)]
        const fn $name<const V: $type>(value: &$type) -> bool { *value == V }
    )*};
}

simple_const_get![
    get_bool as bool,
    get_char as char,
    get_u8 as u8,
    get_u16 as u16,
    get_u32 as u32,
    get_u64 as u64,
    get_u128 as u128,
    get_i8 as i8,
    get_i16 as i16,
    get_i32 as i32,
    get_i64 as i64,
    get_i128 as i128,
];
simple_const_check![
    check_bool as bool,
    check_char as char,
    check_u8 as u8,
    check_u16 as u16,
    check_u32 as u32,
    check_u64 as u64,
    check_u128 as u128,
    check_i8 as i8,
    check_i16 as i16,
    check_i32 as i32,
    check_i64 as i64,
    check_i128 as i128,
];
