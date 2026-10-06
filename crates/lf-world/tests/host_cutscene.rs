//! Host tests for the lifted cutscene object: edge cases a reader would ask
//! about. These run on the 64-bit host; the differential proof against the
//! verified rewrites lives in the `lf-cutdiff` test crate.

use lf_core::Handle32;
use lf_world::cutscene_object::{
    Accumulator, AttachTag, BlockTag, BoneRow, BoneTag, BoundsRect, BoundsScale, ChainTag, CtxTag,
    CutsceneObject, CutsceneWorld, DrawTag, EarlyTag, EntryTag, HelperTag, Matrix34, MemberTag,
    PlacementTag, PoseRecord, UpdateEntry, UpdateScalars, WorldBounds, registry,
};
use std::collections::VecDeque;

/// A scripted world fake for host tests.
#[derive(Default)]
struct Fake {
    matrix: VecDeque<[u32; 16]>,
    corners: VecDeque<[f32; 2]>,
    poses: VecDeque<PoseRecord>,
    scalars: VecDeque<u32>,
    emits: VecDeque<Option<Handle32<DrawTag>>>,
    log: Vec<String>,
}

impl Fake {
    fn new() -> Self {
        Self::default()
    }
}

impl CutsceneWorld for Fake {
    fn placement_matrix(&mut self, _p: Handle32<PlacementTag>) -> [u32; 16] {
        self.log.push("matrix".to_string());
        self.matrix.pop_front().expect("matrix answer queued")
    }

    fn transform_corner(&mut self, _p: Handle32<PlacementTag>, corner: [f32; 3]) -> [f32; 2] {
        self.log.push(format!(
            "corner:{:08x}:{:08x}:{:08x}",
            corner[0].to_bits(),
            corner[1].to_bits(),
            corner[2].to_bits()
        ));
        self.corners.pop_front().expect("corner answer queued")
    }

    fn pose_slot(&mut self) -> PoseRecord {
        self.log.push("pose".to_string());
        self.poses.pop_front().expect("pose answer queued")
    }

