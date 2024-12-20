use iced::Color;
use iced_widget::container;
 
 
 
pub trait Catalog 
 
 
{
    // fn from_style(style: Self::Style) -> Self;
    type Style: Default + Clone;
    type Themes;
    
    fn body(&self, style: &Self::Style) -> container::Style;
    fn cell(&self, _row: usize, _col: usize) -> container::Style;
    fn resolve_theme(&self) -> Self::Themes;
}
 
impl Catalog for iced_core::Theme {
    type Style = container::Style;
    type Themes = iced_core::Theme;

    fn body(&self, _style: &Self::Style) -> Self::Style {
        Self::Style {
            text_color: Some(self.extended_palette().background.strong.text),
            background: Some(self.extended_palette().background.strong.color.into()),
            ..Default::default()
        }
    }
    fn resolve_theme(&self) -> Self::Themes {
        iced_core::Theme::Dark
    }
    fn cell(&self, _row: usize, _col: usize) -> Self::Style {
        Self::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.6, 0.9))),
            ..Default::default()
        }
    }
}

 
pub mod wrapper {
    use iced::Theme;
    use iced_core::{layout::Node, mouse::Cursor, Element, Length, Size, Vector, Widget};
    use iced_widget::{button, container, renderer::wgpu::{self, primitive::Renderer}};
 
    use crate::{Cell, Grid, GridMessage};
 
