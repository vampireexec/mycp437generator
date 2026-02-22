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

    /// Dump hex bitmap to console instead of saving image
    #[arg(long)]
    hex_dump: bool,
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
/// Convert a surface to hex dump format
/// Each scanline is padded to a 32-bit boundary so that font_bitmask
/// can use (x % 32) directly without needing the Y coordinate.
fn dump_surface_as_hex(surface: &Surface, char_width: u32, char_height: u32) -> Result<()> {
    let width = surface.width();
    let height = surface.height();
    // Padded width: round up to the next multiple of 32
    let padded_width = ((width + 31) / 32) * 32;

    eprintln!("// Pixel dimensions: {} wide x {} tall", width, height);
    eprintln!(
        "// Padded scanline width (map_w for shader): {}",
        padded_width
    );
    eprintln!("// Character grid: 16x16");
    eprintln!("// Character cell: {}x{} pixels", char_width, char_height);
    eprintln!("// Packing: per-row, 32-bit aligned");
    eprintln!();

    // Lock the surface to access pixel data
    surface.with_lock(|pixels: &[u8]| {
        let pitch = surface.pitch() as usize;
        let bytes_per_pixel = surface.pixel_format().bytes_per_pixel() as usize;

        let mut all_values: Vec<u32> = Vec::new();

        for y in 0..height as usize {
            // Collect bits for this scanline
            let mut bits: Vec<bool> = Vec::with_capacity(padded_width as usize);

            for x in 0..width as usize {
                let pixel_offset = y * pitch + x * bytes_per_pixel;

                let is_filled = if pixel_offset + 2 < pixels.len() {
                    let r = pixels[pixel_offset];
                    let g = pixels[pixel_offset + 1];
                    let b = pixels[pixel_offset + 2];
                    let brightness = (r as u32 + g as u32 + b as u32) / 3;
                    brightness < 128
                } else {
                    false
                };

                bits.push(is_filled);
            }

            // Pad to 32-bit boundary with zeros
            while bits.len() < padded_width as usize {
                bits.push(false);
            }

            // Pack into 32-bit integers
            for chunk in bits.chunks(32) {
                let mut value: u32 = 0;
                for (i, &bit) in chunk.iter().enumerate() {
                    if bit {
                        value |= 1 << i;
                    }
                }
                all_values.push(value);
            }
        }

        // Print 8 values per line for readability
        for (i, value) in all_values.iter().enumerate() {
            if i % 8 == 0 {
                if i > 0 {
                    println!();
                }
                print!("  ");
            } else {
                print!(" ");
            }
            print!("0x{:08X} ", value);
        }
        println!();
    });

    Ok(())
}
fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize SDL3
    let _sdl_context = sdl3::init()?;

    // Initialize SDL3 TTF
    let ttf_context = sdl3::ttf::init().context("Failed to initialize SDL2_ttf")?;

    // --- Step 1: Find the right font size ---
    // We want the font's rendered height (ascent - descent) == font_height.
    // No vertical padding — the cell height matches the metric height exactly.
    let mut font_size = 1.0; // Start with a reasonable default size
    let cp437chars_string = (32..=126).map(get_cp437_char).collect::<String>();

    for iteration in 1..45 {
        font_size = 45.0 - iteration as f32;

        let mut font = ttf_context
            .load_font(&args.font_path, font_size)
            .context("Failed to load font")?;
        font.set_hinting(sdl3::ttf::Hinting::NONE);

        let texture = font
            .render(&cp437chars_string)
            .solid(Color::RGB(0, 0, 0))
            .context("Failed to render text for size estimation")?;
        let total_height = texture.height() as i32; // Total vertical space needed for baseline alignment

        if total_height <= args.font_height as i32 {
            font_size += 1.0; // Step back to the last size that fit
            break;
        }
    }

    // --- Step 2: Load final font and log metrics ---
    let mut font = ttf_context
        .load_font(&args.font_path, font_size)
        .context("Failed to load font with adjusted size")?;
    font.set_hinting(sdl3::ttf::Hinting::MONO);

    eprintln!(
        "Final: font_size={:.4}pt, ascent={}, descent={}, height={}",
        font_size,
        font.ascent(),
        font.descent(),
        font.height()
    );
    eprintln!(
        "Cell: {}x{} (horizontal centering, baseline-aligned vertically)",
        args.font_width, args.font_height
    );

    // Create the atlas surface (16x16 grid)
    let atlas_width = args.font_width * 16;
    let atlas_height = args.font_height * 16;

    let mut atlas = Surface::new(atlas_width, atlas_height, sdl3::pixels::PixelFormat::RGB24)?;

    // Fill with solid background
    atlas
        .fill_rect(None, Color::RGB(255, 255, 255))
        .context("unable to fill rect")?;

    // --- Step 3: Render each CP437 character ---
    // SDL_ttf's solid() renderer already positions glyphs with the baseline at
    // y=ascent from the top of the surface. So we just blit at y=0 in each cell
    // and all characters will be baseline-aligned automatically.
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

        let glyph_w = text_surface.width();
        let glyph_h = text_surface.height();

        if glyph_w == 0 || glyph_h == 0 {
            eprintln!(
                "Skipping character '{}' (index {}) - zero dimensions",
                ch, i
            );
            continue;
        }

        // Calculate grid position
        let grid_x = (i % 16) as u32;
        let grid_y = (i / 16) as u32;

        // Top-left of cell in atlas
        let cell_x = (grid_x * args.font_width) as i32;
        let cell_y = (grid_y * args.font_height) as i32;

        // Horizontal: center glyph in cell
        let offset_x = (args.font_width as i32 - glyph_w as i32) / 2;

        // Vertical: blit at top of cell (SDL_ttf already handles baseline positioning)
        let offset_y = 0;

        let dest_rect = Rect::new(cell_x + offset_x, cell_y + offset_y, glyph_w, glyph_h);
        if dest_rect.width() > args.font_width
            || dest_rect.height() > args.font_height
            || dest_rect.x() + dest_rect.width() as i32 > cell_x + args.font_width as i32
            || dest_rect.y() + dest_rect.height() as i32 > cell_y + args.font_height as i32
        {
            eprintln!(
                "Warning: character '{}' (index {}) exceeds cell size - dest_rect={:?} cell_x={}, cell_y={} offset_x={}, offset_y={}",
                ch, i, dest_rect, cell_x, cell_y, offset_x, offset_y
            );
        }

        // Blit the character to the atlas
        text_surface.blit(None, &mut atlas, Some(dest_rect))?;
    }

    if args.hex_dump {
        dump_surface_as_hex(&atlas, args.font_width, args.font_height)?;
    } else {
        atlas.save(&args.output).context("Failed to save PNG")?;
        println!("Font atlas saved to {}", args.output.display());
    }

    Ok(())
}
