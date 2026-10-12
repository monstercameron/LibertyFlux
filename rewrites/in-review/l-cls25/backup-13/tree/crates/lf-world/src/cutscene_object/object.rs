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
/// Tag for the opaque cookie of a teardown block.
pub struct BlockTag;
/// Tag for the opaque cookie of the teardown context object.
pub struct CtxTag;
/// Tag for the opaque cookie of the setup store chain.
pub struct ChainTag;
/// Tag for the opaque cookie of an update table entry.
pub struct EntryTag;
/// Tag for the opaque cookie of the early-exit block.
pub struct EarlyTag;
/// Tag for the opaque cookie of a bone set.
pub struct BoneTag;
/// Tag for the opaque cookie of the attached record's identity.
pub struct AttachTag;

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

/// One update table entry: the flag, mode, weight and index words the
/// per-frame update reads, with the entry's identity for the setup calls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpdateEntry {
    /// The entry's identity.
    pub id: Handle32<EntryTag>,
    /// The early-exit flag byte.
    pub flag: u8,
    /// The mode word: 1 selects the two-bone blend, anything else the
    /// four-bone average.
    pub mode: u32,
    /// The blend weight.
    pub weight: f32,
    /// The four index words of the entry's parameter block.
    pub index_words: [u32; 4],
}

/// One bone row answer: the triple and the set it was read from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoneRow {
    /// The set the row was read from.
    pub set: Handle32<BoneTag>,
    /// The row triple.
    pub xyz: [f32; 3],
}

/// The shared scalar words the per-frame update reads.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct UpdateScalars {
    /// The entry-sequence byte: the second entry call runs unless set.
    pub entry_seq_byte: u8,
    /// The selected counter, or -1 to use the alternate.
    pub sel: i32,
    /// The alternate counter.
    pub edx_alt: i32,
    /// The first maze counter.
    pub eax: i32,
    /// The second maze counter.
    pub ecx: i32,
    /// The float window bounds and scale.
    pub win_lo: f32,
    /// The float window bounds and scale.
    pub win_hi: f32,
    /// The float window bounds and scale.
    pub win_scale: f32,
    /// The setup gate: the setup calls run unless bit 1 is set.
    pub setup_flag: u32,
    /// The value stored through the chain on setup.
    pub store_val: f32,
    /// The blend factors.
    pub k0: f32,
    /// The blend factors.
    pub k1: f32,
    /// The weight scale.
    pub wgt_scale: f32,
    /// The matrix-row scale of the blend's B block.
    pub e18_scale: f32,
    /// The shared normalizer.
    pub kn: f32,
    /// The weight mixer.
    pub wx: f32,
    /// The blend's scalar factors.
    pub j_a4: f32,
    /// The blend's scalar factors.
    pub j_a5: f32,
    /// The blend's scalar factors.
    pub j_a6: f32,
    /// The blend's trailing word.
    pub j_a8: u32,
    /// The average's scalar factors.
    pub k_a4: f32,
    /// The average's scalar factors.
    pub k_a5: f32,
    /// The average's scalar factors.
    pub k_a6: f32,
    /// The average's trailing word.
    pub k_a8: u32,
    /// The average's quarter factor.
    pub qk: f32,
}

