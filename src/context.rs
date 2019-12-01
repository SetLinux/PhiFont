/// the FontContext should be created before any API calls because most of them will need
/// a reference to it
use super::scripts;
pub struct FontContext {
    pub ft_library: freetype::freetype::FT_Library,
}
impl FontContext {
    pub fn new() -> Self {
        unsafe {
            let mut ft_lib: freetype::freetype::FT_Library = std::mem::zeroed();
            freetype::freetype::FT_Init_FreeType(&mut ft_lib);
            FontContext { ft_library: ft_lib }
        }
    }
}
