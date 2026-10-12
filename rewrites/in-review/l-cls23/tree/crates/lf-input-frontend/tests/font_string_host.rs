//! Host tests for the lifted UI font string: edge cases a reader of the
//! code would ask about. No addresses, no rewrites; runs everywhere.

use lf_input_frontend::font_string::{
    DEFAULT_COLOUR, FontString, FontWorld, MeasureInputs, RESET_TAG, StringRow, UI_DRAW_MODE,
    registry, text_with_nul, up_to_nul,
};
use std::cell::Cell;

/// A world that answers every query from fields and records pushes.
#[derive(Default)]
struct Still {
    veto: bool,
    visible: bool,
    metrics: [f32; 4],
    advance: f32,
    height: f32,
    line: f32,
    converted: [u32; 8],
    submit: u32,
    sink: u32,
    resolve: [u32; 2],
    pick_a: bool,
    pick_b: bool,
    last_style: Cell<u32>,
    last_size: Cell<u32>,
    sizes: Cell<usize>,
}

impl Still {
    fn with_sink(sink: u32) -> Self {
        Self {
            sink,
            ..Still::default()
        }
    }
}

impl FontWorld for Still {
    fn notify_position(&mut self, _snap: [u32; 4]) {}
    fn resolve_text(&mut self, _key: u32) -> [u32; 2] {
        self.resolve
    }
    fn resolve_sized(&mut self, size: u32) -> [u32; 2] {
        self.last_size.set(size);
        self.resolve
    }
    fn refresh_parent(&mut self) {}
    fn copy_text(&mut self, _text: &[u8; 256]) {}
    fn guard_veto(&mut self) -> bool {
        self.veto
    }
    fn sink_primary(&mut self, _bits: u32) -> u32 {
        self.sink
    }
    fn notify_a(&mut self) {}
    fn notify_b(&mut self) {}
    fn metric_a(&mut self) -> f32 {
        self.metrics[0]
    }
    fn metric_b(&mut self) -> f32 {
        self.metrics[1]
    }
    fn metric_c(&mut self) -> f32 {
        self.metrics[2]
    }
    fn metric_d(&mut self) -> f32 {
        self.metrics[3]
    }
    fn render(&mut self) -> u32 {
        self.sink
    }
    fn refresh_base(&mut self) {}
    fn push_scale(&mut self, _bits: u32) {}
    fn visible(&mut self) -> bool {
        self.visible
    }
    fn advance(&mut self) -> f32 {
        self.advance
    }
    fn push_style(&mut self, style: u32) {
        self.last_style.set(style);
    }
    fn push_position(&mut self, _lo: u32, _hi: u32) {}
    fn push_a(&mut self, _bits: u32) {}
    fn push_colour(&mut self, _colour: u32) {}
    fn push_row_word(&mut self, _bits: u32) {}
    fn push_one(&mut self) {}
    fn push_width(&mut self, _base: u32, _add: u32) {}
    fn push_opacity(&mut self, _bits: u32) {}
    fn convert_text(&mut self, _text: &[u8]) -> [u32; 8] {
        self.converted
    }
    fn resolve_cached(&mut self, _text: &[u8]) -> [u32; 8] {
        self.converted
    }
    fn submit(&mut self, _a: u32, _b: u32, _words: &[u32; 8]) -> u32 {
        self.submit
    }
    fn query_height(&mut self, _words: &[u32; 8]) -> f32 {
        self.height
    }
    fn line_height(&mut self) -> f32 {
        self.line
    }
    fn sink_height(&mut self, _bits: u32) {}
    fn sink_line(&mut self, _bits: u32) {}
    fn sink_scaled(&mut self, _bits: u32) {}
    fn pick_ext_a(&mut self) -> bool {
        self.pick_a
    }
    fn pick_ext_b(&mut self) -> bool {
        self.pick_b
    }
    fn end_frame(&mut self) {}
}

fn blank(rows: Vec<StringRow>) -> FontString {
    FontString::new(
        0,
        0,
        0,
        0.0,
        1.0,
        0.0,
        1.0,
        0,
        0.0,
        0.0,
        [0, 0],
        0,
        0,
        0,
        0,
        0,
        0,
        [0; 256],
        rows,
    )
}

