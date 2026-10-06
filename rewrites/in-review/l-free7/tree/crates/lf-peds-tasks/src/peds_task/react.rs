//! The facing reaction code: one of six answers for a ped facing a task object.
//!
//! The 32-bit routine reads a task object (an inner object holding either
//! a matrix pointer or an inline position) and a ped (a matrix with a
//! direction row and a position, a mode word, a flag byte, a counter
//! reached through two pointers, and a state prober). The lift owns the
//! parsed query as [`FacingQuery`]: the two positions, the direction row,
//! the mode bit, the flag and the counter. Image constants (the counter
//! threshold and four classifier bounds) travel as [`ReactTuning`], and
//! the state probe as the [`PedState`] trait.
//!
//! Behaviour (one method, [`FacingQuery::code`]): dot the ped's direction
//! row with (ped position minus object position) in the original's operand
//! order, then: a non-zero low byte from the state probe answers the near
//! pair; else a set flag with the counter above the threshold (unsigned)
//! answers the far pair; else the dot inside (-1.2, -0.2) answers near,
//! inside (0.2, 1.2) (lower bound compared in double precision) answers
//! mid, anything else (including NaN) answers far. The high member of the
//! pair is used when the mode word is 3 or 4.

/// Near pair, low and high members.
pub const CODE_NEAR_LO: u8 = 0x91;
/// Near pair, high member.
pub const CODE_NEAR_HI: u8 = 0x94;
/// Mid pair, low member.
pub const CODE_MID_LO: u8 = 0x92;
/// Mid pair, high member.
pub const CODE_MID_HI: u8 = 0x95;
/// Far pair, low member.
pub const CODE_FAR_LO: u8 = 0x93;
/// Far pair, high member.
pub const CODE_FAR_HI: u8 = 0x96;

/// Image constants the classification reads: the counter threshold and the
/// four classifier bounds (mid lower bound in double precision).
#[derive(Clone, Copy, Debug)]
pub struct ReactTuning {
    /// Counter threshold: the far pair needs the counter above this.
    pub threshold: u32,
    /// Near window upper bound (-0.2 in the image).
    pub lower: f32,
    /// Near window lower bound (-1.2 in the image).
    pub far: f32,
    /// Mid window lower bound (0.2 in the image, compared as `f64`).
    pub mid: f64,
    /// Mid window upper bound (+1.2 in the image).
    pub near: f32,
}

/// The ped-state prober (one numbered callee); only its low byte is observed.
pub trait PedState {
    /// Probes the ped state; answers a word whose low byte steers.
    fn probe(&mut self) -> u32;
}

impl<F: FnMut() -> u32> PedState for F {
    fn probe(&mut self) -> u32 {
        self()
    }
}

/// The parsed facing query: positions, direction row, mode bit, flag, counter.
#[derive(Clone, Copy, Debug)]
pub struct FacingQuery {
    /// Object position: through the matrix, or the inline fallback.
    pub obj: [f32; 3],
    /// The ped matrix's direction row.
    pub dir: [f32; 3],
    /// The ped matrix's position.
    pub ped: [f32; 3],
    /// True when the mode word is 3 or 4 (the high member of each pair).
    pub mode_hi: bool,
    /// The ped flag byte: nonzero arms the counter gate.
    pub flag: bool,
    /// The counter value behind the ped's two pointers.
    pub counter: u32,
}

#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

impl FacingQuery {
    /// Picks the reaction code, probing the ped state exactly once.
    pub fn code<S: PedState>(&self, state: &mut S, tuning: &ReactTuning) -> u8 {
        // Separation in the original's component order, dot in its sum order.
        let dy = sub(self.ped[1], self.obj[1]);
        let dx = sub(self.ped[0], self.obj[0]);
        let dz = sub(self.ped[2], self.obj[2]);
        let dist = add(
            add(mul(self.dir[1], dy), mul(self.dir[0], dx)),
            mul(self.dir[2], dz),
        );
        let pair = |lo: u8, hi: u8| if self.mode_hi { hi } else { lo };
        if state.probe() & 0xff != 0 {
            return pair(CODE_NEAR_LO, CODE_NEAR_HI);
        }
        if self.flag && self.counter > tuning.threshold {
            return pair(CODE_FAR_LO, CODE_FAR_HI);
        }
        if tuning.lower > dist && dist > tuning.far {
            return pair(CODE_NEAR_LO, CODE_NEAR_HI);
        }
        if f64::from(dist) > tuning.mid && tuning.near > dist {
            return pair(CODE_MID_LO, CODE_MID_HI);
        }
        pair(CODE_FAR_LO, CODE_FAR_HI)
    }
}
