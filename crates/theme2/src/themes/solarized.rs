// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn solarized() -> UserThemeFamily {
    UserThemeFamily {
        name: "Solarized".into(),
        author: "Ethan Schoonover (altercation)".into(),
        themes: vec![
            UserTheme {
                name: "Solarized Dark".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x003847ff).into()),
                        border_variant: Some(rgba(0x003847ff).into()),
                        border_focused: Some(rgba(0x29a19899).into()),
                        border_selected: Some(rgba(0x003847ff).into()),
                        border_transparent: Some(rgba(0x003847ff).into()),
                        border_disabled: Some(rgba(0x003847ff).into()),
                        background: Some(rgba(0x002a35ff).into()),
                        element_background: Some(rgba(0x29a19899).into()),
                        element_hover: Some(rgba(0x004353aa).into()),
                        element_selected: Some(rgba(0x005a6fff).into()),
                        drop_target_background: Some(rgba(0x00435388).into()),
                        ghost_element_hover: Some(rgba(0x004353aa).into()),
                        text: Some(rgba(0xbbbbbbff).into()),
                        tab_inactive_background: Some(rgba(0x003f51ff).into()),
                        tab_active_background: Some(rgba(0x002a36ff).into()),
                        editor_background: Some(rgba(0x002a35ff).into()),
                        editor_gutter_background: Some(rgba(0x002a35ff).into()),
                        editor_line_number: Some(rgba(0x566c74ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x586e75ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xcb4b15ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x839496ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0x6c71c4ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x93a1a1ff).into()),
                        terminal_ansi_bright_white: Some(rgba(0x839496ff).into()),
                        terminal_ansi_black: Some(rgba(0x063642ff).into()),
                        terminal_ansi_red: Some(rgba(0xdc312eff).into()),
                        terminal_ansi_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xb58800ff).into()),
                        terminal_ansi_blue: Some(rgba(0x258ad2ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd33582ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x29a198ff).into()),
                        terminal_ansi_white: Some(rgba(0x839496ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xffeaeaff).into()),
                        error: Some(rgba(0xffeaeaff).into()),
                        hidden: Some(rgba(0x93a1a1ff).into()),
                        hint: Some(rgba(0x969696ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x93a1a1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xb58800ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x657b83ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b15ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "embedded".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x93a1a1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd33582ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis.strong".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd33582ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859900ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd33582ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "property".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x839496ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x657b83ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x29a198ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b15ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x29a198ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b15ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
            UserTheme {
                name: "Solarized Light".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0xddd6c1ff).into()),
                        border_variant: Some(rgba(0xddd6c1ff).into()),
                        border_focused: Some(rgba(0xd3af86ff).into()),
                        border_selected: Some(rgba(0xddd6c1ff).into()),
                        border_transparent: Some(rgba(0xddd6c1ff).into()),
                        border_disabled: Some(rgba(0xddd6c1ff).into()),
                        background: Some(rgba(0xfdf6e3ff).into()),
                        element_background: Some(rgba(0xab9d56ff).into()),
                        element_hover: Some(rgba(0xdec98744).into()),
                        element_selected: Some(rgba(0xdec987ff).into()),
                        ghost_element_hover: Some(rgba(0xdec98744).into()),
                        text: Some(rgba(0x333333ff).into()),
                        tab_inactive_background: Some(rgba(0xd3cbb7ff).into()),
                        tab_active_background: Some(rgba(0xfdf6e3ff).into()),
                        editor_background: Some(rgba(0xfdf6e3ff).into()),
                        editor_gutter_background: Some(rgba(0xfdf6e3ff).into()),
                        editor_line_number: Some(rgba(0x9ca8a6ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xcb4b15ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x839496ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0x6c71c4ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x93a1a1ff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xeee8d5ff).into()),
                        terminal_ansi_black: Some(rgba(0x657b83ff).into()),
                        terminal_ansi_red: Some(rgba(0xdc312eff).into()),
                        terminal_ansi_green: Some(rgba(0x859900ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xb58800ff).into()),
                        terminal_ansi_blue: Some(rgba(0x258ad2ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd33582ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x29a198ff).into()),
                        terminal_ansi_white: Some(rgba(0xeee8d5ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        hidden: Some(rgba(0x586e75ff).into()),
                        hint: Some(rgba(0x969696ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x93a1a1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xb58800ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x93a1a1ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b15ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "embedded".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x657b83ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd33582ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis.strong".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd33582ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859900ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd33582ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x93a1a1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x29a198ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b15ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x29a198ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x258ad2ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
        ],
    }
}
