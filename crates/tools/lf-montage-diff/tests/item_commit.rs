//! Differential proof of the per-item montage visibility commit.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::collections::HashMap;

use lf_core::Handle32;
use lf_input_frontend::montage_container::{ItemGlobals, MontageContainer, Viewport};
use lf_montage_diff::fake::{Ctx, EffectMemory, FakeWorld, Script, normalise};
use lf_montage_diff::mutants;
use lf_montage_diff::rewrites;
use lf_montage_diff::rt::{self, ITEM_CALLS, ITEM_VIRTUALS, ItemVStubs, Kind};

const OBJECT_WORDS: usize = 0x230 / 4;
const LIST_WORD: usize = 0x1E0 / 4;
const AUX_WORD: usize = 0x1E8 / 4;
const CFG_WORD: usize = 0x1EC / 4;
const OUT_WORD: usize = 0x1F0 / 4;
const VIEW_WORD: usize = 0x1F4 / 4;
const SRC_WORD: usize = 0x1F8 / 4;
const DROPPED_WORD: usize = 0x20C / 4;
const PENDING_WORD: usize = 0x210 / 4;
const CURSOR_WORD: usize = 0x214 / 4;
const PHASE_WORD: usize = 0x21C / 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    PhaseSkip,
    GeometryFailPrimary,
    GeometryFailQuiet,
    GeometryNan,
    GateFailSecondary,
    NotifyAndPost,
    TagMismatch,
    SubflagSet,
    NegativePostIndex,
}

const PATHS: &[Path] = &[
    Path::PhaseSkip,
    Path::GeometryFailPrimary,
    Path::GeometryFailQuiet,
    Path::GeometryNan,
    Path::GateFailSecondary,
    Path::NotifyAndPost,
    Path::TagMismatch,
    Path::SubflagSet,
    Path::NegativePostIndex,
];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }

    fn word(&mut self) -> u32 {
        self.next() as u32
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

fn push(script: &mut Script, name: &'static str, answers: Vec<u32>) {
    script.insert(name, answers.into());
}

struct Trial {
    key: u32,
    phase: u8,
    listener_word_204: u32,
    listener_word_200: u32,
    mask_a: u32,
    mask_b: u32,
    viewport: Viewport,
    script: Script,
}

fn choose(rng: &mut Rng, values: &[u32]) -> u32 {
    values[rng.below(values.len() as u64) as usize]
}

fn geometry_answers(script: &mut Script, side: &str, values: [u32; 4]) {
    let (half_width, center_x, half_height, center_y) =
        (values[0], values[1], values[2], values[3]);
    let repeats = |value| vec![value; 8];
    let names = if side == "source" {
        [
            ("geometry_source_half_width", half_width),
            ("geometry_source_center_x", center_x),
            ("geometry_source_half_height", half_height),
            ("geometry_source_center_y", center_y),
        ]
    } else {
        [
            ("geometry_container_half_width", half_width),
            ("geometry_container_center_x", center_x),
            ("geometry_container_half_height", half_height),
            ("geometry_container_center_y", center_y),
        ]
    };
    for (name, bits) in names {
        push(script, name, repeats(bits));
    }
}

fn make_trial(rng: &mut Rng, path: Path) -> Trial {
    let phase = u8::from(path == Path::PhaseSkip);
    let mut script = Script::new();

    let centers = [
        0x3ECC_CCCD,
        0x3EE6_6666,
        0x3F00_0000,
        0x3F0C_CCCD,
        0x3F19_999A,
    ];
    let extents = [0x3F40_0000, 0x3F60_0000, 0x3F80_0000, 0x3F90_0000];
    let source_rect = if path == Path::GeometryFailPrimary || path == Path::GeometryFailQuiet {
        [0x3F00_0000, 0x4000_0000, 0x3F80_0000, 0x3F00_0000]
    } else if path == Path::GeometryNan {
        [0x7FC0_0042, 0x3F50_0000, 0x3F80_0000, 0x3F00_0000]
    } else {
        [
            choose(rng, &extents),
            choose(rng, &centers),
            choose(rng, &extents),
            choose(rng, &centers),
        ]
    };
    let container_rect = [0x3F80_0000, 0x3F00_0000, 0x3F80_0000, 0x3F00_0000];
    geometry_answers(&mut script, "source", source_rect);
    geometry_answers(&mut script, "container", container_rect);

    let source_tag = 0x1357_9BDF;
    let wanted_tag = if path == Path::TagMismatch {
        source_tag ^ 1
    } else {
        source_tag
    };
    let subflag = u32::from(path == Path::SubflagSet);
    let source_index = if path == Path::NegativePostIndex {
        0x8000_0000
    } else {
        rng.word() & 0x7FFF_FFFF
    };
    push(
        &mut script,
        "gatekeeper",
        vec![u32::from(path == Path::GateFailSecondary) * 0x101],
    );
    push(&mut script, "source_tag", vec![source_tag]);
    push(&mut script, "item_tag_id", vec![wanted_tag]);
    push(&mut script, "source_subflag", vec![subflag]);
    push(&mut script, "source_index", vec![source_index]);
    let first_sink = if path == Path::NegativePostIndex {
        0x8000_0000
    } else {
        7
    };
    push(&mut script, "list_sink", vec![first_sink, rng.word()]);
    push(&mut script, "inner_done", vec![rng.word(), rng.word()]);
    push(
        &mut script,
        "measure",
        vec![choose(
            rng,
            &[
                0x0000_0000,
                0x8000_0000,
                0x7FC0_0042,
                0x7F80_0000,
                0xFF80_0000,
                0x0000_0001,
                0x4120_0000,
            ],
        )],
    );

    Trial {
        key: rng.word(),
        phase,
        listener_word_204: u32::from(path == Path::GeometryFailPrimary),
        listener_word_200: u32::from(path == Path::GateFailSecondary),
        mask_a: rng.word(),
        mask_b: rng.word(),
        viewport: Viewport {
            width: 1,
            height: 1,
            width_scale_bits: 0x3F00_0000,
            height_scale_bits: 0x3F00_0000,
        },
        script,
    }
}

struct Block {
    words: Vec<u32>,
}

impl Block {
    fn filled(size: usize, rng: &mut Rng) -> Self {
        Self {
            words: (0..size).map(|_| rng.word()).collect(),
        }
    }

    fn address(&self) -> u32 {
        self.words.as_ptr() as usize as u32
    }
}

fn set_slot(vtable: &mut Block, offset: usize, function: u32) {
    vtable.words[offset / 4] = function;
}

fn bytes(words: &[u32]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_le_bytes()).collect()
}

