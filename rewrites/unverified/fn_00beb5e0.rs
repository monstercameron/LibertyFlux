// original: 0x00beb5e0 row_ptr_or_null_beb5e0
/// Return the address of element `idx` in a row array, or null when empty.
///
/// `this` is an object whose dword at +0x0c is a row-present flag and whose
/// dword at +0x30 is the row base. When the flag is 0 returns 0, else
/// returns `base + idx * 8` with wrapping arithmetic (the original forms
/// the product with two address lea's: idx*8). Index bounds are not checked;
/// a negative index wraps below the base. Reads only. Thiscall, one stack
/// argument.
export!(thiscall, rw_00beb5e0(this: u32, idx: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x0c;
        const BASE: u32 = 0x30;
        const STRIDE: u32 = 8;
        if ((this + FLAG) as *const u32).read_unaligned() == 0 {
            0
        } else {
            ((this + BASE) as *const u32).read_unaligned()
                .wrapping_add(idx.wrapping_mul(STRIDE))
        }
    }
});
