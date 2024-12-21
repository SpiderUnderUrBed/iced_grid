use iced::Color;
use iced_widget::container;
 
 
 
pub trait Catalog 
 
 
{
    
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
 
    use crate::{Cell, CellMessage, Grid, GridMessage};
 
    pub fn style<'a, Message, Theme>
    (
        content: &'a mut Grid<Message, Theme>,
        theme: Theme,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, iced_widget::Renderer>
    where
        
        Theme: super::Catalog<Themes = iced_core::Theme> + Clone + 'a,
        Message: 'a,
        iced_core::Element<'a, Message, Theme, iced_widget::Renderer>: From<Grid<Message, Theme>>
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
        
        pub fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
            Theme: super::Catalog,
        {
            theme.body(style) 
        }
    }
    
    
 
    
 
    
    
    
    
    
    
    
    
    
    
    
    
    
    
 
    impl<'a, Inner, Message, Theme> From<Wrapper<'a, Inner, Theme>>
    for Element<'a, Message, Theme, iced_widget::Renderer>
        where
            Inner: Widget<Message, Theme, iced_widget::Renderer> + 'a,
            
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
       
        pub theme: Theme,
        pub style: <Theme as super::Catalog>::Style,
        pub target: Style,
    }
    impl<'a, Message, Theme> Widget<Message, Theme, iced_widget::Renderer> for Grid<Message, Theme>
    where
    
        
        
        
        Theme: super::Catalog<Themes = iced_core::Theme> + Clone,
        iced_core::Element<'a, Message, Theme, iced_widget::Renderer>: From<Grid<Message, Theme>>,
        
    {
            fn size(&self) -> Size<Length> {
               // let element = self.to_element();
               // element.as_widget().size()
               Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
            }
            fn layout(
                &self,
                _tree: &mut iced_core::widget::Tree,
                _renderer: &iced_widget::Renderer,
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
            
                let cell_width = 20.0;
                let cell_height = 20.0;
                let total_cell_width = cell_width * cols as f32;
                let total_cell_height = cell_height * rows as f32;
            
                let horizontal_spacing = ((width - total_cell_width).max(0.0)) / (cols - 1).max(1) as f32;
                let vertical_spacing = ((height - total_cell_height).max(0.0)) / (rows - 1).max(1) as f32;
            
                let mut children = Vec::new();
            
                for (row_index, row) in self.rows.iter().enumerate() {
                    for col_index in 0..cols {
                        let position = iced_core::Point {
                            x: col_index as f32 * (cell_width + horizontal_spacing),
                            y: row_index as f32 * (cell_height + vertical_spacing),
                        };
            
                        let mut child = iced_core::layout::Node::new(iced_core::Size::new(cell_width, cell_height));
                        children.push(child.move_to(position));
                    }
                }
            
                iced_core::layout::Node::with_children(iced_core::Size::new(width, height), children)
            }
            
            
            fn draw(
                &self,
                tree: &iced_core::widget::Tree,
                renderer: &mut iced_widget::Renderer,                
                theme: &Theme,
                style: &iced_core::renderer::Style,
                layout: iced_core::Layout<'_>,
                cursor: iced_core::mouse::Cursor,
                viewport: &iced_core::Rectangle,
            ) {
                let rows = self.rows.len();
                let cols = self.rows.get(0).map_or(0, |row| row.cells.len());
            
                for (row_index, row) in self.rows.iter().enumerate() {
                    for (col_index, cell) in row.cells.iter().enumerate() {
                        let child_index = row_index * cols + col_index;
            
                        if let Some(bounds) = layout
                            .children()
                            .nth(child_index)
                            .map(|child| child.bounds())
                        {
                            match cell {
                                _ => {}  
                                
                                Cell::Button(button) => {
                                    
                                    button.draw(         
                                        tree,
                                        renderer,
                                        &theme.resolve_theme(),
                                        style,
                                        layout,
                                        cursor,
                                        viewport,
                                    );
                                }
                                Cell::Container(container) => {
                                    container.draw(
                                        tree,
                                        renderer,
                                        &theme.resolve_theme(),
                                        style,
                                        layout,
                                        cursor,
                                        viewport,
                                    );
                                }
                            }
                        }
                    }
                }
            }     
            
    }
    
    impl<'a, Inner, Message, Theme> Widget<Message, Theme, iced_widget::Renderer>
        for Wrapper<'a, Inner, Theme>
    where
        Inner: Widget<Message, Theme, iced_widget::Renderer> + ?Sized,
        
        Theme: super::Catalog<Themes = iced_core::Theme>,
    {
        fn size(&self) -> iced_core::Size<iced_core::Length> {
            self.content.size()
        }
 
        fn layout(
            &self,
            tree: &mut iced_core::widget::Tree,
            renderer: &iced_widget::Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content.layout(tree, renderer, limits)
        }
 
        fn draw(
            &self,
            tree: &iced_core::widget::Tree,
            renderer: &mut iced_widget::Renderer,
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
            renderer: &iced_widget::Renderer,
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
            renderer: &iced_widget::Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content.mouse_interaction(state, layout, cursor, viewport, renderer)
        }
    }
 
 
}