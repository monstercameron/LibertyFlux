// original: 0x00666810 init_keyframe_defaults

/// Sets default keyframe fields through the `this` pointer. It clears the leading words and the word at offset 0x2c, writes four packed values at offsets 0x14 through 0x20, then stores exact single-precision bit patterns at offsets 0x24, 0x28, 0x30, 0x34, and 0x38. It clears the word at 0x3c and the byte at 0x40. Unassigned bytes remain untouched. This thiscall initializer leaves the return register unchanged; callers use its object writes.
lf_checker_rt::export!(thiscall, rw_00666810(object: *mut u8) -> () {
    const OBJECT: u32 = object as u32;
    const FIELD_00_OFFSET: u32 = 0x0;
    const FIELD_00_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_00_OFFSET) as *mut u32) = FIELD_00_VALUE; }
    const FIELD_01_OFFSET: u32 = 0x14;
    const FIELD_01_VALUE: u32 = 0xFFFF0000;
    unsafe { *((OBJECT + FIELD_01_OFFSET) as *mut u32) = FIELD_01_VALUE; }
    const FIELD_02_OFFSET: u32 = 0x18;
    const FIELD_02_VALUE: u32 = 0xFF008000;
    unsafe { *((OBJECT + FIELD_02_OFFSET) as *mut u32) = FIELD_02_VALUE; }
    const FIELD_03_OFFSET: u32 = 0x1C;
    const FIELD_03_VALUE: u32 = 0xFF0000FF;
    unsafe { *((OBJECT + FIELD_03_OFFSET) as *mut u32) = FIELD_03_VALUE; }
    const FIELD_04_OFFSET: u32 = 0x20;
    const FIELD_04_VALUE: u32 = 0xFFA9A9A9;
    unsafe { *((OBJECT + FIELD_04_OFFSET) as *mut u32) = FIELD_04_VALUE; }
    const FIELD_05_OFFSET: u32 = 0x4;
    const FIELD_05_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_05_OFFSET) as *mut u32) = FIELD_05_VALUE; }
    const FIELD_06_OFFSET: u32 = 0x8;
    const FIELD_06_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_06_OFFSET) as *mut u32) = FIELD_06_VALUE; }
    const FIELD_07_OFFSET: u32 = 0xC;
    const FIELD_07_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_07_OFFSET) as *mut u32) = FIELD_07_VALUE; }
    const FIELD_08_OFFSET: u32 = 0x10;
    const FIELD_08_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_08_OFFSET) as *mut u32) = FIELD_08_VALUE; }
    const FIELD_09_OFFSET: u32 = 0x24;
    const FIELD_09_VALUE: u32 = 0xC2C80000;
    unsafe { *((OBJECT + FIELD_09_OFFSET) as *mut u32) = FIELD_09_VALUE; }
    const FIELD_10_OFFSET: u32 = 0x28;
    const FIELD_10_VALUE: u32 = 0x42C80000;
    unsafe { *((OBJECT + FIELD_10_OFFSET) as *mut u32) = FIELD_10_VALUE; }
    const FIELD_11_OFFSET: u32 = 0x2C;
    const FIELD_11_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_11_OFFSET) as *mut u32) = FIELD_11_VALUE; }
    const FIELD_12_OFFSET: u32 = 0x30;
    const FIELD_12_VALUE: u32 = 0x3F800000;
    unsafe { *((OBJECT + FIELD_12_OFFSET) as *mut u32) = FIELD_12_VALUE; }
    const FIELD_13_OFFSET: u32 = 0x34;
    const FIELD_13_VALUE: u32 = 0x3C23D70A;
    unsafe { *((OBJECT + FIELD_13_OFFSET) as *mut u32) = FIELD_13_VALUE; }
    const FIELD_14_OFFSET: u32 = 0x38;
    const FIELD_14_VALUE: u32 = 0x3C23D70A;
    unsafe { *((OBJECT + FIELD_14_OFFSET) as *mut u32) = FIELD_14_VALUE; }
    const FIELD_15_OFFSET: u32 = 0x3C;
    const FIELD_15_VALUE: u32 = 0x00000000;
    unsafe { *((OBJECT + FIELD_15_OFFSET) as *mut u32) = FIELD_15_VALUE; }
    const FIELD_16_OFFSET: u32 = 0x40;
    const FIELD_16_VALUE: u8 = 0x00;
    unsafe { *((OBJECT + FIELD_16_OFFSET) as *mut u8) = FIELD_16_VALUE; }
});