/// The shared blend accumulator: a flag bit plus a triple. The update
/// reloads the triple when the bit is set, else zeroes it and sets the bit.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Accumulator {
    /// The loaded flag.
    pub flag: u32,
    /// The accumulated triple.
    pub vals: [f32; 3],
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
    fn transform_corner(&mut self, placement: Handle32<PlacementTag>, corner: [f32; 3])
    -> [f32; 2];
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
    fn helper_command(&mut self, helper: Handle32<HelperTag>, a: u32, b: u32) -> u32;
    /// The first mode-0 draw emitter; its answer is dropped.
    fn draw_mode0_a(&mut self, a0: u32, a1: u32);
    /// The second mode-0 draw emitter; its answer is dropped.
    fn draw_mode0_b(&mut self, a0: u32, a1: u32);
    /// The shared draw emitter; answers its target, if any.
    fn draw_emit(&mut self, a0: u32, a1: u32, a2: u32) -> Option<Handle32<DrawTag>>;
    /// Marks the mode-1 emit target (the flag-bit set on its byte).
    fn mark_emitted(&mut self, target: Handle32<DrawTag>);
    /// Destroys a linked member through its deleting entry; the answer
    /// is dropped.
    fn destroy_member(&mut self, member: Handle32<MemberTag>);
    /// Tears a block down; the answer is dropped.
    fn teardown_block(&mut self, block: Handle32<BlockTag>);
    /// Frees a torn-down block; the answer is dropped.
    fn free_block(&mut self, block: Handle32<BlockTag>);
    /// Asks whether the registry step runs; only the low byte of the
    /// answer is tested.
    fn ask_registry(&mut self) -> u32;
    /// The registry entry word for a table index (the table itself is
    /// not modelled: one word per index).
    fn registry_word(&mut self, index: i16) -> i32;
    /// Asks whether the registry word passes the gate; only the low
    /// byte of the answer is tested.
    fn ask_gate(&mut self, word: u32) -> u32;
    /// Runs the registry step on its fixed context with a zero word.
    fn registry_run(&mut self);
    /// Runs the teardown context entry with mode 3.
    fn context_run(&mut self, ctx: Option<Handle32<CtxTag>>, mode: u32);
    /// Hands the registry word on.
    fn registry_tell(&mut self, word: u32);
    /// The base teardown entry; answers its answer.
    fn base_destroy(&mut self) -> u32;
    /// The update's first entry call.
    fn entry_notify(&mut self);
    /// The update's second entry call.
    fn entry_second(&mut self);
    /// The object's own first guard slot; only the low byte of the
    /// answer is tested.
    fn guard_a(&mut self) -> u32;
    /// The object's own second guard slot; a zero low byte returns the
    /// whole answer.
    fn guard_b(&mut self) -> u32;
    /// The member probe slot on the second member; its answer is dropped.
    fn member_probe(&mut self, member: Handle32<MemberTag>);
    /// The update table entry for an index (the table itself is not
    /// modelled: one entry per index).
    fn table_entry(&mut self, index: i16) -> UpdateEntry;
    /// The early-exit block; its word is read separately.
    fn early_block(&mut self) -> Handle32<EarlyTag>;
    /// The early-exit block's word.
    fn early_word(&mut self, block: Handle32<EarlyTag>) -> u32;
    /// The early-exit tail; answers its answer.
    fn early_tail(&mut self, word: u32) -> u32;
    /// The early indexed call; answers its answer.
    fn early_call(&mut self, sx: i32, v294: u32, v310: u32) -> u32;
    /// The primary setup call with its fixed words; its answer is dropped.
    fn setup_primary(
        &mut self,
        entry: Handle32<EntryTag>,
        record: Option<Handle32<AttachTag>>,
        fixed: [u32; 6],
    );
    /// The secondary setup call with its fixed words; its answer is dropped.
    fn setup_secondary(
        &mut self,
        entry: Handle32<EntryTag>,
        record: Option<Handle32<AttachTag>>,
        fixed: [u32; 8],
    );
    /// Stores the setup value through the chain.
    fn store_setup(&mut self, chain: Handle32<ChainTag>, value: f32);
    /// One bone row by index, with the set it was read from.
    fn bone_row(&mut self, index: u32) -> BoneRow;
    /// Submits the two-bone blend: the record, three blocks in argument
    /// order, four scalars; answers its answer.
    fn submit_j(
        &mut self,
        record: Option<Handle32<AttachTag>>,
        first: [u32; 4],
        second: [u32; 4],
        third: [u32; 4],
        scalars: [u32; 4],
    ) -> u32;
    /// Submits the four-bone average, shaped like [`CutsceneWorld::submit_j`].
    fn submit_k(
        &mut self,
        record: Option<Handle32<AttachTag>>,
        first: [u32; 4],
        second: [u32; 4],
        third: [u32; 4],
        scalars: [u32; 4],
    ) -> u32;
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
    /// The flag word whose bit the mode-1 teardown sets.
    pub flags_24: u32,
    /// The registry table index.
    pub table_index: i16,
    /// The teardown context object, if one is set.
    pub ctx: Option<Handle32<CtxTag>>,
    /// The registry-step gate word.
    pub gate_d4: u32,
    /// The second linked member object, if one is linked.
    pub member_b: Option<Handle32<MemberTag>>,
    /// The three teardown blocks, in teardown order.
    pub blocks: [Option<Handle32<BlockTag>>; 3],
    /// The done byte, cleared by the teardown.
    pub done_2ac: u8,
    /// The update's script word (only the read half is modelled).
    pub script_word: u16,
    /// The setup store chain, if one is set.
    pub store_chain: Option<Handle32<ChainTag>>,
    /// The attached record's identity for the update's calls.
    pub attached_id: Option<Handle32<AttachTag>>,
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

/// Squares exactly like the original's ordered float sequence.
#[inline(always)]
fn fsqr(x: f32) -> f32 {
    core::hint::black_box(x) * core::hint::black_box(x)
}

