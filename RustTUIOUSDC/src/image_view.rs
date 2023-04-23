use cursive_core::{
    Printer, Vec2,
    view::View,
    theme::{
        ColorStyle,
        Color
    }
};
use image::{
    error::ImageError,
    imageops::FilterType,
    io::Reader as ImageReader
};
use itertools::Itertools;

type ImageResult = Result<Vec<Vec<(Color, Color)>>, ImageError>;

//TODO: static image view
// a view that uses a procedural macro that will generate an image
// to display at compile time

/// View that can render a low res image as text
///
/// It is highly recommended to use `buffered_backend_root()` to create the cursive root, otherwise this view may mess up the terminal color scheme
pub struct ImageView {
    data: Option<Vec<Vec<(Color, Color)>>>,
    width: usize,
    height: usize,
    path: String
}

impl ImageView {
    /// Create a new empty `ImageView`
    pub fn new(width: usize, height: usize) -> ImageView {
        ImageView {
            data: None,
            width,
            height,
            path: String::new()
        }
    }

    /// Set the image to render
    pub fn set_image(&mut self, path: &str) {
        self.path = path.to_string();
        if let Ok(data) = process_image(path, self.width, self.height) {
            self.data = Some(data);
        }
    }

    /// Set the image to render
    ///
    /// Chainable version
    pub fn image(mut self, path: &str) -> ImageView {
        self.set_image(path);
        self
    }

    /// Set the rendering size of the image
    ///
    /// Size is measured in terminal cells
    pub fn set_size(&mut self, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
        if let Ok(data) = process_image(&self.path, self.width, self.height) {
            self.data = Some(data);
        }
    }

    /// Gets the path of the image
    pub fn get_path(&self) -> &str { &self.path }
}

impl View for ImageView {
    fn draw(&self, printer: &Printer) {
        if let Some(ref image_data) = self.data {
            for (y, row) in image_data.iter().enumerate() {
                for (x, style) in row.iter().enumerate() {
                    printer.with_color(ColorStyle::from(*style), |printer| printer.print((x, y), "▄"))
                }
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        if self.data.is_none() {
            Vec2::new(0, 0)
        }
        else {
            Vec2::new(self.width, self.height)
        }
    }
}

fn process_image(path: &str, width: usize, height: usize) -> ImageResult {
    let img_height = height * 2;
    let img = ImageReader::open(path)?.with_guessed_format()?.decode()?;
    let mut img_colors: Vec<Vec<(Color, Color)>> = Vec::new();
    let img_buffer = img
        .adjust_contrast(50.)
        .resize_exact(width as u32, img_height as u32, FilterType::Triangle)
        .to_rgb8();

    for (cur_row, next_row) in img_buffer.rows().tuples() {
        let mut colors: Vec<(Color, Color)> = Vec::new();
        for (top_pxl, bottom_pxl) in cur_row.zip(next_row) {
            colors.push((
                Color::Rgb(bottom_pxl.0[0], bottom_pxl.0[1], bottom_pxl.0[2]),
                Color::Rgb(top_pxl.0[0], top_pxl.0[1], top_pxl.0[2])
            ));
        }
        img_colors.push(colors);
    }

    Ok(img_colors)
}