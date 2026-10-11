//! Host edge tests for the lifted `ped_task` group.
//!
//! One case per question a reader of the code would ask: the flag splits,
//! the copy and transform paths, the sphere boundary, the keep-vs-build
//! split, the maker selection, and every panic domain. Bit-exactness
//! against the verified rewrites is proven by the 32-bit differential
//! crate, not here.

// Exact float asserts throughout: every expected value is hand-computed
// and exactly representable, so strict comparison is the point.
#![allow(clippy::float_cmp)]

use std::collections::VecDeque;

use lf_core::Handle32;
use lf_peds_tasks::ped_task::registry::{self, State};
use lf_peds_tasks::ped_task::{
    AngleTuning, ArgLookup, BuildManagers, ChainClone, ChainCloner, ChainEntry, ChainOwner,
    CloneProduct, ConeSolvers, FLAG_ACTIVE, FLAG_COPY, FLAG_DONE, FLAG_EXTRA, FLAG_SPHERE,
    FLAG_TRANSFORM, FoundEntry, KIND_11, Kind11Task, Link, LinkMatrix, Matrix, NONE, NormSlot,
    PRIORITY, PedMgr, PedTaskOutcome, PoseFill, PoseSample, PoseVolume, TaskBuildCtx,
};

#[test]
fn registry_counts_seven_proven() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 7);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 0);
    assert_eq!(registry::ROWS.len(), 7);
    for row in registry::ROWS {
        assert_eq!(row.state, State::Proven);
        assert!(
            !row.narrows.is_empty(),
            "proven row {} {} states its narrowings",
            row.class,
            row.method
        );
    }
    let names: Vec<(&str, &str)> = registry::ROWS.iter().map(|r| (r.class, r.method)).collect();
    assert_eq!(
        names,
        vec![
            ("PoseVolume", "pose_transform"),
            ("PoseVolume", "pose_blend"),
            ("PoseVolume", "cone_test"),
            ("BuildManagers", "build4"),
            ("BuildManagers", "build5"),
            ("ChainCloner", "clone_chain_a"),
            ("ChainCloner", "clone_chain_b"),
        ]
    );
}

// The world-space transform.

struct LinkScript {
    builds: u32,
    fetches: u32,
    install: Option<Matrix>,
}

impl LinkMatrix for LinkScript {
    fn build_matrix(&mut self, link: &mut Link) {
        self.builds += 1;
        if let Some(m) = self.install.take() {
            link.matrix = Some(m);
        }
    }
    fn fetch_matrix(&mut self, _link: &Link) {
        self.fetches += 1;
    }
}

fn links(install: Option<Matrix>) -> LinkScript {
    LinkScript {
        builds: 0,
        fetches: 0,
        install,
    }
}

fn unit_matrix() -> Matrix {
    Matrix {
        cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        trans: [10.0, 20.0, 30.0],
    }
}

#[test]
fn transform_inactive_answers_none_without_calls() {
    for flags in [0, FLAG_TRANSFORM, FLAG_COPY, 0xFC] {
        let mut vol = PoseVolume::new([1.0; 4], [2.0; 4], 7, None, flags);
        let mut fake = links(None);
        assert_eq!(vol.transform(&mut fake), None);
        assert_eq!(fake.builds, 0);
        assert_eq!(fake.fetches, 0);
    }
}

