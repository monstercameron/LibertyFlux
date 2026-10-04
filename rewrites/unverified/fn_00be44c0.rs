// original: 0x00be44c0 task_build_sit_idle_for_kind (proposed)

/// Build a sit-idle subtask when the requested kind matches, else null.
///
/// `kind` (first stack word; the second is unread) selects the subtask: only
/// `WANT_KIND` (0x119) builds anything, any other value returns zero without
/// touching the pool. On a match, takes a slot from the task pool, runs the
/// base constructor on it, then stamps this task's own vtable pointer and
/// zero-initialises its small fields (`+0x14`, `+0x18`, `+0x1c`). Returns the
/// fresh subtask, or zero when the pool was empty.
///
/// Original: 0x00be44c0 (stdcall, two stack words: kind, unused).
lf_checker_rt::export!(stdcall, rw_00be44c0(kind: u32, _unused: u32) -> u32 {
    unsafe {
        const WANT_KIND: u32 = 0x119;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const VTABLE: u32 = 0x00eb391c;
        const FIELD_A: u32 = 0x14;
        const FLAG_B: u32 = 0x18;
        const FIELD_C: u32 = 0x1c;
        const ALLOC: u32 = 1;
        const BASE_CTOR: u32 = 2;
        if kind != WANT_KIND {
            return 0;
        }
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, slot);
        (slot as *mut u32).write_unaligned(VTABLE);
        (slot.wrapping_add(FIELD_A) as *mut u32).write_unaligned(0);
        (slot.wrapping_add(FLAG_B) as *mut u8).write(0);
        (slot.wrapping_add(FIELD_C) as *mut u32).write_unaligned(0);
        slot
    }
});
