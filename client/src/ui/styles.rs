use bevy::prelude::*;

pub struct UiStyle;

impl UiStyle {
    pub fn text_style(
        asset_server: &AssetServer,
        size: f32,
        color: Color,
    ) -> (TextFont, TextColor) {
        (
            TextFont {
                font_size: size,
                ..default()
            },
            TextColor(color),
        )
    }

    pub const TEXT_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
}
