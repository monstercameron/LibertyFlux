// original: 0x00bf3970 fetch_block_b
/// Same shape as fetch_block_a, writing slots +0x1C/+0x20/+0x24/+0x28.
export!(thiscall, rw_bf3970(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        let mut buf1 = [0u32; 4];
        let _: u32 = callee_thiscall!(1, u32, src as u32, buf1.as_mut_ptr() as u32);
        let mut buf2 = [0u32; 4];
        let ans: u32 = callee_cdecl!(2, u32, buf2.as_mut_ptr() as u32);
        *((this.add(0x1C)) as *mut u32) = ans;
        *((this.add(0x20)) as *mut u32) = *((src.add(0x30)) as *const u32);
        *((this.add(0x24)) as *mut u32) = *((src.add(0x34)) as *const u32);
        let last = *((src.add(0x38)) as *const u32);
        *((this.add(0x28)) as *mut u32) = last;
        last
    }
});
