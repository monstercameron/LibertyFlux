// original: 0x00873340 crmt_member_release_and_clear

/// Release the member at +0x10 through the thread manager when non-null, clear +0x10/+0x14, return 0. (The free call's answer is discarded by an explicit zeroing.)
///
/// Original: 0x00873340 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00873340(this: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    unsafe {
        let member = ((this + 0x10) as *const u32).read_unaligned();
        if member != 0 {
            let tls0 = lf_checker_rt::tls_slot(0);
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                .read_unaligned();
            let free_it: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free_it(manager, member);
        }
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        0
    }
});
