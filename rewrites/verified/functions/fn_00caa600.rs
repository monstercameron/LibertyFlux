// original: 0x00CAA600 handler_release_all (proposed)

/// Release every task object the handler owns and mark it empty.
///
/// For each of the five task slots at `this+0x08` through `this+0x18`, when
/// the slot is non-null, calls virtual slot 0 of that object with argument
/// 1 (a releasing call) and clears the slot. Always writes a zero byte at
/// `this+0x1C`. No stack arguments, no return value.
///
/// Original: 0x00CAA600 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00caa600(this: u32) -> u32 {
    unsafe {
        const SLOT_FIRST: u32 = 0x08;
        const SLOT_LAST: u32 = 0x18;
        const EMPTY_FLAG: u32 = 0x1C;
        const RELEASE_SLOT: u32 = 0x00;
        const RELEASE_ARG: u32 = 1;
        let mut off = SLOT_FIRST;
        while off <= SLOT_LAST {
            let obj = (this.wrapping_add(off) as *const u32).read_unaligned();
            if obj != 0 {
                let vtable = (obj as *const u32).read_unaligned();
                let slot = (vtable.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                release(obj, RELEASE_ARG);
                (this.wrapping_add(off) as *mut u32).write_unaligned(0);
            }
            off += 4;
        }
        (this.wrapping_add(EMPTY_FLAG) as *mut u8).write(0);
        0
    }
});
