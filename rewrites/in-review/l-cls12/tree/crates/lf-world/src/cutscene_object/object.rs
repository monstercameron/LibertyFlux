//! The cutscene object and its world: owning data, static dispatch.
//!
//! Lifted from the verified rewrites of `CCutsceneObject`. The 32-bit
//! object is a header with an inline triple, an attached placement record,
//! an embedded helper, flag words, a bound radius with a corner pair, a
//! member link and a mode word; here those are plain fields, with the
//! placement, helper, member and draw collaborators carried as opaque
//! cookies and reached through [`CutsceneWorld`].

use lf_core::Handle32;

/// Tag for the opaque cookie of the embedded placement member.
pub struct PlacementTag;
/// Tag for the opaque cookie of the embedded helper member.
pub struct HelperTag;
/// Tag for the opaque cookie of the linked member object.
pub struct MemberTag;
/// Tag for the opaque cookie of an emitted draw-command target.
pub struct DrawTag;

/// A 3x4 placement record: the three coefficient columns plus the origin.
///
/// The original stores it as twelve read floats with three unread pad
/// words interleaved; only the read words are modelled. The origin triple
/// doubles as the words the triple copy hands out when a record is
/// attached.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Matrix34 {
    /// Coefficients of the corner x for outputs x, y, z.
    pub vx: [f32; 3],
    /// Coefficients of the corner y for outputs x, y, z.
    pub vy: [f32; 3],
    /// Coefficients of the corner z for outputs x, y, z.
    pub vz: [f32; 3],
    /// The translation triple.
    pub origin: [f32; 3],
}

/// A 16-byte pose record: integer head and tail around two floats.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PoseRecord {
    /// The leading integer word.
    pub head: u32,
    /// The first float word.
    pub mid0: f32,
    /// The second float word.
    pub mid1: f32,
    /// The trailing integer word, also the copy's answer.
    pub tail: u32,
}

/// A world-space bounds box: minimum corner, maximum corner, two tags.
///
/// The two tag words are matrix words the box computation copies through
/// untouched; they travel as words, never as floats.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct WorldBounds {
    /// The minimum corner (center minus extent).
    pub min: [f32; 3],
    /// The copied first tag word.
    pub lo_tag: u32,
    /// The maximum corner (center plus extent).
    pub max: [f32; 3],
    /// The copied second tag word.
    pub hi_tag: u32,
}

/// A bounds rectangle over the pushed corners: one float per side.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundsRect {
    /// The smallest pushed x.
    pub min_x: f32,
    /// The largest pushed y.
    pub max_y: f32,
    /// The largest pushed x.
    pub max_x: f32,
    /// The smallest pushed y.
    pub min_y: f32,
}

/// The four words the world-space box reads from shared state: three
/// scale factors and the absolute-value mask.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundsScale {
    /// The x scale factor.
    pub gx: f32,
    /// The y scale factor.
    pub gy: f32,
    /// The z scale factor.
    pub gz: f32,
    /// The bit mask applied to nine matrix words.
    pub abs_mask: u32,
}

/// What the cutscene object needs from the world around it: its placement
/// transforms, its own pose slots, its member link, its refresh entries,
/// its helper, and its draw emitters.
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with addresses narrowed to opaque cookies, scratch-buffer
/// addresses dropped, and answers the original drops unmodelled (see the
/// registry for the per-method narrowings).
pub trait CutsceneWorld {
    /// The placement matrix for the box computation: sixteen words, read
    /// as floats except the two copied tags.
    fn placement_matrix(&mut self, placement: Handle32<PlacementTag>) -> [u32; 16];
    /// Pushes one corner through the placement, answering x and y (the
    /// callee's third word is never read back).
    fn transform_corner(
        &mut self,
        placement: Handle32<PlacementTag>,
        corner: [f32; 3],
    ) -> [f32; 2];
    /// The object's own pose-record slot.
    fn pose_slot(&mut self) -> PoseRecord;
    /// The object's own follow-up slot after a pose copy; answers its answer.
    fn pose_followup(&mut self) -> u32;
    /// The two-party notification carrying a word; answers its answer.
    fn notify(&mut self, word: u32) -> u32;
    /// The member link's successor slot with the same word; answers its answer.
    fn member_forward(&mut self, member: Handle32<MemberTag>, word: u32) -> u32;
    /// The refresh hook; its answer is dropped.
    fn refresh_hook(&mut self);
    /// The refresh tail; its answer is the result.
    fn refresh_run(&mut self) -> u32;
    /// The helper member's command slot; answers its answer.
    fn helper_command(
        &mut self,
        helper: Handle32<HelperTag>,
        a: u32,
        b: u32,
    ) -> u32;
    /// The first mode-0 draw emitter; its answer is dropped.
    fn draw_mode0_a(&mut self, a0: u32, a1: u32);
    /// The second mode-0 draw emitter; its answer is dropped.
    fn draw_mode0_b(&mut self, a0: u32, a1: u32);
    /// The shared draw emitter; answers its target, if any.
    fn draw_emit(&mut self, a0: u32, a1: u32, a2: u32) -> Option<Handle32<DrawTag>>;
    /// Marks the mode-1 emit target (the flag-bit set on its byte).
    fn mark_emitted(&mut self, target: Handle32<DrawTag>);
}