    pub fn style<'a, Message, Theme, Renderer>
    (
        content: &'a mut Grid<Message, Theme>,
        theme: Theme,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog<Themes = iced_core::Theme> + Clone + 'a,
        Message: 'a,
        iced_core::Element<'a, Message, Theme, Renderer>: From<Grid<Message, Theme>>
    {
        Element::new(
            Wrapper {
                content,
                target: Style,
                theme,
                style: content.style.clone()
            }
        )
 
 
    }
    pub struct Style;

    impl Style {
        // Implement methods for Style, similar to what was done in Target::Style
        pub fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
            Theme: super::Catalog,
        {
            theme.body(style) // This can stay the same if it applies to the new Style struct
        }
    }
    // pub enum Target {
    //     Style,
 
    // }
 
    // impl Target {
    //     fn appearance<Theme>(
    //         &self,
    //         theme: &Theme,
    //         style: &<Theme as super::Catalog>::Style,
    //     ) -> container::Style
    //     where
    //         Theme: super::Catalog,
    //     {
    //         match self {
    //             Target::Style => theme.body(style),
    //         }
    //     }
    // }
 
    impl<'a, Inner, Message, Theme, Renderer> From<Wrapper<'a, Inner, Theme>>
    for Element<'a, Message, Theme, Renderer>
        where
            Inner: Widget<Message, Theme, Renderer> + 'a,
            Renderer: iced_core::Renderer + 'a,
            Theme: super::Catalog<Themes = iced_core::Theme> + 'a,
            Message: 'a,
        {
            fn from(wrapper: Wrapper<'a, Inner, Theme>) -> Self {
                Element::new(wrapper)
            }
    }
 
    pub struct Wrapper<'a, Inner, Theme>
    where
        Inner: ?Sized,
        Theme: super::Catalog<Themes = iced_core::Theme>,
    {
        pub content: &'a Inner,
       // pub target: Target,
        pub theme: Theme,
        pub style: <Theme as super::Catalog>::Style,
        pub target: Style,
    }
    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Grid<Message, Theme>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog<Themes = iced_core::Theme> + Clone,
        iced_core::Element<'a, Message, Theme, Renderer>: From<Grid<Message, Theme>>,
        //iced_core::renderer::Renderer: From<Renderer>, // Ensure Renderer can be cast to the correct type
    {
            fn size(&self) -> Size<Length> {
                let element = self.to_element();
                element.as_widget().size()
            }
            fn layout(
                &self,
                _tree: &mut iced_core::widget::Tree,
                _renderer: &Renderer,
                limits: &iced_core::layout::Limits,
            ) -> iced_core::layout::Node {
                let max_size = limits.max();
                let width = self.width.min(max_size.width);
                let height = self.height.min(max_size.height);
            
                let rows = self.rows.len();
                let cols = self.rows.get(0).map(|row| row.cells.len()).unwrap_or(0);
            
                if rows == 0 || cols == 0 {
                    return iced_core::layout::Node::new(iced_core::Size::ZERO);
                }
            
                
                let cell_width: f32 = 20.0;
                let cell_height: f32 = 20.0;
            
                
                let max_col_spacing: f32 = (width - (cols as f32 * cell_width)) / (cols as f32 - 1.0).max(1.0);
                let max_row_spacing: f32 = (height - (rows as f32 * cell_height)) / (rows as f32 - 1.0).max(1.0);
            
                
                let collum_distance: f32 = 20.0_f32.min(max_col_spacing);
                let row_distance: f32 = 20.0_f32.min(max_row_spacing);
            
                let mut children = Vec::new();
            
                for row_index in 0..rows {
                    for col_index in 0..cols {
                        let size = iced_core::Size {
                            width: cell_width,
                            height: cell_height,
                        };
            
                        let position = iced_core::Point {
                            x: col_index as f32 * (cell_width + collum_distance),
                            y: row_index as f32 * (cell_height + row_distance),
                        };
            
                        println!(
                            "Creating cell at row: {}, col: {}, position: {:?}, size: {:?}",
                            row_index, col_index, position, size
                        );
            
                        let mut child = iced_core::layout::Node::new(size);
                        children.push(child.move_to(position));
                    }
                }
            
                println!("Created {} child nodes", children.len());
            
                iced_core::layout::Node::with_children(
                    iced_core::Size::new(width, height),
                    children,
                )
            }
            
            
 
            fn draw(
                &self,
                tree: &iced_core::widget::Tree,
                renderer: &mut Renderer,
                theme: &Theme,
                style: &iced_core::renderer::Style,
                layout: iced_core::Layout<'_>,
                cursor: iced_core::mouse::Cursor,
                viewport: &iced::Rectangle,
            ) 
                // Theme: crate::style::Catalog,
                // <Theme as crate::style::Catalog>::Style: Into<iced_widget::container::Style>,
                // iced_widget::container::Style: From<<Theme as crate::style::Catalog>::Style>
            {
                let appearance = Style.appearance(theme, &self.style);
            
                renderer.fill_quad(
                    iced_core::renderer::Quad {
                        bounds: layout.bounds(),
                        border: appearance.border,
                        shadow: Default::default(),
                    },
                    appearance
                        .background
                        .unwrap_or_else(|| iced_core::Color::TRANSPARENT.into()),
                );
            
                let rows = self.rows.len();
                let cols = self.rows.get(0).map(|row| row.cells.len()).unwrap_or(0);
            
                for (row_index, row) in self.rows.iter().enumerate() {
                    for (col_index, cell) in row.cells.iter().enumerate() {
                        let child_index = row_index * cols + col_index;
            
                        if let Some(bounds) = layout
                            .children()
                            .nth(child_index)
                            .map(|child| child.bounds())
                        {
                            println!(
                                "Drawing cell at row: {}, col: {}, bounds: {:?}",
                                row_index, col_index, bounds
                            );
            
                            let cell_appearance = theme.cell(row_index, col_index);
                            
                            // match &self.theme {
                            //     _ => {
                                    
                            //         //println!("Handling all variants of MyTheme");
                            //     }
                            // }
                            
                            
                            match cell {
                                Cell::Text(text) => {
                                    text.draw(tree, renderer, &theme.resolve_theme(), style, layout, cursor, viewport);
                                }
                                Cell::Button(button) => {
                                    button.draw(tree, renderer, &theme.resolve_theme(), style, layout, cursor, viewport);
                                }
                                Cell::Container(container) => {
                                    container.draw(tree, renderer, &theme.resolve_theme(), style, layout, cursor, viewport);
                                }
                            }
                        } else {
                            println!(
                                "Missing child for cell at row: {}, col: {}",
                                row_index, col_index
                            );
                        }
                    }
                }
            }
            

            
            
            
    }
    impl<'a, Inner, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Inner, Theme>
    where
        Inner: Widget<Message, Theme, Renderer> + ?Sized,
        Renderer: iced_core::Renderer,
        Theme: super::Catalog<Themes = iced_core::Theme>,
    {
        fn size(&self) -> iced_core::Size<iced_core::Length> {
            self.content.size()
        }
 
        fn layout(
            &self,
            tree: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content.layout(tree, renderer, limits)
        }
 
        fn draw(
            &self,
            tree: &iced_core::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &iced_core::renderer::Style,
            layout: iced_core::Layout<'_>,
            cursor: iced_core::mouse::Cursor,
            viewport: &iced_core::Rectangle,
        ) {
 
 
            self.content
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
 
 
        fn tag(&self) -> iced_core::widget::tree::Tag {
            self.content.tag()
        }
 
        fn state(&self) -> iced_core::widget::tree::State {
            self.content.state()
        }
 
        fn children(&self) -> Vec<iced_core::widget::Tree> {
            self.content.children()
        }
 
        fn diff(&self, tree: &mut iced_core::widget::Tree) {
            self.content.diff(tree)
        }
 
        fn operate(
            &self,
            state: &mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced_core::widget::Operation,
        ) {
            self.content.operate(state, layout, renderer, operation);
        }
 
        fn mouse_interaction(
            &self,
            state: &iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            cursor: iced_core::mouse::Cursor,
            viewport: &iced_core::Rectangle,
            renderer: &Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content.mouse_interaction(state, layout, cursor, viewport, renderer)
        }
    }
 
 
}