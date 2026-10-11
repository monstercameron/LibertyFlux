//! The ped task's mover pose: the blend toward a target pose, the drive
//! step that feeds the blend into a state block, and the commit that
//! writes a blended state back into the task.
//!
//! Three verified routines share this data. The task keeps a pose of
//! three position floats, a heading float, a packed mode byte, a flag
//! byte and a level byte ([`MoverPose`]). The blend state block
//! ([`BlendState`]) holds the blended position and heading (heading twice,
//! as a mirror), three factor floats decoded from the packed mode, a top
//! field, a flag, a level float and a flags word the drive step reads.
//! The state block's height is reached through its own virtual methods,
//! and the task's other behaviour through its callees: both travel as
//! [`MoverCallees`], whose methods are the unlifted parts of the family.
//!
//! [`MoverPose::lerp_into`] blends the pose toward a target into a state
//! block, [`MoverPose::drive_blend`] runs one drive step (a snap or a
//! blend, then the packed mode refresh) and [`MoverPose::commit_blend`]
//! writes a state block back into the pose. Every float operation runs in
//! the original's order, so results match bit for bit.

#![forbid(unsafe_code)]

/// The flags word bit that makes a drive step snap instead of blend.
pub const SNAP_FLAG: u32 = 0x400;
/// The readiness answer that forces a snap.
pub const NOT_READY: u32 = 0x00ff_ffff;
/// The constant the commit passes to the task's flag notification.
pub const COMMIT_FLAG_ARG: u32 = 0x0d;
/// Pi, as the original's single-precision constant.
const HALF_TURN: f32 = f32::from_bits(0x4049_0fdb);
/// Two pi, as the original's single-precision constant.
const FULL_TURN: f32 = f32::from_bits(0x40c9_0fdb);
/// Minus two pi, as the original's single-precision constant.
const NEG_FULL_TURN: f32 = f32::from_bits(0xc0c9_0fdb);
/// The reciprocal of 255, as the original's single-precision constant.
const INV_255: f32 = f32::from_bits(0x3b80_8081);
/// The full scale of the level byte.
const LEVEL_SCALE: f32 = 255.0;

/// The unlifted parts of the mover family, as one trait.
///
/// `Handle` stands for an object the family only passes around: the task,
/// the state block, a target pose. The callees and virtual methods
/// receive handles; nothing in the lifted code reads through them.
pub trait MoverCallees {
    /// An opaque reference to an object the family passes to callees.
    type Handle: Copy;

    /// Task setup, run first in a drive step. `target` is `None` when the
    /// step has no target. The answer is ignored.
    fn setup(&mut self, task: Self::Handle, state: Self::Handle, target: Option<Self::Handle>, t: f32);

    /// Readiness probe of the task. [`NOT_READY`] forces a snap; any other
    /// nonzero answer lets a blend run.
    fn ready(&mut self, task: Self::Handle) -> u32;

    /// The blend into the state block for a target, run by a drive step
    /// that is not snapping. The answer is ignored.
    fn blend_into(&mut self, task: Self::Handle, state: Self::Handle, target: Self::Handle, t: f32);

    /// Encodes a two-bit code of the packed mode as a float.
    fn encode(&mut self, task: Self::Handle, code: u32) -> f32;

    /// Commit's flag notification, with [`COMMIT_FLAG_ARG`].
    fn notify_flag(&mut self, task: Self::Handle, arg: u32);

    /// Commit's state notification, taking the blended state block.
    fn notify_state(&mut self, task: Self::Handle, state: Self::Handle);

    /// Decodes a float, given as its bits, to a two-bit code of the packed mode.
    fn decode(&mut self, task: Self::Handle, bits: u32) -> u32;

    /// The state block's height setter (its own virtual method). The
    /// original passes one trailing padding word the lift does not model;
    /// the proof checks that the rewrite passes zero.
    fn set_height(&mut self, state: Self::Handle, z: f32);

    /// The state block's height getter (its own virtual method).
    fn height(&mut self, state: Self::Handle) -> f32;
}

/// The task's pose: the part of the task the mover family reads and writes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoverPose {
    /// Position: x, y and z.
    pub pos: [f32; 3],
    /// Heading, in radians.
    pub heading: f32,
    /// Packed mode: three two-bit codes (low bits first) and a top field in
    /// the two high bits.
    pub mode: u8,
    /// Flag byte; only bit 0 belongs to the family.
    pub flag: u8,
    /// Level, a byte scaled to the unit interval by the drive step.
    pub level: u8,
}

/// The blend state block: the blended pose and the values the drive step
/// derives from the packed mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlendState {
    /// Blended x position.
    pub x: f32,
    /// Blended y position.
    pub y: f32,
    /// Blended heading.
    pub heading: f32,
    /// Mirror of the blended heading, written with it.
    pub heading_mirror: f32,
    /// The three factor floats encoded from the packed mode.
    pub factors: [f32; 3],
    /// The top two bits of the packed mode.
    pub top: u8,
    /// Bit 0 of the task's flag byte.
    pub flag: u8,
    /// Level as a unit-interval float.
    pub level: f32,
    /// Flags word; the drive step snaps when [`SNAP_FLAG`] is set.
    pub flags: u32,
}