#[test]
fn transform_linkless_copies_straight_through() {
    for flags in [FLAG_ACTIVE, FLAG_ACTIVE | FLAG_TRANSFORM] {
        let mut vol = PoseVolume::new([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], 9, None, flags);
        let mut fake = links(None);
        let got = vol.transform(&mut fake).unwrap();
        assert_eq!(got.a, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(got.b, [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(got.tag, 9);
        assert_eq!(fake.builds, 0);
    }
}

#[test]
fn transform_copy_path_still_adds_the_tail() {
    // No matrix: the tail reads the fallback triple.
    let link = Link {
        matrix: None,
        fallback: [1.0, 1.0, 1.0],
    };
    let mut vol = PoseVolume::new(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        0,
        Some(link),
        FLAG_ACTIVE,
    );
    let mut fake = links(None);
    let got = vol.transform(&mut fake).unwrap();
    assert_eq!(got.a, [2.0, 3.0, 4.0, 4.0]);
    assert_eq!(got.b, [6.0, 7.0, 8.0, 8.0]);
    assert_eq!(fake.builds, 0);
    // Prebuilt matrix: the tail reads its translation instead.
    let link = Link {
        matrix: Some(unit_matrix()),
        fallback: [1.0, 1.0, 1.0],
    };
    let mut vol = PoseVolume::new(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        0,
        Some(link),
        FLAG_ACTIVE,
    );
    let mut fake = links(None);
    let got = vol.transform(&mut fake).unwrap();
    assert_eq!(got.a, [11.0, 22.0, 33.0, 4.0]);
    assert_eq!(got.b, [15.0, 26.0, 37.0, 8.0]);
}

#[test]
fn transform_full_path_builds_once_then_transforms() {
    let link = Link {
        matrix: None,
        fallback: [0.0, 0.0, 0.0],
    };
    let mut vol = PoseVolume::new(
        [1.0, 0.0, 0.0, 9.0],
        [0.0, 1.0, 0.0, 9.0],
        3,
        Some(link),
        FLAG_ACTIVE | FLAG_TRANSFORM,
    );
    let mut fake = links(Some(unit_matrix()));
    let got = vol.transform(&mut fake).unwrap();
    assert_eq!(fake.builds, 1);
    assert_eq!(fake.fetches, 1);
    // Unit columns copy the vector; the w slot is zero, not the input's.
    assert_eq!(got.a, [11.0, 20.0, 30.0, 0.0]);
    assert_eq!(got.b, [10.0, 21.0, 30.0, 0.0]);
    assert_eq!(got.tag, 3);
}

#[test]
fn transform_prebuilt_matrix_makes_no_calls() {
    let link = Link {
        matrix: Some(unit_matrix()),
        fallback: [0.0, 0.0, 0.0],
    };
    let mut vol = PoseVolume::new(
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        0,
        Some(link),
        FLAG_ACTIVE | FLAG_TRANSFORM,
    );
    let mut fake = links(None);
    let got = vol.transform(&mut fake).unwrap();
    assert_eq!(fake.builds, 0);
    assert_eq!(fake.fetches, 0);
    assert_eq!(got.a, [11.0, 20.0, 30.0, 0.0]);
}

#[test]
#[should_panic(expected = "link build left no matrix")]
fn transform_panics_when_the_build_leaves_nothing() {
    let link = Link {
        matrix: None,
        fallback: [0.0, 0.0, 0.0],
    };
    let mut vol = PoseVolume::new(
        [1.0; 4],
        [1.0; 4],
        0,
        Some(link),
        FLAG_ACTIVE | FLAG_TRANSFORM,
    );
    let mut fake = links(None);
    let _ = vol.transform(&mut fake);
}

// The pose blend.

struct FillScript {
    sample: PoseSample,
    calls: u32,
}

impl PoseFill for FillScript {
    fn fill_pose(&mut self, _vol: &PoseVolume) -> PoseSample {
        self.calls += 1;
        self.sample
    }
}

fn filler(a: [f32; 4], b: [f32; 4], c: u32) -> FillScript {
    FillScript {
        sample: PoseSample { a, b, c },
        calls: 0,
    }
}

#[test]
fn blend_copy_path_copies_and_squares() {
    let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, FLAG_COPY);
    let mut fake = filler([1.0, 2.0, 3.0, 4.0], [5.0; 4], 3.0f32.to_bits());
    let got = vol.blend(&mut fake, 0.5);
    assert_eq!(fake.calls, 1);
    assert_eq!(got.out1, [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(got.out2, 9.0);
}

#[test]
fn blend_path_averages_and_sums_deviations() {
    // Equal fills: the vector is the value, the sum is the tag term alone.
    let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, 0);
    let mut fake = filler([2.0, 2.0, 2.0, 7.0], [2.0, 2.0, 2.0, 8.0], 4.0f32.to_bits());
    let got = vol.blend(&mut fake, 0.5);
    assert_eq!(got.out1, [2.0, 2.0, 2.0, 8.0]);
    assert_eq!(got.out2, 4.0);
    // Split fills: out1 = [1, 2, 3], deviations 1 each way per lane.
    let mut fake = filler([0.0, 0.0, 0.0, 0.0], [2.0, 4.0, 6.0, 1.0], 0.0f32.to_bits());
    let got = vol.blend(&mut fake, 0.5);
    assert_eq!(got.out1, [1.0, 2.0, 3.0, 1.0]);
    assert_eq!(got.out2, 1.0 + 4.0 + 9.0);
}

// The volume test.

struct SolverScript {
    angle: f32,
    cos: f32,
    sin: f32,
    norms: VecDeque<[u32; 3]>,
    norm_calls: Vec<(NormSlot, [u32; 3], u32)>,
}

impl ConeSolvers for SolverScript {
    fn base_angle(&mut self, _a0: f32, _a1: f32, _b0: f32, _b1: f32) -> f32 {
        self.angle
    }
    fn cos_factor(&mut self, _ang: f32) -> f32 {
        self.cos
    }
    fn sin_factor(&mut self, _ang: f32) -> f32 {
        self.sin
    }
    fn normalise(&mut self, slot: NormSlot, dst: &mut [u32; 3], src: &[u32; 3], count: u32) {
        self.norm_calls.push((slot, *src, count));
        *dst = self.norms.pop_front().unwrap();
    }
}

fn tuning() -> AngleTuning {
    AngleTuning {
        half: 0.5,
        one: 1.0,
        half_pi: 0.0,
        tau: 6.283_185_5,
        neg_mask: 0,
        abs_mask: 0x7FFF_FFFF,
    }
}

fn solvers(norms: Vec<[u32; 3]>) -> SolverScript {
    SolverScript {
        angle: 0.0,
        cos: 1.0,
        sin: 0.0,
        norms: VecDeque::from(norms),
        norm_calls: Vec::new(),
    }
}

#[test]
fn cone_inactive_is_false_without_fills() {
    let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, 0);
    let mut fake = filler([0.0; 4], [0.0; 4], 0);
    let mut s = solvers(vec![]);
    assert!(!vol.contains(&mut fake, &mut s, &tuning(), [0.0, 0.0, 0.0]));
    assert_eq!(fake.calls, 0);
}

#[test]
fn cone_sphere_gate_is_strict() {
    let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, FLAG_ACTIVE | FLAG_SPHERE);
    // Tag 2 at the origin: inside at distance 1, out at distance 2 and 3.
    for (dist, want) in [(1.0, true), (2.0, false), (3.0, false)] {
        let mut fake = filler([0.0; 4], [0.0; 4], 2.0f32.to_bits());
        let mut s = solvers(vec![]);
        assert_eq!(
            vol.contains(&mut fake, &mut s, &tuning(), [dist, 0.0, 0.0]),
            want,
            "dist {dist}"
        );
        assert!(s.norm_calls.is_empty());
    }
}

#[test]
fn cone_full_pass_and_early_exit() {
    let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, FLAG_ACTIVE);
    let a = [0.0, 0.0, 0.0, 0.0];
    let b = [4.0, 0.0, 10.0, 0.0];
    // Dots 2 and 0.5 inside lengths 4 and 1, height 5 between 0 and 10.
    let pass = vec![
        [1.0f32.to_bits(), 0, 0],
        [2.0f32.to_bits(), 0, 0],
        [0.5f32.to_bits(), 0, 0],
        [1.0f32.to_bits(), 0, 0],
    ];
    let mut fake = filler(a, b, 2.0f32.to_bits());
    let mut s = solvers(pass);
    assert!(vol.contains(&mut fake, &mut s, &tuning(), [2.0, 0.0, 5.0]));
    assert_eq!(s.norm_calls.len(), 4);
    assert_eq!(
        s.norm_calls.iter().map(|c| c.0).collect::<Vec<_>>(),
        vec![
            NormSlot::First,
            NormSlot::Second,
            NormSlot::Third,
            NormSlot::Fourth
        ]
    );
    // A negative first dot exits after the second normaliser.
    let mut fake = filler(a, b, 2.0f32.to_bits());
    let mut s = solvers(vec![[1.0f32.to_bits(), 0, 0], [(-1.0f32).to_bits(), 0, 0]]);
    assert!(!vol.contains(&mut fake, &mut s, &tuning(), [2.0, 0.0, 5.0]));
    assert_eq!(s.norm_calls.len(), 2);
}

