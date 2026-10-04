// original: 0x00be3680 CTaskComplexUseEffect::vf19 (symbols)

/// Rebind this task's effect target and resolve its handle, or build fresh.
///
/// Stores the incoming argument into the target slot (`this + TARGET_OFF`,
/// 0x20), releasing the previous target first and referencing the new one
/// (either may be null, skipping its call). Then resolves the effect handle
/// (`this + HANDLE_OFF`, 0x1c): when the manual flag (`+ MANUAL_OFF`, 0x28)
/// is set the handle is cleared; otherwise the lookup parameters come from
/// this task's own words (`+ PAR_A`, 0x18, when set, else an index-to-id
/// mapping of `+ PAR_B`, 0x14, whose negative answers also clear the
/// handle), and the lookup callee (cdecl: it pops nothing itself) runs over
/// (argument, PAR_B, PAR_A-or-zero, `+ PAR_C`, 0x24), with the resolve callee
/// consuming the four leftover stack words as its own arguments.
///
/// A resolved nonzero handle runs the check callee with the argument and its
/// answer is returned. A zero handle builds a fresh subtask from the pool
/// instead (base-constructed and stamped with this task's vtable pointer),
/// returning it, or zero when the pool is empty.
///
/// The rewrite passes the four lookup words to both callees explicitly; the
/// original lets the resolve callee inherit them from the stack.
///
/// Original: 0x00be3680 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be3680(this: u32, arg: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x20;
        const HANDLE_OFF: u32 = 0x1c;
        const MANUAL_OFF: u32 = 0x28;
        const PAR_A: u32 = 0x18;
        const PAR_B: u32 = 0x14;
        const PAR_C: u32 = 0x24;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const VTABLE_FILE_VA: u32 = 0x00eb38c4;
        const UNREF: u32 = 1;
        const REF: u32 = 2;
        const INDEX_TO_ID: u32 = 3;
        const LOOKUP: u32 = 4;
        const RESOLVE: u32 = 5;
        const CHECK: u32 = 6;
        const ALLOC: u32 = 7;
        const BASE_CTOR: u32 = 8;
        let target = this.wrapping_add(TARGET_OFF);
        let old = (target as *const u32).read_unaligned();
        if old != 0 {
            lf_checker_rt::callee_thiscall!(UNREF, u32, old, target);
        }
        (target as *mut u32).write_unaligned(arg);
        if arg != 0 {
            lf_checker_rt::callee_thiscall!(REF, u32, arg, target);
        }
        let manual = (this.wrapping_add(MANUAL_OFF) as *const u8).read();
        if manual == 0 {
            let a = (this.wrapping_add(PAR_A) as *const u32).read_unaligned();
            let a = if a != 0 {
                a
            } else {
                let b = (this.wrapping_add(PAR_B) as *const u32).read_unaligned();
                let id = lf_checker_rt::callee_cdecl!(INDEX_TO_ID, u32, b) as i32;
                if id < 0 {
                    (this.wrapping_add(HANDLE_OFF) as *mut u32).write_unaligned(0);
                    return rw_00be3680_tail(this, arg);
                }
                0
            };
            let b = (this.wrapping_add(PAR_B) as *const u32).read_unaligned();
            let c = (this.wrapping_add(PAR_C) as *const u32).read_unaligned();
            let found = lf_checker_rt::callee_cdecl!(LOOKUP, u32, arg, b, a, c);
            let resolved = lf_checker_rt::callee_thiscall!(RESOLVE, u32, found, arg, b, a, c);
            (this.wrapping_add(HANDLE_OFF) as *mut u32).write_unaligned(resolved);
        } else {
            (this.wrapping_add(HANDLE_OFF) as *mut u32).write_unaligned(0);
        }
        rw_00be3680_tail(this, arg)
    }
});

/// Tail of the effect rebind: check a resolved handle or build fresh.
///
/// Shared by the resolve path and the cleared-handle paths above. Not an
/// export: it is this function's own second half, split only so the early
/// negative-index return can reach it.
#[inline(always)]
unsafe fn rw_00be3680_tail(this: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x1c;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const VTABLE_FILE_VA: u32 = 0x00eb38c4;
        const CHECK: u32 = 6;
        const ALLOC: u32 = 7;
        const BASE_CTOR: u32 = 8;
        let handle = (this.wrapping_add(HANDLE_OFF) as *const u32).read_unaligned();
        if handle != 0 {
            return lf_checker_rt::callee_thiscall!(CHECK, u32, handle, arg);
        }
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let got = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if got == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, got);
        (got as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
        got
    }
}