fn row_with(text: &[u8], ready: bool) -> StringRow {
    StringRow {
        text: text.to_vec(),
        ready,
        ..StringRow::default()
    }
}

#[test]
fn registry_counts_are_pinned() {
    assert_eq!(registry::counts(), (18, 0, 1));
    assert_eq!(registry::ROWS.len(), 19);
}

#[test]
fn style_round_trips() {
    let mut s = blank(Vec::new());
    assert_eq!(s.set_style(0x1234_5678), 0x1234_5678);
    let mut out = 0;
    s.style_word_into(&mut out);
    assert_eq!(out, 0x1234_5678);
    assert_eq!(s.style(), 0x1234_5678);
}

#[test]
fn float_setters_keep_bits() {
    let mut s = blank(Vec::new());
    for bits in [0x7FC0_0001, 0x8000_0000, 0x0000_0001, 0xFF80_0000] {
        let v = f32::from_bits(bits);
        s.set_size(v);
        assert_eq!(s.size().to_bits(), bits);
        s.set_push_a(v);
        s.set_push_b(v);
        assert_eq!(s.pushed().0.to_bits(), bits);
        assert_eq!(s.pushed().1.to_bits(), bits);
    }
}

#[test]
fn flag_setters_store_bytes() {
    let mut s = blank(Vec::new());
    s.set_styled(0xA1);
    s.set_flag_209(0xB2);
    s.set_flag_20a(0xC3);
    s.set_flag_20d(0xD4);
    assert_eq!(s.flags(), [0xA1, 0xB2, 0xC3, 0, 0, 0xD4]);
}

#[test]
fn snapshot_width_pair_follows_mode() {
    let mut text = [0u8; 256];
    text[0] = b'A';
    let mut s = FontString::new(
        0,
        0,
        0x11,
        2.0,
        3.0,
        0.0,
        1.0,
        0x22,
        5.0,
        6.0,
        [7, 8],
        1,
        0,
        0,
        1,
        0,
        0,
        text,
        vec![StringRow::default()],
    );
    s.snapshot_row(0);
    let row = &s.rows()[0];
    assert_eq!(row.width_base.to_bits(), 5.0f32.to_bits());
    assert_eq!(row.width_add.to_bits(), (3.0f32 + 5.0).to_bits());
    assert_eq!(row.text, vec![b'A', 0]);
    assert!(row.ready);
    assert_eq!(row.use_scratch, 1);
    assert_eq!((row.colour, row.tag), (0x11, 0x22));

    let mut s2 = blank(vec![StringRow::default()]);
    s2.snapshot_row(0);
    let row = &s2.rows()[0];
    assert_eq!(row.width_base.to_bits(), 0.0f32.to_bits());
    assert_eq!(row.width_add.to_bits(), 1.0f32.to_bits());
}

#[test]
fn emit_early_outs_answer_shifted_slot() {
    let mut world = Still::default();
    let mut s = blank(vec![row_with(&[0], true)]);
    assert_eq!(s.emit_row(&mut world, 0, 0, 0), 0);
    assert!(s.rows()[0].ready);
    let mut s = blank(vec![
        row_with(&[b'Q', 0], true),
        row_with(&[b'Q', 0], true),
        row_with(&[b'Q', 0], false),
    ]);
    assert_eq!(s.emit_row(&mut world, 2, 0, 0), 2 << 8);
}

#[test]
fn emit_clears_ready_and_answers_submit() {
    let mut world = Still {
        submit: 0xBEEF,
        ..Still::default()
    };
    let mut s = blank(vec![row_with(&[b'Q', 0], true)]);
    assert_eq!(s.emit_row(&mut world, 0, 0, 0), 0xBEEF);
    assert!(!s.rows()[0].ready);
}

