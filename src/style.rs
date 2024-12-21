use iced::{Color, Element};
use iced_core::Widget;
use iced_widget::container;
use wrapper::Wrapper;
pub mod renderer;

 
 
pub trait Catalog 
 
 
{
    // fn from_style(style: Self::Style) -> Self;
    type Style: Default + Clone;
    type Themes;
    
    fn body(&self, style: &Self::Style) -> Self::Style;
    fn cell(&self, _row: usize, _col: usize) -> Self::Style;
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
    use iced::{Font, Pixels, Theme};
    use iced_core::{layout::Node, mouse::Cursor, text::Renderer, Element, Length, Size, Vector, Widget};
    use iced_widget::{button, canvas::path::lyon_path::geom::euclid::default, container, renderer::wgpu::{self}};
 
    use crate::{Cell, Grid, GridMessage};

    use super::renderer;
 
    pub fn style<'a, Message, Theme, Renderer>
    (
        content: &'a mut Grid<Message, Theme>,
        theme: Theme,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, crate::Renderer>
    where
        //Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog<Themes = iced_core::Theme, Style = iced_widget::container::Style> + Clone + 'a,
        Message: 'a,
        iced_core::Element<'a, Message, Theme, crate::Renderer>: From<Grid<Message, Theme>>
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
           // Theme: super::Catalog,
            Theme: super::Catalog<Style = iced_widget::container::Style>,
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
//     impl<'a, Message, Theme> From<Grid<Message, Theme>>
//     for iced::Element<'a, Message>
// where
//     Message: 'a,
//     Theme: crate::style::Catalog<Style = iced_widget::container::Style, Themes = iced::Theme> + Clone, 
// {
//     fn from(grid: Grid<Message, Theme>) -> Self {
//         iced::Element::new(grid)
//     }
// }

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

// impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Grid<Message, Theme>
// where
//     Renderer: iced_core::text::Renderer<Font = iced_core::Font>,
//     //Renderer: iced_core::Renderer + crate::style::renderer::Renderer,
//     //crate::style::renderer::Renderer +
//     //Renderer: renderer::Renderer<iced::Renderer, iced_tiny_skia::Renderer>,
//     //iced::advanced::Renderer,
//     Theme: super::Catalog<Style = iced_widget::container::Style, Themes = iced::Theme> + Clone,
//     //iced_core::Element<'a, Message, Theme, Renderer>,
//    // crate::Renderer: iced_core::Renderer,
// {
//             fn size(&self) -> Size<Length> {
//                 let element = self.to_element();
//                 element.as_widget().size()
//             }
//             fn layout(
//                 &self,
//                 _tree: &mut iced_core::widget::Tree,
//                 _renderer: &Renderer,
//                 limits: &iced_core::layout::Limits,
//             ) -> iced_core::layout::Node {
//                 let max_size = limits.max();
//                 let width = self.width.min(max_size.width);
//                 let height = self.height.min(max_size.height);
            
//                 let rows = self.rows.len();
//                 let cols = self.rows.get(0).map(|row| row.cells.len()).unwrap_or(0);
            
//                 if rows == 0 || cols == 0 {
//                     return iced_core::layout::Node::new(iced_core::Size::ZERO);
//                 }
            
                
//                 let cell_width: f32 = 20.0;
//                 let cell_height: f32 = 20.0;
            
                
//                 let max_col_spacing: f32 = (width - (cols as f32 * cell_width)) / (cols as f32 - 1.0).max(1.0);
//                 let max_row_spacing: f32 = (height - (rows as f32 * cell_height)) / (rows as f32 - 1.0).max(1.0);
            
                
//                 let collum_distance: f32 = 20.0_f32.min(max_col_spacing);
//                 let row_distance: f32 = 20.0_f32.min(max_row_spacing);
            
//                 let mut children = Vec::new();
            
//                 for row_index in 0..rows {
//                     for col_index in 0..cols {
//                         let size = iced_core::Size {
//                             width: cell_width,
//                             height: cell_height,
//                         };
            
//                         let position = iced_core::Point {
//                             x: col_index as f32 * (cell_width + collum_distance),
//                             y: row_index as f32 * (cell_height + row_distance),
//                         };
            
//                         println!(
//                             "Creating cell at row: {}, col: {}, position: {:?}, size: {:?}",
//                             row_index, col_index, position, size
//                         );
            
//                         let mut child = iced_core::layout::Node::new(size);
//                         children.push(child.move_to(position));
//                     }
//                 }
            
//                 println!("Created {} child nodes", children.len());
            
//                 iced_core::layout::Node::with_children(
//                     iced_core::Size::new(width, height),
//                     children,
//                 )
//             }
            
            
 
//             fn draw(
//                 &self,
//                 tree: &iced_core::widget::Tree,
//                 renderer: &mut Renderer,
//                 theme: &Theme,
//                 style: &iced_core::renderer::Style,
//                 layout: iced_core::Layout<'_>,
//                 cursor: iced_core::mouse::Cursor,
//                 viewport: &iced::Rectangle,
//             ) {
//                 let appearance = Style.appearance(theme, &self.style);
            
//                 // Draw the grid background
//                 renderer.fill_quad(
//                     iced_core::renderer::Quad {
//                         bounds: layout.bounds(),
//                         border: appearance.border,
//                         shadow: Default::default(),
//                     },
//                     appearance
//                         .background
//                         .unwrap_or_else(|| iced_core::Color::TRANSPARENT.into()),
//                 );
            
//                 let rows = self.rows.len();
//                 let cols = self.rows.get(0).map(|row| row.cells.len()).unwrap_or(0);
            
//                 for (row_index, row) in self.rows.iter().enumerate() {
//                     for (col_index, cell) in row.cells.iter().enumerate() {
//                         let child_index = row_index * cols + col_index;
            
//                         if let Some(bounds) = layout
//                             .children()
//                             .nth(child_index)
//                             .map(|child| child.bounds())
//                         {
//                             println!(
//                                 "Drawing cell at row: {}, col: {}, bounds: {:?}",
//                                 row_index, col_index, bounds
//                             );
            
//                             let cell_appearance = theme.cell(row_index, col_index);
            
//                             // Draw the cell background
//                             renderer.fill_quad(
//                                 iced_core::renderer::Quad {
//                                     bounds,
//                                     border: cell_appearance.border,
//                                     shadow: Default::default(),
//                                 },
//                                 cell_appearance
//                                     .background
//                                     .unwrap_or_else(|| iced_core::Color::TRANSPARENT.into()),
//                             );
            
//                             // Handle specific cell content
//                             match cell {
//                                 Cell::Text(text) => {
//                                     // Define text properties
//                                     let rendered_text = iced_core::Text {
//                                         content: "e".to_string(), // Ensure it's a String
//                                         bounds: bounds.size(),
//                                         font: iced::Font {
//                                             family: iced::font::Family::Cursive, // You can replace this with any font family you prefer
//                                             weight: iced::font::Weight::Normal, // You can use Normal, Bold, or other weights as needed
//                                             stretch: iced::font::Stretch::Normal, // Stretch can be Normal, Condensed, or Expanded
//                                             style: iced::font::Style::Italic, // Style can be Regular, Italic, or other styles
//                                         },
//                                         size: Pixels::from(20.0), // Customize as needed
//                                         line_height: iced_core::text::LineHeight::Absolute(Pixels::from(20.0)),
//                                         horizontal_alignment: iced_core::alignment::Horizontal::Center,
//                                         vertical_alignment: iced_core::alignment::Vertical::Center,
//                                         shaping: iced_core::text::Shaping::Basic, // Basic shaping for most text
//                                         wrapping: iced_core::text::Wrapping::None, // Adjust as necessary
//                                     };
                                    
//                                     // Render the text
//                                     renderer.fill_text(
//                                         rendered_text,
//                                         bounds.position(),
//                                         cell_appearance.text_color.unwrap_or(iced_core::Color::BLACK),
//                                         bounds,
//                                     );
                                    
//                                 }
//                                 Cell::Button(button) => {
//                                     // Placeholder for button rendering
//                                     println!("Rendering button at row: {}, col: {}", row_index, col_index);
//                                 }
//                                 Cell::Container(container) => {
//                                     // Placeholder for container rendering
//                                     println!("Rendering container at row: {}, col: {}", row_index, col_index);
//                                 }
//                             }
//                         } else {
//                             println!(
//                                 "Missing child for cell at row: {}, col: {}",
//                                 row_index, col_index
//                             );
//                         }
//                     }
//                 }
//             }
            
            
//                 // Optionally draw additional content by delegating
//                 // self.to_element().as_widget().draw(tree, renderer, theme, style, layout, cursor, viewport);
//             }
                
    }

