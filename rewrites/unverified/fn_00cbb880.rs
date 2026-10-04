// original: 0x00cbb880 move_flag_clear
/// Clear move-task flag bits through paired table calls (2 calls).
///
/// Reads the flag word at `[a0 + 0x26C]` into eax (stdcall, one stack
/// argument). When bit `0x80000` is set, runs slot 8 of the table of the
/// object at `[a0 + 0x224]` with (`a0`, 0), runs the direct callee with
/// its answer (which replaces eax), and clears `0x80000` in the flag word
/// in place. Then tests bit `0x100000` of the current eax (the flag word
/// when no call ran, else the direct callee's answer): when set, runs
/// the pair again with (`a0`, 1) and clears `0x100000` in place. Returns
/// the final eax. Both callees are intercepted (the indirect one through
/// a planted table) and answered by the checker.
lf_checker_rt::export!(stdcall, rw_00cbb880(a0: u32) -> u32 {
    unsafe {
        /// Object and flag offsets in the argument.
        const OBJ_OFF: u32 = 0x224;
        const FLAG_OFF: u32 = 0x26C;
        /// Cleared bits.
        const BIT_A: u32 = 0x80000;
        const BIT_B: u32 = 0x100000;
        /// Table slot of the indirect callee.
        const SLOT: u32 = 8;
        /// Direct callee id.
        const FOLLOW: u32 = 2;
        type Slot2 = extern "thiscall" fn(u32, u32, u32) -> u32;
        let v = ((a0 + FLAG_OFF) as *const u32).read_unaligned();
        let mut e = v;
        if v & BIT_A != 0 {
            let o = ((a0 + OBJ_OFF) as *const u32).read_unaligned();
            let va = (o as *const u32).read_unaligned();
            let sa = ((va + SLOT) as *const u32).read_unaligned();
            let f: Slot2 = core::mem::transmute(sa as usize);
            let r = f(o, a0, 0);
            e = lf_checker_rt::callee_thiscall!(FOLLOW, u32, r);
            let m = ((a0 + FLAG_OFF) as *const u32).read_unaligned();
            ((a0 + FLAG_OFF) as *mut u32).write_unaligned(m & !BIT_A);
        }
        if e & BIT_B != 0 {
            let o = ((a0 + OBJ_OFF) as *const u32).read_unaligned();
            let va = (o as *const u32).read_unaligned();
            let sa = ((va + SLOT) as *const u32).read_unaligned();
            let f: Slot2 = core::mem::transmute(sa as usize);
            let r = f(o, a0, 1);
            e = lf_checker_rt::callee_thiscall!(FOLLOW, u32, r);
            let m = ((a0 + FLAG_OFF) as *const u32).read_unaligned();
            ((a0 + FLAG_OFF) as *mut u32).write_unaligned(m & !BIT_B);
        }
        e
    }
});
