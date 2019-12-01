use super::context;
use std::fs;
/// the main Font Object contains the FreeType Face with the Size used in it
/// the size used in this is the size that the FontCache will use

pub struct Font {
    pub ft_face: freetype::freetype::FT_Face,
    pub hb_font: *mut harfbuzz_sys::hb_font_t,
    size: i32,
    font_data: Vec<u8>,
}
impl Font {
    pub fn new(
        context: &context::FontContext,
        font_file: String,
        size_font: i32,
    ) -> Result<Self, Box<std::io::Error>> {
        unsafe {
            let mut ft_face: freetype::freetype::FT_Face = std::mem::zeroed();
            let font_data = fs::read(font_file.as_str())?;
            freetype::freetype::FT_New_Memory_Face(
                context.ft_library,
                std::mem::transmute((font_data).as_ptr()),
                font_data.len() as i64,
                0,
                &mut ft_face,
            );

            freetype::freetype::FT_Set_Char_Size(ft_face, 0, size_font as i64 * 64, 0, 0);
            //        freetype::freetype::FT_Set_Pixel_Sizes(ft_face,128, 128);
            let fonter = harfbuzz_sys::hb_ft_font_create_referenced(ft_face);
            Ok(Font {
                ft_face: ft_face,
                hb_font: fonter,
                size: size_font,
                font_data: font_data,
            })
        }
    }
}
impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            freetype::freetype::FT_Done_Face(self.ft_face);
            harfbuzz_sys::hb_font_destroy(self.hb_font);
        }
    }
}
