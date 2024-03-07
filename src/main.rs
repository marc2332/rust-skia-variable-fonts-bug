pub use skia_safe::{
    font_style::{Slant, Weight, Width},
    gpu::{
        backend_render_targets,
        gl::{Format, FramebufferInfo, Interface},
        surfaces::wrap_backend_render_target,
        BackendRenderTarget, DirectContext, RecordingContext, SurfaceOrigin,
    },
    gradient_shader::GradientShaderColors,
    path::ArcSize,
    rrect::Corner,
    runtime_effect::Uniform,
    svg,
    textlayout::{
        paragraph::GlyphClusterInfo, Decoration, FontCollection, FontFeature, LineMetrics,
        Paragraph, ParagraphBuilder, ParagraphStyle, PlaceholderStyle, PositionWithAffinity,
        RectHeightStyle, RectWidthStyle, StrutStyle, TextAlign, TextBaseline, TextBox,
        TextDecoration, TextDecorationStyle, TextDirection, TextHeightBehavior, TextIndex,
        TextRange, TextShadow, TextStyle, TypefaceFontProvider,
    },
    BlurStyle, ClipOp, Color, ColorSpace, ColorType, Data, FilterMode, FontArguments, FontMgr,
    FontStyle, IRect, Image, MaskFilter, Matrix, Paint, PaintStyle, Path, PathDirection, Point,
    RRect, Rect, RuntimeEffect, Shader, Surface, TileMode, Typeface, HSV, RGB,
};
use skia_safe::{surfaces, EncodedImageFormat};

use std::fs::File;
use std::io::Write;

static INTER_VARIABLE: &[u8] = include_bytes!("../inter-variable.ttf");

fn main() {
    let mut surface = surfaces::raster_n32_premul((500, 150)).expect("surface");
    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.set_anti_alias(true);
    surface.canvas().clear(Color::WHITE);

    let def_mgr = FontMgr::default();
    let mut provider = TypefaceFontProvider::new();
    let typeface = def_mgr.new_from_data(INTER_VARIABLE, None).unwrap();
    provider.register_typeface(typeface, Some("inter-variable"));

    let font_mgr: FontMgr = provider.into();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(def_mgr, "inter-variable");
    font_collection.set_dynamic_font_manager(font_mgr);
    let paragraph_style = ParagraphStyle::default();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut text_style = TextStyle::new();

    // Hmm
    let font_style = FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Upright);

    text_style.set_font_style(font_style);
    text_style.set_color(Color::BLACK);
    text_style.set_font_size(50.);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text("Hello, World!");
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(350.);
    paragraph.paint(surface.canvas(), (0., 0.));

    let image = surface.image_snapshot();
    let mut context = surface.direct_context();
    let data = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();
}
