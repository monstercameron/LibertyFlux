// original: 0x00bf38b0 fetch_block_a
/// Resolve two scratch values through helpers, then copy words +0x30..+0x38
/// from `src` to slots +8/+0xC/+0x10; returns the last copied word.
export!(thiscall, rw_bf38b0(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let mut buf1 = [0u32; 4];
        let _: u32 = callee_thiscall!(1, u32, src as u32, buf1.as_mut_ptr() as u32);
        let mut buf2 = [0u32; 4];
        let ans: u32 = callee_cdecl!(2, u32, buf2.as_mut_ptr() as u32);
        *((this.add(4)) as *mut u32) = ans;
        *((this.add(8)) as *mut u32) = *((src.add(0x30)) as *const u32);
        *((this.add(0xC)) as *mut u32) = *((src.add(0x34)) as *const u32);
        let last = *((src.add(0x38)) as *const u32);
        *((this.add(0x10)) as *mut u32) = last;
        last
    }
});
