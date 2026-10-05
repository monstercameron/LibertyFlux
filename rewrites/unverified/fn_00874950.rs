// original: 0x00874950 crmt_subobject_release

/// Release the sub-object at `[this+4]`: first offer it to the 0x605b20 hook, then when non-null free its base block through the thread manager if the flag byte at +0x10 has bit 0 set and the base is non-null, clear the flag bit and the words at +4/+0xc/+0, free the sub-object itself and clear the slot. Returns the hook's answer when the slot was empty, else the final free's answer.
///
/// Original: 0x00874950 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00874950(this: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const PREP: u32 = 1;
    unsafe {
        let sub0 = ((this + 4) as *const u32).read_unaligned();
        let ans1: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, sub0);
        let sub = ((this + 4) as *const u32).read_unaligned();
        if sub == 0 {
            ((this + 4) as *mut u32).write_unaligned(0);
            return ans1;
        }
        let fl = ((sub + 0x10) as *const u8).read();
        if fl & 1 != 0 {
            let base = (sub as *const u32).read_unaligned();
            if base != 0 {
                let tls0 = lf_checker_rt::tls_slot(0);
                let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
                let vtable = (manager as *const u32).read_unaligned();
                let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                    .read_unaligned();
                let free_it: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                free_it(manager, base);
            }
        }
        ((sub + 0x10) as *mut u8).write(fl & 0xfe);
        ((sub + 4) as *mut u32).write_unaligned(0);
        ((sub + 0xc) as *mut u32).write_unaligned(0);
        (sub as *mut u32).write_unaligned(0);
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
            .read_unaligned();
        let free_it: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let out = free_it(manager, sub);
        ((this + 4) as *mut u32).write_unaligned(0);
        out
    }
});
