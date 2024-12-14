#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub background_color: Color,
    // /// The track [`Color`] of the progress indicator.
    // pub track_color: Color,
    // /// The bar [`Color`] of the progress indicator.
    // pub bar_color: Color,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            background_color: Color::BLACK,
        }
    }
}

/// A set of rules that dictate the style of an indicator.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the active [`Appearance`] of a indicator.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

impl StyleSheet for iced::Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        Appearance {
            background_color: palette.background.weak.color,
        }
    }
}