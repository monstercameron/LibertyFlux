// original: 0x008DC6C0 CDrawMobilePhoneCameraDC::vf0

/// Run the intercepted member and base destruction sequence. The low bit of the stack deletion flag gates the operator-delete call; the command pointer is returned in EAX.
lf_checker_rt::export!(thiscall, rw_008dc6c0(this: u32, delete_flag: u32) -> u32 {
    const COMMAND_VTABLE_VA: u32 = 0x00E81D10;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const MEMBER_DTOR_CALLEE: u32 = 62;
    const DELETE_CALLEE: u32 = 63;
    unsafe {
        let object = this as *mut u8;
        core::ptr::write_unaligned(object.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        let _ = lf_checker_rt::callee_thiscall!(MEMBER_DTOR_CALLEE, u32, this.wrapping_add(0x10));
        core::ptr::write_unaligned(object.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        if delete_flag & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(63, u32, this,);
        }
    }
    this
});
