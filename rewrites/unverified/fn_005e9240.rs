// original: 0x005E9240 normalize_vector_safe

/// Normalize a three-float vector when its squared length exceeds per-axis thresholds.
///
/// `vector` points to three unaligned-capable `f32` fields at offsets 0, 4 and
/// 8. The squared length is formed as `(x*x + y*y) + z*z`, then compared with
/// the three global thresholds. The data mask selects either the scaled unit
/// vector or the global fallback vector, component by component. The original
/// uses a 1.0 numerator for each separate division and multiplies each quotient
/// by its component; `black_box` pins that operand order for bit-exact floats.
/// The function writes the three result bit patterns and has no defined scalar
/// return value.
lf_checker_rt::export!(thiscall, rw_005e9240(vector: u32) -> () {
    const X_OFFSET: u32 = 0;
    const Y_OFFSET: u32 = 4;
    const Z_OFFSET: u32 = 8;
    const MASK_SOURCE_VA: u32 = 0x017AD148;
    const X_THRESHOLD_VA: u32 = 0x0110DAD0;
    const Y_THRESHOLD_VA: u32 = 0x0110DAD4;
    const Z_THRESHOLD_VA: u32 = 0x0110DAD8;
    const NORMALIZE_NUMERATOR_VA: u32 = 0x00FE88E8;
    const FALLBACK_VECTOR_VA: u32 = 0x0110DB50;

    let x_bits = unsafe { (vector.wrapping_add(X_OFFSET) as *const u32).read_unaligned() };
    let y_bits = unsafe { (vector.wrapping_add(Y_OFFSET) as *const u32).read_unaligned() };
    let z_bits = unsafe { (vector.wrapping_add(Z_OFFSET) as *const u32).read_unaligned() };
    let x = f32::from_bits(x_bits);
    let y = f32::from_bits(y_bits);
    let z = f32::from_bits(z_bits);

    let y_squared = core::hint::black_box(y) * core::hint::black_box(y);
    let x_squared = core::hint::black_box(x) * core::hint::black_box(x);
    let xy_squared = core::hint::black_box(x_squared) + core::hint::black_box(y_squared);
    let z_squared = core::hint::black_box(z) * core::hint::black_box(z);
    let length_squared = core::hint::black_box(xy_squared) + core::hint::black_box(z_squared);

    let mask_source = unsafe {
        lf_checker_rt::global::<u32>(MASK_SOURCE_VA).read_unaligned()
    };
    let x_threshold = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(X_THRESHOLD_VA).read_unaligned()
    });
    let y_threshold = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(Y_THRESHOLD_VA).read_unaligned()
    });
    let z_threshold = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(Z_THRESHOLD_VA).read_unaligned()
    });
    let x_mask = if length_squared > x_threshold { mask_source } else { 0 };
    let y_mask = if length_squared > y_threshold { mask_source } else { 0 };
    let z_mask = if length_squared > z_threshold { mask_source } else { 0 };

    let root_input = core::hint::black_box(length_squared);
    let root_length = unsafe {
        #[cfg(target_arch = "x86")]
        {
            let value = core::arch::x86::_mm_set_ss(root_input);
            core::arch::x86::_mm_cvtss_f32(core::arch::x86::_mm_sqrt_ss(value))
        }
        #[cfg(target_arch = "x86_64")]
        {
            let value = core::arch::x86_64::_mm_set_ss(root_input);
            core::arch::x86_64::_mm_cvtss_f32(core::arch::x86_64::_mm_sqrt_ss(value))
        }
    };
    let numerator = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(NORMALIZE_NUMERATOR_VA).read_unaligned()
    });
    let x_scale = core::hint::black_box(numerator) / core::hint::black_box(root_length);
    let y_scale = core::hint::black_box(numerator) / core::hint::black_box(root_length);
    let z_scale = core::hint::black_box(numerator) / core::hint::black_box(root_length);
    let normalized_x = core::hint::black_box(x_scale) * core::hint::black_box(x);
    let normalized_y = core::hint::black_box(y_scale) * core::hint::black_box(y);
    let normalized_z = core::hint::black_box(z_scale) * core::hint::black_box(z);

    let fallback_x = unsafe {
        lf_checker_rt::global::<u32>(FALLBACK_VECTOR_VA).read_unaligned()
    };
    let fallback_y = unsafe {
        lf_checker_rt::global::<u32>(FALLBACK_VECTOR_VA.wrapping_add(4)).read_unaligned()
    };
    let fallback_z = unsafe {
        lf_checker_rt::global::<u32>(FALLBACK_VECTOR_VA.wrapping_add(8)).read_unaligned()
    };
    let output_x = (normalized_x.to_bits() & x_mask) | (fallback_x & !x_mask);
    let output_y = (normalized_y.to_bits() & y_mask) | (fallback_y & !y_mask);
    let output_z = (normalized_z.to_bits() & z_mask) | (fallback_z & !z_mask);

    unsafe {
        (vector.wrapping_add(X_OFFSET) as *mut u32).write_unaligned(output_x);
        (vector.wrapping_add(Y_OFFSET) as *mut u32).write_unaligned(output_y);
        (vector.wrapping_add(Z_OFFSET) as *mut u32).write_unaligned(output_z);
    }
});
