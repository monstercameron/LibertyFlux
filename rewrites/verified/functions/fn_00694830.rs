// original: 0x00694830 rage::crCreatureComponentBlendShapes::vf8

/// Clear the second word of each fixed-size record in the component's array.
///
/// The record count is the unsigned 16-bit value at `this + 0x10`, and the
/// record array begins at the pointer stored at `this + 0x0c`. Each record is
/// 0x14 bytes; only its word at offset four is cleared. EAX returns the count.
/// The function is a thiscall method with the object in ECX and no stack args.
lf_checker_rt::export!(thiscall, rw_00694830(this: u32) -> u32 {
    unsafe {
        const ARRAY_PTR: u32 = 0x0C;
        const COUNT: u32 = 0x10;
        const RECORD_STRIDE: u32 = 0x14;
        const CLEARED_WORD: u32 = 4;
        let object = this as *const u8;
        let records = (object.add(ARRAY_PTR as usize) as *const u32).read_unaligned();
        let count = (object.add(COUNT as usize) as *const u16).read_unaligned() as u32;
        let mut index = 0u32;
        while index < count {
            let field = records.wrapping_add(index.wrapping_mul(RECORD_STRIDE).wrapping_add(CLEARED_WORD));
            (field as *mut u32).write_unaligned(0);
            index += 1;
        }
        count
    }
});
