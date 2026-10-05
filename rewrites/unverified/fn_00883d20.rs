// original: 0x00883d20 stream_channel_clear (proposed)
/// Release every live slot of a channel and mark the channel empty.
///
/// Walks the eight half-word slots at `this+0x4c`: a slot already holding
/// `FREE` (`0xffff`) is skipped, otherwise its pool index is resolved to an
/// object through the pool lookup (intercepted callee 1, cdecl, one
/// argument), the object's owner (at `+0x04`) detaches it (intercepted
/// callee 2, thiscall: owner in `ecx`, index as the stack argument), the
/// object itself is freed (intercepted callee 3, cdecl, one argument), and
/// the slot is marked `FREE`. The stack argument is unused padding (the
/// callee still cleans it).
///
/// Original: thiscall, one stack argument, callee cleans 4, no return value.
lf_checker_rt::export!(thiscall, rw_00883d20(this: u32, _pad: u32) -> u32 {
    unsafe {
        const SLOT_TABLE: u32 = 0x4c;
        const SLOT_COUNT: u32 = 8;
        const OWNER: u32 = 0x04;
        const FREE: u16 = 0xffff;
        const LOOKUP_CALLEE: u32 = 1;
        const DETACH_CALLEE: u32 = 2;
        const FREE_CALLEE: u32 = 3;
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let slot_ptr = (this + SLOT_TABLE + i.wrapping_mul(2)) as *mut u16;
            let index = slot_ptr.read_unaligned();
            if index != FREE {
                let obj: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, index as u32);
                let owner = ((obj + OWNER) as *const u32).read_unaligned();
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(DETACH_CALLEE, u32, owner, index as u32);
                let _: u32 = lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, obj);
                slot_ptr.write_unaligned(FREE);
            }
            i += 1;
        }
        0
    }
});
