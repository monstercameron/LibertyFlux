// original: 0x00c6a3c0 stream_try_acquire (proposed)

/// Try to acquire stream slot `id` through its four gates.
///
/// Each gate calls out with (id, lock word from its global); a zero
/// low byte returns that answer at once. Between the second and third
/// gates the slot's object must show a clear word at offset 0x44, or
/// the object itself is returned. Surviving all four returns
/// the last answer.
///
/// Original: stdcall with one stack word, four calls, reads two globals.
lf_checker_rt::export!(stdcall, rw_00c6a3c0(id: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 0x012B_4138;
        const TABLE: u32 = 0x0129_5CD8;
        const BUSY_OFF: u32 = 0x44;
        const GATE_A: u32 = 1;
        const GATE_B: u32 = 2;
        const GATE_C: u32 = 3;
        const GATE_D: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let lock = rd32(lf_checker_rt::relocated(LOCK));
        let table = lf_checker_rt::relocated(TABLE);
        let a: u32 = lf_checker_rt::callee_cdecl!(GATE_A, u32, id, lock);
        if (a & 0xFF) == 0 {
            return a;
        }
        let _b: u32 = lf_checker_rt::callee_cdecl!(GATE_B, u32, id, lock);
        let obj = rd32(table.wrapping_add(id.wrapping_mul(4)));
        if rd32(obj.wrapping_add(BUSY_OFF)) != 0 {
            return obj;
        }
        let c: u32 = lf_checker_rt::callee_cdecl!(GATE_C, u32, id, lock);
        if (c & 0xFF) == 0 {
            return c;
        }
        lf_checker_rt::callee_cdecl!(GATE_D, u32, id, lock)
    }
});
