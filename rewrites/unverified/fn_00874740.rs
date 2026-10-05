// original: 0x00874740 crmt_array_teardown

/// Tear down a pointer array (`[this]` = entries, `[this+4]` = count): for each non-null entry run the element teardown (0x874950) then free the entry through the thread manager; free the array itself when non-null; clear both words. Returns 0.
///
/// Original: 0x00874740 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00874740(this: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEARDOWN: u32 = 1;
    unsafe {
        let count = ((this + 4) as *const u16).read_unaligned();
        if count > 0 {
            let arr = (this as *const u32).read_unaligned();
            let mut i = 0u32;
            while i < count as u32 {
                let ent = ((arr + i * 4) as *const u32).read_unaligned();
                if ent != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, ent);
                    let tls0 = lf_checker_rt::tls_slot(0);
                    let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
                    let vtable = (manager as *const u32).read_unaligned();
                    let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                        .read_unaligned();
                    let free_it: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    free_it(manager, ent);
                }
                i += 1;
            }
        }
        let arr = (this as *const u32).read_unaligned();
        if arr != 0 {
            let tls0 = lf_checker_rt::tls_slot(0);
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                .read_unaligned();
            let free_it: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free_it(manager, arr);
        }
        (this as *mut u32).write_unaligned(0);
        ((this + 4) as *mut u32).write_unaligned(0);
        0
    }
});
