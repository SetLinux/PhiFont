use super::context::FontContext;
use super::font::Font;
use super::scripts;
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TextDirection {
    RTL = harfbuzz_sys::HB_DIRECTION_RTL as isize,
    LTR = harfbuzz_sys::HB_DIRECTION_LTR as isize,
}

pub struct DrawableGlyph {
    pub position: (f32, f32),
    pub scale: (f32, f32),
    pub codepoint: u32,
}
#[derive(PartialEq, Eq)]
pub enum TextSpacing {
    ChromeLike,
    FireFoxLike,
}

pub struct TextFeature {
    name: String,
    value: bool,
}
impl TextFeature {
    pub fn new(aname: String, avalue: bool) -> Self {
        TextFeature {
            name: aname,
            value: avalue,
        }
    }
}


pub fn draw_text(
    text: &str,
    font: &Font,
    script_type: scripts::Script,
    spacing: TextSpacing,
    position: (f32, f32),
    language : &str,
    direction : TextDirection,
    features : &[TextFeature]
) -> Vec<DrawableGlyph> {
    unsafe {
        let hfbuffer = harfbuzz_sys::hb_buffer_create();
        harfbuzz_sys::hb_buffer_add_utf8(
            hfbuffer,
            text.as_bytes().as_ptr() as *const ::std::os::raw::c_char,
            text.len() as i32,
            0,
            text.len() as i32,
        );
        harfbuzz_sys::hb_buffer_set_direction(hfbuffer, direction as _);
        harfbuzz_sys::hb_buffer_set_script(hfbuffer, script_type as _);
        harfbuzz_sys::hb_buffer_set_language(
            hfbuffer,
            harfbuzz_sys::hb_language_from_string(
                language.as_ptr() as *const ::std::os::raw::c_char,
                -1,
            ),
        );
        
        let mut vec : Vec<harfbuzz_sys::hb_feature_t> = vec![];
        for x in features.iter() {
            let mut feature : harfbuzz_sys::hb_feature_t = std::mem::uninitialized();
            if harfbuzz_sys::hb_feature_from_string(x.name.as_ptr() as _, x.name.len() as _, &mut feature) != 1 {
                panic!(format!("sorry but the feature : {:?} requested couldn't be done successfully : {:?}" , x.name,harfbuzz_sys::hb_feature_from_string(x.name.as_ptr() as _, x.name.len() as _, &mut feature)));
            }
            feature.value = x.value as _;
            vec.push(feature);
        }
        harfbuzz_sys::hb_shape(font.hb_font, hfbuffer, vec.as_slice().as_ptr() as _, vec.len() as u32);
        let mut glyph_count: std::os::raw::c_uint = 0;
        let glyphsinfo = std::slice::from_raw_parts(
            harfbuzz_sys::hb_buffer_get_glyph_infos(hfbuffer, &mut glyph_count),
            glyph_count as usize,
        )
        .to_vec();
        let glyph_kerning = std::slice::from_raw_parts(
            harfbuzz_sys::hb_buffer_get_glyph_positions(hfbuffer, &mut glyph_count),
            glyph_count as usize,
        )
        .to_vec();
        
        let mut offset = (position.0.round() + {if spacing == TextSpacing::ChromeLike {0.5}else {0.0}}, position.1.round());
        let mut result: Vec<DrawableGlyph> = vec![];
        
        for (i, c) in glyph_kerning.iter().enumerate() {
            //TODO : Make this better from perf prespective
            freetype::freetype::FT_Load_Glyph(
                font.ft_face,
                glyphsinfo[i].codepoint,
                freetype::freetype::FT_LOAD_RENDER as i32,
            );
            
            let min = na::Vector2::<f32>::new(
                (*(*font.ft_face).glyph).bitmap_left as f32,
                (*(*font.ft_face).glyph).bitmap_top as f32
                    - ((*(*font.ft_face).glyph).metrics.height as f32) / 64.0,
            );
            let max = na::Vector2::<f32>::new(
                (*(*font.ft_face).glyph).bitmap_left as f32
                    + ((*(*font.ft_face).glyph).metrics.width as f32 / 64.0),
                (*(*font.ft_face).glyph).bitmap_top as f32,
            );
            let glyph = DrawableGlyph {
                position: (
                    ((max.x + min.x) as f32 / 2.0) + offset.0 + c.x_offset as f32 / 64.0,
                    ((max.y + min.y) as f32 / 2.0) as f32 + offset.1 + c.y_offset as f32 / 64.0,
                ),
                scale: (
                    ((max.x - min.x) as f32 / 2.0),
                    ((max.y - min.y) as f32 / 2.0),
                ),
                codepoint: glyphsinfo[i].codepoint,
            };
            result.push(glyph);
            println!("TEST CHAR : {:?}",c);
            match &spacing {
                x if *x == TextSpacing::ChromeLike => {
                    offset.0 += c.x_advance as f32 / 64.0 ;
                }
                x if *x == TextSpacing::FireFoxLike => {
                    offset.0 += (c.x_advance as f32 / 64.0).round();
                }
                _ => {}
            }
        }
        result
    }
}
