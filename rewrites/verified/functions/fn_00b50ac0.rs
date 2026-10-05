// original: 0x00b50ac0 select_best_task (proposed)

/// Pick the best-scoring task candidate for a context.
///
/// `this + 8` holds a signed candidate count (zero or negative returns
/// null); the candidates start at `this + 0x0C`. Each candidate is scored
/// (virtual slot `+0x08`, thiscall on it) and classified (slot `+0x04`).
/// A `SPECIAL`-kind candidate replaces the leader only when its score is
/// strictly greater and it passes three gates in order: slot `+0x14` and
/// slot `+0x44` (each thiscall on the candidate with the context word at
/// `this + 4`), then the direct check (callee 5, same shape); each gate
/// passes on a non-zero low byte. Any other kind with a score below the
/// leader merely re-runs the first gate for effect and keeps the leader,
/// otherwise it must pass all three gates to take over (ties keep the
/// earlier candidate for the special kind but yield to the later one for
/// the rest). Scores compare signed from an initial best of -1; the
/// winning candidate pointer, or null, is returned.
///
/// Original: 0x00b50ac0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b50ac0(this: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x4;
        const COUNT: u32 = 0x8;
        const ITEMS: u32 = 0xc;
        const SCORE_SLOT: u32 = 0x08;
        const KIND_SLOT: u32 = 0x04;
        const GATE_SLOT: u32 = 0x14;
        const VETO_SLOT: u32 = 0x44;
        const SPECIAL: u32 = 0x20;
        const DIRECT: u32 = 5;
        let n = ((this + COUNT) as *const i32).read_unaligned();
        if n <= 0 {
            return 0;
        }
        let ctx = ((this + CTX) as *const u32).read_unaligned();
        let mut best: u32 = 0;
        let mut best_score: i32 = -1;
        let mut i: i32 = 0;
        while i < n {
            let elem = ((this + ITEMS + (i as u32) * 4) as *const u32).read_unaligned();
            let vt = (elem as *const u32).read_unaligned();
            let score_fn: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + SCORE_SLOT) as *const u32).read_unaligned() as usize,
            );
            let score = score_fn(elem) as i32;
            let kind_fn: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + KIND_SLOT) as *const u32).read_unaligned() as usize,
            );
            let kind = kind_fn(elem);
            let gate_fn: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((vt + GATE_SLOT) as *const u32).read_unaligned() as usize,
            );
            let veto_fn: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                ((vt + VETO_SLOT) as *const u32).read_unaligned() as usize,
            );
            if kind == SPECIAL {
                if score > best_score
                    && gate_fn(elem, ctx) & 0xff != 0
                    && veto_fn(elem, ctx) & 0xff != 0
                    && lf_checker_rt::callee_thiscall!(DIRECT, u32, elem, ctx) & 0xff != 0
                {
                    best = elem;
                    best_score = score;
                }
            } else if score < best_score {
                gate_fn(elem, ctx);
            } else if gate_fn(elem, ctx) & 0xff != 0
                && veto_fn(elem, ctx) & 0xff != 0
                && lf_checker_rt::callee_thiscall!(DIRECT, u32, elem, ctx) & 0xff != 0
            {
                best = elem;
                best_score = score;
            }
            i += 1;
        }
        best
    }
});