// The kind-0x11 builder.

struct BuildScript {
    ped: PedTaskOutcome,
    found: u32,
    built: Option<Handle32<Kind11Task>>,
    ped_calls: Vec<(u32, u32)>,
    resolve_calls: Vec<(u32, u32)>,
    build_calls: Vec<(u32, u32, Vec<u32>, u32)>,
}

impl TaskBuildCtx for BuildScript {
    fn ped_task(&mut self, mgr: Option<Handle32<PedMgr>>, handle: u32) -> PedTaskOutcome {
        self.ped_calls.push((Handle32::raw_or_zero(mgr), handle));
        self.ped
    }
    fn resolve_arg(&mut self, lookup: Option<Handle32<ArgLookup>>, arg1: u32) -> u32 {
        self.resolve_calls
            .push((Handle32::raw_or_zero(lookup), arg1));
        self.found
    }
    fn build_kind11(
        &mut self,
        handle: u32,
        found: u32,
        words: &[u32],
        kind: u32,
    ) -> Option<Handle32<Kind11Task>> {
        self.build_calls.push((handle, found, words.to_vec(), kind));
        self.built
    }
}

fn managers() -> BuildManagers {
    BuildManagers::new(Handle32::new(0x100), Handle32::new(0x200))
}

fn build_script(ped: PedTaskOutcome) -> BuildScript {
    BuildScript {
        ped,
        found: 0x300,
        built: Handle32::new(0x400),
        ped_calls: Vec::new(),
        resolve_calls: Vec::new(),
        build_calls: Vec::new(),
    }
}

