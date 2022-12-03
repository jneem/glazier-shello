use parley::{FontContext, Layout};
use piet_scene::glyph::pinot::types::Tag;
use piet_scene::glyph::pinot::FontRef;
use piet_scene::glyph::GlyphContext;
use piet_scene::kurbo::{Affine, PathEl, Point, Rect};
use piet_scene::*;

#[derive(Clone, Debug)]
pub struct ParleyBrush(pub Brush);

impl Default for ParleyBrush {
    fn default() -> ParleyBrush {
        ParleyBrush(Brush::Solid(Color::rgb8(0, 0, 0)))
    }
}

impl PartialEq<ParleyBrush> for ParleyBrush {
    fn eq(&self, _other: &ParleyBrush) -> bool {
        true // FIXME
    }
}

impl parley::style::Brush for ParleyBrush {}

pub fn render_text(builder: &mut SceneBuilder, transform: Affine, layout: &Layout<ParleyBrush>) {
    let mut gcx = GlyphContext::new();
    for line in layout.lines() {
        for glyph_run in line.glyph_runs() {
            let mut x = glyph_run.offset();
            let y = glyph_run.baseline();
            let run = glyph_run.run();
            let font = run.font().as_ref();
            let font_size = run.font_size();
            let font_ref = FontRef {
                data: font.data,
                offset: font.offset,
            };
            let style = glyph_run.style();
            let vars: [(Tag, f32); 0] = [];
            let mut gp = gcx.new_provider(&font_ref, None, font_size, false, vars);
            for glyph in glyph_run.glyphs() {
                if let Some(fragment) = gp.get(glyph.id, Some(&style.brush.0)) {
                    let gx = x + glyph.x;
                    let gy = y - glyph.y;
                    let xform = Affine::translate((gx as f64, gy as f64))
                        * Affine::scale_non_uniform(1.0, -1.0);
                    builder.append(&fragment, Some(transform * xform));
                }
                x += glyph.advance;
            }
        }
    }
}

pub fn render_anim_frame(scene: &mut Scene, fcx: &mut FontContext, i: u64) {
    let mut sb = SceneBuilder::for_scene(scene);
    let rect = Rect::from_origin_size(Point::new(0.0, 0.0), (1000.0, 1000.0));
    sb.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        &Brush::Solid(Color::rgb8(128, 128, 128)),
        None,
        &rect,
    );

    let scale = (i as f64 * 0.01).sin() * 0.5 + 1.5;
    let mut lcx = parley::LayoutContext::new();
    let mut layout_builder =
        lcx.ranged_builder(fcx, "Hello piet-gpu! ഹലോ ਸਤ ਸ੍ਰੀ ਅਕਾਲ مرحبا!", scale as f32);
    layout_builder.push_default(&parley::style::StyleProperty::FontSize(34.0));
    layout_builder.push(
        &parley::style::StyleProperty::Brush(ParleyBrush(Brush::Solid(Color::rgb8(255, 255, 0)))),
        6..10,
    );
    layout_builder.push(&parley::style::StyleProperty::FontSize(48.0), 6..10);
    layout_builder.push_default(&parley::style::StyleProperty::Brush(ParleyBrush(
        Brush::Solid(Color::rgb8(255, 255, 255)),
    )));
    let mut layout = layout_builder.build();
    layout.break_all_lines(None, parley::layout::Alignment::Start);
    render_text(&mut sb, Affine::translate((100.0, 400.0)), &layout);

    let th = (std::f64::consts::PI / 180.0) * (i as f64);
    let center = Point::new(500.0, 500.0);
    let mut p1 = center;
    p1.x += 400.0 * th.cos();
    p1.y += 400.0 * th.sin();
    sb.stroke(
        &Stroke::new(5.0),
        Affine::IDENTITY,
        &Brush::Solid(Color::rgb8(128, 0, 0)),
        None,
        &[PathEl::MoveTo(center), PathEl::LineTo(p1)],
    );
    sb.fill(
        Fill::NonZero,
        Affine::translate((150.0, 150.0)) * Affine::scale(0.2),
        Color::RED,
        None,
        &rect,
    );
    let alpha = (i as f64 * 0.03).sin() as f32 * 0.5 + 0.5;
    sb.push_layer(Mix::Normal, alpha, Affine::IDENTITY, &rect);
    sb.fill(
        Fill::NonZero,
        Affine::translate((100.0, 100.0)) * Affine::scale(0.2),
        Color::BLUE,
        None,
        &rect,
    );
    sb.fill(
        Fill::NonZero,
        Affine::translate((200.0, 200.0)) * Affine::scale(0.2),
        Color::GREEN,
        None,
        &rect,
    );
    sb.pop_layer();
}