#[test]
fn emit_style_override_needs_unstyled() {
    // Draw mode + unstyled forces style 2.
    let mut world = Still::default();
    let mut row = row_with(&[b'Q', 0], true);
    row.tag = 0x77;
    blank(vec![row]).emit_row(&mut world, 0, UI_DRAW_MODE, 0);
    assert_eq!(world.last_style.get(), 2);
    // Styled keeps the row tag even in draw mode.
    let mut world = Still::default();
    let mut row = row_with(&[b'Q', 0], true);
    row.tag = 0x77;
    let mut s = blank(vec![row]);
    s.set_styled(1);
    s.emit_row(&mut world, 0, UI_DRAW_MODE, 0);
    assert_eq!(world.last_style.get(), 0x77);
    // Alt mode nonzero also forces style 2 while unstyled.
    let mut world = Still::default();
    let mut row = row_with(&[b'Q', 0], true);
    row.tag = 0x77;
    blank(vec![row]).emit_row(&mut world, 0, 0, 1);
    assert_eq!(world.last_style.get(), 2);
    // Neither mode: the row tag.
    let mut world = Still::default();
    let mut row = row_with(&[b'Q', 0], true);
    row.tag = 0x77;
    blank(vec![row]).emit_row(&mut world, 0, 0, 0);
    assert_eq!(world.last_style.get(), 0x77);
}

#[test]
fn reset_veto_changes_nothing() {
    let mut world = Still {
        veto: true,
        ..Still::default()
    };
    let before = blank(Vec::new());
    let mut s = before.clone();
    s.reset(&mut world, 9.0, 1, 2, 3, 4.0);
    assert_eq!(s, before);
}

#[test]
fn reset_installs_defaults() {
    let mut world = Still::default();
    let mut s = blank(Vec::new());
    s.set_styled(5);
    s.reset(&mut world, 9.0, 0xAA, 0xBB, 0xCC, 4.0);
    assert_eq!(s.style(), 0xAA);
    assert_eq!(s.mode(), 0xBB);
    assert_eq!(s.tags(), (0xCC, RESET_TAG));
    assert_eq!(s.size().to_bits(), 9.0f32.to_bits());
    assert_eq!(s.scale().to_bits(), 1.0f32.to_bits());
    assert_eq!(s.pushed(), (f32::from_bits(0), 1.0));
    assert_eq!(s.flags(), [0, 0, 0, 0, 0, 0]);
}

#[test]
fn refresh_modes_combine_metrics() {
    // Mode 0: m1 - m3*scale, m2 - m4*scale.
    let mut world = Still {
        metrics: [10.0, 20.0, 3.0, 4.0],
        ..Still::default()
    };
    let mut s = blank(Vec::new());
    s.reset(&mut Still::default(), 0.0, 0, 0, 0, 1.0);
    s.refresh(&mut world, 2.0);
    assert_eq!(s.corners().0.to_bits(), (10.0f32 - 3.0 * 2.0).to_bits());
    assert_eq!(s.corners().1.to_bits(), (20.0f32 - 4.0 * 2.0).to_bits());
    // Mode 1: m3*scale + m1.
    let mut world = Still {
        metrics: [10.0, 20.0, 3.0, 4.0],
        ..Still::default()
    };
    let mut s = blank(Vec::new());
    s.reset(&mut Still::default(), 0.0, 0, 1, 0, 1.0);
    s.refresh(&mut world, 2.0);
    assert_eq!(s.corners().0.to_bits(), (3.0f32 * 2.0 + 10.0).to_bits());
    // Any other mode: m1 alone.
    let mut world = Still {
        metrics: [10.0, 20.0, 3.0, 4.0],
        ..Still::default()
    };
    let mut s = blank(Vec::new());
    s.reset(&mut Still::default(), 0.0, 0, 0xFFFF, 0, 1.0);
    s.refresh(&mut world, 2.0);
    assert_eq!(s.corners().0.to_bits(), 10.0f32.to_bits());
}

#[test]
fn refresh_answers_render() {
    let world = Still::with_sink(0x77AA);
    let mut world = world;
    let mut s = blank(Vec::new());
    assert_eq!(s.refresh(&mut world, 1.0), 0x77AA);
}