// The float operations are pinned in the original's operand order: each
// operand passes through `black_box`, so the optimiser cannot commute the
// operations, and a NaN's payload comes out as the original's does.
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Truncates toward zero the way the original's float-to-integer convert
/// does: NaN and out-of-range values give the indefinite value `i32::MIN`.
fn truncate_convert(x: f32) -> i32 {
    if x.is_nan() || x >= 2_147_483_648.0 || x < -2_147_483_648.0 {
        i32::MIN
    } else {
        // In range, so the truncating cast is the convert's answer.
        x as i32
    }
}

impl MoverPose {
    /// Blends this pose toward `src` by `t` into `state`, sending the
    /// blended height to the state block's height setter.
    ///
    /// Position components are `(src - this) * t + this`. The heading
    /// takes the short way round: when the absolute difference exceeds
    /// pi, the target is first shifted down by two pi, and a result below
    /// minus two pi is shifted back up. A NaN difference takes the short
    /// path.
    pub fn lerp_into<C: MoverCallees>(
        &self,
        cb: &mut C,
        state_handle: C::Handle,
        state: &mut BlendState,
        src: &MoverPose,
        t: f32,
    ) {
        let blend = |from: f32, to: f32| add(mul(sub(to, from), t), from);
        state.x = blend(self.pos[0], src.pos[0]);
        state.y = blend(self.pos[1], src.pos[1]);
        let z = blend(self.pos[2], src.pos[2]);
        cb.set_height(state_handle, z);

        let current = self.heading;
        let wanted = src.heading;
        // A NaN difference fails the comparison and takes the short path.
        let blended = if !(sub(current, wanted).abs() > HALF_TURN) {
            add(mul(sub(wanted, current), t), current)
        } else {
            let mut b = add(mul(sub(sub(wanted, FULL_TURN), current), t), current);
            if NEG_FULL_TURN > b {
                b = add(b, FULL_TURN);
            }
            b
        };
        state.heading = blended;
        state.heading_mirror = blended;
    }

    /// One drive step: a snap or a blend toward `target` (when present),
    /// then the packed mode refresh of the state block.
    ///
    /// The snap runs when the state block has [`SNAP_FLAG`] set, when `t`
    /// is zero, or when the readiness probe answers [`NOT_READY`]; the
    /// probe is asked only while the earlier tests fail. Otherwise the
    /// probe runs again, and a nonzero answer with a target blends. A NaN
    /// `t` is not zero, so it blends.
    pub fn drive_blend<C: MoverCallees>(
        &self,
        cb: &mut C,
        task: C::Handle,
        state_handle: C::Handle,
        state: &mut BlendState,
        target: Option<C::Handle>,
        t: f32,
    ) {
        cb.setup(task, state_handle, target, t);
        let snap = state.flags & SNAP_FLAG != 0 || t == 0.0 || cb.ready(task) == NOT_READY;
        if snap {
            state.x = self.pos[0];
            state.y = self.pos[1];
            cb.set_height(state_handle, self.pos[2]);
            state.heading = self.heading;
            state.heading_mirror = self.heading;
        } else if cb.ready(task) != 0 {
            if let Some(goal) = target {
                cb.blend_into(task, state_handle, goal, t);
            }
        }

        state.top = self.mode >> 6;
        state.flag = self.flag & 1;
        state.factors[0] = cb.encode(task, u32::from(self.mode & 3));
        state.factors[1] = cb.encode(task, u32::from((self.mode >> 2) & 3));
        state.factors[2] = cb.encode(task, u32::from((self.mode >> 4) & 3));
        state.level = mul(f32::from(self.level), INV_255);
    }

    /// Commits a blended state block into this pose.
    ///
    /// Notifies the flag and then the state, copies the position and
    /// heading from the block (the height comes from the block's height
    /// getter), refreshes bit 0 of the flag byte, rebuilds the packed mode
    /// from the three factors and the top field, and stores the level as a
    /// byte: the block's level clamped to the unit interval (NaN passes
    /// through), scaled by 255 and truncated.
    pub fn commit_blend<C: MoverCallees>(
        &mut self,
        cb: &mut C,
        task: C::Handle,
        state_handle: C::Handle,
        state: &BlendState,
    ) {
        cb.notify_flag(task, COMMIT_FLAG_ARG);
        cb.notify_state(task, state_handle);
        self.pos[0] = state.x;
        self.pos[1] = state.y;
        self.pos[2] = cb.height(state_handle);
        self.heading = state.heading;

        self.flag = (self.flag & 0xfe) | (state.flag & 1);
        let c0 = cb.decode(task, state.factors[0].to_bits());
        let c1 = cb.decode(task, state.factors[1].to_bits());
        let c2 = cb.decode(task, state.factors[2].to_bits());
        let mode = (c0 & 3) | ((c1 & 3) << 2) | ((c2 & 3) << 4) | ((u32::from(state.top) & 3) << 6);
        self.mode = mode as u8;

        let level = state.level;
        let clamped = if 0.0 > level {
            0.0
        } else if level > 1.0 {
            1.0
        } else {
            level
        };
        self.level = truncate_convert(mul(clamped, LEVEL_SCALE)) as u8;
    }
}
