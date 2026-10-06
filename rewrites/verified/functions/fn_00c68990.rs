// original: 0x00C68990 sample_tick_and_decay_counters (proposed)

/// Reset the controller's slot arrays, sample a 64-bit tick, and decay two
/// global counters when the sample is below a threshold.
///
/// `this` holds seven 64-slot id arrays (at `+0x0`/`+0x100`/`+0x200`/
/// `+0x300`/`+0x404`/`+0x504`/`+0x604`), a flag byte at `+0x400` and state
/// dwords at `+0x704`/`+0x70c`/`+0x710`. The function resets all seven
/// arrays, clears the flag and `+0x704`, runs a pool callback, then queries
/// the slot helper with an index built from a global (`table[g*100] + base`)
/// and three stack out-pointers plus the index and a zero. It adds the two
/// dwords the helper wrote through the first two out-pointers into
/// `+0x70c` (wrapping). It then copies four globals to four shadow slots
/// (two destinations share one source), sets `+0x710` to 1, and samples a
/// 64-bit tick: the value is converted exactly to float (the original does
/// it with two x87 integer loads around the sign bit, which is one correct
/// rounding of the unsigned value) and compared against a global float
/// threshold. If the sample is strictly below, two global counters are
/// divided by 3 in place (SIGNED, truncating; the original uses the
/// 0x55555556 magic multiply) and the second quotient is returned;
/// otherwise the tick's sign bit (0 or 0x80000000) is returned.
///
/// Original: 0x00C68990 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00C68990(this: u32) -> u32 {
    unsafe {
        const IDX_PTR: u32 = 0x0103_2F58;
        const WORD_TABLE: u32 = 0x0130_53A8;
        const WORD_BASE: u32 = 0x0103_DBBC;
        const THRESH: u32 = 0x00EC_BE8C;
        const DIV_A: u32 = 0x0104_96DC;
        const DIV_B: u32 = 0x0104_96D8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn w32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut u32).write(v) }
        }

        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(1, u32, this + 0x100);
        lf_checker_rt::callee_thiscall!(1, u32, this + 0x200);
        lf_checker_rt::callee_thiscall!(1, u32, this + 0x300);
        (this.wrapping_add(0x400) as *mut u8).write(0);
        lf_checker_rt::callee_thiscall!(1, u32, this + 0x404);
        lf_checker_rt::callee_thiscall!(1, u32, this + 0x504);
        lf_checker_rt::callee_thiscall!(1, u32, this + 0x604);
        (this.wrapping_add(0x704) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_cdecl!(2, u32,);
        let g = g32(IDX_PTR);
        let a1 = rd32(lf_checker_rt::relocated(WORD_TABLE).wrapping_add(g.wrapping_mul(100)))
            .wrapping_add(g32(WORD_BASE));
        let mut o3 = 0u32;
        let mut o4 = 0u32;
        let mut o5 = 0u32;
        lf_checker_rt::callee_cdecl!(3, u32, a1, g, &mut o3 as *mut u32 as u32, &mut o4 as *mut u32 as u32, &mut o5 as *mut u32 as u32, 0);
        let out_sum = o3.wrapping_add(o4);
        (this.wrapping_add(0x70c) as *mut u32).write_unaligned(out_sum);
        w32(0x012F_A3F8, g32(0x012F_9FB4));
        w32(0x012F_9D44, g32(0x012F_9ED0));
        w32(0x012F_A050, g32(0x012F_A0D4));
        w32(0x012F_9F48, g32(0x012F_A0D4));
        (this.wrapping_add(0x710) as *mut u32).write_unaligned(1);
        let tick = lf_checker_rt::callee_cdecl!(4, u64,);
        let f = tick as f32;
        let k = (lf_checker_rt::global::<f32>(THRESH) as *const f32).read();
        if f < k {
            let qa = (g32(DIV_A) as i32) / 3;
            let qb = (g32(DIV_B) as i32) / 3;
            w32(DIV_A, qa as u32);
            w32(DIV_B, qb as u32);
            qb as u32
        } else {
            ((tick >> 32) as u32) & 0x8000_0000
        }
    }
});