#[allow(dead_code)]
struct Scene {
    object: Block,
    object_vtable: Block,
    list: Block,
    list_vtable: Block,
    source: Block,
    resolved_source: Block,
    source_vtable: Block,
    inner: Block,
    inner_vtable: Block,
    aux: Block,
    cfg: Block,
    out: Block,
    view: Block,
    state: Block,
}

impl Scene {
    fn build(rng: &mut Rng, trial: &mut Trial) -> Self {
        let stubs = ItemVStubs::new();
        let mut object_vtable = Block::filled(0x224 / 4, rng);
        set_slot(&mut object_vtable, 0xB8, stubs.container_half_width);
        set_slot(&mut object_vtable, 0xC0, stubs.container_half_height);
        set_slot(&mut object_vtable, 0xC8, stubs.container_center_x);
        set_slot(&mut object_vtable, 0xD0, stubs.container_center_y);
        set_slot(&mut object_vtable, 0x1A4, stubs.own_fail);
        set_slot(&mut object_vtable, 0x1C0, stubs.own_dismiss);

        let mut list_vtable = Block::filled(0x1E8 / 4, rng);
        set_slot(&mut list_vtable, 0x1C, stubs.list_subflag);
        set_slot(&mut list_vtable, 0x4C, stubs.list_current_key);
        set_slot(&mut list_vtable, 0x1D4, stubs.list_probe);
        set_slot(&mut list_vtable, 0x1E4, stubs.list_sink);
        let mut source_vtable = Block::filled(0xD4 / 4, rng);
        set_slot(&mut source_vtable, 0x0, stubs.source_tag);
        set_slot(&mut source_vtable, 0x1C, stubs.source_subflag);
        set_slot(&mut source_vtable, 0x4C, stubs.source_index);
        set_slot(&mut source_vtable, 0xB8, stubs.source_half_width);
        set_slot(&mut source_vtable, 0xC0, stubs.source_half_height);
        set_slot(&mut source_vtable, 0xC8, stubs.source_center_x);
        set_slot(&mut source_vtable, 0xD0, stubs.source_center_y);
        let mut inner_vtable = Block::filled(0x220 / 4, rng);
        set_slot(&mut inner_vtable, 0x21C, stubs.inner_done);

        let mut object = Block::filled(OBJECT_WORDS, rng);
        let mut list = Block::filled(0x224 / 4, rng);
        let source = Block::filled(4, rng);
        let mut resolved_source = Block::filled(0xD4 / 4, rng);
        let mut inner = Block::filled(0x220 / 4, rng);
        let aux = Block::filled(4, rng);
        let cfg = Block::filled(0x200 / 4, rng);
        let out = Block::filled(4, rng);
        let view = Block::filled(4, rng);
        let mut state = Block::filled(0x210 / 4, rng);

        object.words[0] = object_vtable.address();
        object.words[LIST_WORD] = list.address();
        object.words[AUX_WORD] = aux.address();
        object.words[CFG_WORD] = cfg.address();
        object.words[OUT_WORD] = out.address();
        object.words[VIEW_WORD] = view.address();
        object.words[SRC_WORD] = source.address();
        object.words[0x1BC / 4] = out.address();
        object.words[PHASE_WORD] =
            (object.words[PHASE_WORD] & 0xFFFF_FF00) | u32::from(trial.phase);
        list.words[0] = list_vtable.address();
        list.words[0x1E0 / 4] = inner.address();
        resolved_source.words[0] = source_vtable.address();
        inner.words[0] = inner_vtable.address();
        state.words[0x200 / 4] = trial.listener_word_200;
        state.words[0x204 / 4] = trial.listener_word_204;

        push(
            &mut trial.script,
            "lookup_source",
            vec![resolved_source.address(), resolved_source.address()],
        );
        push(
            &mut trial.script,
            "build_configuration",
            vec![out.address()],
        );

        Self {
            object,
            object_vtable,
            list,
            list_vtable,
            source,
            resolved_source,
            source_vtable,
            inner,
            inner_vtable,
            aux,
            cfg,
            out,
            view,
            state,
        }
    }

