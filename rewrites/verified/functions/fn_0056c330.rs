// original: 0x0056c330 rlConcreteLeaderboardInfo_Race111_ctor

/// Leaderboard-info object initializer: plant the vtable and clear fields.
///
/// Writes the board's vtable pointer at `this`, zeroes the words at `+4`,
/// `+8` and `+0x0c`, stores the board tag at `+0x10`, and returns `this`.
/// Sits between two leaderboard template instantiations and stores the
/// same tag the neighbouring board publishes, so it reads as that board's
/// constructor; named as a proposal since the inventory has no symbol here.
///
/// Original: 0x0056C330 (thiscall, no stack words; object pointer in ecx).
lf_checker_rt::export!(thiscall, rw_0056c330(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const VTABLE_PTR: u32 = 0x00fdecc4;
        const BOARD_TAG: u32 = 0x00fd6864;
        // Both immediates carry relocation entries, so they are written
        // relocated (in the game, loaded at its preferred base, these are
        // the file values themselves).
        wr32(this, lf_checker_rt::relocated(VTABLE_PTR));
        wr32(this.wrapping_add(4), 0);
        wr32(this.wrapping_add(8), 0);
        wr32(this.wrapping_add(0x0c), 0);
        wr32(this.wrapping_add(0x10), lf_checker_rt::relocated(BOARD_TAG));
        this
    }
});
