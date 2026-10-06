// original: 0x008f02f0 n:/data/controls/controls (symbols)

/// Reset the controls manager for one input mode and lazily allocate its tables.
///
/// `this` is the manager object (about 0x3a80 bytes). `mode` selects the
/// control profile: it is stored into the mode slot (`+0x04`) of each of the
/// five binding sub-objects (`+0x00`, `+0x7b8`, `+0xf70`, `+0x1728`, `+0x1ee0`),
/// into the profile index at `+0x32a4` (which the resolver callee scales by
/// 0xbc) and its neighbour at `+0x32b0`. Four state words at `+0x3a70` are
/// cleared and the shared tick counter (global) is snapshotted at `+0x3a6c`.
///
/// The manager is then re-initialised through three helpers: an init call, a
/// resolver whose answer is forwarded as the object of a readiness check, and
/// -- only when the readiness check reports false in its LOW byte (a full-word
/// zero test would wrongly treat 0x100 as ready) -- a reload from the
/// `common:/data/controls/controls` descriptor. Finally each of the 27 table
/// slots that is still null is filled with a fresh 0x200-byte block that is
/// zeroed in 64 eight-byte chunks (a zero byte followed by a zero dword).
///
/// Returns the last callee answer: the final block pointer when any block was
/// allocated, else the reload answer when it ran, else the readiness answer.
/// Original: 0x008f02f0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_008f02f0(this: u32, mode: u32) -> u32 {
    unsafe {
        const TICK: u32 = 0x011735b4;
        const CONTROLS_DESC: u32 = 0x00e83708;
        const FIELDS_MODE: [u32; 7] = [0x04, 0x7bc, 0xf74, 0x172c, 0x1ee4, 0x32a4, 0x32b0];
        const STATE_ZERO: [u32; 4] = [0x3a70, 0x3a74, 0x3a78, 0x3a7c];
        const TICK_SNAP: u32 = 0x3a6c;
        const BLOCK: u32 = 0x200;
        const CHUNKS: u32 = 64;
        const TABLES: [u32; 27] = [
            0x2a54, 0x2a64, 0x2a74, 0x2bf4, 0x2a84, 0x2a94, 0x2704, 0x27a4, 0x27b4, 0x2764,
            0x2774, 0x2784, 0x2794, 0x2954, 0x29e4, 0x29f4, 0x2a04, 0x2804, 0x2944, 0x2c34,
            0x2c14, 0x2f04, 0x2f14, 0x2dc4, 0x3094, 0x30a4, 0x2fc4,
        ];
        const MALLOC_FIRST: u32 = 4;
        const MALLOC_REST: u32 = 5;
        const MALLOC_SPLIT: usize = 14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        for off in FIELDS_MODE {
            wr32(this.wrapping_add(off), mode);
        }
        for off in STATE_ZERO {
            wr32(this.wrapping_add(off), 0);
        }
        let tick: u32 = lf_checker_rt::global::<u32>(TICK).read_unaligned();
        wr32(this.wrapping_add(TICK_SNAP), tick);

        lf_checker_rt::callee_thiscall!(0, u32, this);
        let profile = lf_checker_rt::callee_thiscall!(1, u32, this);
        let ready = lf_checker_rt::callee_thiscall!(2, u32, profile);
        // The original tests only AL after this call, so only the low byte
        // decides; values such as 0x100 count as "not ready".
        let mut ans = ready;
        if (ready as u8) == 0 {
            ans = lf_checker_rt::callee_thiscall!(
                3,
                u32,
                this,
                lf_checker_rt::relocated(CONTROLS_DESC),
                2
            );
        }
        for (k, off) in TABLES.iter().enumerate() {
            let slot = this.wrapping_add(*off);
            if rd32(slot) == 0 {
                let id = if k < MALLOC_SPLIT { MALLOC_FIRST } else { MALLOC_REST };
                let p: u32 = lf_checker_rt::callee_cdecl!(id, u32, BLOCK);
                wr32(slot, p);
                let mut k2: u32 = 0;
                while k2 < CHUNKS {
                    let base = p.wrapping_add(k2.wrapping_mul(8));
                    (base as *mut u8).write(0);
                    (base.wrapping_add(4) as *mut u32).write_unaligned(0);
                    k2 = k2.wrapping_add(1);
                }
                ans = p;
            }
        }
        ans
    }
});
