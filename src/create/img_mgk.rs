use super::Quote;
use magick_rust::{
    DrawingWand, GravityType, MagickError, MagickWand, PixelWand, magick_wand_genesis,
};
use std::io::Error;
use std::path::PathBuf;
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we do not
// bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

/// default file locations relative to cargo.toml
const DEFAULT_WORDS_PER_LINE: i32 = 11;
const DEFAULT_FONT_SIZE: f64 = 85.0;
const DEFAULT_FONT_COLOR: &str = "white";
const DEFAULT_FONT: &str = "DejaVu-Sans";
const DEFAULT_GRAVITY: &str = "North";
const DEFAULT_ANNOTATE: &str = "+0+120";

/// Maps a gravity string (e.g. "south", "center") to its ImageMagick enum.
fn gravity_type(gravity: &str) -> GravityType {
    match gravity.to_ascii_lowercase().as_str() {
        "northwest" => GravityType::NorthWest,
        "north" => GravityType::North,
        "northeast" => GravityType::NorthEast,
        "west" => GravityType::West,
        "east" => GravityType::East,
        "southwest" => GravityType::SouthWest,
        "south" => GravityType::South,
        "southeast" => GravityType::SouthEast,
        _ => GravityType::Center,
    }
}

/// Parses an annotate location like "+100+100" into (x, y) offsets.
fn parse_annotate_loc(loc: &str) -> Option<(f64, f64)> {
    let mut parts = loc.split(['+', '-']).filter(|p| !p.is_empty());
    let x: f64 = parts.next()?.parse().ok()?;
    let y: f64 = parts.next()?.parse().ok()?;
    Some((x, y))
}

fn io<T>(result: Result<T, MagickError>) -> Result<T, Error> {
    result.map_err(Error::other)
}

fn split_quote(quote: &str, words_per_line: i32) -> String {
    quote
        .split(' ')
        .collect::<Vec<_>>()
        .chunks(words_per_line as usize)
        .map(|c| c.join(" "))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Renders `quote` onto `src` and writes the result to `dest`.
pub(super) fn generate_wp(quote: Quote, src: PathBuf, dest: PathBuf) -> Result<(), Error> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    // -pointsize / -fill / -gravity, with fallbacks for optional JSON fields
    let font_size: f64 = quote.font_size.unwrap_or(DEFAULT_FONT_SIZE);
    let font_color = quote.font_color.as_deref().unwrap_or(DEFAULT_FONT_COLOR);
    let gravity = gravity_type(quote.gravity.as_deref().unwrap_or(DEFAULT_GRAVITY));
    let (annotate_x, annotate_y) = quote
        .annotate
        .as_deref()
        .and_then(parse_annotate_loc)
        .unwrap_or_else(|| {
            parse_annotate_loc(DEFAULT_ANNOTATE).expect("DEFAULT_ANNOTATE is a valid location")
        });
    let font = match quote.font {
        Some(font) => font,
        _ => DEFAULT_FONT.to_string(),
    };
    let words_per_line = match quote.words_per_line {
        Some(w) => w,
        _ => DEFAULT_WORDS_PER_LINE,
    };

    // read the base wallpaper
    let mut wand = MagickWand::new();
    io(wand.read_image(&src.to_string_lossy()))?;

    // set draw settings
    let mut draw = DrawingWand::new();
    io(draw.set_font(&font))?;
    draw.set_font_size(font_size);
    let mut fill = PixelWand::new();
    io(fill.set_color(font_color))?;
    draw.set_fill_color(&fill);
    draw.set_gravity(gravity);

    // add quote
    let text = format!(
        "{}\n{}",
        split_quote(&quote.quote, words_per_line),
        quote.author
    );
    io(draw.draw_annotation(annotate_x, annotate_y, &text))?;
    io(wand.draw_image(&draw))?;

    // draw image
    io(wand.set_image_compression_quality(100))?;
    io(wand.write_image(&dest.to_string_lossy()))?;
    Ok(())
}
