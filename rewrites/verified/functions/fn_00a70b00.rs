// original: 0x00A70B00 CTaskComplexPlayerGun::vf19

/// Build the gun-task replacement for a player: either delegate through a
/// tail call or construct a gun task and flag it.
///
/// `this` is the task, `arg` a game object (never null here: it is read at
/// once). First, unless a ready predicate (thiscall on `arg`) holds, a
/// ducking predicate (thiscall on `arg`) may trigger a reset call (thiscall
/// on `arg`: 0, -1).
///
/// The current weapon id is read from `arg` at `+0x2B0` through a
/// scaled index (`((id + 3) * 3) * 4 + 0x2B0`, wrapping) and resolved to a
/// weapon info record by a cdecl lookup. When the record selects slot 3
/// (`+0x08 == 3`, `+0x0C != 0`, bit 6 of `+0x20` set), a notifier runs
/// (thiscall on the task: arg), a block is allocated from the pool behind a
/// data global, and a null block returns null while a live one tail-calls
/// the finisher (thiscall: block, 0, with the incoming argument slot
/// overwritten by 0), whose result is the result.
///
/// Otherwise a block is allocated and constructed as a gun task (thiscall:
/// 1, 0, 0, `-1.0f`, 0, 1, 1, `1.0f`; a null block faults on the flag write
/// exactly as the original does), and flag word `+0x74` accumulates:
/// always `0x20040`; `0x1000008` when the record's `+0x04` is 6; `0x100`
/// when a data-global byte is set. Then the object's virtual slot at
/// `+0x128` (thiscall on `arg`) may yield a secondary object, and when a
/// data-global word is set while that object is null or its byte at
/// `+0x328D` is clear, `0x80000000` is added too. A final notifier runs and
/// the block is returned.
///
/// Bytes past this function's return belong to the next routine (the
/// inventory size overruns); they never execute here.
///
/// Original: 0x00A70B00 (thiscall, one stack word). Returns a pointer, the
/// tail result, or null in `eax`; predicate answers use `al` only.
lf_checker_rt::export!(thiscall, rw_00A70B00(this: u32, arg: u32) -> u32 {
    unsafe {
        const WARRAY: u32 = 0x2b0;
        const INFO_SLOT: u32 = 0x08;
        const INFO_WHAT: u32 = 0x0c;
        const INFO_BITS: u32 = 0x20;
        const INFO_KIND: u32 = 0x04;
        const WANT_SLOT: u32 = 3;
        const WANT_KIND: u32 = 6;
        const FLAGS: u32 = 0x74;
        const F_BASE: u32 = 0x20040;
        const F_KIND: u32 = 0x1000008;
        const F_GLOB: u32 = 0x100;
        const F_SEC: u32 = 0x80000000;
        const VT_SLOT: u32 = 0x128;
        const SEC_BYTE: u32 = 0x328d;
        const POOL_GLOBAL: u32 = 0x167e2a0;
        const GBYTE: u32 = 0x103ce46;
        const GDWORD: u32 = 0x1160c74;
        const C_READY: u32 = 1;
        const C_DUCK: u32 = 2;
        const C_RESET: u32 = 3;
        const C_WINFO: u32 = 4;
        const C_NOTE: u32 = 5;
        const C_ALLOC_T: u32 = 6;
        const C_TAIL: u32 = 7;
        const C_ALLOC_B: u32 = 8;
        const C_CTOR: u32 = 9;
        const C_VCALL: u32 = 10;
        const C_SEC: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn low_set(ans: u32) -> bool {
            ans & 0xff != 0
        }

        if !low_set(lf_checker_rt::callee_thiscall!(C_READY, u32, arg)) {
            if low_set(lf_checker_rt::callee_thiscall!(C_DUCK, u32, arg)) {
                lf_checker_rt::callee_thiscall!(C_RESET, u32, arg, 0, 0xffff_ffff);
            }
        }
        let w = rd32(arg + WARRAY).wrapping_add(3);
        let w3 = w.wrapping_mul(3);
        let wid = rd32(arg.wrapping_add(w3.wrapping_mul(4)).wrapping_add(WARRAY));
        let info = lf_checker_rt::callee_cdecl!(C_WINFO, u32, wid);
        if rd32(info + INFO_SLOT) == WANT_SLOT
            && rd32(info + INFO_WHAT) != 0
            && rd32(info + INFO_BITS) >> 6 & 1 == 1
        {
            lf_checker_rt::callee_thiscall!(C_NOTE, u32, this, arg);
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let block = lf_checker_rt::callee_thiscall!(C_ALLOC_T, u32, pool);
            if block == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(C_TAIL, u32, block, 0);
        }
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let raw = lf_checker_rt::callee_thiscall!(C_ALLOC_B, u32, pool);
        let built = if raw == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(
                C_CTOR, u32, raw, 1, 0, 0, 0xbf80_0000, 0, 1, 1, 0x3f80_0000
            )
        };
        wr32(built + FLAGS, rd32(built + FLAGS) | F_BASE);
        if rd32(info + INFO_KIND) == WANT_KIND {
            wr32(built + FLAGS, rd32(built + FLAGS) | F_KIND);
        }
        if rd8(lf_checker_rt::relocated(GBYTE)) != 0 {
            wr32(built + FLAGS, rd32(built + FLAGS) | F_GLOB);
        }
        let slot = rd32(rd32(arg) + VT_SLOT);
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let mut sec = 0u32;
        if low_set(f(arg)) {
            sec = lf_checker_rt::callee_thiscall!(C_SEC, u32, arg);
        }
        if rd32(lf_checker_rt::relocated(GDWORD)) != 0
            && (sec == 0 || rd8(sec + SEC_BYTE) == 0)
        {
            wr32(built + FLAGS, rd32(built + FLAGS) | F_SEC);
        }
        lf_checker_rt::callee_thiscall!(C_NOTE, u32, this, arg);
        built
    }
});
