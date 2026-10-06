//! The input-ui distance gates: a float predicate and an entry scanner.
//!
//! [`float_gate`] restates `input_ui_float_gate`: it measures the squared
//! xy distance between a point and an anchor, asks a membership query,
//! then applies one of two threshold ladders. [`ScanRegistry::scan`]
//! restates `input_ui_entry_scanner`: it walks a registry from the top
//! entry down, tests each live entry against its own distance gate, and
//! notifies through the entry's slots. Both share the ladder style (a
//! near step and a far step selected by scripted answers); each owns its
//! own data.
//!
//! Evidence: `fn_00afe3b0.rs` + `r-b172/contracts/fn_00afe3b0.json` and
//! `fn_00afe030.rs` + `r-b172/contracts/fn_00afe030.json` (contract file
//! names per the lane's folder layout).

/// Threshold constants of the float predicate (read-only image words in
/// the 32-bit form, parameters here, so no game data is copied into
/// source; the differential test plants arbitrary floats).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GateConsts {
    /// Cap for the `f3` scale on both paths.
    pub cap: f32,
    /// Near-path window scale.
    pub window: f32,
    /// Near-path floor: the point must lie inside its square.
    pub floor: f32,
    /// Far-path base span.
    pub base: f32,
    /// Far-path inset subtracted from the base.
    pub inset: f32,
    /// Far-path fraction multiplied with the base.
    pub frac: f32,
}

/// Asks whether a point is a member (the float predicate's callee 1).
///
/// The 32-bit form also forwards a table word and two constant words;
/// those are pinned on the 32-bit side of the differential test.
pub trait Membership {
    /// One membership query about `point`, in call order.
    fn query(&mut self, point: [f32; 3]) -> bool;
}

/// Selects the near or far ladder (the float predicate's callee 2).
pub trait GateProbe {
    /// One probe; only the low byte of the answer decides.
    fn probe(&mut self) -> u32;
}

/// Decides whether `point` passes the two-stage float gate.
///
/// `point` holds three floats (x, y, z); `anchor` two (x, y). `under`
/// is the under-threshold flag the 32-bit form reads from two globals.
/// Returns true when the point passes (the 32-bit form answers the
/// byte 1; 0 otherwise).
///
/// Float expressions mirror the 32-bit form exactly, including the
/// negated comparisons (`!(a > b)` is true for NaN where `a <= b` is
/// false), so unordered inputs take the same path on both sides.
pub fn float_gate<M: Membership + ?Sized, P: GateProbe + ?Sized>(
    point: [f32; 3],
    anchor: [f32; 2],
    f2: f32,
    f3: f32,
    under: bool,
    consts: &GateConsts,
    member: &mut M,
    probe: &mut P,
) -> bool {
    let dx = anchor[0] - point[0];
    let dy = anchor[1] - point[1];
    let dist2 = dx * dx + dy * dy;

    let present = member.query(point);
    let under = u8::from(under);

    if present {
        if under == 0 {
            return false;
        }
    } else {
        let r2 = probe.probe();
        if (r2 & 0xFF) == 0 {
            // Near path.
            let k = consts.cap;
            let mut t = f3;
            if !(t > k) {
                t = k;
            }
            t = t / k;
            t = t * consts.window;
            t = t * f2;
            t = t * t;
            if dist2 > t {
                return false;
            }
            let s = consts.floor;
            let s2 = s * s;
            if !(s2 > dist2) {
                return true;
            } else {
                return false;
            }
        }
        if under == 0 {
            return false;
        }
    }

    // Far path.
    let k = consts.cap;
    let m = if k > f3 { k } else { f3 };
    let base = consts.base;
    let c3 = m * f2;
    let c3sq = c3 * c3;
    let c2 = (base - consts.inset) * f2;
    let c2sq = c2 * c2;
    let c1 = (base * consts.frac) * f2;
    let c1sq = c1 * c1;
    if dist2 > c3sq {
        return false;
    }
    if c2sq > dist2 {
        return false;
    }
    if c1sq > dist2 {
        return false;
    }
    true
}

/// Threshold constants of the entry scanner (read-only image words in
/// the 32-bit form, parameters here).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScanConsts {
    /// Ladder scale when the kind answer selects the detailed branch.
    pub detailed: f32,
    /// Ladder scale otherwise.
    pub plain: f32,
    /// Floor for the combined scale.
    pub floor: f32,
    /// Near-step scale applied to the argument.
    pub near: f32,
    /// Far-step scale applied to the argument.
    pub far: f32,
}

/// Fills one scan triple (the scanner's callee 1).
pub trait ScanQuery {
    /// One query, in visit order.
    fn query(&mut self) -> [f32; 3];
}

/// Selects the refined distance (the scanner's callee 2).
pub trait ScanGate {
    /// One gate answer; only the low byte decides.
    fn gate(&mut self) -> u32;
}

