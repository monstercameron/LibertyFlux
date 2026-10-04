// original: 0x00ab8430 record_slot_init
/// Stores two tag bytes and clears two words plus a flag bit in record `idx`
/// of the table. The return register keeps entry garbage, so it is unchecked.

export!(thiscall, rw_00ab8430(this: *mut u8, idx: u32, b1: u32, b2: u32) -> u32 {
    unsafe {
        let i = idx as usize;
        *this.add(i + 0x5C) = b1 as u8;
        *this.add(i + 0x67) = b2 as u8;
        *((this.add(i * 4 + 4)) as *mut u32) = 0;
        *((this.add(i * 4 + 0x30)) as *mut u32) = 0;
        *this.add(0x72) &= !4u8;
        0 // unchecked: original leaves entry-dependent garbage in eax
    }
});
