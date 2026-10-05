// original: 0x00d58c20 ccam_view_find_child_value
/// Return the value field of the head child when it is of kind `WANT_KIND`.
///
/// Loads the head child at `[this+0x124]`; when null, or when the kind query
/// (the virtual slot at `+0x28`, intercepted callee 1) answers anything but
/// `WANT_KIND` (0xe), returns 0. Otherwise returns the dword at `VALUE`
/// (+0x244) of the child.
///
/// Original: thiscall, no stack arguments, returns `eax`.
lf_checker_rt::export!(thiscall, rw_00d58c20 (this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x124;
        const KIND_SLOT: u32 = 0x28;
        const VALUE: u32 = 0x244;
        const WANT_KIND: u32 = 0xe;
        let child = ((this + HEAD) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }
        let vtable = (child as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vtable + KIND_SLOT) as *const u32).read_unaligned() as usize);
        if kind_of(child) != WANT_KIND {
            return 0;
        }
        ((child + VALUE) as *const u32).read_unaligned()
    }
});
