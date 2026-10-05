// original: 0x00b513b0 update_ped_blends (proposed)

/// Refresh a ped's movement blends and prune settled ones.
///
/// When the helper at `this + 0x434` is present with a holder that is
/// either missing or idle (zero flag byte at holder `+0x0E`), the blend
/// solver runs (callee 1, stdcall with the two scratch areas at
/// `this + 0x440` and `this + 0x450` and a zero kind). Then five event slots
/// from `this + 0x460` are visited: a slot whose event is present, live
/// (non-null word at `+0x38`) and settled (matching word at `+0x7B4`) has
/// the solver re-run on its row and, when the event's virtual slot at
/// `+0x128` reports ready (non-zero low byte), the closer runs (callee 3,
/// thiscall on the event). No meaningful return value.
///
/// Original: 0x00b513b0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b513b0(this: u32) -> u32 {
    unsafe {
        const SOLVE: u32 = 1;
        const READY_SLOT: u32 = 0x128;
        const CLOSE: u32 = 3;
        const HELPER: u32 = 0x434;
        const HOLDER: u32 = 0x6c;
        const IDLE_FLAG: u32 = 0x0e;
        const SCRATCH_A: u32 = 0x440;
        const SCRATCH_B: u32 = 0x450;
        const SLOTS: u32 = 0x460;
        const ROWS: u32 = 0x480;
        const ROW_STRIDE: u32 = 0x10;
        const NSLOTS: u32 = 5;
        const LIVE: u32 = 0x38;
        const SETTLED: u32 = 0x7b4;
        let helper = ((this + HELPER) as *const u32).read_unaligned();
        if helper != 0 {
            let holder = ((helper + HOLDER) as *const u32).read_unaligned();
            if holder == 0 || ((holder + IDLE_FLAG) as *const u8).read() == 0 {
                lf_checker_rt::callee_stdcall!(SOLVE, u32, this + SCRATCH_A, this + SCRATCH_B, 0);
            }
        }
        let mut i: u32 = 0;
        while i < NSLOTS {
            let slot = this + SLOTS + i.wrapping_mul(4);
            let row = this + ROWS + i.wrapping_mul(ROW_STRIDE);
            let ev = (slot as *const u32).read_unaligned();
            if ev != 0 {
                let live = ((ev + LIVE) as *const u32).read_unaligned();
                if live != 0 && live == ((ev + SETTLED) as *const u32).read_unaligned() {
                    lf_checker_rt::callee_stdcall!(SOLVE, u32, row, row + 0x50, 0);
                    let vt = (ev as *const u32).read_unaligned();
                    let ready: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                        ((vt + READY_SLOT) as *const u32).read_unaligned() as usize,
                    );
                    if ready(ev) & 0xff != 0 {
                        lf_checker_rt::callee_thiscall!(CLOSE, u32, ev);
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        0
    }
});