/// Selects the ladder scale (the scanner's callee 7).
pub trait EntryKind {
    /// One kind answer for `entry`; only the low byte decides.
    fn kind(&mut self, entry: &ScanEntry) -> u32;
}

/// Refines an entry's distance through its object (the scanner's slot
/// at `+0x104`, reached only when the entry carries an object).
pub trait EntryRefine {
    /// One refinement of `entry`, replacing the measured distance.
    fn refine(&mut self, entry: &ScanEntry) -> f32;
}

/// Notifies an entry through its slots (the scanner's slots at
/// `+0x144`, `+0x148` and `+0x14C`).
pub trait EntryNotify {
    /// Slot `+0x144`; only the low byte of the answer decides.
    fn notify_main(&mut self, entry: &ScanEntry) -> u32;
    /// Slot `+0x148`, reached when the main answer's low byte is clear.
    fn notify_alt(&mut self, entry: &ScanEntry) -> u32;
    /// Slot `+0x14C`, reached when the main answer's low byte is set.
    fn notify_far(&mut self, entry: &ScanEntry) -> u32;
}

/// One scanned entry: flag, anchor, mode, and whether it carries the
/// refinement object the `+0x104` slot runs on.
#[derive(Clone, PartialEq, Debug)]
pub struct ScanEntry {
    /// Flag byte: the top bit skips the entry.
    pub flag: u8,
    /// Anchor point the measured triple is compared against.
    pub anchor: [f32; 3],
    /// Mode word: value 2 selects the plain ladder scale.
    pub mode: u32,
    /// Whether the refinement slot runs for this entry.
    pub has_object: bool,
}

/// A registry of scanned entries, visited from the top down.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ScanRegistry {
    /// Entries in slot order; the scan visits the last slot first.
    pub entries: Vec<ScanEntry>,
}

impl ScanRegistry {
    /// Scans every live entry against the distance gate.
    ///
    /// An entry is live when its flag byte has the top bit clear. The
    /// scan visits slots from the last to the first; per entry it
    /// measures, optionally refines, selects a ladder scale, then takes
    /// the near step (main notify, else the alternate) or the far step
    /// (main notify, then the far notify when set).
    #[allow(clippy::too_many_arguments)]
    pub fn scan<
        Q: ScanQuery + ?Sized,
        G: ScanGate + ?Sized,
        K: EntryKind + ?Sized,
        R: EntryRefine + ?Sized,
        N: EntryNotify + ?Sized,
    >(
        &self,
        arg: f32,
        consts: &ScanConsts,
        query: &mut Q,
        gate: &mut G,
        kind: &mut K,
        refine: &mut R,
        notify: &mut N,
    ) {
        if self.entries.is_empty() {
            return;
        }
        let mut count = self.entries.len();
        loop {
            count = count.wrapping_sub(1);
            let entry = &self.entries[count];
            if (entry.flag & 0x80) == 0 {
                Self::test_entry(arg, consts, entry, query, gate, kind, refine, notify);
            }
            if count == 0 {
                return;
            }
        }
    }

    /// Tests one entry against the distance gate.
    fn test_entry<
        Q: ScanQuery + ?Sized,
        G: ScanGate + ?Sized,
        K: EntryKind + ?Sized,
        R: EntryRefine + ?Sized,
        N: EntryNotify + ?Sized,
    >(
        arg: f32,
        consts: &ScanConsts,
        entry: &ScanEntry,
        query: &mut Q,
        gate: &mut G,
        kind: &mut K,
        refine: &mut R,
        notify: &mut N,
    ) {
        let measured = query.query();
        let dx = measured[0] - entry.anchor[0];
        let dy = measured[1] - entry.anchor[1];
        let dz = measured[2] - entry.anchor[2];
        let mut slot = dx * dx + dy * dy + dz * dz;
        let r2 = gate.gate();
        if (r2 & 0xFF) != 0 && entry.has_object {
            slot = refine.refine(entry);
        }
        let r7 = kind.kind(entry);
        let x2 = if (r7 & 0xFF) != 0 && entry.mode != 2 {
            consts.detailed
        } else {
            consts.plain
        };
        let x3 = consts.detailed;
        let mut t0 = consts.floor;
        if !(t0 > x3) {
            t0 = x3;
        }
        let mut t1 = consts.near * arg;
        let x2v = x2 * t0;
        let mut s0 = consts.far * arg;
        t1 = t1 * x2v;
        s0 = s0 * x2v;
        s0 = s0 * s0;
        if slot > s0 {
            // Near step.
            if (notify.notify_main(entry) & 0xFF) == 0 {
                notify.notify_alt(entry);
            }
        } else {
            // Far step.
            t1 = t1 * t1;
            if t1 > slot {
                if (notify.notify_main(entry) & 0xFF) != 0 {
                    notify.notify_far(entry);
                }
            }
        }
    }
}
