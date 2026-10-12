// original: 0x0065F810 init_id_record

/// Initializes a 0x15-byte ID record through its `this` pointer. The leading words are cleared, the three words at offsets 8, 12, and 16 receive the all-bits-set sentinel, and the byte at offset 20 is cleared. This thiscall constructor returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_0065f810(object: *mut u8) -> u32 {
    const OBJECT: u32 = object as u32;
    const FIELD_00_OFFSET: u32 = 0x0;
    const FIELD_00_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_00_OFFSET) as *mut u32) = FIELD_00_VALUE; }
    const FIELD_01_OFFSET: u32 = 0x4;
    const FIELD_01_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_01_OFFSET) as *mut u32) = FIELD_01_VALUE; }
    const FIELD_02_OFFSET: u32 = 0x8;
    const FIELD_02_VALUE: u32 = 0xFFFFFFFF;
    unsafe { *((OBJECT + FIELD_02_OFFSET) as *mut u32) = FIELD_02_VALUE; }
    const FIELD_03_OFFSET: u32 = 0xC;
    const FIELD_03_VALUE: u32 = 0xFFFFFFFF;
    unsafe { *((OBJECT + FIELD_03_OFFSET) as *mut u32) = FIELD_03_VALUE; }
    const FIELD_04_OFFSET: u32 = 0x10;
    const FIELD_04_VALUE: u32 = 0xFFFFFFFF;
    unsafe { *((OBJECT + FIELD_04_OFFSET) as *mut u32) = FIELD_04_VALUE; }
    const FIELD_05_OFFSET: u32 = 0x14;
    const FIELD_05_VALUE: u8 = 0x00;
    unsafe { *((OBJECT + FIELD_05_OFFSET) as *mut u8) = FIELD_05_VALUE; }
    OBJECT
});
