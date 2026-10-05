// original: 0x00ccfa70 melee_cleanup_refs
/// Release the two referenced objects at `this+0xe8` and `this+0xec`.
///
/// Each non-null pointer is passed to the release helper (cdecl, one
/// argument) and the slot is then zeroed. Thiscall with no stack arguments.
export!(thiscall, rw_00ccfa70(this: u32) -> u32 {
    unsafe {
        const REF_A_OFF: u32 = 0xe8;
        const REF_B_OFF: u32 = 0xec;
        let a = (this.wrapping_add(REF_A_OFF) as *const u32).read_unaligned();
        if a != 0 {
            let _: u32 = callee_cdecl!(1, u32, a);
            (this.wrapping_add(REF_A_OFF) as *mut u32).write_unaligned(0);
        }
        let b = (this.wrapping_add(REF_B_OFF) as *const u32).read_unaligned();
        if b != 0 {
            let _: u32 = callee_cdecl!(2, u32, b);
            (this.wrapping_add(REF_B_OFF) as *mut u32).write_unaligned(0);
        }
        0
    }
});
