// original: 0x00d3eb10 jump_subtask_is_plain (proposed)

/// Report whether this jump task has no special subtask.
///
/// `this` is the task; the dword at `SUB` (+8) points to its subtask or
/// is null. A null subtask returns 1. Otherwise the subtask's type slot
/// (virtual slot at +0xc, thiscall on the subtask) is queried twice: a
/// first answer of `TYPE_A` (0xcf) returns 0, else a second answer of
/// `TYPE_B` (0x846) returns 0, and anything else returns 1. Only al
/// carries the result (the original's xor/mov leave the upper bytes from
/// the last call answer, which is not compared).
///
/// Original: 0x00d3eb10 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d3eb10(this: u32) -> u32 {
    unsafe {
        const SUB: u32 = 8;
        const TYPE_SLOT: u32 = 0x0c;
        const TYPE_A: u32 = 0xcf;
        const TYPE_B: u32 = 0x846;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn type_of(sub: u32) -> u32 {
            unsafe {
                let vtable = rd32(sub);
                let slot = rd32(vtable + TYPE_SLOT);
                let query: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                query(sub)
            }
        }

        let sub = rd32(this + SUB);
        if sub == 0 {
            return 1;
        }
        if type_of(sub) == TYPE_A {
            return 0;
        }
        let sub2 = rd32(this + SUB);
        if type_of(sub2) == TYPE_B { 0 } else { 1 }
    }
});
