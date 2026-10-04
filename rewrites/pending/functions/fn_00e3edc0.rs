// original: 0x00e3edc0 StatsBlock_Init
// 0x00E3EDC0: initialise an 11-row stats block: header sentinels plus one
// cleared flag per row. Returns the block pointer. (thiscall/0)
export!(thiscall, rw_00e3edc0(this: *mut u8) -> u32 {
    unsafe {
        const ROWS: u32 = 11;
        const STRIDE: u32 = 0x3c;
        callee_thiscall!(1, u32, this as u32);
        *(this.add(0x10) as *mut u32) = 0;
        *(this.add(0x14) as *mut i32) = -1;
        *(this.add(0x18) as *mut i32) = -1;
        callee_thiscall!(2, u32, this as u32);
        let mut i = 0u32;
        while i < ROWS {
            *(this.add(0x1c).add((i.wrapping_mul(STRIDE)) as usize)) = 0u8;
            i += 1;
        }
        this as u32
    }
});
