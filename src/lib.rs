//! # grid
//! A thin wrapper around `iced`'s `Column` widget to create a grid of cells.
#![warn(missing_docs, clippy::pedantic, clippy::nursery)]
use iced::{
    Element, Pixels, Theme, advanced,
    widget::{
        Column, Container, Row,
        container::{self, Style, StyleFn},
    },
};

/// A cell in a grid, storing information about the inner element and the style of the cell.
pub struct Cell<'a, M, T, R>(Element<'a, M, T, R>, Style);

impl<'a, M, T, R, E: Into<Element<'a, M, T, R>> + 'a> From<E> for Cell<'a, M, T, R> {
    /// Create a new grid cell with an element in it. The style of the grid cell is set to the default style. See
    /// the implementation of [`Default`] for [`Style`] for more information.
    fn from(element: E) -> Self {
        Self(element.into(), Style::default())
    }
}

impl<M, T, R> Cell<'_, M, T, R> {
    /// Set the style of the cell.
    #[must_use]
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.1 = style.into();
        self
    }
}

/// A factory for creating cells in a grid.
pub struct Factory<'a, M, T, R>(Box<dyn Fn() -> Cell<'a, M, T, R> + 'a>);

impl<'a, M, T, R> Factory<'a, M, T, R> {
    /// Create a new factory that creates a grid cell with an element in it, with the default style.
    /// See the implementation of [`Default`] for [`Style`] for more information.
    ///
    /// The [`Clone`] constraint is necessary because the factory creates owned elements for `iced`
    /// to consume on each `view` cycle.
    pub fn from_element<E: Into<Element<'a, M, T, R>> + Clone + 'a>(element: E) -> Self {
        Self(Box::new(move || {
            Cell(element.clone().into(), Style::default())
        }))
    }

    /// Create a new factory that creates a grid cell with an element in it, with the given style.
    /// The [`Clone`] constraint is necessary because the factory creates owned elements for `iced`
    /// to consume on each `view` cycle.
    pub fn from_element_and_style<E: Into<Element<'a, M, T, R>> + Clone + 'a>(
        element: E,
        style: Style,
    ) -> Self {
        Self(Box::new(move || Cell(element.clone().into(), style)))
    }

    /// Create a new factory from the given function.
    pub fn from_factory<F: Fn() -> Cell<'a, M, T, R> + 'a>(factory: F) -> Self {
        Self(Box::new(factory))
    }
}

impl<'a, M, T, R, E: Into<Element<'a, M, T, R>> + Clone + 'a> From<E> for Factory<'a, M, T, R> {
    /// Convert an element into a factory that creates a grid cell with the element in it.
    /// See [`Self::from_element`] for more information.
    fn from(element: E) -> Self {
        Self::from_element(element)
    }
}

/// A grid of cells.
pub struct Grid<'a, M: 'a, T: 'a = Theme, R: advanced::Renderer + 'a = iced::Renderer> {
    rows: Vec<Vec<Factory<'a, M, T, R>>>,
    cell_width: Pixels,
    cell_height: Pixels,
    gutter: Pixels,
    padding: Pixels,
}

impl<'a, M: 'a, T: 'a, R: advanced::Renderer + 'a> Default for Grid<'a, M, T, R> {
    /// Create a new grid with no rows, and zero width, height, gutter, and padding.
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            cell_width: Pixels::default(),
            cell_height: Pixels::default(),
            gutter: Pixels::default(),
            padding: Pixels::default(),
        }
    }
}

impl<'a, M: 'a, T: 'a + container::Catalog, R: advanced::Renderer + 'a> From<&Grid<'a, M, T, R>>
    for Element<'a, M, T, R>
where
    T::Class<'a>: From<StyleFn<'a, T>>,
{
    fn from(
        Grid {
            rows,
            cell_width,
            cell_height,
            gutter,
            padding,
        }: &Grid<'a, M, T, R>,
    ) -> Self {
        Container::new(
            rows.iter()
                .map(|row| {
                    row.iter()
                        .map(|column| {
                            let Cell(element, style) = column.0();
                            Container::new(element)
                                .center_x(*cell_width)
                                .center_y(*cell_height)
                                .style(move |_| style)
                                .into()
                        })
                        .collect::<Row<M, T, R>>()
                        .spacing(*gutter)
                        .into()
                })
                .collect::<Column<M, T, R>>()
                .spacing(*gutter),
        )
        .padding(padding.0)
        .into()
    }
}

impl<'a, M: 'a, T: 'a, R: advanced::Renderer + 'a> Grid<'a, M, T, R> {
    /// Create a new grid with no rows, and zero width, height, gutter, and padding.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }



    /// Add a row to the grid.
    #[must_use]
    pub fn with_row<C: Into<Factory<'a, M, T, R>>>(
        mut self,
        row: impl IntoIterator<Item = C>,
    ) -> Self {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    /// Add multiple rows to the grid.
    #[must_use]
    pub fn with_rows<B: IntoIterator<Item = C>, C: Into<Factory<'a, M, T, R>>>(
        mut self,
        rows: impl IntoIterator<Item = B>,
    ) -> Self {
        self.rows.extend(
            rows.into_iter()
                .map(|row| row.into_iter().map(Into::into).collect()),
        );
        self
    }

    /// Set the width of each cell in the grid.
    #[must_use]
    pub fn cell_width(mut self, cell_width: impl Into<Pixels>) -> Self {
        self.cell_width = cell_width.into();
        self
    }

    /// Set the height of each cell in the grid.
    #[must_use]
    pub fn cell_height(mut self, cell_height: impl Into<Pixels>) -> Self {
        self.cell_height = cell_height.into();
        self
    }

    /// Set the size of the gutter between rows and columns in the grid.
    #[must_use]
    pub fn gutter(mut self, gutter: impl Into<Pixels>) -> Self {
        self.gutter = gutter.into();
        self
    }

    /// Set the padding around the grid.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into();
        self
    }
}