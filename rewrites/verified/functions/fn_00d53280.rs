// original: 0x00D53280 camera_helper_update_float_fields

/// Gets the helper target associated with this camera and returns immediately
/// if it is null. Otherwise it writes the helper object's flag and updates the
/// camera's two float fields. A zero first value selects exactly 800.0; NaN is
/// nonzero. The second input is copied to its output unless it equals zero,
/// when the value from the data word is used. The two LAHF operations also
/// determine the returned EAX high-low combination.
lf_checker_rt::export!(thiscall, rw_00d53280(this: u32) -> u32 {
    const GET_HELPER: u32 = 1;
    const FIRST_INPUT: u32 = 0x154;
    const SECOND_INPUT: u32 = 0x158;
    const HELPER_FLAG: u32 = 0x1b4;
    const FIRST_OUTPUT: u32 = 0x68;
    const SECOND_OUTPUT: u32 = 0x64;
    const DATA_FLOAT_VA: u32 = 0x0104_8ab4;
    const EIGHT_HUNDRED_BITS: u32 = 0x4448_0000;

    let helper = lf_checker_rt::callee_thiscall!(GET_HELPER, u32, this);
    if helper == 0 {
        return helper;
    }

    let first_bits = unsafe { (this.wrapping_add(FIRST_INPUT) as *const u32).read_unaligned() };
    let first = f32::from_bits(first_bits);
    let mut eax_before_second_lahf = helper;
    if first != 0.0 {
        unsafe {
            (helper.wrapping_add(HELPER_FLAG) as *mut u8).write(0);
            (this.wrapping_add(FIRST_OUTPUT) as *mut u32).write_unaligned(first_bits);
        }
        eax_before_second_lahf = first_bits;
    } else {
        unsafe {
            (helper.wrapping_add(HELPER_FLAG) as *mut u8).write(1);
            (this.wrapping_add(FIRST_OUTPUT) as *mut u32).write_unaligned(EIGHT_HUNDRED_BITS);
        }
    }

    let second_bits = unsafe { (this.wrapping_add(SECOND_INPUT) as *const u32).read_unaligned() };
    let second = f32::from_bits(second_bits);
    let lahf = if second.is_nan() {
        0x47
    } else if second == 0.0 {
        0x42
    } else if second < 0.0 {
        0x03
    } else {
        0x02
    };
    let output_bits = if second == 0.0 {
        unsafe { lf_checker_rt::global::<u32>(DATA_FLOAT_VA).read_unaligned() }
    } else {
        second_bits
    };
    unsafe { (this.wrapping_add(SECOND_OUTPUT) as *mut u32).write_unaligned(output_bits); }

    (eax_before_second_lahf & 0xffff_00ff) | ((lahf as u32) << 8)
});