    fn context(&self) -> Ctx {
        let mut ctx = Ctx::default();
        ctx.helper_context = rt::relocated(0x0198_1A4C);
        ctx.lookup_context = 0;
        ctx.notify_context = rt::relocated(0x0117_6888);
        ctx.tail_context = rt::relocated(0x019D_2F18);
        ctx.state = self.state.address();
        ctx.own = self.object.address();
        ctx.list = self.list.address();
        ctx.aux = self.aux.address();
        ctx.cfg = self.cfg.address();
        ctx.out = self.out.address();
        ctx.view = self.view.address();
        ctx.src = self.source.address();
        ctx.resolved_source = self.resolved_source.address();
        ctx.inner = self.inner.address();
        ctx
    }

    fn effect_memory(&self) -> EffectMemory {
        HashMap::from([(self.state.address(), bytes(&self.state.words))])
    }
}

fn handle<T>(value: u32) -> Handle32<T> {
    Handle32::new(value).expect("test handles are nonzero")
}

fn decode(words: &[u32], ctx: &Ctx) -> MontageContainer {
    MontageContainer {
        own: handle(ctx.own),
        list: handle(words[LIST_WORD]),
        aux: handle(words[AUX_WORD]),
        cfg: handle(words[CFG_WORD]),
        out: handle(words[OUT_WORD]),
        view: handle(words[VIEW_WORD]),
        src: handle(words[SRC_WORD]),
        want: words[0x204 / 4],
        dropped: words[DROPPED_WORD],
        pending: words[PENDING_WORD],
        cursor: words[CURSOR_WORD],
        mode: (words[0x218 / 4] & 0xFF) as u8,
        phase: (words[PHASE_WORD] & 0xFF) as u8,
        epilogue: Handle32::new(words[0x1BC / 4]),
        quotient_bits: words[0x220 / 4],
    }
}

