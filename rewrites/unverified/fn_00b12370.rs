// original: 0x00B12370 dynamic_entity_update_fraction_byte

/// Pass a selected storage pointer, model index and signed byte value to the
/// fraction helper. Without the object's mode bit, force the index and byte
/// arguments to -1. The helper's exact single-precision result is compared to
/// the threshold: equality stores 0xfe, while every other value is multiplied
/// by the scale and truncated to x86's signed 32-bit integer result before its
/// low byte is stored. This is a no-argument thiscall method.
lf_checker_rt::export!(thiscall, rw_00b12370(this: u32) -> () {
    const MODE: u32 = 0x24;
    const MODEL_INDEX: u32 = 0x48;
    const BYTE_VALUE: u32 = 0x40;
    const ARRAY: u32 = 0x20;
    const INLINE_STORAGE: u32 = 0x10;
    const OUTPUT_BYTE: u32 = 0x5B;
    const THRESHOLD_VA: u32 = 0x00FE_88E8;
    const SCALE_VA: u32 = 0x00FE_8C08;

    #[inline(always)]
    fn multiply_in_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    fn cvttss2si(value: f32) -> i32 {
        if !value.is_finite() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
            i32::MIN
        } else {
            value as i32
        }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn read_f32(address: u32) -> f32 {
        unsafe { (address as *const f32).read_unaligned() }
    }
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        let mode = read_u32(this.wrapping_add(MODE));
        let model_index = if mode & 0x0800_0000 != 0 {
            read_u32(this.wrapping_add(MODEL_INDEX))
        } else {
            u32::MAX
        };
        let byte_value = if mode & 0x0800_0000 != 0 {
            (this.wrapping_add(BYTE_VALUE) as *const i8).read() as i32 as u32
        } else {
            u32::MAX
        };
        let array = read_u32(this.wrapping_add(ARRAY));
        let storage = if array != 0 {
            array.wrapping_add(0x30)
        } else {
            this.wrapping_add(INLINE_STORAGE)
        };
        let fraction = lf_checker_rt::callee_cdecl!(1, f32, storage, model_index, byte_value);
        let threshold = read_f32(lf_checker_rt::global::<u32>(THRESHOLD_VA) as u32);
        if fraction == threshold {
            write_u8(this.wrapping_add(OUTPUT_BYTE), 0xFE);
            return;
        }
        let scale = read_f32(lf_checker_rt::global::<u32>(SCALE_VA) as u32);
        let scaled = multiply_in_order(fraction, scale);
        write_u8(this.wrapping_add(OUTPUT_BYTE), cvttss2si(scaled) as u8);
    }
});