/// A cutscene object, owning its words, flags, corners and links.
#[derive(Debug, Clone, PartialEq)]
pub struct CutsceneObject {
    /// The inline triple, handed out when no record is attached.
    pub inline_triple: [u32; 3],
    /// The embedded placement member, reached only through the world.
    pub placement: Handle32<PlacementTag>,
    /// The attached placement record, if one is attached.
    pub attached: Option<Matrix34>,
    /// The embedded helper member, reached only through the world.
    pub helper: Handle32<HelperTag>,
    /// The helper's flag byte, cleared by the helper reset.
    pub helper_flag: u8,
    /// The flag word tested for zero.
    pub flag_word: u32,
    /// The first refresh flag byte.
    pub refresh_a: u8,
    /// The second refresh flag byte.
    pub refresh_b: u8,
    /// The bound radius.
    pub radius: f32,
    /// The minimum corner of the local bounds.
    pub corner_a: [f32; 3],
    /// The maximum corner of the local bounds.
    pub corner_b: [f32; 3],
    /// The linked member object, if one is linked.
    pub member_a: Option<Handle32<MemberTag>>,
    /// The mode word selecting the draw path.
    pub mode: u32,
}

/// Adds exactly like the original's ordered float sequence.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Subtracts exactly like the original's ordered float sequence.
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// Multiplies exactly like the original's ordered float sequence.
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// The rectangle seeds: plus and minus one million, exactly.
const SEED_POS: f32 = f32::from_bits(0x4974_2400);
/// The rectangle seeds: plus and minus one million, exactly.
const SEED_NEG: f32 = f32::from_bits(0xC974_2400);

impl CutsceneObject {
    /// The bound radius. The original answers through the float stack,
    /// which quiets a signalling NaN (sets the quiet bit, keeps the
    /// payload); the lift reproduces that quieting as bit operations, so
    /// the bits match exactly, NaNs included.
    #[must_use]
    pub fn bound_radius(&self) -> f32 {
        let bits = self.radius.to_bits();
        let is_nan = bits & 0x7F80_0000 == 0x7F80_0000 && bits & 0x007F_FFFF != 0;
        if is_nan {
            f32::from_bits(bits | 0x0040_0000)
        } else {
            self.radius
        }
    }

    /// True when the mode word selects the first draw path.
    #[must_use]
    pub fn is_state_0(&self) -> bool {
        self.mode == 0
    }

    /// True when the mode word selects the second draw path.
    #[must_use]
    pub fn is_state_1(&self) -> bool {
        self.mode == 1
    }

    /// True when the mode word selects the third draw path.
    #[must_use]
    pub fn is_state_2(&self) -> bool {
        self.mode == 2
    }

    /// True when the flag word is nonzero.
    #[must_use]
    pub fn is_flag_word_nonzero(&self) -> bool {
        self.flag_word != 0
    }

    /// Copies the three describing words: the attached record's origin
    /// triple when one is attached, otherwise the inline triple.
    pub fn describe_into(&self, out: &mut [u32; 3]) {
        *out = match &self.attached {
            Some(m) => [
                m.origin[0].to_bits(),
                m.origin[1].to_bits(),
                m.origin[2].to_bits(),
            ],
            None => self.inline_triple,
        };
    }