    fn pose_followup(&mut self) -> u32 {
        self.log.push("followup".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn notify(&mut self, word: u32) -> u32 {
        self.log.push(format!("notify:{word:#x}"));
        self.scalars.pop_front().expect("scalar queued")
    }

    fn member_forward(&mut self, _m: Handle32<MemberTag>, word: u32) -> u32 {
        self.log.push(format!("forward:{word:#x}"));
        self.scalars.pop_front().expect("scalar queued")
    }

    fn refresh_hook(&mut self) {
        self.log.push("hook".to_string());
    }

    fn refresh_run(&mut self) -> u32 {
        self.log.push("refresh".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn helper_command(&mut self, _h: Handle32<HelperTag>, a: u32, b: u32) -> u32 {
        self.log.push(format!("helper:{a:#x}:{b:#x}"));
        self.scalars.pop_front().expect("scalar queued")
    }

    fn draw_mode0_a(&mut self, a0: u32, a1: u32) {
        self.log.push(format!("draw.a:{a0:#x}:{a1:#x}"));
    }

    fn draw_mode0_b(&mut self, a0: u32, a1: u32) {
        self.log.push(format!("draw.b:{a0:#x}:{a1:#x}"));
    }

    fn draw_emit(&mut self, a0: u32, a1: u32, a2: u32) -> Option<Handle32<DrawTag>> {
        self.log.push(format!("draw.emit:{a0:#x}:{a1:#x}:{a2:#x}"));
        self.emits.pop_front().expect("emit answer queued")
    }

    fn mark_emitted(&mut self, _t: Handle32<DrawTag>) {
        self.log.push("draw.mark".to_string());
    }

    fn destroy_member(&mut self, _m: Handle32<MemberTag>) {
        self.log.push("destroy".to_string());
    }

    fn teardown_block(&mut self, _b: Handle32<BlockTag>) {
        self.log.push("block.teardown".to_string());
    }

    fn free_block(&mut self, _b: Handle32<BlockTag>) {
        self.log.push("block.free".to_string());
    }

    fn ask_registry(&mut self) -> u32 {
        self.log.push("reg.ask".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn registry_word(&mut self, index: i16) -> i32 {
        self.log.push(format!("reg.word:{index}"));
        self.scalars.pop_front().expect("scalar queued") as i32
    }

    fn ask_gate(&mut self, word: u32) -> u32 {
        self.log.push(format!("reg.gate:{word:#x}"));
        self.scalars.pop_front().expect("scalar queued")
    }

    fn registry_run(&mut self) {
        self.log.push("reg.run".to_string());
    }

    fn context_run(&mut self, _ctx: Option<Handle32<CtxTag>>, mode: u32) {
        self.log.push(format!("reg.ctx:{mode}"));
    }

    fn registry_tell(&mut self, word: u32) {
        self.log.push(format!("reg.tell:{word:#x}"));
    }

    fn base_destroy(&mut self) -> u32 {
        self.log.push("base".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn entry_notify(&mut self) {
        self.log.push("entry.1".to_string());
    }

    fn entry_second(&mut self) {
        self.log.push("entry.2".to_string());
    }

    fn guard_a(&mut self) -> u32 {
        self.log.push("guard.a".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn guard_b(&mut self) -> u32 {
        self.log.push("guard.b".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn member_probe(&mut self, _m: Handle32<MemberTag>) {
        self.log.push("probe".to_string());
    }

    fn table_entry(&mut self, index: i16) -> UpdateEntry {
        self.log.push(format!("table:{index}"));
        UpdateEntry {
            id: cookie(),
            flag: 0,
            mode: 1,
            weight: 1.0,
            index_words: [0, 1, 2, 3],
        }
    }

    fn early_block(&mut self) -> Handle32<EarlyTag> {
        self.log.push("early.block".to_string());
        cookie()
    }

    fn early_word(&mut self, _b: Handle32<EarlyTag>) -> u32 {
        self.log.push("early.word".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn early_tail(&mut self, word: u32) -> u32 {
        self.log.push(format!("early.tail:{word:#x}"));
        self.scalars.pop_front().expect("scalar queued")
    }

    fn early_call(&mut self, sx: i32, v294: u32, v310: u32) -> u32 {
        self.log
            .push(format!("early.call:{sx}:{v294:#x}:{v310:#x}"));
        self.scalars.pop_front().expect("scalar queued")
    }

    fn setup_primary(
        &mut self,
        _e: Handle32<EntryTag>,
        _r: Option<Handle32<AttachTag>>,
        _f: [u32; 6],
    ) {
        self.log.push("setup.9".to_string());
    }

    fn setup_secondary(
        &mut self,
        _e: Handle32<EntryTag>,
        _r: Option<Handle32<AttachTag>>,
        _f: [u32; 8],
    ) {
        self.log.push("setup.10".to_string());
    }

    fn store_setup(&mut self, _c: Handle32<ChainTag>, value: f32) {
        self.log.push(format!("store:{:#x}", value.to_bits()));
    }

    fn bone_row(&mut self, index: u32) -> BoneRow {
        self.log.push(format!("bone:{index}"));
        BoneRow {
            set: cookie::<BoneTag>(),
            xyz: [index as f32, index as f32 + 1.0, index as f32 + 2.0],
        }
    }

    fn submit_j(
        &mut self,
        _r: Option<Handle32<AttachTag>>,
        _f: [u32; 4],
        _s: [u32; 4],
        _t: [u32; 4],
        _c: [u32; 4],
    ) -> u32 {
        self.log.push("submit.j".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }

    fn submit_k(
        &mut self,
        _r: Option<Handle32<AttachTag>>,
        _f: [u32; 4],
        _s: [u32; 4],
        _t: [u32; 4],
        _c: [u32; 4],
    ) -> u32 {
        self.log.push("submit.k".to_string());
        self.scalars.pop_front().expect("scalar queued")
    }
}

fn cookie<T>() -> Handle32<T> {
    Handle32::new(0x1234_5678).unwrap()
}

fn maybe_cookie<T>(some: bool) -> Option<Handle32<T>> {
    if some {
        Handle32::new(0x0BAD_F00D)
    } else {
        None
    }
}

fn test_object() -> CutsceneObject {
    CutsceneObject {
        inline_triple: [11, 22, 33],
        placement: cookie(),
        attached: None,
        helper: cookie(),
        helper_flag: 0xA5,
        flag_word: 0,
        refresh_a: 0,
        refresh_b: 0,
        radius: 2.5,
        corner_a: [1.0, 1.0, 1.0],
        corner_b: [3.0, 4.0, 5.0],
        member_a: None,
        mode: 0,
        flags_24: 0,
        table_index: 0,
        ctx: None,
        gate_d4: 0,
        member_b: None,
        blocks: [None, None, None],
        done_2ac: 0x5A,
        script_word: 0,
        store_chain: None,
        attached_id: None,
    }
}

#[test]
fn registry_counts_pinned() {
    assert_eq!(registry::ROWS.len(), 21);
    assert_eq!(registry::counts(), (16, 0, 5));
}

#[test]
fn state_predicates_cover_all_modes() {
    for mode in [0u32, 1, 2, 3, 7, 0x7FFF_FFFF, 0x8000_0000, 0xFFFF_FFFF] {
        let mut o = test_object();
        o.mode = mode;
        assert_eq!(o.is_state_0(), mode == 0, "mode {mode}");
        assert_eq!(o.is_state_1(), mode == 1, "mode {mode}");
        assert_eq!(o.is_state_2(), mode == 2, "mode {mode}");
    }
}

#[test]
fn flag_word_zero_versus_nonzero() {
    for word in [0u32, 1, 0x80, 0x8000_0000, 0xFFFF_FFFF] {
        let mut o = test_object();
        o.flag_word = word;
        assert_eq!(o.is_flag_word_nonzero(), word != 0, "word {word:#x}");
    }
}

#[test]
fn bound_radius_keeps_bits() {
    // (stored bits, answered bits): signalling NaNs come back quieted,
    // everything else bit-identical.
    for (bits, answered) in [
        (0x0000_0000, 0x0000_0000),
        (0x8000_0000, 0x8000_0000),
        (0x3F80_0000, 0x3F80_0000),
        (0x7F80_0000, 0x7F80_0000),
        (0x7FC0_0000, 0x7FC0_0000),
        (0x7F80_0001, 0x7FC0_0001),
        (0xFF80_00FF, 0xFFC0_00FF),
        (0xDEAD_BEEF, 0xDEAD_BEEF),
    ] {
        let mut o = test_object();
        o.radius = f32::from_bits(bits);
        assert_eq!(o.bound_radius().to_bits(), answered, "bits {bits:#x}");
    }
}

#[test]
fn describe_prefers_attached_origin() {
    let mut o = test_object();
    let mut out = [0u32; 3];
    o.describe_into(&mut out);
    assert_eq!(out, [11, 22, 33], "detached hands out the triple");
    o.attached = Some(Matrix34 {
        vx: [0.0; 3],
        vy: [0.0; 3],
        vz: [0.0; 3],
        origin: [9.0, 8.0, 7.0],
    });
    o.describe_into(&mut out);
    assert_eq!(
        out,
        [9.0f32.to_bits(), 8.0f32.to_bits(), 7.0f32.to_bits()],
        "attached hands out the origin"
    );
}

#[test]
fn pose_answers_tail_and_followup() {
    let o = test_object();
    let mut fake = Fake::new();
    let rec = PoseRecord {
        head: 0xAAAA_AAAA,
        mid0: -3.25,
        mid1: f32::NAN,
        tail: 0x5555_5555,
    };
    fake.poses.push_back(rec);
    fake.poses.push_back(rec);
    fake.scalars.push_back(0xBEEF);
    let mut out = PoseRecord {
        head: 0,
        mid0: 0.0,
        mid1: 0.0,
        tail: 0,
    };
    assert_eq!(o.pose_into(&mut fake, &mut out), 0x5555_5555);
    assert_eq!(out.head, 0xAAAA_AAAA);
    assert_eq!(out.mid0.to_bits(), (-3.25f32).to_bits());
    assert!(out.mid1.is_nan());
    assert_eq!(o.pose_and_followup(&mut fake, &mut out), 0xBEEF);
    assert_eq!(fake.log, vec!["pose", "pose", "followup"]);
}

#[test]
fn forward_uses_link_when_present() {
    let mut o = test_object();
    let mut fake = Fake::new();
    fake.scalars.push_back(0x1111);
    assert_eq!(o.forward_word(&mut fake, 0x42), 0x1111);
    assert_eq!(fake.log, vec!["notify:0x42"]);
    o.member_a = maybe_cookie(true);
    let mut fake = Fake::new();
    fake.scalars.push_back(0x1111);
    fake.scalars.push_back(0x2222);
    assert_eq!(o.forward_word(&mut fake, 0x42), 0x2222);
    assert_eq!(fake.log, vec!["notify:0x42", "forward:0x42"]);
}

#[test]
fn refresh_needs_one_flag() {
    for (a, b, calls) in [
        (0u8, 0u8, false),
        (1, 0, true),
        (0, 1, true),
        (1, 1, true),
        (0x80, 0, true),
        (0, 0xFF, true),
    ] {
        let mut o = test_object();
        o.refresh_a = a;
        o.refresh_b = b;
        let mut fake = Fake::new();
        fake.scalars.push_back(0x7777);
        assert_eq!(o.maybe_refresh(&mut fake), if calls { 0x7777 } else { 0 });
        assert_eq!(
            fake.log,
            if calls {
                vec!["hook".to_string(), "refresh".to_string()]
            } else {
                vec![]
            },
            "flags {a:#x}/{b:#x}"
        );
    }
}

#[test]
fn helper_reset_clears_then_calls_fixed_pair() {
    let mut o = test_object();
    let mut fake = Fake::new();
    fake.scalars.push_back(0x99);
    assert_eq!(o.reset_helper(&mut fake), 0x99);
    assert_eq!(o.helper_flag, 0);
    assert_eq!(fake.log, vec!["helper:0x0:0xfffffffe"]);
}

#[test]
fn draw_dispatches_on_mode() {
    // Mode 0: both emitters in order.
    let mut o = test_object();
    o.mode = 0;
    let mut fake = Fake::new();
    o.emit_draw_commands(&mut fake, 1, 2, 3);
    assert_eq!(fake.log, vec!["draw.a:0x1:0x2", "draw.b:0x1:0x2"]);
    // Mode 1 with a target: emit then mark.
    o.mode = 1;
    let mut fake = Fake::new();
    fake.emits.push_back(maybe_cookie(true));
    o.emit_draw_commands(&mut fake, 1, 2, 3);
    assert_eq!(fake.log, vec!["draw.emit:0x1:0x2:0x3", "draw.mark"]);
    // Mode 1 without a target: emit only.
    let mut fake = Fake::new();
    fake.emits.push_back(None);
    o.emit_draw_commands(&mut fake, 1, 2, 3);
    assert_eq!(fake.log, vec!["draw.emit:0x1:0x2:0x3"]);
    // Mode 2 with a clear flag: emit, never mark.
    o.mode = 2;
    o.flag_word = 0;
    let mut fake = Fake::new();
    fake.emits.push_back(maybe_cookie(true));
    o.emit_draw_commands(&mut fake, 1, 2, 3);
    assert_eq!(fake.log, vec!["draw.emit:0x1:0x2:0x3"]);
    // Mode 2 with a set flag: nothing.
    o.flag_word = 0xDEAD;
    let mut fake = Fake::new();
    o.emit_draw_commands(&mut fake, 1, 2, 3);
    assert_eq!(fake.log, Vec::<String>::new());
    // Other modes: nothing.
    for mode in [3u32, 4, 99, 0xFFFF_FFFF] {
        o.mode = mode;
        let mut fake = Fake::new();
        o.emit_draw_commands(&mut fake, 1, 2, 3);
        assert_eq!(fake.log, Vec::<String>::new(), "mode {mode}");
    }
}

#[test]
fn box_identity_case() {
    let mut o = test_object();
    o.corner_a = [0.0, 0.0, 0.0];
    o.corner_b = [1.0, 1.0, 1.0];
    let scales = BoundsScale {
        gx: 1.0,
        gy: 1.0,
        gz: 1.0,
        abs_mask: 0x7FFF_FFFF,
    };
    let mut m = [0u32; 16];
    m[0] = 1.0f32.to_bits();
    m[5] = 1.0f32.to_bits();
    m[10] = 1.0f32.to_bits();
    m[3] = 0xAAAA_AAAA;
    m[7] = 0xBBBB_BBBB;
    let mut fake = Fake::new();
    fake.matrix.push_back(m);
    let mut out = WorldBounds {
        min: [0.0; 3],
        lo_tag: 0,
        max: [0.0; 3],
        hi_tag: 0,
    };
    o.world_bounds(&mut fake, &scales, &mut out);
    assert_eq!(out.min, [0.0, 0.0, 0.0]);
    assert_eq!(out.max, [2.0, 2.0, 2.0]);
    assert_eq!(out.lo_tag, 0xAAAA_AAAA);
    assert_eq!(out.hi_tag, 0xBBBB_BBBB);
}

#[test]
fn box_zero_extent_meets_at_center() {
    let mut o = test_object();
    o.corner_a = [4.0, 5.0, 6.0];
    o.corner_b = [4.0, 5.0, 6.0];
    let scales = BoundsScale {
        gx: 2.0,
        gy: 2.0,
        gz: 2.0,
        abs_mask: 0x7FFF_FFFF,
    };
    let mut m = [0u32; 16];
    m[0] = 1.0f32.to_bits();
    m[5] = 1.0f32.to_bits();
    m[10] = 1.0f32.to_bits();
    let mut fake = Fake::new();
    fake.matrix.push_back(m);
    let mut out = WorldBounds {
        min: [0.0; 3],
        lo_tag: 0,
        max: [0.0; 3],
        hi_tag: 0,
    };
    o.world_bounds(&mut fake, &scales, &mut out);
    // Center is twice the corner (sums scaled, no translation); the
    // extent is zero, so minimum and maximum meet there.
    assert_eq!(out.min, [16.0, 20.0, 24.0]);
    assert_eq!(out.max, [16.0, 20.0, 24.0]);
}

#[test]
fn rect_identity_case() {
    let mut o = test_object();
    o.attached = Some(Matrix34 {
        vx: [1.0, 0.0, 0.0],
        vy: [0.0, 1.0, 0.0],
        vz: [0.0, 0.0, 1.0],
        origin: [0.0, 0.0, 0.0],
    });
    let mut fake = Fake::new();
    let mut out = BoundsRect {
        min_x: 0.0,
        max_y: 0.0,
        max_x: 0.0,
        min_y: 0.0,
    };
    o.bounding_rect(&mut fake, &mut out);
    assert_eq!(
        fake.log,
        Vec::<String>::new(),
        "attached path calls nothing"
    );
    assert_eq!(out.min_x.to_bits(), 1.0f32.to_bits());
    assert_eq!(out.max_x.to_bits(), 3.0f32.to_bits());
    assert_eq!(out.min_y.to_bits(), 1.0f32.to_bits());
    assert_eq!(out.max_y.to_bits(), 4.0f32.to_bits());
}

#[test]
fn rect_nan_corners_keep_seeds() {
    let mut o = test_object();
    o.corner_a = [f32::NAN, f32::NAN, f32::NAN];
    o.corner_b = [f32::NAN, f32::NAN, f32::NAN];
    o.attached = Some(Matrix34 {
        vx: [1.0, 0.0, 0.0],
        vy: [0.0, 1.0, 0.0],
        vz: [0.0, 0.0, 1.0],
        origin: [0.0, 0.0, 0.0],
    });
    let mut fake = Fake::new();
    let mut out = BoundsRect {
        min_x: 0.0,
        max_y: 0.0,
        max_x: 0.0,
        min_y: 0.0,
    };
    o.bounding_rect(&mut fake, &mut out);
    // Unordered comparisons never replace: the million seeds stand.
    assert_eq!(out.min_x.to_bits(), 0x4974_2400);
    assert_eq!(out.max_x.to_bits(), 0xC974_2400);
    assert_eq!(out.min_y.to_bits(), 0x4974_2400);
    assert_eq!(out.max_y.to_bits(), 0xC974_2400);
}

#[test]
fn teardown_clears_and_hands_off() {
    // Mode 0 with everything linked and the registry step running.
    let mut o = test_object();
    o.mode = 0;
    o.member_a = maybe_cookie(true);
    o.member_b = maybe_cookie(true);
    o.blocks = [maybe_cookie(true), None, maybe_cookie(true)];
    o.table_index = 3;
    o.gate_d4 = 1;
    let mut fake = Fake::new();
    fake.scalars.push_back(1); // ask: yes
    fake.scalars.push_back(7); // word
    fake.scalars.push_back(1); // gate: yes
    fake.scalars.push_back(0xBEEF); // base answer
    assert_eq!(o.tear_down(&mut fake), 0xBEEF);
    assert_eq!(o.member_a, None);
    assert_eq!(o.member_b, None);
    assert_eq!(o.blocks, [None, None, None]);
    assert_eq!(o.done_2ac, 0);
    assert_eq!(
        fake.log,
        vec![
            "destroy",
            "block.teardown",
            "block.free",
            "block.teardown",
            "block.free",
            "destroy",
            "reg.ask",
            "reg.word:3",
            "reg.gate:0x7",
            "reg.run",
            "reg.ctx:3",
            "reg.tell:0x7",
            "base",
        ]
    );
    // Mode 1 sets the flag bit and skips the teardown.
    let mut o = test_object();
    o.mode = 1;
    o.member_a = maybe_cookie(true);
    let mut fake = Fake::new();
    fake.scalars.push_back(0x22);
    assert_eq!(o.tear_down(&mut fake), 0x22);
    assert_eq!(o.flags_24, 0x0400_0000);
    assert!(o.member_a.is_some(), "mode 1 destroys nothing");
    assert_eq!(fake.log, vec!["base"]);
    // The registry step stops at each gate in turn. Each entry carries
    // the gate word, the scalars its path consumes, and its call tail.
    for (gate, scalars, tail) in [
        (1u32, vec![0u32, 0x44], vec!["reg.ask"]),
        (
            1,
            vec![1, (-1i32) as u32, 0x44],
            vec!["reg.ask", "reg.word:0"],
        ),
        (1, vec![0x100, 0x44], vec!["reg.ask"]),
        (0, vec![1, 7, 0x44], vec!["reg.ask", "reg.word:0"]),
        (
            1,
            vec![1, 7, 0, 0x44],
            vec!["reg.ask", "reg.word:0", "reg.gate:0x7"],
        ),
    ] {
        let mut o = test_object();
        o.mode = 0;
        o.gate_d4 = gate;
        let mut fake = Fake::new();
        for s in scalars {
            fake.scalars.push_back(s);
        }
        assert_eq!(o.tear_down(&mut fake), 0x44);
        let mut expect = tail;
        expect.push("base");
        assert_eq!(fake.log, expect, "gate {gate}");
    }
}

#[test]
fn update_blend_path_and_early_exit() {
    let cfg = UpdateScalars {
        entry_seq_byte: 0,
        sel: 0,
        edx_alt: 0,
        eax: 0,
        ecx: 0,
        win_lo: -100.0,
        win_hi: 100.0,
        win_scale: 1.0,
        setup_flag: 0,
        store_val: 3.5,
        k0: 1.0,
        k1: 0.5,
        wgt_scale: 1.0,
        e18_scale: 1.0,
        kn: 1.0,
        wx: 1.0,
        j_a4: 1.0,
        j_a5: 1.0,
        j_a6: 1.0,
        j_a8: 0,
        k_a4: 1.0,
        k_a5: 1.0,
        k_a6: 1.0,
        k_a8: 0,
        qk: 0.25,
    };
    // Full blend: entry pair, guards, table, setup, two bone rows, submit.
    let mut o = test_object();
    o.attached = Some(Matrix34 {
        vx: [1.0, 0.0, 0.0],
        vy: [0.0, 1.0, 0.0],
        vz: [0.0, 0.0, 1.0],
        origin: [0.0, 0.0, 0.0],
    });
    o.attached_id = maybe_cookie(true);
    o.store_chain = maybe_cookie(true);
    let mut acc = Accumulator {
        flag: 1,
        vals: [1.0, 2.0, 3.0],
    };
    let mut fake = Fake::new();
    fake.scalars.push_back(0); // guard A: main path
    fake.scalars.push_back(1); // guard B: proceed
    fake.scalars.push_back(0xABCD); // submit answer
    assert_eq!(o.update(&mut fake, &cfg, &mut acc), 0xABCD);
    assert_eq!(
        fake.log,
        vec![
            "entry.1",
            "entry.2",
            "guard.a",
            "guard.b",
            "table:0",
            "setup.9",
            "setup.10",
            "store:0x40600000",
            "bone:0",
            "bone:2",
            "submit.j",
        ]
    );
    assert_eq!(acc.flag, 1, "loaded accumulator untouched");
    assert_eq!(acc.vals, [1.0, 2.0, 3.0]);
    // Zero guard B returns the whole word with no further calls.
    let mut fake = Fake::new();
    fake.scalars.push_back(0);
    fake.scalars.push_back(0x1234_5600);
    assert_eq!(o.update(&mut fake, &cfg, &mut acc), 0x1234_5600);
    assert_eq!(fake.log, vec!["entry.1", "entry.2", "guard.a", "guard.b"]);
}

#[test]
fn rect_detached_path_uses_corner_answers() {
    let o = test_object();
    let mut fake = Fake::new();
    fake.corners.push_back([10.0, 1.0]);
    fake.corners.push_back([-5.0, 20.0]);
    fake.corners.push_back([0.0, 0.0]);
    fake.corners.push_back([7.0, -3.0]);
    let mut out = BoundsRect {
        min_x: 0.0,
        max_y: 0.0,
        max_x: 0.0,
        min_y: 0.0,
    };
    o.bounding_rect(&mut fake, &mut out);
    assert_eq!(fake.log.len(), 4, "one transform per corner");
    assert_eq!(out.min_x.to_bits(), (-5.0f32).to_bits());
    assert_eq!(out.max_x.to_bits(), 10.0f32.to_bits());
    assert_eq!(out.min_y.to_bits(), (-3.0f32).to_bits());
    assert_eq!(out.max_y.to_bits(), 20.0f32.to_bits());
}
