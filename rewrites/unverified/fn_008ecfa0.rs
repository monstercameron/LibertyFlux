// original: 0x008ECFA0 init_tables_and_globals (proposed)

/// Initialise the object tables and the shared global grid.
///
/// Notifies the initializer (callee 1, thiscall on `obj`) for slots 0, 1
/// and 2, zeroes `FLAG`, fills 32 words at `T_ZERO` with 0 and the 32 words
/// `T_NEG_OFF` bytes below them with -1, zeroes the global grid words from
/// `G_START` below `G_END` in steps of `G_STEP` (four words per step: the
/// step word, its neighbours and the word 12 past), zeroes the global
/// `G_TAIL` word, and clears the `DONE` byte. No meaningful return value.
///
/// Original: 0x008ECFA0 (thiscall, no stack arguments; true size 136).
lf_checker_rt::export!(thiscall, rw_008ECFA0(obj: u32) -> u32 {
    unsafe {
        /// Object flag zeroed after the callbacks.
        const FLAG: u32 = 0xE08;
        /// Start of the 32-word zero run.
        const T_ZERO: u32 = 0x1B44;
        /// Words in each run.
        const T_N: u32 = 32;
        /// The -1 run sits this far below the zero run.
        const T_NEG_OFF: u32 = 0x80;
        /// Global grid range (file VAs), stepped by G_STEP.
        const G_START: u32 = 0x1177684;
        const G_END: u32 = 0x1177A84;
        const G_STEP: u32 = 0x20;
        /// Trailing global word.
        const G_TAIL: u32 = 0x1176E48;
        /// Object done byte.
        const DONE: u32 = 0x1C04;
        /// Initializer callee id.
        const INIT: u32 = 1;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let mut i: u32 = 0;
        while i < 3 {
            let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, obj, i);
            i = i.wrapping_add(1);
        }
        wr32(obj.wrapping_add(FLAG), 0);
        let mut k: u32 = 0;
        while k < T_N {
            let w = obj.wrapping_add(T_ZERO).wrapping_add(k.wrapping_mul(4));
            wr32(w.wrapping_sub(T_NEG_OFF), 0xFFFF_FFFF);
            wr32(w, 0);
            k = k.wrapping_add(1);
        }
        let gend = lf_checker_rt::relocated(G_END);
        let mut g = lf_checker_rt::relocated(G_START);
        while (g as i32) < (gend as i32) {
            wr32(g.wrapping_add(4), 0);
            wr32(g, 0);
            wr32(g.wrapping_sub(4), 0);
            wr32(g.wrapping_add(0x0C), 0);
            g = g.wrapping_add(G_STEP);
        }
        wr32(lf_checker_rt::relocated(G_TAIL), 0);
        wr8(obj.wrapping_add(DONE), 0);
        0
    }
});