fn encode(container: &MontageContainer, words: &mut [u32]) {
    words[0x204 / 4] = container.want;
    words[DROPPED_WORD] = container.dropped;
    words[PENDING_WORD] = container.pending;
    words[CURSOR_WORD] = container.cursor;
    words[0x218 / 4] = (words[0x218 / 4] & 0xFFFF_FF00) | u32::from(container.mode);
    words[PHASE_WORD] = (words[PHASE_WORD] & 0xFFFF_FF00) | u32::from(container.phase);
    words[0x1BC / 4] = Handle32::raw_or_zero(container.epilogue);
    words[0x220 / 4] = container.quotient_bits;
}

fn globals(ctx: &Ctx, trial: &Trial) -> ItemGlobals {
    ItemGlobals {
        registry: handle(ctx.helper_context),
        listener: handle(ctx.state),
        notify_context: handle(ctx.notify_context),
        tail_context: handle(ctx.tail_context),
        listener_word_204: trial.listener_word_204,
        listener_word_200: trial.listener_word_200,
        mask_a: trial.mask_a,
        mask_b: trial.mask_b,
        viewport: trial.viewport,
    }
}

fn item_call_table() -> Vec<(lf_montage_diff::fake::CallSpec, Kind)> {
    ITEM_CALLS
        .iter()
        .copied()
        .map(|spec| {
            let kind = match spec.id {
                1 => Kind::T2,
                11 => Kind::C1,
                12 => Kind::T2,
                14 => Kind::T1,
                18 => Kind::T3,
                19 => Kind::T2,
                20 => Kind::T2f32,
                21 => Kind::T3,
                22 | 24 | 25 => Kind::T1,
                27 => Kind::T2,
                34 => Kind::C0,
                35 | 36 => Kind::T2,
                37 => Kind::T3,
                other => panic!("unexpected item callee {other}"),
            };
            (spec, kind)
        })
        .collect()
}

fn install(scene: &Scene, trial: &Trial, script: &Script) {
    rt::set_state_ptr(scene.state.address());
    rt::set_item_viewport(trial.viewport);
    rt::set_item_masks(trial.mask_a, trial.mask_b);
    rt::install(&item_call_table(), ITEM_VIRTUALS, script);
}

fn run_lift(
    initial: &[u32],
    initial_effects: &EffectMemory,
    trial: &Trial,
    ctx: &Ctx,
) -> (Vec<String>, Vec<u32>, EffectMemory) {
    let mut container = decode(initial, ctx);
    let mut world = FakeWorld::new(*ctx, trial.script.clone(), initial_effects.clone());
    container.commit_item(&mut world, &globals(ctx, trial), trial.key);
    let mut object = initial.to_vec();
    encode(&container, &mut object);
    (world.events, object, world.effects)
}

fn run_wrong_phase_lift(
    initial: &[u32],
    initial_effects: &EffectMemory,
    trial: &Trial,
    ctx: &Ctx,
) -> (Vec<String>, Vec<u32>, EffectMemory) {
    let mut container = decode(initial, ctx);
    container.phase = 0;
    let mut world = FakeWorld::new(*ctx, trial.script.clone(), initial_effects.clone());
    container.commit_item(&mut world, &globals(ctx, trial), trial.key);
    container.phase = trial.phase;
    let mut object = initial.to_vec();
    encode(&container, &mut object);
    (world.events, object, world.effects)
}

