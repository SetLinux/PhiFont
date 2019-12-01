use super::font::Font;
#[derive(Debug)]
pub struct CachedGlyph {
    pub codepoint: u32,
    pub uv_min: (f32, f32),
    pub uv_max: (f32, f32),
}
pub struct FontCache<F: Fn((f32, f32), (f32, f32), *mut u8)> {
    offset: (f32, f32),
    cache: std::collections::HashMap<u32, CachedGlyph>,
    last_glyph_height: f32,
    first_glyph_height: f32,
    current_line_height: f32,
    ft_font: freetype::freetype::FT_Face,
    width: i32,
    height: i32,
    cache_update: F,
}
impl<F: Fn((f32, f32), (f32, f32), *mut u8)> FontCache<F> {
    pub fn new(a_font: &Font, width: f32, height: f32, acache_update: F) -> Self {
        FontCache {
            width: width as i32,
            height: height as i32,
            offset: (0.0, 0.0),
            cache: std::collections::HashMap::new(),
            ft_font: a_font.ft_face,
            current_line_height: 0.0,
            last_glyph_height: 0.0,
            first_glyph_height: 0.0,
            cache_update: acache_update,
        }
    }

    pub fn cache(&mut self, symbol: u32) -> &CachedGlyph {
        //   let glyph = self.font.glyph(symbol).scaled(self.scale).positioned(point(0.0,100.0));

        if self.cache.get(&symbol).is_some() {
            return self.cache.get(&symbol).unwrap();
        }

        unsafe {
            freetype::freetype::FT_Load_Glyph(
                self.ft_font,
                symbol,
                freetype::freetype::FT_LOAD_RENDER as _,
            );
            freetype::freetype::FT_Render_Glyph(
                (*self.ft_font).glyph,
                freetype::freetype::FT_Render_Mode::FT_RENDER_MODE_LIGHT,
            );
            
            let (gwidth, gheight) = (
                (*(*self.ft_font).glyph).bitmap.width as f32,
                (*(*self.ft_font).glyph).bitmap.rows as f32,
            );

            if self.offset.0 + gwidth as f32 > self.width as f32 {
                self.current_line_height +=
                    f32::max(self.first_glyph_height, self.last_glyph_height);
                self.offset.0 = 0.0;
            }
            if self.offset.0 == 0.0 {
                self.first_glyph_height = gheight as f32;
            }

            self.last_glyph_height = gheight as f32;
            (self.cache_update)(
                (
                    self.offset.0 as _,
                    (self.height as f32 - (self.current_line_height as f32 + gheight as f32)) as _,
                ),
                (gwidth, gheight),
                (*(*self.ft_font).glyph)
                            .bitmap
                            .buffer
            );

            self.offset.0 += gwidth as f32;

            let toberett = CachedGlyph {
                codepoint: symbol,
                uv_min: (
                    self.offset.0 - gwidth as f32,
                    (self.height as f32 - (self.current_line_height as f32 + gheight as f32)),
                ),
                uv_max: (
                    self.offset.0  ,
                    (self.height as f32 - (self.current_line_height as f32 + gheight as f32))
                        + gheight as f32,
                ),
            };
            self.cache.insert(symbol, toberett);
            &self.cache[&symbol]
        }
    }
}
