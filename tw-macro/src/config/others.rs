use std::collections::HashMap;

use super::TailwindField;
use crate::tailwind::class_type::TAILWIND_CSS;
use crate::tailwind::tailwind_config::TailwindConfig;

macro_rules! define_tailwind_field {
    ($name:ident, $prefix:expr, $field_name:ident) => {
        struct $name;

        impl TailwindField for $name {
            fn get_prefix(&self) -> &'static str {
                $prefix
            }

            fn get_variants(&self) -> Vec<&'static str> {
                vec![]
            }

            fn get_default(&self) -> Vec<&str> {
                TAILWIND_CSS.$field_name.to_vec()
            }

            fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
                if let Some(overrides) = &config.theme.overrides.$field_name {
                    return overrides.keys().cloned().collect();
                }
                vec![]
            }

            fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
                if let Some(extends) = &config.theme.extend.$field_name {
                    return extends.keys().cloned().collect();
                }
                vec![]
            }

            fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
                vec![]
            }
        }
    };
}

// Tailwind doesn’t include a large set of aspect ratio values out of the box since it’s easier to
// just use arbitrary values.
// if you need a one-off custom def, u can use the square-bracket: class="aspect-[4/3]"
// https://tailwindcss.com/docs/aspect-ratio
define_tailwind_field!(AspectRatio, "aspect", aspect_ratio);
// Configurable but not all values are predefined by tailwindcss
// Which means you can change the behaviour within the config
// but the namings are constant i.e: ["container", "mx-auto", "max-width", "min-width"].
// define_tailwind_field!(Container, "container", container);
// https://tailwindcss.com/docs/container
define_tailwind_field!(Columns, "columns", columns);
//
// Not configurable - limited i.e only a few of them exists. So, custom not really useful.
// define_tailwind_field!(BreakAfter, "break-after", break_after);
// define_tailwind_field!(BreakBefore, "break-before", break_before);
// define_tailwind_field!(BreakInside, "break-inside", break_inside);
// define_tailwind_field!(BoxDecorationBreak, "box-decoration", box_decoration_break);
// define_tailwind_field!(BoxSizing, "box-sizing", box_sizing);
// define_tailwind_field!(Display, "display", display);
// define_tailwind_field!(Float, "float", float);
// define_tailwind_field!(Clear, "clear", clear);
// define_tailwind_field!(Isolation, "isolation", isolation);
// define_tailwind_field!(ObjectFit, "object", object_fit);
// define_tailwind_field!(Overflow, "overflow", overflow);
// define_tailwind_field!(
//     OverscrollBehavior,
//     "overscroll-behavior",
//     overscroll_behavior
// );
// define_tailwind_field!(Position, "position", position);
// define_tailwind_field!(Visibility, "visibility", visibility);
// define_tailwind_field!(FlexDirection, "flex", flex_direction);
// define_tailwind_field!(FlexWrap, "flex", flex_wrap);
// define_tailwind_field!(GridAutoFlow, "grid-flow", grid_auto_flow);
// define_tailwind_field!(JustifyContent, "justify", justify_content);
// define_tailwind_field!(JustifyItems, "justify-items", justify_items);
// define_tailwind_field!(JustifySelf, "justify-self", justify_self);
// define_tailwind_field!(AlignContent, "align-content", align_content);
// define_tailwind_field!(AlignItems, "align-items", align_items);
// define_tailwind_field!(AlignSelf, "align-self", align_self);
// define_tailwind_field!(PlaceContent, "place-content", place_content);
// define_tailwind_field!(PlaceItems, "place-items", place_items);
// define_tailwind_field!(PlaceSelf, "place-self", place_self);
// define_tailwind_field!(FontSmoothing, "font-smoothing", font_smoothing);
// define_tailwind_field!(FontStyle, "font-style", font_style);
// define_tailwind_field!(
//     FontVariantNumeric,
//     "font-variant-numeric",
//     font_variant_numeric
// );
// define_tailwind_field!(ListStylePosition, "list", list_style_position);

// By default, Tailwind provides nine object position utilities. You can customize these values by editing theme.objectPosition or theme.extend.objectPosition in your tailwind.config.js file.
// To use an arb value e.g: object-[center_bottom]
define_tailwind_field!(ObjectPosition, "object", object_position);
define_tailwind_field!(ZIndex, "z", z_index);
define_tailwind_field!(Flex, "flex", flex);
define_tailwind_field!(FlexGrow, "grow", flex_grow);
define_tailwind_field!(FlexShrink, "shrink", flex_shrink);
define_tailwind_field!(Order, "order", order);
define_tailwind_field!(GridTemplateColumns, "grid-cols", grid_template_columns);
define_tailwind_field!(GridColumn, "col", grid_column);
define_tailwind_field!(GridColumnStart, "col-start", grid_column_start);
define_tailwind_field!(GridColumnEnd, "col-end", grid_column_end);

