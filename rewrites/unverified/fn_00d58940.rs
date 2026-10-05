// original: 0x00d58940 ccam_view_seq_child_ready
/// Walk the child list at `[this+0x124]`; succeed when a child of kind
/// `WANT_KIND` has its ready flag set.
///
/// Follows the `NEXT` (+0x11c) links from the head child. For each child it
/// calls the kind query (the virtual slot at `+0x28`, intercepted callee 1);
/// when the answer equals `WANT_KIND` (0x22) and byte `READY` (+0x14c) is
/// nonzero the walk succeeds. Returns 1 on success, 0 when the list is
/// empty or no child qualified.
///
/// Original: thiscall, no stack arguments, returns `al`.
lf_checker_rt::export!(thiscall, rw_00d58940 (this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x124;
        const NEXT: u32 = 0x11c;
        const KIND_SLOT: u32 = 0x28;
        const READY: u32 = 0x14c;
        const WANT_KIND: u32 = 0x22;
        let mut child = ((this + HEAD) as *const u32).read_unaligned();
        while child != 0 {
            let vtable = (child as *const u32).read_unaligned();
            let kind_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vtable + KIND_SLOT) as *const u32).read_unaligned() as usize);
            if kind_of(child) == WANT_KIND && ((child + READY) as *const u8).read() != 0 {
                return 1;
            }
            child = ((child + NEXT) as *const u32).read_unaligned();
        }
        0
    }
});
