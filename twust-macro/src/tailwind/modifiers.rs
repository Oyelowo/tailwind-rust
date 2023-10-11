/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
// Modifier	CSS
// hover	&:hover
// focus	&:focus
// focus-within	&:focus-within
// focus-visible	&:focus-visible
// active	&:active
// visited	&:visited
// target	&:target
// first	&:first-child
// last	&:last-child
// only	&:only-child
// odd	&:nth-child(odd)
// even	&:nth-child(even)
// first-of-type	&:first-of-type
// last-of-type	&:last-of-type
// only-of-type	&:only-of-type
// empty	&:empty
// disabled	&:disabled
// enabled	&:enabled
// checked	&:checked
// indeterminate	&:indeterminate
// default	&:default
// required	&:required
// valid	&:valid
// invalid	&:invalid
// in-range	&:in-range
// out-of-range	&:out-of-range
// placeholder-shown	&:placeholder-shown
// autofill	&:autofill
// read-only	&:read-only
// before	&::before
// after	&::after
// first-letter	&::first-letter
// first-line	&::first-line
// marker	&::marker
// selection	&::selection
// file	&::file-selector-button
// backdrop	&::backdrop
// placeholder	&::placeholder
// sm	@media (min-width: 640px)
// md	@media (min-width: 768px)
// lg	@media (min-width: 1024px)
// xl	@media (min-width: 1280px)
// 2xl	@media (min-width: 1536px)
// min-[…]	@media (min-width: …)
// max-sm	@media not all and (min-width: 640px)
// max-md	@media not all and (min-width: 768px)
// max-lg	@media not all and (min-width: 1024px)
// max-xl	@media not all and (min-width: 1280px)
// max-2xl	@media not all and (min-width: 1536px)
// max-[…]	@media (max-width: …)
// dark	@media (prefers-color-scheme: dark)
// portrait	@media (orientation: portrait)
// landscape	@media (orientation: landscape)
// motion-safe	@media (prefers-reduced-motion: no-preference)
// motion-reduce	@media (prefers-reduced-motion: reduce)
// contrast-more	@media (prefers-contrast: more)
// contrast-less	@media (prefers-contrast: less)
// print	@media print
// supports-[…]	@supports (…)
// aria-checked	&[aria-checked=“true”]
// aria-disabled	&[aria-disabled=“true”]
// aria-expanded	&[aria-expanded=“true”]
// aria-hidden	&[aria-hidden=“true”]
// aria-pressed	&[aria-pressed=“true”]
// aria-readonly	&[aria-readonly=“true”]
// aria-required	&[aria-required=“true”]
// aria-selected	&[aria-selected=“true”]
// aria-[…]	&[aria-…]
// data-[…]	&[data-…]
// rtl	[dir=“rtl”] &
// ltr	[dir=“ltr”] &
// open	&[open]

use crate::config::{add_classes_for_field, modifiers};

use super::tailwind_config::TailwindConfig;

const MODIFIERS: [&str; 49] = [
    "hover",
    "focus",
    "focus-within",
    "focus-visible",
    "active",
    "visited",
    "target",
    "first",
    "last",
    "only",
    "odd",
    "even",
    "first-of-type",
    "last-of-type",
    "only-of-type",
    "empty",
    "disabled",
    "enabled",
    "checked",
    "indeterminate",
    "default",
    "required",
    "valid",
    "invalid",
    "in-range",
    "out-of-range",
    "placeholder-shown",
    "autofill",
    "read-only",
    "before",
    "after",
    "first-letter",
    "first-line",
    "marker",
    "selection",
    "file",
    "backdrop",
    "placeholder",
    "dark",
    "portrait",
    "landscape",
    "motion-safe",
    "motion-reduce",
    "contrast-more",
    "contrast-less",
    "print",
    // supports-[…]	@supports (…)
    "ltr",
    "rtl",
    "open",
    // "aria-[…]	",
    // data-[…]	&[data-…]
    // rtl	[dir=“rtl”] &
    // ltr	[dir=“ltr”] &
    // open	&[open]
];

pub const ARIA_DEFAULT: [&str; 8] = [
    "aria-checked",
    "aria-disabled",
    "aria-expanded",
    "aria-hidden",
    "aria-pressed",
    "aria-readonly",
    "aria-required",
    "aria-selected",
];

pub fn get_modifiers(config: &TailwindConfig) -> Vec<String> {
    let mut modifiers = Vec::new();
    modifiers.extend(MODIFIERS.iter().map(ToString::to_string));
    let mut default_screens = vec![
        "sm", "md", "lg", "xl", "2xl", // "min-[…]",
        "max-sm", "max-md", "max-lg", "max-xl", "max-2xl",
        // "max-[…]",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

    if let Some(ref screens) = config.theme.overrides.screens {
        if !screens.is_empty() {
            default_screens = screens.keys().map(ToString::to_string).collect();
        }
    }

    if let Some(ref screens) = config.theme.extend.screens {
        if !screens.is_empty() {
            let screens = screens.keys().map(ToString::to_string).collect::<Vec<_>>();
            default_screens.extend(screens);
        }
    }

    let allowed_extra_modifiers = config
        .allowed_lists
        .as_ref()
        .and_then(|x| x.classes.to_owned())
        .unwrap_or_default();
    modifiers.extend(allowed_extra_modifiers);

    modifiers.extend(default_screens);

    add_classes_for_field(&modifiers::Aria, config, &mut modifiers);
    add_classes_for_field(&modifiers::Supports, config, &mut modifiers);
    add_classes_for_field(&modifiers::Data, config, &mut modifiers);

    modifiers.extend(
        modifiers
            .iter()
            .map(|x| format!("group-{x}"))
            .collect::<Vec<_>>(),
    );
    modifiers
}
