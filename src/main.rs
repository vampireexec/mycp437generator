use anyhow::{Context, Result};
use clap::Parser;
use sdl3::image::SaveSurface;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::surface::Surface;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mycp437generator")]
#[command(about = "Generate a CP437 font atlas from a TTF file")]
struct Args {
    /// Path to the TTF font file
    font_path: PathBuf,

    /// Width of each character cell in pixels
    font_width: u32,

    /// Height of each character cell in pixels
    font_height: u32,

    /// Output PNG file path
    output: PathBuf,
}

// CP437 character mapping
fn get_cp437_char(index: u8) -> char {
    match index {
        0 => ' ',                  // Null - render as space
        1 => '☺',                  // White smiling face
        2 => '☻',                  // Black smiling face
        3 => '♥',                  // Heart
        4 => '♦',                  // Diamond
        5 => '♣',                  // Club
        6 => '♠',                  // Spade
        7 => '•',                  // Bullet
        8 => '◘',                  // Inverse bullet
        9 => '○',                  // White circle
        10 => '◙',                 // Inverse white circle
        11 => '♂',                 // Male sign
        12 => '♀',                 // Female sign
        13 => '♪',                 // Eighth note
        14 => '♫',                 // Beamed eighth notes
        15 => '☼',                 // White sun with rays
        16 => '►',                 // Black right-pointing pointer
        17 => '◄',                 // Black left-pointing pointer
        18 => '↕',                 // Up down arrow
        19 => '‼',                 // Double exclamation mark
        20 => '¶',                 // Pilcrow sign
        21 => '§',                 // Section sign
        22 => '▬',                 // Black rectangle
        23 => '↨',                 // Up down arrow with base
        24 => '↑',                 // Upwards arrow
        25 => '↓',                 // Downwards arrow
        26 => '→',                 // Rightwards arrow
        27 => '←',                 // Leftwards arrow
        28 => '∟',                 // Right angle
        29 => '↔',                 // Left right arrow
        30 => '▲',                 // Black up-pointing triangle
        31 => '▼',                 // Black down-pointing triangle
        32..=126 => index as char, // Standard ASCII
        127 => '⌂',                // House
        128 => 'Ç',
        129 => 'ü',
        130 => 'é',
        131 => 'â',
        132 => 'ä',
        133 => 'à',
        134 => 'å',
        135 => 'ç',
        136 => 'ê',
        137 => 'ë',
        138 => 'è',
        139 => 'ï',
        140 => 'î',
        141 => 'ì',
        142 => 'Ä',
        143 => 'Å',
        144 => 'É',
        145 => 'æ',
        146 => 'Æ',
        147 => 'ô',
        148 => 'ö',
        149 => 'ò',
        150 => 'û',
        151 => 'ù',
        152 => 'ÿ',
        153 => 'Ö',
        154 => 'Ü',
        155 => '¢',
        156 => '£',
        157 => '¥',
        158 => '₧',
        159 => 'ƒ',
        160 => 'á',
        161 => 'í',
        162 => 'ó',
        163 => 'ú',
        164 => 'ñ',
        165 => 'Ñ',
        166 => 'ª',
        167 => 'º',
        168 => '¿',
        169 => '⌐',
        170 => '¬',
        171 => '½',
        172 => '¼',
        173 => '¡',
        174 => '«',
        175 => '»',
        176 => '░',
        177 => '▒',
        178 => '▓',
        179 => '│',
        180 => '┤',
        181 => '╡',
        182 => '╢',
        183 => '╖',
        184 => '╕',
        185 => '╣',
        186 => '║',
        187 => '╗',
        188 => '╝',
        189 => '╜',
        190 => '╛',
        191 => '┐',
        192 => '└',
        193 => '┴',
        194 => '┬',
        195 => '├',
        196 => '─',
        197 => '┼',
        198 => '╞',
        199 => '╟',
        200 => '╚',
        201 => '╔',
        202 => '╩',
        203 => '╦',
        204 => '╠',
        205 => '═',
        206 => '╬',
        207 => '╧',
        208 => '╨',
        209 => '╤',
        210 => '╥',
        211 => '╙',
        212 => '╘',
        213 => '╒',
        214 => '╓',
        215 => '╫',
        216 => '╪',
        217 => '┘',
        218 => '┌',
        219 => '█',
        220 => '▄',
        221 => '▌',
        222 => '▐',
        223 => '▀',
        224 => 'α',
        225 => 'ß',
        226 => 'Γ',
        227 => 'π',
        228 => 'Σ',
        229 => 'σ',
        230 => 'µ',
        231 => 'τ',
        232 => 'Φ',
        233 => 'Θ',
        234 => 'Ω',
        235 => 'δ',
        236 => '∞',
        237 => 'φ',
        238 => 'ε',
        239 => '∩',
        240 => '≡',
        241 => '±',
        242 => '≥',
        243 => '≤',
        244 => '⌠',
        245 => '⌡',
        246 => '÷',
        247 => '≈',
        248 => '°',
        249 => '∙',
        250 => '·',
        251 => '√',
        252 => 'ⁿ',
        253 => '²',
        254 => '■',
        255 => ' ', // Non-breaking space
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize SDL3
    let _sdl_context = sdl3::init()?;

    // Initialize SDL3 TTF
    let ttf_context = sdl3::ttf::init().context("Failed to initialize SDL2_ttf")?;

    // Calculate font size to achieve pixel-perfect height
    // Use iterative refinement for accurate sizing
    let mut font_size = args.font_height as f32 * 0.75;
    let desired_height = args.font_height as f32;

    // Iteratively adjust font size to get exact pixel height
    for iteration in 0..15 {
        let mut font = ttf_context
            .load_font(&args.font_path, font_size)
            .context("Failed to load font")?;

        // Disable hinting for pixel-perfect rendering
        font.set_hinting(sdl3::ttf::Hinting::MONO);

        // Measure actual rendered height with a test character
        let test_surface = font
            .render("M")
            .solid(Color::RGB(255, 255, 255))
            .context("Failed to render test character")?;

        let actual_height = test_surface.height() as f32;
        let height_error = (actual_height - desired_height).abs();

        eprintln!(
            "Iteration {}: font_size={:.4}, actual_height={:.2}, target={}, error={:.2}",
            iteration, font_size, actual_height, args.font_height, height_error
        );

        // If we're within 0.5 pixels, we're close enough
        if height_error < 0.5 {
            break;
        }

        // Adjust font size based on error
        let scale_factor = desired_height / actual_height;
        font_size *= scale_factor;
    }

    // Load final font
    let mut font = ttf_context
        .load_font(&args.font_path, font_size)
        .context("Failed to load font with adjusted size")?;

    // Disable hinting for pixel-perfect rendering
    font.set_hinting(sdl3::ttf::Hinting::MONO);

    eprintln!(
        "Final font size: {:.4} pt for target height {}",
        font_size, args.font_height
    );

    // Create the atlas surface (16x16 grid)
    let atlas_width = args.font_width * 16;
    let atlas_height = args.font_height * 16;

    let mut atlas = Surface::new(
        atlas_width,
        atlas_height,
        sdl3::pixels::PixelFormat::RGBA8888,
    )?;

    // Fill with transparent background
    atlas
        .fill_rect(None, Color::RGBA(225, 225, 255, 255))
        .context("unable to fill rect")?;

    // Render each CP437 character
    for i in 0..=u8::MAX {
        let ch = get_cp437_char(i);
        let char_str = ch.to_string();

        // Render the character
        let text_surface = match font.render(&char_str).solid(Color::RGB(0, 0, 0)) {
            Ok(surface) => surface,
            Err(e) => {
                eprintln!(
                    "Skipping character '{}' (index {}) - render error: {}",
                    ch, i, e
                );
                continue;
            }
        };

        // Skip characters with zero dimensions (not in font or whitespace)
        let char_width = text_surface.width();
        let char_height = text_surface.height();

        if char_width == 0 || char_height == 0 {
            eprintln!(
                "Skipping character '{}' (index {}) - zero dimensions",
                ch, i
            );
            continue;
        }

        // Calculate grid position
        let grid_x = (i % 16) as u32;
        let grid_y = (i / 16) as u32;

        // Calculate cell position
        let cell_x = grid_x * args.font_width;
        let cell_y = grid_y * args.font_height;

        // Center the character in the cell

        let offset_x = if char_width < args.font_width {
            (args.font_width - char_width) / 2
        } else {
            0
        };

        let offset_y = if char_height < args.font_height {
            (args.font_height - char_height) / 2
        } else {
            0
        };

        let dest_rect = Rect::new(
            (cell_x + offset_x) as i32,
            (cell_y + offset_y) as i32,
            char_width.min(args.font_width),
            char_height.min(args.font_height),
        );

        // Blit the character to the atlas
        text_surface.blit(None, &mut atlas, Some(dest_rect))?;
    }
    atlas.save(&args.output).context("Failed to save PNG")?;
    println!("Font atlas saved to {}", args.output.display());
    Ok(())
}
