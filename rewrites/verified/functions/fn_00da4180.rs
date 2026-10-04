// original: 0x00da4180 count_two_ptr_tables
// s10f07: count of live entries across two pointer tables (thiscall/0).
//
// The object holds two begin/end pointer pairs; the result is the number of
// dwords between each pair added together. Differences are signed (sar), so a
// malformed empty range contributes zero or a negative count exactly as the
// original's shifts do.
export!(thiscall, rw_s10f07(this: *const u8) -> u32 {
    unsafe {
        let a0 = *(this.add(0x18) as *const i32);
        let a1 = *(this.add(0x1c) as *const i32);
        let b0 = *(this.add(0x24) as *const i32);
        let b1 = *(this.add(0x28) as *const i32);
        ((a1.wrapping_sub(a0) >> 2) + (b1.wrapping_sub(b0) >> 2)) as u32
    }
});
