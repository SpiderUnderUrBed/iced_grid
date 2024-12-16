use iced_widget::container;

/// A set of rules that dictate the styling of a [`Table`](crate::Table).
pub trait Catalog {
    /// The supported style of the [`Catalog`].
    type Style: Default + Clone;

    /// The header [`Style`](iced_widget::container::Style) of the [`Catalog`].
    fn TARGET(&self, style: &Self::Style) -> container::Style;
}

impl Catalog for iced_core::Theme {
    type Style = ();

    fn TARGET(&self, _style: &Self::Style) -> container::Style {
        container::Style {
            text_color: Some(self.extended_palette().background.strong.text),
            background: Some(self.extended_palette().background.strong.color.into()),
            ..Default::default()
        }
    }

}

pub(crate) mod wrapper {
    use iced_core::{mouse::Cursor, Color, Element, Length, Size, Vector, Widget};
    use iced_widget::container;

    pub fn TARGET<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
        index: usize,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::TARGET,
            style,
        }
        .into()
    }

    enum Target {
        TARGET,
        // Add a target
    }

    impl Target {
        fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
            Theme: super::Catalog,
        {
            match self {
                Target::TARGET => theme.TARGET(style)
                // Target::Header => theme.header(style),
                // Target::Footer => theme.footer(style),
                // Target::Row { index } => theme.row(style, *index),
            }
        }
    }

    struct Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
    {
        content: Element<'a, Message, Theme, Renderer>,
        target: Target,
        style: <Theme as super::Catalog>::Style,
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
    {
        fn size(&self) -> Size<Length> {
            self.content.as_widget().size()
        }

        fn layout(
            &self,
            state: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content.as_widget().layout(state, renderer, limits)
        }

        fn draw(
            &self,
            state: &iced_core::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &iced_core::renderer::Style,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
        ) {
            let appearance = self.target.appearance::<Theme>(theme, &self.style);

            renderer.fill_quad(
                iced_core::renderer::Quad {
                    bounds: layout.bounds(),
                    border: appearance.border,
                    shadow: Default::default(),
                },
                appearance
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );

            let style = appearance
                .text_color
                .map(|text_color| iced_core::renderer::Style { text_color })
                .unwrap_or(*style);

            self.content
                .as_widget()
                .draw(state, renderer, theme, &style, layout, cursor, viewport)
        }

        fn tag(&self) -> iced_core::widget::tree::Tag {
            self.content.as_widget().tag()
        }

        fn state(&self) -> iced_core::widget::tree::State {
            self.content.as_widget().state()
        }

        fn children(&self) -> Vec<iced_core::widget::Tree> {
            self.content.as_widget().children()
        }

        fn diff(&self, tree: &mut iced_core::widget::Tree) {
            self.content.as_widget().diff(tree)
        }

        fn operate(
            &self,
            state: &mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced_core::widget::Operation,
        ) {
            self.content
                .as_widget()
                .operate(state, layout, renderer, operation)
        }

        fn on_event(
            &mut self,
            state: &mut iced_core::widget::Tree,
            event: iced_core::Event,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn iced_core::Clipboard,
            shell: &mut iced_core::Shell<'_, Message>,
            viewport: &iced_core::Rectangle,
        ) -> iced_core::event::Status {
            self.content.as_widget_mut().on_event(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            )
        }

        fn mouse_interaction(
            &self,
            state: &iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
            renderer: &Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content
                .as_widget()
                .mouse_interaction(state, layout, cursor, viewport, renderer)
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            translation: Vector,
        ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
            self.content
                .as_widget_mut()
                .overlay(state, layout, renderer, translation)
        }
    }

    impl<'a, Message, Theme, Renderer> From<Wrapper<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        fn from(wrapper: Wrapper<'a, Message, Theme, Renderer>) -> Self {
            Element::new(wrapper)
        }
    }
}

// use iced::{
//     Color, Rectangle, Size, Length,
//     advanced::{Widget, layout::Limits, renderer::Style, layout::Node},
//     mouse::Cursor,
//     event::Status,
// };
// use iced_core::{Layout as CoreLayout, renderer::Renderer as CoreRenderer};
// use iced::advanced::widget::Tree;
// // use iced::core::event::Status;
// use iced_core::widget::tree::{Tag, State};
// #[derive(Debug, Clone, Copy)]
// pub struct Appearance {
//     pub background_color: Color,
// }

