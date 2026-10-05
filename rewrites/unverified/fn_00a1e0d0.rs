// original: 0x00A1E0D0 task_sync_anchor_and_solve (proposed)

/// Copy the task's anchor record to its three working slots, derive the
/// offset vector from a target point, and hand both to a solver callee.
///
/// `this` is the task object, `out` is a caller-provided 16-byte vector slot,
/// `target` points at three floats (x, y, z). Nothing happens unless the
/// ready flag (bit 0x10 of the flag byte at `+0x38d`) is set, in which case:
///
/// 1. The 16-byte anchor record at `+0x270` is copied verbatim to `+0x40`,
///    `+0x230` and `+0x240` (each copy is one dword, two float moves and one
///    dword, so the two middle words keep their exact bits).
/// 2. The busy flag (bit 0x08) is set.
/// 3. `out` receives the difference `target - slot230`: `out[0] =
///    target.x - this[0x230]`, `out[4] = target.y - this[0x234]`,
///    `out[8] = target.z - this[0x238]`. The fourth word the original writes
///    is a copy of its own uninitialised stack scratch (the compiler reloads
///    a dead slot above the pushed arguments); under the checker's defined
///    stack fill that word is 0, which is what this rewrite stores.
/// 4. The solver callee runs as `solver(out, this + 0x324, this + 0x328)`
///    (cdecl, three arguments).
/// 5. The ready flag is cleared (busy stays set) and the dwords at `+0x30c`
///    and `+0x310` are zeroed.
///
/// Original: 0x00A1E0D0 (thiscall, two stack words). No return value is
/// defined: on the early-exit path `eax` still holds its entry value, and on
/// the late path it holds the solver's answer, so the checker compares no
/// return channel.
lf_checker_rt::export!(thiscall, rw_00A1E0D0(this: u32, out: u32, target: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x38d;
        const READY: u8 = 0x10;
        const BUSY: u8 = 0x08;
        const ANCHOR: u32 = 0x270;
        const SLOT_A: u32 = 0x40;
        const SLOT_B: u32 = 0x230;
        const SLOT_C: u32 = 0x240;
        const SOLVER_IN0: u32 = 0x324;
        const SOLVER_IN1: u32 = 0x328;
        const CLEAR_LO: u32 = 0x30c;
        const CLEAR_HI: u32 = 0x310;
        const SOLVER: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        if (this as *const u8).add(FLAG as usize).read() & READY == 0 {
            return 0;
        }
        let w0 = rd32(this + ANCHOR);
        let w1 = rd32(this + ANCHOR + 4);
        let w2 = rd32(this + ANCHOR + 8);
        let w3 = rd32(this + ANCHOR + 12);
        for slot in [SLOT_A, SLOT_B, SLOT_C] {
            wr32(this + slot, w0);
            wr32(this + slot + 4, w1);
            wr32(this + slot + 8, w2);
            wr32(this + slot + 12, w3);
        }
        let flag = (this as *mut u8).add(FLAG as usize);
        flag.write(flag.read() | BUSY);
        wrf(out, sub(rdf(target), rdf(this + SLOT_B)));
        wrf(out + 4, sub(rdf(target + 4), rdf(this + SLOT_B + 4)));
        wrf(out + 8, sub(rdf(target + 8), rdf(this + SLOT_B + 8)));
        // The original's fourth word is its own uninitialised scratch (see
        // the doc comment); the proof defines that fill as 0.
        wr32(out + 12, 0);
        lf_checker_rt::callee_cdecl!(SOLVER, u32, out, this + SOLVER_IN0, this + SOLVER_IN1);
        flag.write(flag.read() & !READY);
        wr32(this + CLEAR_LO, 0);
        wr32(this + CLEAR_HI, 0);
        0
    }
});