    impl<'a, Inner, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Inner, Theme>
    where
        Inner: ?Sized,
        Renderer: iced_core::Renderer,
        Theme: super::Catalog<Themes = iced_core::Theme>,
    {
        fn size(&self) -> iced_core::Size<iced_core::Length> {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::size(self)
            //Element::from(self.content).as_widget().size()
            //self.content.size()
        }
 
        fn layout(
            &self,
            tree: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::layout(self, tree, renderer, limits)
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
 
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::draw(self, tree, renderer, theme, style, layout, cursor, viewport);
            // self
            //     .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
 
 
        fn tag(&self) -> iced_core::widget::tree::Tag {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::tag(self)
        }
 
        fn state(&self) -> iced_core::widget::tree::State {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::state(self)
        }
 
        fn children(&self) -> Vec<iced_core::widget::Tree> {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::children(self)
        }
 
        fn diff(&self, tree: &mut iced_core::widget::Tree) {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::diff(self, tree)
        }
 
        fn operate(
            &self,
            state: &mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced_core::widget::Operation,
        ) {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::operate(self, state, layout, renderer, operation);
        }
 
        fn mouse_interaction(
            &self,
            state: &iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            cursor: iced_core::mouse::Cursor,
            viewport: &iced_core::Rectangle,
            renderer: &Renderer,
        ) -> iced_core::mouse::Interaction {
            <Wrapper<'_, Inner, Theme> as iced_core::Widget<Message, Theme, Renderer>>::mouse_interaction(self, state, layout, cursor, viewport, renderer)
        }
    }
// renderer.fill_quad(
//     iced_core::renderer::Quad {
//         bounds,
//         border: cell_appearance.border,
//         shadow: Default::default(),
//     },
//     cell_appearance
//         .background
//         .unwrap_or_else(|| iced_core::Color::TRANSPARENT.into()),
// );
                                    // let final_renderer = crate::Renderer::new(default_font, default_text_size);
                                    // final_renderer.fill_text(text, position, color, clip_bounds);
                                    // renderer.draw(pixels, clip_mask, viewport, damage, background_color, overlay);
                                    // renderer.fill_quad(
                                    //     iced_core::renderer::Quad {
                                    //         bounds,
                                    //         border: cell_appearance.border,
                                    //         shadow: Default::default(),
                                    //     },
                                    //     cell_appearance
                                    //         .background
                                    //         .unwrap_or_else(|| iced_core::Color::LIGHT_GRAY.into()),
                                    // );
                                    // //renderer::draw_text()
            
                                    // // Optionally, draw text or symbols inside the button
                                    // renderer.draw_text(
                                    //     &iced_core::text::Text {
                                    //         content: "Button".to_string(), // Placeholder text
                                    //         bounds: layout.bounds().size(),
                                    //         size: iced::Pixels(14.0),
                                    //         //color: iced::Color::BLACK,
                                    //         //cell_appearance.text_color.unwrap_or(iced_core::Color::BLACK),
                                    //         font: iced_core::Font::default(),
                                    //         horizontal_alignment: iced_core::alignment::Horizontal::Center,
                                    //         vertical_alignment: iced_core::alignment::Vertical::Center,
                                    //         line_height: todo!(),
                                    //         shaping: todo!(),
                                    //         wrapping: todo!(),
                                    //     },
                                    // );
                // let appearance = Style.appearance(theme, &self.style);
            
                // renderer.fill_quad(
                //     iced_core::renderer::Quad {
                //         bounds: layout.bounds(),
                //         border: appearance.border,
                //         shadow: Default::default(),
                //     },
                //     appearance
                //         .background
                //         .unwrap_or_else(|| iced_core::Color::TRANSPARENT.into()),
                // );
            
                // let rows = self.rows.len();
                // let cols = self.rows.get(0).map(|row| row.cells.len()).unwrap_or(0);
            
                // for (row_index, row) in self.rows.iter().enumerate() {
                //     for (col_index, cell) in row.cells.iter().enumerate() {
                //         let child_index = row_index * cols + col_index;
            
                //         if let Some(bounds) = layout
                //             .children()
                //             .nth(child_index)
                //             .map(|child| child.bounds())
                //         {
                //             println!(
                //                 "Drawing cell at row: {}, col: {}, bounds: {:?}",
                //                 row_index, col_index, bounds
                //             );
            
                //             let cell_appearance = theme.cell(row_index, col_index);
                                               
                //             match cell {
                //                 Cell::Text(text) => {
                                
                //                 }
                //                 Cell::Button(button) => {
                //                     renderer.fill_quad(
                //                         iced_core::renderer::Quad {
                //                             bounds,
                //                             border: cell_appearance.border,
                //                             shadow: Default::default(),
                //                         },
                //                         cell_appearance
                //                             .background
                //                             .unwrap_or_else(|| iced_core::Color::LIGHT_GRAY.into()),
                //                     );
            
                //                     // Optionally, draw text or symbols inside the button
                //                     renderer.draw_text(
                //                         &iced_core::text::Text {
                //                             content: "Button".to_string(), // Placeholder text
                //                             bounds: layout.bounds().size(),
                //                             size: iced::Pixels(14.0),
                //                             //color: iced::Color::BLACK,
                //                             //cell_appearance.text_color.unwrap_or(iced_core::Color::BLACK),
                //                             font: iced_core::Font::default(),
                //                             horizontal_alignment: iced_core::alignment::Horizontal::Center,
                //                             vertical_alignment: iced_core::alignment::Vertical::Center,
                //                             line_height: todo!(),
                //                             shaping: todo!(),
                //                             wrapping: todo!(),
                //                         },
                //                     );
                //                 }
                //                 Cell::Container(container) => {
                //                     //let container_appearance = theme.container();
                //                     renderer.fill_quad(
                //                         iced_core::renderer::Quad {
                //                             bounds,
                //                             border: cell_appearance.border,
                //                             shadow: Default::default(),
                //                         },
                //                         cell_appearance
                //                             .background
                //                             .unwrap_or_else(|| iced_core::Color::TRANSPARENT.into()),
                //                     );
                //                 }
                //             }
                //         } else {
                //             println!(
                //                 "Missing child for cell at row: {}, col: {}",
                //                 row_index, col_index
                //             );
                //         }
                //     }
                // }