#[test]
fn build_kind_constant_is_seventeen() {
    assert_eq!(KIND_11, 0x11);
}

#[test]
fn build_null_handle_skips_the_ped_lookup() {
    let mgrs = managers();
    let mut ctx = build_script(PedTaskOutcome::Build);
    assert_eq!(mgrs.build_task(&mut ctx, 0, 11, [12]), Handle32::new(0x400));
    assert!(ctx.ped_calls.is_empty());
    assert_eq!(ctx.resolve_calls, vec![(0x200, 11)]);
    assert_eq!(ctx.build_calls, vec![(0, 0x300, vec![12], KIND_11)]);
    let mut ctx = build_script(PedTaskOutcome::Build);
    assert_eq!(
        mgrs.build_task(&mut ctx, 0, 11, [12, 13]),
        Handle32::new(0x400)
    );
    assert_eq!(ctx.build_calls, vec![(0, 0x300, vec![12, 13], KIND_11)]);
}

#[test]
fn build_kept_task_returns_without_resolving() {
    let mgrs = managers();
    let mut ctx = build_script(PedTaskOutcome::Keep(Handle32::new(0x500).unwrap()));
    assert_eq!(mgrs.build_task(&mut ctx, 9, 11, [12]), Handle32::new(0x500));
    assert_eq!(ctx.ped_calls, vec![(0x100, 9)]);
    assert!(ctx.resolve_calls.is_empty());
    assert!(ctx.build_calls.is_empty());
}

