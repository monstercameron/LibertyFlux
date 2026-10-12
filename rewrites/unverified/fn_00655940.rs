// original: 0x00655940 pose_set_identity
use lf_checker_rt::export;

/// Initialise the pose record's three diagonal basis words and clear its
/// selected translation and flag words. The object pointer arrives in ECX;
/// the function takes no stack arguments and has no defined return value.
export!(thiscall, rw_00655940(this: *mut u8) -> () {
    const OBJECT_FLAGS: usize = 0x00;
    const BASIS_X: usize = 0x10;
    const BASIS_Y: usize = 0x24;
    const BASIS_Z: usize = 0x38;
    const ZERO_A: usize = 0x14;
    const ZERO_B: usize = 0x18;
    const ZERO_C: usize = 0x20;
    const ZERO_D: usize = 0x28;
    const ZERO_E: usize = 0x30;
    const ZERO_F: usize = 0x34;
    const ZERO_G: usize = 0x40;
    const ZERO_H: usize = 0x44;
    const ZERO_I: usize = 0x48;
    const ZERO_J: usize = 0x50;
    const ZERO_K: usize = 0x54;
    const UNIT_BITS: u32 = 0x3F80_0000;

    unsafe {
        this.add(BASIS_X).cast::<u32>().write_unaligned(UNIT_BITS);
        this.add(ZERO_A).cast::<u32>().write_unaligned(0);
        this.add(ZERO_B).cast::<u32>().write_unaligned(0);
        this.add(ZERO_C).cast::<u32>().write_unaligned(0);
        this.add(BASIS_Y).cast::<u32>().write_unaligned(UNIT_BITS);
        this.add(ZERO_D).cast::<u32>().write_unaligned(0);
        this.add(ZERO_E).cast::<u32>().write_unaligned(0);
        this.add(ZERO_F).cast::<u32>().write_unaligned(0);
        this.add(BASIS_Z).cast::<u32>().write_unaligned(UNIT_BITS);
        this.add(ZERO_G).cast::<u32>().write_unaligned(0);
        this.add(ZERO_H).cast::<u32>().write_unaligned(0);
        this.add(ZERO_I).cast::<u32>().write_unaligned(0);
        this.add(ZERO_J).cast::<u32>().write_unaligned(0);
        this.add(ZERO_K).cast::<u32>().write_unaligned(0);
        this.add(OBJECT_FLAGS).write(0);
    }
});
