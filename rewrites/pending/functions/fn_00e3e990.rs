// original: 0x00e3e990 TxdSlot_ReleaseAll
// 0x00E3E990: release every entry of a texture-dictionary slot, then close
// the slot. Returns the close answer, or the entry count when the slot
// was already empty or missing. (thiscall/0)
export!(thiscall, rw_00e3e990(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x30;
        const ROWS_OFF: u32 = 0x28;
        let slot = callee_cdecl!(1, u32, relocated(0xF15500));
        let count = *((this.wrapping_add(COUNT_OFF)) as *const i32);
        let result = if count > 0 && slot != 0xFFFF_FFFF {
            let mut i = 0i32;
            while i < *((this.wrapping_add(COUNT_OFF)) as *const i32) {
                let row = (this.wrapping_add(ROWS_OFF))
                    .wrapping_add((i as u32).wrapping_mul(4));
                callee_thiscall!(2, u32, row);
                i += 1;
            }
            callee_cdecl!(3, u32, slot)
        } else {
            count as u32
        };
        *((this.wrapping_add(COUNT_OFF)) as *mut u32) = 0;
        result
    }
});