    /// Copies the pose record from the object's own slot, answering the
    /// trailing integer word.
    pub fn pose_into<W: CutsceneWorld>(&self, world: &mut W, out: &mut PoseRecord) -> u32 {
        *out = world.pose_slot();
        out.tail
    }

    /// Copies the pose record like [`CutsceneObject::pose_into`], then runs
    /// the follow-up slot and answers its answer.
    pub fn pose_and_followup<W: CutsceneWorld>(
        &self,
        world: &mut W,
        out: &mut PoseRecord,
    ) -> u32 {
        *out = world.pose_slot();
        world.pose_followup()
    }

    /// Notifies with the word, then hands the word to the linked member's
    /// successor slot when one is linked, answering that answer; with no
    /// link the notification answer stands.
    pub fn forward_word<W: CutsceneWorld>(&self, world: &mut W, word: u32) -> u32 {
        let first = world.notify(word);
        match self.member_a {
            Some(m) => world.member_forward(m, word),
            None => first,
        }
    }

    /// Runs the refresh hook and tail when either refresh flag is set,
    /// answering the tail's answer; otherwise answers zero with no calls.
    pub fn maybe_refresh<W: CutsceneWorld>(&self, world: &mut W) -> u32 {
        if self.refresh_a != 0 || self.refresh_b != 0 {
            world.refresh_hook();
            world.refresh_run()
        } else {
            0
        }
    }

    /// Clears the helper flag byte, then issues the helper command with
    /// the fixed pair, answering its answer.
    pub fn reset_helper<W: CutsceneWorld>(&mut self, world: &mut W) -> u32 {
        self.helper_flag = 0;
        world.helper_command(self.helper, 0, 0xFFFF_FFFE)
    }

    /// Emits draw commands for the mode word: both mode-0 emitters in
    /// order for mode 0; the shared emitter plus the mark for mode 1 when
    /// it answers a target; the shared emitter for mode 2 unless the flag
    /// word is set; nothing for any other mode.
    pub fn emit_draw_commands<W: CutsceneWorld>(
        &self,
        world: &mut W,
        a0: u32,
        a1: u32,
        a2: u32,
    ) {
        if self.mode == 0 {
            world.draw_mode0_a(a0, a1);
            world.draw_mode0_b(a0, a1);
        } else if self.mode == 1 {
            if let Some(t) = world.draw_emit(a0, a1, a2) {
                world.mark_emitted(t);
            }
        } else if self.mode == 2 {
            if !self.is_flag_word_nonzero() {
                let _ = world.draw_emit(a0, a1, a2);
            }
        }
    }

    /// Pushes one corner through the attached record: each output row is
    /// the vy term plus the vx term, plus the vz term, plus the origin.
    fn push_corner(mat: &Matrix34, vx: f32, vy: f32, vz: f32) -> (f32, f32) {
        let rx = fadd(
            fadd(
                fadd(fmul(mat.vy[0], vy), fmul(mat.vx[0], vx)), fmul(mat.vz[0], vz)),
            mat.origin[0],
        );
        let ry = fadd(
            fadd(
                fadd(fmul(mat.vy[1], vy), fmul(mat.vx[1], vx)), fmul(mat.vz[1], vz)),
            mat.origin[1],
        );
        // The third row is computed by three of the four original rounds
        // and read back by none of them; computing it always is identical
        // in every observable bit.
        let _rz = fadd(
            fadd(
                fadd(fmul(mat.vy[2], vy), fmul(mat.vx[2], vx)), fmul(mat.vz[2], vz)),
            mat.origin[2],
        );
        (rx, ry)
    }

    /// Folds one pushed corner into the rectangle with ordered strict
    /// comparisons, so a NaN corner (or a tie) never replaces a bound.
    fn fold_corner(out: &mut BoundsRect, rx: f32, ry: f32) {
        if rx < out.min_x {
            out.min_x = rx;
        }
        if out.max_x < rx {
            out.max_x = rx;
        }
        if ry < out.min_y {
            out.min_y = ry;
        }
        if out.max_y < ry {
            out.max_y = ry;
        }
    }

