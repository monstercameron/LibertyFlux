// original: 0x00ca4ec0 propose: release_adjusted_members
/// Adjusted teardown: shifts this by +0x20, releases the member at the
/// adjusted +4 through its vtable slot 7, then tail-releases the member at
/// the adjusted +8 through the same slot. The both-null combination returns
/// entry garbage in the original and is never generated; every branch is
/// still covered both ways by alternating which slot is null.
lf_rs75_rt::export!(thiscall, rw_00ca4ec0(this: u32) -> u32 {
    unsafe {
        let base = this.wrapping_add(0x20);
        let first = *((base + 4) as *const u32);
        let mut ans = 0u32;
        if first != 0 {
            let vtbl_a = *(first as *const u32);
            let slot = *((vtbl_a + 0x1C) as *const u32);
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            ans = release(first);
        }
        let second = *((base + 8) as *const u32);
        if second != 0 {
            let vtbl_b = *(second as *const u32);
            let slot = *((vtbl_b + 0x1C) as *const u32);
            let tail: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            return tail(second);
        }
        ans
    }
});