#[test]
fn commit_item_matches_its_rewrite() {
    let _lock = rt::case_lock();
    const TRIALS_PER_PATH: usize = 100;
    let mut rng = Rng(0x1A7E_97C0_2026);
    let mut path_counts = [0usize; 9];
    let mut caught_missing_phase_gate = false;

    for (path_index, &path) in PATHS.iter().enumerate() {
        for trial_index in 0..TRIALS_PER_PATH {
            let mut trial = make_trial(&mut rng, path);
            let scene = Scene::build(&mut rng, &mut trial);
            let ctx = scene.context();
            let initial = scene.object.words.clone();
            let initial_effects = scene.effect_memory();

            install(&scene, &trial, &trial.script);
            let _rewrite_result = rewrites::rw_00dd3040(scene.object.address(), trial.key);
            let rewrite_events = normalise(&ctx, ITEM_CALLS, ITEM_VIRTUALS, &rt::take_log());
            let rewrite_object = scene.object.words.clone();
            let rewrite_effects = scene.effect_memory();

            let (lift_events, lift_object, lift_effects) =
                run_lift(&initial, &initial_effects, &trial, &ctx);
            assert_eq!(
                lift_events, rewrite_events,
                "trial {trial_index} ({path:?}): calls"
            );
            assert_eq!(
                lift_object, rewrite_object,
                "trial {trial_index} ({path:?}): object"
            );
            assert_eq!(
                lift_effects, rewrite_effects,
                "trial {trial_index} ({path:?}): listener bytes"
            );

            if path == Path::PhaseSkip && !caught_missing_phase_gate {
                let (wrong_events, wrong_object, wrong_effects) =
                    run_wrong_phase_lift(&initial, &initial_effects, &trial, &ctx);
                caught_missing_phase_gate = wrong_events != rewrite_events
                    || wrong_object != rewrite_object
                    || wrong_effects != rewrite_effects;
            }

            match path {
                Path::PhaseSkip => assert!(rewrite_events.is_empty()),
                Path::GeometryFailPrimary => assert!(
                    rewrite_events
                        .iter()
                        .any(|e| e == "listener_primary(state)")
                ),
                Path::GeometryFailQuiet => {
                    assert!(rewrite_events.iter().any(|e| e == "own_fail(own)"))
                }
                Path::GeometryNan => assert!(rewrite_events.iter().any(|e| e == "own_fail(own)")),
                Path::GateFailSecondary => assert!(
                    rewrite_events
                        .iter()
                        .any(|e| e == "listener_secondary(state)")
                ),
                Path::NotifyAndPost => {
                    assert!(rewrite_events.iter().any(|e| e.starts_with("notify(")));
                    assert!(rewrite_events.iter().any(|e| e.starts_with("post_index(")));
                }
                Path::TagMismatch => {
                    assert!(!rewrite_events.iter().any(|e| e.starts_with("notify(")))
                }
                Path::SubflagSet => {
                    assert!(!rewrite_events.iter().any(|e| e.starts_with("notify(")))
                }
                Path::NegativePostIndex => {
                    assert!(!rewrite_events.iter().any(|e| e.starts_with("post_index(")))
                }
            }
            path_counts[path_index] += 1;
        }
    }

    assert!(path_counts.iter().all(|&count| count == TRIALS_PER_PATH));
    assert!(
        caught_missing_phase_gate,
        "wrong lift without the phase guard passed"
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PickPath {
    PreSubReturn,
    PreProbeZero,
    PreMaskReturn,
    PreDismiss,
    GateFailTagMismatch,
    GateFailTagMatch,
    QuotientBoundary,
    QuotientNan,
    QuotientBelow,
    QuotientNotBelow,
    TagMismatchMain,
    GateTwoFail,
    EpiloguePresent,
}

const PICK_PATHS: &[PickPath] = &[
    PickPath::PreSubReturn,
    PickPath::PreProbeZero,
    PickPath::PreMaskReturn,
    PickPath::PreDismiss,
    PickPath::GateFailTagMismatch,
    PickPath::GateFailTagMatch,
    PickPath::QuotientBoundary,
    PickPath::QuotientNan,
    PickPath::QuotientBelow,
    PickPath::QuotientNotBelow,
    PickPath::TagMismatchMain,
    PickPath::GateTwoFail,
    PickPath::EpiloguePresent,
];

fn make_pick_trial(rng: &mut Rng, path: PickPath) -> Trial {
    let mut trial = make_trial(rng, Path::NotifyAndPost);
    trial.script = Script::new();
    trial.phase = 0;
    trial.listener_word_204 = 0;
    trial.listener_word_200 = 0;
    trial.viewport = Viewport {
        width: 1,
        height: 1,
        width_scale_bits: 0x3F00_0000,
        height_scale_bits: 0x3F00_0000,
    };

    let source_rect = match path {
        PickPath::GateFailTagMismatch | PickPath::GateFailTagMatch => {
            [0x3F00_0000, 0x4000_0000, 0x3F80_0000, 0x3F00_0000]
        }
        _ => [0x3F80_0000, 0x3F00_0000, 0x3F80_0000, 0x3F00_0000],
    };
    let container_rect = [0x3F80_0000, 0x3F00_0000, 0x3F80_0000, 0x3F00_0000];
    geometry_answers(&mut trial.script, "source", source_rect);
    geometry_answers(&mut trial.script, "container", container_rect);
    if path == PickPath::GateTwoFail {
        trial.script.insert(
            "geometry_source_center_x",
            [
                0x3F00_0000,
                0x3F00_0000,
                0x3F00_0000,
                0x3F00_0000,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ]
            .into(),
        );
    }

    let pre_gate = matches!(
        path,
        PickPath::PreSubReturn
            | PickPath::PreProbeZero
            | PickPath::PreMaskReturn
            | PickPath::PreDismiss
    );
    let list_key = if pre_gate { trial.key } else { trial.key ^ 1 };
    let sub = u32::from(path == PickPath::PreSubReturn) * 0x123;
    let probe = if path == PickPath::PreProbeZero { 0 } else { 7 };
    trial.mask_a = 0;
    trial.mask_b = if path == PickPath::PreDismiss { 1 } else { 2 };
    push(&mut trial.script, "list_current_key", vec![list_key]);
    push(&mut trial.script, "list_subflag", vec![sub]);
    push(&mut trial.script, "list_probe", vec![probe]);
    push(&mut trial.script, "own_dismiss", vec![0x333]);

    let source_tag = 0x0000_5678;
    let mismatch = matches!(
        path,
        PickPath::GateFailTagMismatch | PickPath::TagMismatchMain
    );
    push(&mut trial.script, "source_tag", vec![source_tag]);
    push(
        &mut trial.script,
        "item_tag_id",
        vec![if mismatch { source_tag ^ 1 } else { source_tag }],
    );

    let sink = match path {
        PickPath::QuotientBelow => 0x7FFF_FFFF,
        PickPath::QuotientNotBelow => 0x9000_0000,
        _ => 10,
    };
    push(&mut trial.script, "list_sink", vec![sink]);
    push(
        &mut trial.script,
        "random_word",
        match path {
            PickPath::QuotientBoundary | PickPath::EpiloguePresent => vec![1, 1],
            PickPath::QuotientNan => vec![0, 0],
            PickPath::QuotientBelow | PickPath::QuotientNotBelow => vec![2, 1],
            _ => Vec::new(),
        },
    );
    if matches!(path, PickPath::QuotientBelow | PickPath::QuotientNotBelow) {
        push(&mut trial.script, "list_probe", vec![0x8000_0000]);
    }
    push(&mut trial.script, "listener_refresh", vec![0x444]);
    push(&mut trial.script, "epilogue_finish", vec![0x555]);
    trial
}

fn run_pick_lift(
    initial: &[u32],
    initial_effects: &EffectMemory,
    trial: &Trial,
    ctx: &Ctx,
) -> (Vec<String>, u32, Vec<u32>, EffectMemory) {
    let mut container = decode(initial, ctx);
    let mut world = FakeWorld::new(*ctx, trial.script.clone(), initial_effects.clone());
    let result = container.indexed_pick(&mut world, &globals(ctx, trial), trial.key);
    let mut object = initial.to_vec();
    encode(&container, &mut object);
    (world.events, result, object, world.effects)
}

fn run_pick_mutant(
    initial: &[u32],
    initial_effects: &EffectMemory,
    trial: &Trial,
    ctx: &Ctx,
) -> (Vec<String>, u32, Vec<u32>, EffectMemory) {
    let mut container = decode(initial, ctx);
    let mut world = FakeWorld::new(*ctx, trial.script.clone(), initial_effects.clone());
    let result = mutants::indexed_pick_inclusive_threshold(
        &mut container,
        &mut world,
        &globals(ctx, trial),
        trial.key,
    );
    let mut object = initial.to_vec();
    encode(&container, &mut object);
    (world.events, result, object, world.effects)
}

#[test]
fn indexed_pick_matches_its_rewrite() {
    let _lock = rt::case_lock();
    const TRIALS_PER_PATH: usize = 100;
    let mut rng = Rng(0x94A1_2026_5EED);
    let mut path_counts = [0usize; 13];
    let mut caught_inclusive_threshold = false;

    for (path_index, &path) in PICK_PATHS.iter().enumerate() {
        for trial_index in 0..TRIALS_PER_PATH {
            let mut trial = make_pick_trial(&mut rng, path);
            let mut scene = Scene::build(&mut rng, &mut trial);
            if !matches!(path, PickPath::QuotientNotBelow | PickPath::EpiloguePresent) {
                scene.object.words[0x1BC / 4] = 0;
            }
            let ctx = scene.context();
            let initial = scene.object.words.clone();
            let initial_effects = scene.effect_memory();

            install(&scene, &trial, &trial.script);
            let rewrite_result = rewrites::rw_00dd37e0(scene.object.address(), trial.key);
            let rewrite_events = normalise(&ctx, ITEM_CALLS, ITEM_VIRTUALS, &rt::take_log());
            let rewrite_object = scene.object.words.clone();
            let rewrite_effects = scene.effect_memory();

            let (lift_events, lift_result, lift_object, lift_effects) =
                run_pick_lift(&initial, &initial_effects, &trial, &ctx);
            assert_eq!(
                lift_events, rewrite_events,
                "trial {trial_index} ({path:?}): calls"
            );
            assert_eq!(
                lift_result, rewrite_result,
                "trial {trial_index} ({path:?}): result"
            );
            assert_eq!(
                lift_object, rewrite_object,
                "trial {trial_index} ({path:?}): object"
            );
            assert_eq!(
                lift_effects, rewrite_effects,
                "trial {trial_index} ({path:?}): listener bytes"
            );

            if path == PickPath::QuotientBoundary && !caught_inclusive_threshold {
                let (wrong_events, wrong_result, wrong_object, wrong_effects) =
                    run_pick_mutant(&initial, &initial_effects, &trial, &ctx);
                caught_inclusive_threshold = wrong_events != rewrite_events
                    || wrong_result != rewrite_result
                    || wrong_object != rewrite_object
                    || wrong_effects != rewrite_effects;
            }

            match path {
                PickPath::PreSubReturn => assert_eq!(rewrite_result, 0x123),
                PickPath::PreProbeZero => assert_eq!(rewrite_result, 0),
                PickPath::PreMaskReturn => assert_eq!(rewrite_result, 7),
                PickPath::PreDismiss => {
                    assert!(rewrite_events.iter().any(|e| e == "own_dismiss(own)"))
                }
                PickPath::GateFailTagMismatch | PickPath::TagMismatchMain => {
                    assert!(
                        !rewrite_events
                            .iter()
                            .any(|e| e == "listener_refresh(state,0x2)")
                    );
                }
                PickPath::GateFailTagMatch => assert_eq!(rewrite_result, 0x3F00_0000),
                PickPath::QuotientBoundary => {
                    assert_eq!(rewrite_object[0x220 / 4], 0x3F00_0000);
                    assert!(!rewrite_events.iter().any(|e| e.starts_with("list_probe(")));
                }
                PickPath::QuotientNan => {
                    assert!(f32::from_bits(rewrite_object[0x220 / 4]).is_nan())
                }
                PickPath::QuotientBelow => assert_eq!(rewrite_object[0x204 / 4], 0x8000_0000),
                PickPath::QuotientNotBelow => assert_eq!(rewrite_object[0x204 / 4], u32::MAX),
                PickPath::GateTwoFail => {
                    assert!(
                        !rewrite_events
                            .iter()
                            .any(|e| e.starts_with("listener_refresh("))
                    );
                    assert_eq!(rewrite_result, 0x3F00_0000);
                }
                PickPath::EpiloguePresent => assert!(
                    rewrite_events
                        .iter()
                        .any(|e| e.starts_with("epilogue_finish(")),
                    "events={rewrite_events:?}; result={rewrite_result:#x}"
                ),
            }
            path_counts[path_index] += 1;
        }
    }

    assert!(path_counts.iter().all(|&count| count == TRIALS_PER_PATH));
    assert!(
        caught_inclusive_threshold,
        "inclusive quotient mutant was never caught"
    );
}
