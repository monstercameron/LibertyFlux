// original: 0x00e3f2f0 NameTable_Reset
// 0x00E3F2F0: reset an 11-slot name table to "..." entries, clearing the
// text of every slot from the given index on. (thiscall/1)
export!(thiscall, rw_00e3f2f0(this: *mut u8, first: u32) -> u32 {
    unsafe {
        const SLOTS: i32 = 11;
        const STRIDE: u32 = 60;
        *(this.add(0x10) as *mut u32) = 0;
        *(this.add(0x14) as *mut i32) = -1;
        callee_thiscall!(1, u32, this as u32);
        *(this.add(0x18) as *mut i32) = -1;
        let mut i = 0u32;
        while i < (SLOTS as u32) {
            *(this.add(0x1c).add((i.wrapping_mul(STRIDE)) as usize) as *mut u32) =
                0x002E_2E2E;
            i += 1;
        }
        let n = first as i32;
        if n < SLOTS {
            let mut p = (this as u32)
                .wrapping_add(0x1c)
                .wrapping_add((n as u32).wrapping_mul(STRIDE));
            let mut c = SLOTS - n;
            while c != 0 {
                *(p as *mut u8) = 0;
                p = p.wrapping_add(STRIDE);
                c -= 1;
            }
        }
        (this as u32).wrapping_add(0x1c).wrapping_add((SLOTS as u32).wrapping_mul(STRIDE))
    }
});
