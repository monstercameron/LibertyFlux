// original: 0x00935CB0 notify_active_network_state

/// Call the state helper with argument zero only when the receiver's byte at
/// `+0x164` is nonzero. The helper receives the receiver in ECX and cleans its
/// one stack argument. The function itself takes no stack arguments and has
/// no defined return value.
lf_checker_rt::export!(thiscall, rw_00935cb0(this: u32) -> () {
    unsafe {
        const ACTIVE_FLAG: u32 = 0x164;

        if (this.wrapping_add(ACTIVE_FLAG) as *const u8).read() != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this, 0);
        }
    }
});