#[test]
fn build_fallthrough_builds_and_answers_null_when_nothing_answers() {
    let mgrs = managers();
    let mut ctx = build_script(PedTaskOutcome::Build);
    assert_eq!(mgrs.build_task(&mut ctx, 9, 11, [12]), Handle32::new(0x400));
    assert_eq!(ctx.ped_calls, vec![(0x100, 9)]);
    assert_eq!(ctx.resolve_calls, vec![(0x200, 11)]);
    assert_eq!(ctx.build_calls, vec![(9, 0x300, vec![12], KIND_11)]);
    let mut ctx = build_script(PedTaskOutcome::Build);
    ctx.built = None;
    assert_eq!(mgrs.build_task(&mut ctx, 9, 11, [12]), None);
}

#[test]
#[should_panic(expected = "null ped lookup faults")]
fn build_null_ped_panics() {
    let mgrs = managers();
    let mut ctx = build_script(PedTaskOutcome::NullPed);
    let _ = mgrs.build_task(&mut ctx, 9, 11, [12]);
}

// The entry-chain cloner.

struct CloneScript {
    finds: VecDeque<Option<FoundEntry>>,
    products: VecDeque<Option<Handle32<CloneProduct>>>,
    makes: Vec<(bool, u32, u32, u32, u32)>,
    sets: Vec<(u32, u32, u32)>,
    stores: Vec<(u32, u32)>,
    marks: Vec<u32>,
}

impl ChainClone for CloneScript {
    fn find_first(&mut self, _key: u32) -> Option<FoundEntry> {
        self.finds.pop_front().unwrap()
    }
    fn find_next(&mut self, _key: u32) -> Option<FoundEntry> {
        self.finds.pop_front().unwrap()
    }
    fn make_full(
        &mut self,
        _owner: Option<Handle32<ChainOwner>>,
        y: u32,
        x: u32,
        flags: u32,
        aux: u32,
        priority: u32,
        none: u32,
    ) -> Option<Handle32<CloneProduct>> {
        assert_eq!((priority, none), (PRIORITY, NONE));
        self.makes.push((true, y, x, flags, aux));
        self.products.pop_front().unwrap()
    }
    fn make_alt(
        &mut self,
        _owner: Option<Handle32<ChainOwner>>,
        a: u32,
        b: u32,
        flags: u32,
        aux: u32,
        priority: u32,
    ) -> Option<Handle32<CloneProduct>> {
        assert_eq!(priority, PRIORITY);
        self.makes.push((false, a, b, flags, aux));
        self.products.pop_front().unwrap()
    }
    fn set_first(&mut self, product: Handle32<CloneProduct>, f1: u32) {
        self.sets.push((0, product.get(), f1));
    }
    fn store_second(&mut self, product: Handle32<CloneProduct>, f2: u32) {
        self.stores.push((product.get(), f2));
    }
    fn set_third(&mut self, product: Handle32<CloneProduct>, f3: u32) {
        self.sets.push((2, product.get(), f3));
    }
    fn mark_done(&mut self, product: Handle32<CloneProduct>) {
        self.marks.push(product.get());
    }
}

fn chain_entry(flags: u32, aux: u32, x: u32, y: u32) -> ChainEntry {
    ChainEntry {
        flags,
        aux,
        x,
        y,
        alt_a: 51,
        alt_b: 52,
        f1: 61,
        f2: 62,
        f3: 63,
    }
}

fn clone_script(entries: &[ChainEntry], products: &[u32]) -> CloneScript {
    CloneScript {
        finds: entries
            .iter()
            .map(|e| {
                Some(FoundEntry {
                    node: Handle32::new(0x700).unwrap(),
                    entry: *e,
                })
            })
            .chain([None])
            .collect(),
        products: products.iter().map(|p| Handle32::new(*p)).collect(),
        makes: Vec::new(),
        sets: Vec::new(),
        stores: Vec::new(),
        marks: Vec::new(),
    }
}