/// Takes the square root exactly like the original.
#[inline(always)]
fn fsqrt(x: f32) -> f32 {
    core::hint::black_box(x).sqrt()
}

/// One as the update passes it.
const ONE_BITS: u32 = 0x3F80_0000;

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
    pub fn pose_and_followup<W: CutsceneWorld>(&self, world: &mut W, out: &mut PoseRecord) -> u32 {
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
    pub fn emit_draw_commands<W: CutsceneWorld>(&self, world: &mut W, a0: u32, a1: u32, a2: u32) {
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

    /// Tears the object down and hands off to the base entry, answering
    /// its answer. In mode 0 the linked members are destroyed and their
    /// slots cleared, the set blocks are torn down, freed and cleared,
    /// and the registry step runs when asked, the entry word is not -1,
    /// the gate is set and the gate agrees. In mode 1 the flag bit is
    /// set instead. The done byte is cleared either way. (The
    /// destruction-phase table stamp is 32-bit plumbing: not modelled.)
    pub fn tear_down<W: CutsceneWorld>(&mut self, world: &mut W) -> u32 {
        if self.mode == 0 {
            if let Some(ha) = self.member_a.take() {
                world.destroy_member(ha);
            }
            for slot in self.blocks.iter_mut() {
                if let Some(blk) = slot.take() {
                    world.teardown_block(blk);
                    world.free_block(blk);
                }
            }
            if let Some(hb) = self.member_b.take() {
                world.destroy_member(hb);
            }
            if world.ask_registry() & 0xff != 0 {
                let word = world.registry_word(self.table_index);
                if word != -1 && self.gate_d4 != 0 && world.ask_gate(word as u32) & 0xff != 0 {
                    world.registry_run();
                    world.context_run(self.ctx, 3);
                    world.registry_tell(word as u32);
                }
            }
        }
        if self.mode == 1 {
            self.flags_24 |= 0x0400_0000;
        }
        self.done_2ac = 0;
        world.base_destroy()
    }

    /// The per-frame update: guard dispatch, counter maze with setup,
    /// then either the two-bone blend or the four-bone average submitted
    /// to the pose entry. Every float operation is in the original's order.
    /// Panics where the original dereferences blindly (an unlinked second
    /// member on the early path, a missing record or chain on the setup
    /// and blend paths): those are narrowed domains, never guesses.
    pub fn update<W: CutsceneWorld>(
        &self,
        world: &mut W,
        cfg: &UpdateScalars,
        acc: &mut Accumulator,
    ) -> u32 {
        world.entry_notify();
        if cfg.entry_seq_byte == 0 {
            world.entry_second();
        }
        if world.guard_a() & 0xff != 0 {
            let member = self
                .member_b
                .expect("update early path needs the second member");
            world.member_probe(member);
            let ent = world.table_entry(self.table_index);
            if ent.flag != 0 {
                let block = world.early_block();
                let w = world.early_word(block);
                return world.early_tail(w);
            }
            return world.early_call(
                i32::from(self.table_index),
                Handle32::raw_or_zero(self.blocks[2]),
                Handle32::raw_or_zero(self.member_a),
            );
        }
        let r2 = world.guard_b();
        if r2 & 0xff == 0 {
            return r2;
        }
        let ent = world.table_entry(self.table_index);
        let edx = if cfg.sel != -1 { cfg.sel } else { cfg.edx_alt };
        let w2c = u32::from(self.script_word);
        let go_g = if edx > 0x14 {
            true
        } else {
            let eax = cfg.eax;
            let mut ecx = cfg.ecx;
            if edx > 0x13 {
                if eax != -1 {
                    ecx = eax;
                }
                ecx > ((w2c & 0x3f) as i32)
            } else if edx < 6 {
                true
            } else if edx >= 7 {
                false
            } else {
                if eax != -1 {
                    ecx = eax;
                }
                ecx < ((w2c & 0x3f) as i32)
            }
        };
        let go_g = if go_g {
            true
        } else {
            let x = fmul((w2c as i32) as f32, cfg.win_scale);
            cfg.win_lo > x || cfg.win_hi > x
        };
        if go_g && (cfg.setup_flag & 2) == 0 {
            world.setup_primary(ent.id, self.attached_id, [0x32, 0x33, 1, 1, ONE_BITS, 0]);
            world.setup_secondary(
                ent.id,
                self.attached_id,
                [0x34, 0xffff_ffff, 0x35, 1, 0, 1, ONE_BITS, 0],
            );
            let chain = self
                .store_chain
                .expect("update setup needs the store chain");
            world.store_setup(chain, cfg.store_val);
        }
        let idx = ent.index_words;
        if ent.mode != 1 {
            if (idx[0] as i32) < 0
                || (idx[1] as i32) < 0
                || (idx[2] as i32) < 0
                || (idx[3] as i32) < 0
            {
                return idx[3];
            }
            return self.average_k(world, cfg, &ent, idx);
        }
        if (idx[0] as i32) < 0 {
            return idx[2];
        }
        if (idx[2] as i32) < 0 {
            return idx[2];
        }
        self.blend_j(world, cfg, acc, &ent, idx[0], idx[2])
    }

    /// The four-bone average: mean triple, difference lengths, matrix
    /// blocks, submitted in argument order.
    fn average_k<W: CutsceneWorld>(
        &self,
        world: &mut W,
        cfg: &UpdateScalars,
        ent: &UpdateEntry,
        idx: [u32; 4],
    ) -> u32 {
        let r1 = world.bone_row(idx[0]);
        let r2 = world.bone_row(idx[1]);
        let r3 = world.bone_row(idx[2]);
        let r4 = world.bone_row(idx[3]);
        let (x1, y1, z1) = (r1.xyz[0], r1.xyz[1], r1.xyz[2]);
        let (x2, y2, z2) = (r2.xyz[0], r2.xyz[1], r2.xyz[2]);
        let (x3, y3, z3) = (r3.xyz[0], r3.xyz[1], r3.xyz[2]);
        let (x4, y4, z4) = (r4.xyz[0], r4.xyz[1], r4.xyz[2]);
        let qx = fmul(fadd(fadd(x3, fadd(x2, x1)), x4), cfg.qk);
        let qy = fmul(fadd(fadd(y3, fadd(y2, y1)), y4), cfg.qk);
        let qz = fmul(fadd(fadd(z3, fadd(z2, z1)), z4), cfg.qk);
        let m = self
            .attached
            .as_ref()
            .expect("pose blend needs the attached record");
        let zero = 0.0f32;
        let b0 = fsub(
            fadd(fmul(m.vy[0], zero), fmul(m.vx[0], zero)),
            fmul(m.vz[0], cfg.k0),
        );
        let b4 = fsub(
            fadd(fmul(m.vy[1], zero), fmul(m.vx[1], zero)),
            fmul(m.vz[1], cfg.k0),
        );
        let b8 = fsub(
            fadd(fmul(m.vy[2], zero), fmul(m.vx[2], zero)),
            fmul(m.vz[2], cfg.k0),
        );
        let c0 = fadd(fadd(m.vy[0], fmul(m.vx[0], zero)), fmul(m.vz[0], zero));
        let c4 = fadd(fadd(m.vy[1], fmul(m.vx[1], zero)), fmul(m.vz[1], zero));
        let c8 = fadd(fadd(m.vy[2], fmul(m.vx[2], zero)), fmul(m.vz[2], zero));
        let len_a = fsqrt(fadd(
            fadd(fsqr(fsub(y2, y4)), fsqr(fsub(x2, x4))),
            fsqr(fsub(z2, z4)),
        ));
        let len_b = fsqrt(fadd(
            fadd(fsqr(fsub(y1, y3)), fsqr(fsub(x1, x3))),
            fsqr(fsub(z1, z3)),
        ));
        let arg5 = fmul(fmul(fadd(len_a, len_b), cfg.kn), cfg.k_a5);
        let arg5 = fmul(arg5, cfg.kn);
        let len_c = fsqrt(fadd(
            fadd(fsqr(fsub(y3, y4)), fsqr(fsub(x3, x4))),
            fsqr(fsub(z3, z4)),
        ));
        let len_d = fsqrt(fadd(
            fadd(fsqr(fsub(y2, y1)), fsqr(fsub(x2, x1))),
            fsqr(fsub(z2, z1)),
        ));
        let arg4 = fmul(fmul(fadd(len_c, len_d), cfg.kn), cfg.k_a4);
        let arg4 = fmul(arg4, cfg.kn);
        let arg6 = fmul(fmul(fmul(ent.weight, cfg.wx), cfg.k_a6), cfg.kn);
        world.submit_k(
            self.attached_id,
            [qx.to_bits(), qy.to_bits(), qz.to_bits(), 0],
            [b0.to_bits(), b4.to_bits(), b8.to_bits(), 0],
            [c0.to_bits(), c4.to_bits(), c8.to_bits(), 0],
            [arg4.to_bits(), arg5.to_bits(), arg6.to_bits(), cfg.k_a8],
        )
    }

    /// The two-bone blend: weighted triple plus accumulator, difference
    /// shade, matrix blocks, submitted in argument order.
    fn blend_j<W: CutsceneWorld>(
        &self,
        world: &mut W,
        cfg: &UpdateScalars,
        acc: &mut Accumulator,
        ent: &UpdateEntry,
        idx_a: u32,
        idx_b: u32,
    ) -> u32 {
        let r1 = world.bone_row(idx_a);
        let dk = fsub(cfg.k0, cfg.k1);
        let r2 = world.bone_row(idx_b);
        let (t1x, t1y, t1z) = (r1.xyz[0], r1.xyz[1], r1.xyz[2]);
        let (t2x, t2y, t2z) = (r2.xyz[0], r2.xyz[1], r2.xyz[2]);
        let ix = fadd(fmul(t1x, cfg.k1), fmul(t2x, dk));
        let iy = fadd(fmul(t1y, cfg.k1), fmul(t2y, dk));
        let iz = fadd(fmul(t1z, cfg.k1), fmul(t2z, dk));
        let wgt = fmul(ent.weight, cfg.wgt_scale);
        let (g0, g1, g2) = if acc.flag & 1 != 0 {
            (acc.vals[0], acc.vals[1], acc.vals[2])
        } else {
            acc.flag |= 1;
            acc.vals = [0.0, 0.0, 0.0];
            (0.0, 0.0, 0.0)
        };
        let jx = fadd(ix, g0);
        let jy = fadd(iy, g1);
        let jz = fadd(iz, g2);
        let dx = fsub(t1x, t2x);
        let dy = fsub(t1y, t2y);
        let dz = fsub(t1z, t2z);
        let m = self
            .attached
            .as_ref()
            .expect("pose blend needs the attached record");
        let zero = 0.0f32;
        let b0 = fsub(
            fadd(fmul(m.vy[0], zero), fmul(m.vx[0], zero)),
            fmul(m.vz[0], cfg.k0),
        );
        let b4 = fsub(
            fadd(fmul(m.vy[1], zero), fmul(m.vx[1], zero)),
            fmul(m.vz[1], cfg.k0),
        );
        let b8 = fsub(
            fadd(fmul(m.vy[2], cfg.e18_scale), fmul(m.vx[2], zero)),
            fmul(m.vz[2], cfg.k0),
        );
        let a0 = fadd(fadd(fmul(m.vx[0], zero), m.vy[0]), fmul(m.vz[0], zero));
        let a4 = fadd(fadd(fmul(m.vx[1], zero), m.vy[1]), fmul(m.vz[1], zero));
        let a8 = fadd(fadd(fmul(m.vx[2], zero), m.vy[2]), fmul(m.vz[2], zero));
        let arg4 = fmul(fmul(cfg.j_a4, wgt), cfg.kn);
        let shade = fadd(
            fsqrt(fadd(fadd(fsqr(dy), fsqr(dx)), fsqr(dz))),
            fmul(wgt, cfg.wx),
        );
        let arg5 = fmul(fmul(shade, cfg.j_a5), cfg.kn);
        let arg6 = fmul(fmul(cfg.j_a6, wgt), cfg.kn);
        world.submit_j(
            self.attached_id,
            [jx.to_bits(), jy.to_bits(), jz.to_bits(), 0],
            [b0.to_bits(), b4.to_bits(), b8.to_bits(), 0],
            [a0.to_bits(), a4.to_bits(), a8.to_bits(), 0],
            [arg4.to_bits(), arg5.to_bits(), arg6.to_bits(), cfg.j_a8],
        )
    }

    /// Pushes one corner through the attached record: each output row is
    /// the vy term plus the vx term, plus the vz term, plus the origin.
    fn push_corner(mat: &Matrix34, vx: f32, vy: f32, vz: f32) -> (f32, f32) {
        let rx = fadd(
            fadd(
                fadd(fmul(mat.vy[0], vy), fmul(mat.vx[0], vx)),
                fmul(mat.vz[0], vz),
            ),
            mat.origin[0],
        );
        let ry = fadd(
            fadd(
                fadd(fmul(mat.vy[1], vy), fmul(mat.vx[1], vx)),
                fmul(mat.vz[1], vz),
            ),
            mat.origin[1],
        );
        // The third row is computed by three of the four original rounds
        // and read back by none of them; computing it always is identical
        // in every observable bit.
        let _rz = fadd(
            fadd(
                fadd(fmul(mat.vy[2], vy), fmul(mat.vx[2], vx)),
                fmul(mat.vz[2], vz),
            ),
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