#[test]
fn measure_answers_last_sink() {
    let inputs = MeasureInputs {
        ui_mode: 0,
        ui_mode_alt: 0,
        scale_mul: 2.0,
        scale_base: 3.0,
        scale_div: 4.0,
        extents: [800, 600, 1024, 768],
    };
    let mut world = Still {
        sink: 0xCAFE,
        height: 5.0,
        ..Still::default()
    };
    let mut s = blank(Vec::new());
    assert_eq!(s.measure(&mut world, &inputs), 0xCAFE);
}

#[test]
fn select_handle_sizes_by_kind() {
    let mut world = Still::default();
    let mut s = blank(Vec::new());
    let mut seen = Vec::new();
    for kind in [0u32, 2, 1, 5, u32::MAX] {
        s.select_handle(&mut world, kind);
        seen.push(world.last_size.get());
    }
    assert_eq!(seen, vec![2, 8, 8, 8, 8]);
    let _ = world.sizes.get();
}

#[test]
fn set_text_forces_terminator_only() {
    let mut world = Still::default();
    let buf = [0x41u8; 256];
    let text = [0x43u8; 256];
    let mut s = FontString::new(
        0,
        0,
        0,
        0.0,
        1.0,
        0.0,
        1.0,
        0,
        0.0,
        0.0,
        [0, 0],
        0,
        0,
        0,
        0,
        0,
        0,
        text,
        Vec::new(),
    );
    s.set_text(&mut world, Some(&buf), false);
    // The copier's copy is the world's work: only the terminator moves.
    assert_eq!(&s.text()[..255], &[0x43u8; 255]);
    assert_eq!(s.text()[255], 0);
    let before = s.clone();
    s.set_text(&mut world, None, false);
    assert_eq!(s, before);
}

#[test]
fn text_helpers_cut_at_first_nul() {
    assert_eq!(up_to_nul(&[b'a', b'b', 0, b'c']), &[b'a', b'b']);
    assert_eq!(text_with_nul(&[b'a', 0, b'c'], 1), &[b'a', 0]);
    assert_eq!(DEFAULT_COLOUR, 0xFF00_0000);
}

#[test]
fn position_setter_stores_pair() {
    let mut world = Still::default();
    let mut s = blank(Vec::new());
    s.set_position(&mut world, 1.5, -2.5, true);
    assert_eq!(s.pos(), [1.5f32.to_bits(), (-2.5f32).to_bits()]);
    s.resolve_handle(&mut world, 0x1234);
    assert_eq!(s.pos(), [0, 0]);
}

#[test]
#[should_panic(expected = "past the rows")]
fn snapshot_past_rows_panics() {
    blank(Vec::new()).snapshot_row(0);
}

#[test]
#[should_panic(expected = "past the rows")]
fn emit_past_rows_panics() {
    blank(Vec::new()).emit_row(&mut Still::default(), 3, 0, 0);
}

#[test]
#[should_panic(expected = "NUL-terminated")]
fn snapshot_unterminated_text_panics() {
    let mut t = FontString::new(
        0,
        0,
        0,
        0.0,
        1.0,
        0.0,
        1.0,
        0,
        0.0,
        0.0,
        [0, 0],
        0,
        0,
        0,
        0,
        0,
        0,
        [0x41; 256],
        vec![StringRow::default()],
    );
    t.snapshot_row(0);
}

#[test]
#[should_panic(expected = "NUL-terminated")]
fn measure_unterminated_text_panics() {
    let inputs = MeasureInputs {
        ui_mode: 0,
        ui_mode_alt: 0,
        scale_mul: 1.0,
        scale_base: 1.0,
        scale_div: 1.0,
        extents: [1, 1, 1, 1],
    };
    let mut t = FontString::new(
        0,
        0,
        0,
        0.0,
        1.0,
        0.0,
        1.0,
        0,
        0.0,
        0.0,
        [0, 0],
        0,
        0,
        0,
        0,
        0,
        0,
        [0x41; 256],
        Vec::new(),
    );
    t.measure(&mut Still::default(), &inputs);
}
