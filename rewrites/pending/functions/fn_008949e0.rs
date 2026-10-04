// original: 0x008949e0 store_quantized_slot_byte
/// Store a quantized slot value derived from a strided global table.
///
/// A zero input stores 0xFF and returns 0. Otherwise the low byte of the
/// selector picks one row of a global table (rows are 0x6F40 bytes apart,
/// starting at offset 0x6F10 past the table base); the stored byte and the
/// return value are the low byte of the input minus that row's word divided
/// by a global divisor. The divisor is never zero in practice.
crate::rt::export!(thiscall, rs17_008949e0(this: u32, selector: u32, value: u32) -> u32 {
    if value == 0 {
        unsafe { ((this + 5) as *mut u8).write(0xFF) };
        return 0;
    }
    const ROW_STRIDE: u32 = 0x6F40;
    const TABLE_BIAS: u32 = 0x6F10;
    let base = unsafe { *crate::rt::global::<u32>(0x0115D988) };
    let offset = (selector & 0xFF)
        .wrapping_mul(ROW_STRIDE)
        .wrapping_add(TABLE_BIAS);
    let cell = unsafe { (base.wrapping_add(offset) as *const u32).read() };
    let divisor = unsafe { *crate::rt::global::<u32>(0x0115D964) };
    let quotient = value.wrapping_sub(cell) / divisor;
    let low = (quotient & 0xFF) as u8;
    unsafe { ((this + 5) as *mut u8).write(low) };
    quotient & 0xFF
});