    /// The bounding rectangle over four corner pushes: the two corners as
    /// stored, then each corner's x against the other's y and z. Through
    /// the attached record when one is attached, else through the
    /// placement transform one corner at a time.
    pub fn bounding_rect<W: CutsceneWorld>(&self, world: &mut W, out: &mut BoundsRect) {
        let corners = [
            (self.corner_a[0], self.corner_a[1], self.corner_a[2]),
            (self.corner_b[0], self.corner_b[1], self.corner_b[2]),
            (self.corner_b[0], self.corner_a[1], self.corner_a[2]),
            (self.corner_a[0], self.corner_b[1], self.corner_b[2]),
        ];
        *out = BoundsRect {
            min_x: SEED_POS,
            max_y: SEED_NEG,
            max_x: SEED_NEG,
            min_y: SEED_POS,
        };
        for (cx, cy, cz) in corners {
            let (rx, ry) = match &self.attached {
                Some(m) => Self::push_corner(m, cx, cy, cz),
                None => {
                    let p = world.transform_corner(self.placement, [cx, cy, cz]);
                    (p[0], p[1])
                }
            };
            Self::fold_corner(out, rx, ry);
        }
    }

    /// The world-space bounds box: the scaled corner sums (center) and
    /// differences (extent) through the placement matrix, the center
    /// through the plain matrix with translation, the extent through the
    /// masked matrix, answering center-minus-extent, a tag, center-plus-
    /// extent, a tag. Every float operation is in the original's order.
    pub fn world_bounds<W: CutsceneWorld>(
        &self,
        world: &mut W,
        scales: &BoundsScale,
        out: &mut WorldBounds,
    ) {
        let buf = world.placement_matrix(self.placement);
        let word = |i: usize| f32::from_bits(buf[i]);
        let masked = |i: usize| f32::from_bits(buf[i] & scales.abs_mask);
        let (ax, ay, az) = (self.corner_a[0], self.corner_a[1], self.corner_a[2]);
        let (bx, by, bz) = (self.corner_b[0], self.corner_b[1], self.corner_b[2]);

        // Scaled corner sums (center) and differences (extent). The two
        // operand orders below are the original's, mixed as written.
        let px_sum = fmul(scales.gx, fadd(bx, ax));
        let pz_sum = fmul(fadd(bz, az), scales.gz);
        let py_sum = fmul(scales.gy, fadd(by, ay));
        let px_dif = fmul(fsub(bx, ax), scales.gx);
        let pz_dif = fmul(fsub(bz, az), scales.gz);
        let py_dif = fmul(fsub(by, ay), scales.gy);

        // Center through the matrix.
        let b0_px = fmul(word(0), px_sum);
        let b4_py = fmul(word(4), py_sum);
        let b1_px = fmul(word(1), px_sum);
        let cx_a = fadd(b4_py, b0_px);
        let b8_pz = fmul(word(8), pz_sum);
        let b2_px = fmul(word(2), px_sum);
        let cx_b = fadd(cx_a, b8_pz);
        let b9_pz = fmul(word(9), pz_sum);
        let b10_pz = fmul(word(10), pz_sum);
        let cx = fadd(cx_b, word(12));

        let b5_py = fmul(word(5), py_sum);
        let cy_a = fadd(b5_py, b1_px);
        let cy_b = fadd(cy_a, b9_pz);
        let b6_py = fmul(word(6), py_sum);
        let cy = fadd(cy_b, word(13));

        let cz_a = fadd(b6_py, b2_px);
        let cz_b = fadd(cz_a, b10_pz);
        let cz = fadd(cz_b, word(14));

        // Extent through the masked matrix.
        let ex = fadd(
            fadd(fmul(masked(1), py_dif), fmul(masked(0), px_dif)),
            fmul(masked(2), pz_dif),
        );
        let ey = fadd(
            fadd(fmul(masked(5), py_dif), fmul(masked(4), px_dif)),
            fmul(masked(6), pz_dif),
        );
        let ez = fadd(
            fadd(fmul(masked(9), py_dif), fmul(masked(8), px_dif)),
            fmul(masked(10), pz_dif),
        );

        out.min = [fsub(cx, ex), fsub(cy, ey), fsub(cz, ez)];
        out.lo_tag = buf[3];
        out.max = [fadd(cx, ex), fadd(cy, ey), fadd(cz, ez)];
        out.hi_tag = buf[7];
    }
}
