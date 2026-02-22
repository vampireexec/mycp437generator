use anyhow::{Context, Result, bail};
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
    #[arg(long)]
    font_path: PathBuf,

    /// Width of each character cell in pixels
    #[arg(long)]
    font_width: u32,

    /// Output PNG file path
    /// Ignored if --hex-dump is provided
    #[arg(long)]
    output: Option<PathBuf>,

    /// Dump hex bitmap to console instead of saving image
    #[arg(long)]
    hex_dump: Option<String>,

    /// Enable debug output
    #[arg(long)]
    debug: bool,
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
fn dump_surface_as_hex<T: AsRef<str>>(
    surface: &Surface,
    char_width: u32,
    char_height: u32,
    name: T,
) -> Result<()> {
    let width = surface.width();
    let height = surface.height();
    // Padded width: round up to the next multiple of 32
    let padded_width = ((width + 31) / 32) * 32;

    println!("// Pixel dimensions: {} wide x {} tall", width, height);
    println!(
        "// Padded scanline width (map_w for shader): {}",
        padded_width
    );
    println!("// Character grid: 16x16");
    println!("// Character cell: {}x{} pixels", char_width, char_height);
    println!("// Packing: per-row, 32-bit aligned");
    println!();

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

        println!("//!LONGVAR uint[] font_data_{}", name.as_ref());
        // Print 8 values per line for readability
        for (i, value) in all_values.iter().enumerate() {
            if i % 8 == 0 {
                if i > 0 {
                    println!();
                }
                print!("//!  ");
            } else {
                print!(" ");
            }
            print!("0x{:08X} ", value);
        }
        println!();
        println!("//!ENDLONGVAR");
        println!("#define font_{}_width ({})", name.as_ref(), char_width);
        println!("#define font_{}_height ({})", name.as_ref(), char_height);
        println!("#define font_{}(uv,pos,txt,start,len) (fontstr(uv,pos,txt,start,len,{char_width},{char_height},{padded_width},{}))", name.as_ref(), name.as_ref());
        println!("#define multiline_{}(uv,pos,txt,starts,lens) multiline_font((uv), (pos), (txt), (starts), (lens), {char_width}, {char_height}, {padded_width}, {})", name.as_ref(), name.as_ref())});

    Ok(())
}
fn main() -> Result<()> {
    let args = Args::parse();

    if args.output.is_none() && args.hex_dump.is_none() {
        bail!("Error: either --output or --hex-dump must be provided");
    }

    // Initialize SDL3
    let _sdl_context = sdl3::init()?;

    // Initialize SDL3 TTF
    let ttf_context = sdl3::ttf::init().context("Failed to initialize SDL2_ttf")?;

    // --- Step 1: Find the right font size ---
    // Iterate font sizes until the widest CP437 glyph's width == font_width.
    let mut font_size = 1.0_f32;
    let mut max_width: u32 = 0;
    for iteration in 1..128 {
        font_size = iteration as f32; // Start with integer sizes for faster convergence
        let mut font = ttf_context
            .load_font(&args.font_path, font_size)
            .context("Failed to load font")?;
        font.set_hinting(sdl3::ttf::Hinting::NONE);

        // Find the widest glyph across all 256 CP437 characters
        max_width = 0;
        for i in 0..=u8::MAX {
            let ch = get_cp437_char(i);
            let metrics = match font.find_glyph_metrics(ch) {
                Some(m) => m,
                None => continue, // Character not in font, skip
            };
            max_width = max_width.max(metrics.maxx as u32);
        }

        if max_width >= args.font_width {
            eprintln!(
                "Iteration {}: font_size={:.4}pt, max_width={} == font_width={} — done",
                iteration, font_size, max_width, args.font_width
            );
            break;
        }
        eprintln!(
            "Iteration {}: font_size={:.4}pt, max_width={} < font_width={}",
            iteration, font_size, max_width, args.font_width
        );
    }

    // --- Step 2: Load final font, derive cell dimensions ---
    let mut font = ttf_context
        .load_font(&args.font_path, font_size)
        .context("Failed to load font with adjusted size")?;
    font.set_hinting(sdl3::ttf::Hinting::NONE);

    let font_width = max_width;

    // --- Step 3: Render all 256 glyphs, find true cell height, build atlas ---
    // shaded() produces surfaces where baseline is at font.ascent() from top,
    // so blitting all at y=0 gives automatic baseline alignment.
    // First pass: render all chars and find the max surface height.
    let mut rendered: Vec<(u8, char, Option<Surface>)> = Vec::with_capacity(256);

    let cp437_all_string = (0..=u8::MAX).map(|i| get_cp437_char(i)).collect::<String>();
    let texture = match font
        .render(&cp437_all_string)
        .shaded(Color::RGB(0, 0, 0), Color::RGB(255, 255, 255))
    {
        Ok(s) => s,
        Err(e) => {
            bail!(
                "Warning: failed to render all CP437 chars in one string: {}",
                e
            );
        }
    };
    let font_height = texture.height();
    // If no glyphs rendered, fall back to font.height()
    if font_height == 0 {
        bail!(
            "Error: all rendered glyphs have zero height. This likely means the font size is too small or the font file is invalid."
        );
    }

    for i in 0..=u8::MAX {
        let ch = get_cp437_char(i);
        let surface = font
            .render(&ch.to_string())
            .shaded(Color::RGB(0, 0, 0), Color::RGB(255, 255, 255))
            .ok();
        rendered.push((i, ch, surface));
    }

    eprintln!(
        "Final: font_size={:.4}pt, ascent={}, descent={}, height={}, max_width={}",
        font_size,
        font.ascent(),
        font.descent(),
        font.height(),
        max_width
    );
    eprintln!(
        "Cell: {}x{} (width specified, height derived)",
        font_width, font_height
    );

    // --- Step 3: Render each character individually into a 16x16 grid atlas ---
    let atlas_width = font_width * 16;
    let atlas_height = font_height * 16;

    let mut atlas = Surface::new(atlas_width, atlas_height, sdl3::pixels::PixelFormat::RGB24)?;

    // Fill with solid white background
    atlas
        .fill_rect(None, Color::RGB(255, 255, 255))
        .context("unable to fill rect")?;

    // Second pass: blit all pre-rendered surfaces into the atlas.
    // Since shaded() places the baseline at font.ascent() from the top of every
    // surface, blitting at y=0 in each cell keeps all glyphs baseline-aligned.
    for &(i, ch, ref surface_opt) in &rendered {
        let char_surface = match surface_opt {
            Some(s) => s,
            None => {
                if args.debug {
                    eprintln!("Skipping '{}' (index {}) — not in font", ch, i);
                }
                continue;
            }
        };

        let col = i % 16;
        let row = i / 16;
        let cell_x = col as i32 * font_width as i32;
        let cell_y = row as i32 * font_height as i32;

        // Horizontal: center glyph in cell
        let x_offset = ((font_width as i32 - char_surface.width() as i32) / 2).max(0);

        let metrics = match font.find_glyph_metrics(get_cp437_char(i)) {
            Some(m) => m,
            None => {
                if args.debug {
                    eprintln!(
                        "Warning: failed to get metrics for char '{}' (index {}) — skipping",
                        ch, i
                    );
                }
                continue;
            }
        };

        if metrics.miny == metrics.maxy || metrics.minx == metrics.maxx {
            if args.debug {
                eprintln!(
                    "Warning: char '{}' (index {}) has a zero dimension (miny == maxy == {} or minx == maxx == {}) — skipping",
                    ch, i, metrics.miny, metrics.minx
                );
            }
            continue;
        }

        let y_offset = if char_surface.height() == font_height {
            0
        } else if metrics.miny + font.descent() <= 1 {
            //descent without ascent
            font_height as i32 - char_surface.height() as i32
        } else {
            //no ascent or descent
            0
        };

        if y_offset + char_surface.height() as i32 > font_height as i32 {
            eprintln!(
                "Warning: char '{}' (index {}) has a y_offset={} that causes it to exceed cell height ({} + {} > {})",
                ch,
                i,
                y_offset,
                y_offset,
                char_surface.height(),
                font_height
            );
        }

        let dst_rect = Rect::new(
            cell_x + x_offset,
            cell_y + y_offset,
            char_surface.width().min(font_width),
            char_surface.height().min(font_height),
        );

        if args.debug {
            eprintln!(
                "{}  miny={}, maxy={}, asc={}, dsc={}, intern={}, tex_height={}, font_height={}, y_offset={}",
                ch,
                metrics.miny,
                metrics.maxy,
                font.ascent(),
                font.descent(),
                font.height(),
                char_surface.height(),
                font_height,
                y_offset
            );
        }
        char_surface.blit(None, &mut atlas, Some(dst_rect))?;
    }

    eprintln!("Atlas: {}x{}", atlas_width, atlas_height);

    if let Some(name) = &args.hex_dump {
        dump_surface_as_hex(&atlas, font_width, font_height, name)?;
    } else if let Some(path) = &args.output {
        atlas.save(path).context("Failed to save PNG")?;
        println!("Font atlas saved to {}", path.display());
    }

    Ok(())
}
