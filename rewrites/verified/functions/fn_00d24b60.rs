// original: 0x00D24B60 copy_vehicle_paint_state
/// Copies the paint-state block from a source object into this object: one word at offset 0x74,
/// fifteen words beginning at 0x90, two flag bytes, and three trailing state words. It then calls
/// the effect update method with the two remaining arguments. The method is thiscall with three
/// stack words; source pointers may alias the destination or point to disjoint data.
lf_checker_rt::export!(thiscall, rw_00d24b60(this: u32, source: u32, frame_argument: u32, update_mask: u32) -> () {
    const PAINT_WORD_OFFSET: u32 = 0x74;
    const TRACK_WORD_OFFSET: u32 = 0x90;
    const TRACK_WORD_COUNT: usize = 15;
    const PAINT_FLAG_LOW: u32 = 0xcc;
    const PAINT_FLAG_HIGH: u32 = 0xcd;
    const PAINT_STATE_TAIL: u32 = 0xd0;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { (base.wrapping_add(offset) as *const u8).read() }
    }
    unsafe fn write_u8(base: u32, offset: u32, value: u8) {
        unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
    }
    unsafe {
        write_u32(this, PAINT_WORD_OFFSET, read_u32(source, PAINT_WORD_OFFSET));
        for word_index in 0..TRACK_WORD_COUNT {
            let offset = TRACK_WORD_OFFSET + (word_index as u32) * 4;
            let value = read_u32(source, offset);
            write_u32(this, offset, value);
        }
        write_u8(this, PAINT_FLAG_LOW, read_u8(source, PAINT_FLAG_LOW));
        write_u8(this, PAINT_FLAG_HIGH, read_u8(source, PAINT_FLAG_HIGH));
        for tail_offset in [PAINT_STATE_TAIL, PAINT_STATE_TAIL + 4, PAINT_STATE_TAIL + 8] {
            let value = read_u32(source, tail_offset);
            write_u32(this, tail_offset, value);
        }
        let _ = lf_checker_rt::callee_thiscall!(1, (), this, frame_argument, update_mask);
    }
});
