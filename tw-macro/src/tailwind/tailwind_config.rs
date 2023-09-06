use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TailwindConfig {
    pub theme: Theme,
    pub variants: Variants,
    pub plugins: Plugins,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    #[serde(flatten)]
    pub overrides: CustomisableClasses,
    pub extend: CustomisableClasses,
}

// Represents a color which can either be a simple string or a nested structure.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ColorValue {
    Simple(String),
    Shades(HashMap<String, String>),
}

// #[derive(Debug, Deserialize)]
// struct Theme {
//     colors: Option<HashMap<String, ColorValue>>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScreenValue {
    Simple(String),
    Range(RangeBreakpoint),
    MultiRange(Vec<RangeBreakpoint>),
    Raw(RawBreakpoint),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeBreakpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawBreakpoint {
    raw: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomisableClasses {
    // pub screens: Option<HashMap<String, String>>,
    pub screens: HashMap<String, ScreenValue>,
    pub colors: Option<HashMap<String, ColorValue>>,
    pub spacing: Option<HashMap<String, String>>,

    pub border_widths: Option<HashMap<String, String>>,

    pub width: Option<HashMap<String, String>>,
    pub height: Option<HashMap<String, String>>,

    pub min_width: Option<HashMap<String, String>>,

    pub min_height: Option<HashMap<String, String>>,

    pub max_width: Option<HashMap<String, String>>,

    pub max_height: Option<HashMap<String, String>>,

    pub padding: Option<HashMap<String, String>>,
    pub margin: Option<HashMap<String, String>>,
    pub negative_margin: Option<HashMap<String, String>>,

    pub shadows: Option<HashMap<String, String>>,

    pub z_index: Option<HashMap<String, String>>,
    pub opacity: Option<HashMap<String, String>>,
    pub stroke: Option<HashMap<String, ColorValue>>,

    pub accent_color: Option<HashMap<String, ColorValue>>,
    pub accessibility: Option<HashMap<String, String>>,

    pub align_content: Option<HashMap<String, String>>,
    pub align_items: Option<HashMap<String, String>>,
    pub align_self: Option<HashMap<String, String>>,
    pub animation: Option<HashMap<String, String>>,
    pub appearance: Option<HashMap<String, String>>,
    pub aspect_ratio: Option<HashMap<String, String>>,
    pub backdrop_blur: Option<HashMap<String, String>>,
    pub backdrop_brightness: Option<HashMap<String, String>>,
    pub backdrop_contrast: Option<HashMap<String, String>>,
    pub backdrop_filter: Option<HashMap<String, String>>,
    pub backdrop_grayscale: Option<HashMap<String, String>>,
    pub backdrop_hue_rotate: Option<HashMap<String, String>>,
    pub backdrop_invert: Option<HashMap<String, String>>,
    pub backdrop_opacity: Option<HashMap<String, String>>,
    pub backdrop_saturate: Option<HashMap<String, String>>,
    pub backdrop_sepia: Option<HashMap<String, String>>,
    pub background_attachment: Option<HashMap<String, String>>,
    pub background_blend_mode: Option<HashMap<String, String>>,
    pub background_clip: Option<HashMap<String, String>>,
    pub background_color: Option<HashMap<String, ColorValue>>,
    pub background_image: Option<HashMap<String, String>>,
    pub background_opacity: Option<HashMap<String, String>>,
    pub background_origin: Option<HashMap<String, String>>,
    pub background_position: Option<HashMap<String, String>>,
    pub background_repeat: Option<HashMap<String, String>>,
    pub background_size: Option<HashMap<String, String>>,
    pub blur: Option<HashMap<String, String>>,
    pub border_collapse: Option<HashMap<String, String>>,
    pub border_color: Option<HashMap<String, ColorValue>>,
    pub border_opacity: Option<HashMap<String, String>>,
    pub border_radius: Option<HashMap<String, String>>,
    pub border_spacing: Option<HashMap<String, String>>,
    pub border_style: Option<HashMap<String, String>>,
    pub border_width: Option<HashMap<String, String>>,
    pub box_decoration_break: Option<HashMap<String, String>>,
    pub box_shadow: Option<HashMap<String, String>>,
    pub box_shadow_color: Option<HashMap<String, ColorValue>>,
    pub box_sizing: Option<HashMap<String, String>>,
    pub break_after: Option<HashMap<String, String>>,
    pub break_before: Option<HashMap<String, String>>,
    pub break_inside: Option<HashMap<String, String>>,
    pub brightness: Option<HashMap<String, String>>,
    pub caption_side: Option<HashMap<String, String>>,
    pub caret_color: Option<HashMap<String, ColorValue>>,
    pub clear: Option<HashMap<String, String>>,
    pub columns: Option<HashMap<String, String>>,
    pub container: Option<HashMap<String, String>>,
    pub content: Option<HashMap<String, String>>,
    pub contrast: Option<HashMap<String, String>>,
    pub cursor: Option<HashMap<String, String>>,
    pub display: Option<HashMap<String, String>>,
    pub divide_color: Option<HashMap<String, ColorValue>>,
    pub divide_opacity: Option<HashMap<String, String>>,
    pub divide_style: Option<HashMap<String, String>>,
    pub divide_width: Option<HashMap<String, String>>,
    pub drop_shadow: Option<HashMap<String, String>>,
    pub fill: Option<HashMap<String, ColorValue>>,
    pub filter: Option<HashMap<String, String>>,
    pub flex: Option<HashMap<String, String>>,
    pub flex_basis: Option<HashMap<String, String>>,
    pub flex_direction: Option<HashMap<String, String>>,
    pub flex_grow: Option<HashMap<String, String>>,
    pub flex_shrink: Option<HashMap<String, String>>,
    pub flex_wrap: Option<HashMap<String, String>>,
    pub float: Option<HashMap<String, String>>,
    pub font_family: Option<HashMap<String, String>>,
    pub font_size: Option<HashMap<String, String>>,
    pub font_smoothing: Option<HashMap<String, String>>,
    pub font_style: Option<HashMap<String, String>>,
    pub font_variant_numeric: Option<HashMap<String, String>>,
    pub font_weight: Option<HashMap<String, String>>,
    pub gap: Option<HashMap<String, String>>,
    pub gradient_color_stops: Option<HashMap<String, String>>,
    pub grayscale: Option<HashMap<String, String>>,
    pub grid_auto_columns: Option<HashMap<String, String>>,
    pub grid_auto_flow: Option<HashMap<String, String>>,
    pub grid_auto_rows: Option<HashMap<String, String>>,
    pub grid_column: Option<HashMap<String, String>>,
    pub grid_column_end: Option<HashMap<String, String>>,
    pub grid_column_start: Option<HashMap<String, String>>,
    pub grid_row: Option<HashMap<String, String>>,
    pub grid_row_end: Option<HashMap<String, String>>,
    pub grid_row_start: Option<HashMap<String, String>>,
    pub grid_template_columns: Option<HashMap<String, String>>,
    pub grid_template_rows: Option<HashMap<String, String>>,
    pub hue_rotate: Option<HashMap<String, String>>,
    pub hyphens: Option<HashMap<String, String>>,
    pub inset: Option<HashMap<String, String>>,
    pub invert: Option<HashMap<String, String>>,
    pub isolation: Option<HashMap<String, String>>,
    pub justify_content: Option<HashMap<String, String>>,
    pub justify_items: Option<HashMap<String, String>>,
    pub justify_self: Option<HashMap<String, String>>,
    pub letter_spacing: Option<HashMap<String, String>>,
    pub line_clamp: Option<HashMap<String, String>>,
    pub line_height: Option<HashMap<String, String>>,
    pub list_style_image: Option<HashMap<String, String>>,
    pub list_style_position: Option<HashMap<String, String>>,
    pub list_style_type: Option<HashMap<String, String>>,
    pub mix_blend_mode: Option<HashMap<String, String>>,
    pub object_fit: Option<HashMap<String, String>>,
    pub object_position: Option<HashMap<String, String>>,
    pub order: Option<HashMap<String, String>>,
    pub outline_color: Option<HashMap<String, ColorValue>>,
    pub outline_offset: Option<HashMap<String, String>>,
    pub outline_style: Option<HashMap<String, String>>,
    pub outline_width: Option<HashMap<String, String>>,
    pub overflow: Option<HashMap<String, String>>,
    pub overscroll_behavior: Option<HashMap<String, String>>,
    pub place_content: Option<HashMap<String, String>>,
    pub place_items: Option<HashMap<String, String>>,
    pub place_self: Option<HashMap<String, String>>,
    pub placeholder_color: Option<HashMap<String, ColorValue>>,
    pub placeholder_opacity: Option<HashMap<String, String>>,
    pub pointer_events: Option<HashMap<String, String>>,
    pub position: Option<HashMap<String, String>>,
    pub preflight: Option<HashMap<String, String>>,
    pub resize: Option<HashMap<String, String>>,
    pub ring_color: Option<HashMap<String, ColorValue>>,
    pub ring_offset_color: Option<HashMap<String, ColorValue>>,
    pub ring_offset_width: Option<HashMap<String, String>>,
    pub ring_opacity: Option<HashMap<String, String>>,
    pub ring_width: Option<HashMap<String, String>>,
    pub rotate: Option<HashMap<String, String>>,
    pub saturate: Option<HashMap<String, String>>,
    pub scale: Option<HashMap<String, String>>,
    pub scroll_behavior: Option<HashMap<String, String>>,
    pub scroll_margin: Option<HashMap<String, String>>,
    pub scroll_padding: Option<HashMap<String, String>>,
    pub scroll_snap_align: Option<HashMap<String, String>>,
    pub scroll_snap_stop: Option<HashMap<String, String>>,
    pub scroll_snap_type: Option<HashMap<String, String>>,
    pub sepia: Option<HashMap<String, String>>,
    pub skew: Option<HashMap<String, String>>,
    pub space: Option<HashMap<String, String>>,
    pub stroke_width: Option<HashMap<String, String>>,
    pub table_layout: Option<HashMap<String, String>>,
    pub text_align: Option<HashMap<String, String>>,
    pub text_color: Option<HashMap<String, ColorValue>>,
    pub text_decoration: Option<HashMap<String, String>>,
    pub text_decoration_color: Option<HashMap<String, ColorValue>>,
    pub text_decoration_style: Option<HashMap<String, String>>,
    pub text_decoration_thickness: Option<HashMap<String, String>>,
    pub text_indent: Option<HashMap<String, String>>,
    pub text_opacity: Option<HashMap<String, String>>,
    pub text_overflow: Option<HashMap<String, String>>,
    pub text_transform: Option<HashMap<String, String>>,
    pub text_underline_offset: Option<HashMap<String, String>>,
    pub touch_action: Option<HashMap<String, String>>,
    pub transform: Option<HashMap<String, String>>,
    pub transform_origin: Option<HashMap<String, String>>,

    pub transition_delay: Option<HashMap<String, String>>,
    pub transition_duration: Option<HashMap<String, String>>,
    pub transition_property: Option<HashMap<String, String>>,
    pub transition_timing_function: Option<HashMap<String, String>>,
    pub translate: Option<HashMap<String, String>>,
    pub user_select: Option<HashMap<String, String>>,
    pub vertical_align: Option<HashMap<String, String>>,
    pub visibility: Option<HashMap<String, String>>,
    pub whitespace: Option<HashMap<String, String>>,
    pub will_change: Option<HashMap<String, String>>,
    pub word_break: Option<HashMap<String, String>>,
    // pub extend: HashMap<String, HashMap<String, String>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Extend {
//     pub screens: Option<HashMap<String, String>>,
//     // add other fields as needed
// }

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Variants {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Plugins {}