fn cloner() -> ChainCloner {
    ChainCloner::new(Handle32::new(0x600))
}

#[test]
fn clone_constants_match_the_32bit_form() {
    assert_eq!(FLAG_EXTRA, 0x4000);
    assert_eq!(FLAG_DONE, 0x0040_0000);
    assert_eq!(PRIORITY, 0xC100_0000);
    assert_eq!(NONE, 0xFFFF_FFFF);
}

#[test]
fn clone_empty_chain_makes_no_calls() {
    let mut ctx = clone_script(&[], &[]);
    cloner().clone_chain::<CloneScript, true>(&mut ctx, 5);
    assert!(ctx.makes.is_empty());
    assert!(ctx.sets.is_empty());
    assert!(ctx.stores.is_empty());
    assert!(ctx.marks.is_empty());
}

#[test]
fn clone_picks_full_or_alt_by_selectors() {
    // Zero aux takes the full maker even with NONE selectors.
    let mut ctx = clone_script(&[chain_entry(1, 0, NONE, NONE)], &[0x800]);
    cloner().clone_chain::<CloneScript, true>(&mut ctx, 5);
    assert_eq!(ctx.makes, vec![(true, NONE, NONE, 1 | FLAG_EXTRA, 0)]);
    // Nonzero aux with a NONE selector takes the alt maker.
    let mut ctx = clone_script(&[chain_entry(1, 7, 11, NONE)], &[0x800]);
    cloner().clone_chain::<CloneScript, true>(&mut ctx, 5);
    assert_eq!(ctx.makes, vec![(false, 51, 52, 1 | FLAG_EXTRA, 7)]);
    let mut ctx = clone_script(&[chain_entry(1, 7, NONE, 12)], &[0x800]);
    cloner().clone_chain::<CloneScript, true>(&mut ctx, 5);
    assert_eq!(ctx.makes, vec![(false, 51, 52, 1 | FLAG_EXTRA, 7)]);
    // Nonzero aux with valid selectors takes the full maker.
    let mut ctx = clone_script(&[chain_entry(1, 7, 11, 12)], &[0x800]);
    cloner().clone_chain::<CloneScript, true>(&mut ctx, 5);
    assert_eq!(ctx.makes, vec![(true, 12, 11, 1 | FLAG_EXTRA, 7)]);
}

#[test]
fn clone_null_product_skips_the_finish() {
    let mut ctx = clone_script(&[chain_entry(1, 0, 11, 12)], &[0]);
    cloner().clone_chain::<CloneScript, true>(&mut ctx, 5);
    assert_eq!(ctx.makes.len(), 1);
    assert!(ctx.sets.is_empty());
    assert!(ctx.stores.is_empty());
    assert!(ctx.marks.is_empty());
}

#[test]
fn clone_done_flag_marks_only_when_set() {
    let mut ctx = clone_script(&[chain_entry(1, 0, 11, 12)], &[0x800]);
    cloner().clone_chain::<CloneScript, true>(&mut ctx, 5);
    assert_eq!(ctx.sets, vec![(0, 0x800, 61), (2, 0x800, 63)]);
    assert_eq!(ctx.stores, vec![(0x800, 62)]);
    assert_eq!(ctx.marks, vec![0x800]);
    let mut ctx = clone_script(&[chain_entry(1, 0, 11, 12)], &[0x800]);
    cloner().clone_chain::<CloneScript, false>(&mut ctx, 5);
    assert_eq!(ctx.sets, vec![(0, 0x800, 61), (2, 0x800, 63)]);
    assert_eq!(ctx.stores, vec![(0x800, 62)]);
    assert!(ctx.marks.is_empty());
}