// impl Default for Appearance {
//     fn default() -> Self {
//         Self {
//             background_color: Color::BLACK,
//         }
//     }
// }

// pub trait StyleSheet {
//     type Style: Default;

//     fn appearance(&self, style: &Self::Style) -> Appearance;
// }

// impl StyleSheet for iced::Theme {
//     type Style = ();

//     fn appearance(&self, _style: &Self::Style) -> Appearance {
//         let palette = self.extended_palette();
//         Appearance {
//             background_color: palette.background.weak.color,
//             //text_color: todo!(),
//         }
//     }
// }

// pub struct Grid<Theme>
// where
//     Theme: StyleSheet,
// {
//     width: u32,
//     height: u32,
//     background_color: Option<Color>,
//     theme: Theme,
// }

// impl<Theme> Grid<Theme>
// where
//     Theme: StyleSheet,
// {
//     pub fn new(width: u32, height: u32, theme: Theme) -> Self {
//         Self {
//             width,
//             height,
//             background_color: None,
//             theme,
//         }
//     }

//     pub fn background_color(mut self, color: Color) -> Self {
//         self.background_color = Some(color);
//         self
//     }
// }

// impl<Theme> Default for Grid<Theme>
// where
//     Theme: StyleSheet,
// {
//     fn default() -> Self {
//         Self::new(10, 10, Theme::default())
//     }
// }

// // Fix: Remove unused lifetime and constraint the Renderer type
// impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Grid<Theme>
// where
//     Message: Clone + 'a,
//     Theme: StyleSheet + 'a,
//     Renderer: CoreRenderer + 'a,
// {
//     fn size(&self) -> Size<Length> {
//         Size::new(self.width as f32, self.height as f32)
//     }

//     fn layout(
//         &self,
//         _tree: &mut Tree,
//         _renderer: &Renderer,
//         limits: &Limits,
//     ) -> Node {
//         let size = limits
//             .max()
//             .clamp((self.width as f32, self.height as f32), limits.max());
//         Node::new(size)
//     }

//     fn draw(
//         &self,
//         _tree: &Tree,
//         renderer: &mut Renderer,
//         _theme: &Theme,
//         _style: &Style,
//         layout: CoreLayout<'_>,
//         _cursor: Cursor,
//         _viewport: &Rectangle,
//     ) {
//         if let Some(background_color) = self.background_color {
//             renderer.fill_quad(
//                 iced::advanced::renderer::Quad {
//                     bounds: layout.bounds(),
//                     border: Default::default(),
//                     shadow: Default::default(),
//                 },
//                 background_color,
//             );
//         }
//     }

//     fn size_hint(&self) -> Size<Length> {
//         self.size()
//     }

//     fn tag(&self) -> Tag {
//         Tag::stateless()
//     }

//     fn state(&self) -> State {
//         State::None
//     }

//     fn children(&self) -> Vec<Tree> {
//         Vec::new()
//     }

//     fn diff(&self, _tree: &mut Tree) {}

//     fn operate(
//         &self,
//         _state: &mut Tree,
//         _layout: CoreLayout<'_>,
//         _renderer: &Renderer, // Use the concrete Renderer type
//         _operation: &mut dyn iced_core::widget::Operation,
//     ) {
//         // Your function logic here
//     }

//     fn on_event(
//         &mut self,
//         _state: &mut Tree,
//         _event: iced::Event,
//         _layout: CoreLayout<'_>,
//         _cursor: iced_core::mouse::Cursor,
//         _renderer: &Renderer,
//         _clipboard: &mut dyn iced_core::Clipboard,
//         _shell: &mut iced_core::Shell<'_, Message>,
//         _viewport: &Rectangle,
//     ) -> Status {
//         Status::Ignored
//     }

//     fn mouse_interaction(
//         &self,
//         _state: &Tree,
//         _layout: CoreLayout<'_>,
//         _cursor: Cursor,
//         _viewport: &Rectangle,
//         _renderer: &Renderer,
//     ) -> iced_core::mouse::Interaction {
//         iced_core::mouse::Interaction::None
//     }
//     fn overlay(
//         &mut self,  // No need for an explicit lifetime 'a
//         state: &mut Tree,
//         layout: CoreLayout<'_>,
//         renderer: &Renderer,
//         translation: iced::Vector,
//     ) -> Option<iced_core::overlay::Element<'_, Message, Theme, Renderer>> {
//         None
//     }
    
// }
