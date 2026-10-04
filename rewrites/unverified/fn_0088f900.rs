// original: 0x0088F900 
// 0088F900 audSound voice bind: allocate a voice for the row, then store the
// pool-relative slot index.
export!(thiscall, rw_0088f900(this: *mut u8) -> () {
    unsafe {
        let sel = *this.add(0x40);
        let got = callee_thiscall!(1, u32, relocated(0x115D8A0), 0xF0, sel as u32);
        if got != 0 {
            callee_thiscall!(2, u32, got);
            let base = *global::<u32>(0x115D988);
            let entry = *((sel as u32)
                .wrapping_mul(0x6F40)
                .wrapping_add(base)
                .wrapping_add(0x6F14) as *const u32);
            let rel = got.wrapping_sub(entry);
            *this.add(4) = (rel / *global::<u32>(0x115D968)) as u8;
        }
    }
});
