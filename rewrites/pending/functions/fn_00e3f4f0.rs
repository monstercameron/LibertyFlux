// original: 0x00e3f4f0 SlotTable_Compact
// 0x00E3F4F0: compact a row table: copy the header of every used (not -1)
// row into the output. Returns the probe answer. (stdcall/3)
export!(stdcall, rw_00e3f4f0(src: u32, count: u32, dst: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        const HEAD: u32 = 16;
        let answer = callee_stdcall!(1, u32, src, count);
        let n = count as i32;
        if n > 0 {
            let mut out = dst;
            let mut i = 0i32;
            while i < n {
                let row = src.wrapping_add((i as u32).wrapping_mul(STRIDE));
                if *((row.wrapping_add(0x14)) as *const i32) != -1 {
                    *(out as *mut u64) = *(row as *const u64);
                    *((out.wrapping_add(8)) as *mut u64) =
                        *((row.wrapping_add(8)) as *const u64);
                    out = out.wrapping_add(HEAD);
                }
                i += 1;
            }
        }
        answer
    }
});
