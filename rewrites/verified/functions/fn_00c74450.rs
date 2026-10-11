// original: 0x00c74450 CPedModelInfo::vf11

/// Release the optional resources at `+0x12c` and `+0x128`, clearing each
/// field after its cleanup and free calls. If `+0x130` holds a third resource,
/// invoke virtual slot `+0x18` with that resource as both the this pointer and
/// stack argument, then clear the field. Finish with the base-model cleanup
/// tail operation and return its EAX value.
lf_checker_rt::export!(thiscall, rw_00c74450(this: u32) -> u32 {
    const CLEAN_FIRST: u32 = 1;
    const FREE_FIRST: u32 = 2;
    const CLEAN_SECOND: u32 = 3;
    const FREE_SECOND: u32 = 4;
    const BASE_CLEANUP: u32 = 6;

    unsafe {
        let first = ((this.wrapping_add(0x12c)) as *const u32).read_unaligned();
        if first != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CLEAN_FIRST, u32, first);
            let _ = lf_checker_rt::callee_cdecl!(FREE_FIRST, u32, first);
            ((this.wrapping_add(0x12c)) as *mut u32).write_unaligned(0);
        }

        let second = ((this.wrapping_add(0x128)) as *const u32).read_unaligned();
        if second != 0 {
            let _ = lf_checker_rt::callee_thiscall!(CLEAN_SECOND, u32, second);
            let _ = lf_checker_rt::callee_cdecl!(FREE_SECOND, u32, second);
            ((this.wrapping_add(0x128)) as *mut u32).write_unaligned(0);
        }

        let third = ((this.wrapping_add(0x130)) as *const u32).read_unaligned();
        if third != 0 {
            let vtable = (third as *const u32).read_unaligned();
            let action_address = ((vtable.wrapping_add(0x18)) as *const u32).read_unaligned();
            let action: unsafe extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(action_address as usize);
            let _ = action(third, third);
            ((this.wrapping_add(0x130)) as *mut u32).write_unaligned(0);
        }
    }

    lf_checker_rt::callee_thiscall!(BASE_CLEANUP, u32, this)
});