define_tailwind_field!(GridTemplateRows, "grid-rows", grid_template_rows);
define_tailwind_field!(GridRow, "row", grid_row);
define_tailwind_field!(GridRowStart, "row-start", grid_row_start);
define_tailwind_field!(GridRowEnd, "row-end", grid_row_end);
define_tailwind_field!(GridAutoColumns, "auto-cols", grid_auto_columns);
define_tailwind_field!(GridAutoRows, "auto-rows", grid_auto_rows);

define_tailwind_field!(MinWidth, "min-w", min_width);
define_tailwind_field!(MaxWidth, "max-w", max_width);
define_tailwind_field!(MinHeight, "min-h", min_height);

define_tailwind_field!(
    PlaceholderOpacity,
    "placeholder-opacity",
    placeholder_opacity
);

// Typography
define_tailwind_field!(FontFamily, "font", font_family);
define_tailwind_field!(FontSize, "text", font_size);
define_tailwind_field!(FontWeight, "font-weight", font_weight);

define_tailwind_field!(LetterSpacing, "tracking", letter_spacing);
define_tailwind_field!(LineClamp, "line-clamp", line_clamp);
define_tailwind_field!(LineHeight, "leading", line_height);
define_tailwind_field!(ListStyleImage, "list", list_style_image);
define_tailwind_field!(ListStyleType, "list", list_style_type);

// define_tailwind_field!(TextAlign, "text-align", text_align);
// define_tailwind_field!(TextDecoration, "decoration", text_decoration);
// define_tailwind_field!(TextDecorationStyle, "decoration", text_decoration_style);
define_tailwind_field!(
    TextDecorationThickness,
    "decoration",
    text_decoration_thickness
);
define_tailwind_field!(TextUnderlineOffset, "decoration", text_underline_offset);

// define_tailwind_field!(TextTransform, "text", text_transform);

// define_tailwind_field!(TextOverflow, "overflow", text_overflow);
// define_tailwind_field!(VerticalAlign, "align", vertical_align);
// define_tailwind_field!(Whitespace, "whitespace", whitespace);
// define_tailwind_field!(WordBreak, "break", word_break);
// define_tailwind_field!(Hyphens, "hyphens", hyphens);
define_tailwind_field!(Content, "content", content);

// ---
//
// Backgrounds
// define_tailwind_field!(BackgroundAttachment, "bg", background_attachment);
// define_tailwind_field!(BackgroundClip, "bg", background_clip);
// define_tailwind_field!(BackgroundOrigin, "bg", background_origin);
define_tailwind_field!(BackgroundPosition, "bg", background_position);
// define_tailwind_field!(BackgroundRepeat, "bg", background_repeat);
define_tailwind_field!(BackgroundSize, "bg", background_size);
define_tailwind_field!(BackgroundImage, "bg", background_image);

/// ---
define_tailwind_field!(TableLayout, "table-layout", table_layout);
define_tailwind_field!(TransitionDelay, "transition-delay", transition_delay);
define_tailwind_field!(
    TransitionDuration,
    "transition-duration",
    transition_duration
);
define_tailwind_field!(
    TransitionProperty,
    "transition-property",
    transition_property
);
define_tailwind_field!(
    TransitionTimingFunction,
    "transition-timing-function",
    transition_timing_function
);
define_tailwind_field!(UserSelect, "user-select", user_select);
define_tailwind_field!(AccentColor, "accent-color", accent_color);
define_tailwind_field!(ScreenReaders, "accessibility", screen_readers);
define_tailwind_field!(Animation, "animation", animation);
define_tailwind_field!(Appearance, "appearance", appearance);
define_tailwind_field!(BackdropBlur, "backdrop-blur", backdrop_blur);
define_tailwind_field!(
    BackdropBrightness,
    "backdrop-brightness",
    backdrop_brightness
);
define_tailwind_field!(BackdropContrast, "backdrop-contrast", backdrop_contrast);
define_tailwind_field!(BackdropFilter, "backdrop-filter", backdrop_filter);
define_tailwind_field!(BackdropGrayscale, "backdrop-grayscale", backdrop_grayscale);
define_tailwind_field!(
    BackdropHueRotate,
    "backdrop-hue-rotate",
    backdrop_hue_rotate
);
define_tailwind_field!(BackdropInvert, "backdrop-invert", backdrop_invert);
define_tailwind_field!(BackdropOpacity, "backdrop-opacity", backdrop_opacity);
define_tailwind_field!(BackdropSaturate, "backdrop-saturate", backdrop_saturate);
define_tailwind_field!(BackdropSepia, "backdrop-sepia", backdrop_sepia);
define_tailwind_field!(
    BackgroundBlendMode,
    "background-blend-mode",
    background_blend_mode
);
