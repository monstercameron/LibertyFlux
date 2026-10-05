// original: 0x00a35cd0 vehicle_linked_notify (proposed)

/// Notify the helper linked at `+0xc`, if there is one.
///
/// Reads the helper pointer at `obj + 0xc`; a null pointer answers 0 with
/// no call. Otherwise calls the helper callee (id 1, thiscall/1) with the
/// helper in ECX and `obj + 0xc` as the stack argument, and returns its
/// answer. Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a35cd0(obj: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x0C;
        const HELPER: u32 = 1;
        let helper = core::ptr::read_unaligned((obj + LINK) as *const u32);
        if helper == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(HELPER, u32, helper, obj.wrapping_add(LINK))
    }
});
