// original: 0x00be33d0 CTaskComplexSitIdle::vf19 (symbols)

/// Build this task's sit-idle subtask unless it is already sitting.
///
/// Asks this task (through its own vtable slot `READY_SLOT`, 0x54, with the
/// incoming argument) whether it is ready: a nonzero answer returns zero at
/// once. When both state bytes at `+ STATE_A` (0x3c) and `+ STATE_B` (0x3d)
/// are also nonzero, the fast path takes a pool slot, runs the base
/// constructor, and stamps this task's vtable pointer plus zeroed small
/// fields (`+0x14`, `+0x18`, `+0x1c`); an empty pool returns zero. When
/// either state byte is zero, the slow path takes a pool slot and constructs
/// there with the fixed arguments (0, -1.0f, 0, 0, 0, 12), returning the new
/// subtask, or zero when the pool is empty.
///
/// Original: 0x00be33d0 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be33d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const READY_SLOT: u32 = 0x54;
        const STATE_A: u32 = 0x3c;
        const STATE_B: u32 = 0x3d;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const VTABLE_FILE_VA: u32 = 0x00eb391c;
        const FIELD_A: u32 = 0x14;
        const FLAG_B: u32 = 0x18;
        const FIELD_C: u32 = 0x1c;
        const NEG_ONE_BITS: u32 = 0xbf800000; // -1.0f
        const SLOW_LAST: u32 = 12;
        const ALLOC_FAST: u32 = 2;
        const BASE_CTOR: u32 = 3;
        const ALLOC_SLOW: u32 = 4;
        const CONSTRUCT_SLOW: u32 = 5;
        // The readiness query runs through this task's own vtable (stub id
        // 1 in the contract), exactly like the original: no ctable use here.
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(READY_SLOT) as *const u32).read_unaligned();
        let is_ready: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        if (is_ready(this, arg) & 0xff) != 0 {
            return 0;
        }
        let a = (this.wrapping_add(STATE_A) as *const u8).read();
        let b = (this.wrapping_add(STATE_B) as *const u8).read();
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        if a != 0 && b != 0 {
            let got = lf_checker_rt::callee_thiscall!(ALLOC_FAST, u32, pool);
            if got == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, got);
            (got as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
            (got.wrapping_add(FIELD_A) as *mut u32).write_unaligned(0);
            (got.wrapping_add(FLAG_B) as *mut u8).write(0);
            (got.wrapping_add(FIELD_C) as *mut u32).write_unaligned(0);
            return got;
        }
        let got = lf_checker_rt::callee_thiscall!(ALLOC_SLOW, u32, pool);
        if got == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            CONSTRUCT_SLOW,
            u32,
            got,
            0,
            NEG_ONE_BITS,
            0,
            0,
            0,
            SLOW_LAST
        )
    }
});
