// original: 0x005f28b0 BlendVectorByLengthThresholds

/// Blends a three-float vector with a normalized copy of the receiver's vector.
/// It computes the squared length in x/y/z order, builds three independent
/// masks from the live threshold words, and chooses each component with SSE
/// bitwise selection. The fourth source word is loaded but never stored. This
/// preserves the source bits when a mask is zero, including NaN payloads.
lf_checker_rt::export!(thiscall, rw_005f28b0(this: u32, source: u32) -> u32 {
    #[inline(never)]
    fn vector_mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(never)]
    fn vector_add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(never)]
    fn vector_div(numerator: f32, denominator: f32) -> f32 {
        core::hint::black_box(numerator) / core::hint::black_box(denominator)
    }

    const VECTOR_X: u32 = 0x00;
    const VECTOR_Y: u32 = 0x04;
    const VECTOR_Z: u32 = 0x08;
    const SOURCE_WORDS: usize = 4;
    const SOURCE_X: usize = 0;
    const SOURCE_Y: usize = 1;
    const SOURCE_Z: usize = 2;
    const SOURCE_UNUSED_W: usize = 3;
    const MASK_WORD: u32 = 0x017a_d148;
    const Z_LENGTH_THRESHOLD: u32 = 0x0110_dad8;
    const Y_LENGTH_THRESHOLD: u32 = 0x0110_dad4;
    const X_LENGTH_THRESHOLD: u32 = 0x0110_dad0;
    const NORMALIZATION_NUMERATOR: u32 = 0x00fe_88e8;

    let source_words = unsafe {
        core::ptr::read_unaligned(source as *const [u32; SOURCE_WORDS])
    };
    let _loaded_w = core::hint::black_box(source_words[SOURCE_UNUSED_W]);

    let x = f32::from_bits(unsafe { (this.wrapping_add(VECTOR_X) as *const u32).read_unaligned() });
    let y = f32::from_bits(unsafe { (this.wrapping_add(VECTOR_Y) as *const u32).read_unaligned() });
    let z = f32::from_bits(unsafe { (this.wrapping_add(VECTOR_Z) as *const u32).read_unaligned() });
    let x_squared = vector_mul(x, x);
    let y_squared = vector_mul(y, y);
    let xy_squared = vector_add(x_squared, y_squared);
    let z_squared = vector_mul(z, z);
    let squared_length = vector_add(xy_squared, z_squared);

    let mask_bits = unsafe { lf_checker_rt::global::<u32>(MASK_WORD).read_unaligned() };
    let z_threshold = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(Z_LENGTH_THRESHOLD).read_unaligned()
    });
    let y_threshold = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(Y_LENGTH_THRESHOLD).read_unaligned()
    });
    let x_threshold = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(X_LENGTH_THRESHOLD).read_unaligned()
    });
    let mask_z = if squared_length > z_threshold { mask_bits } else { 0 };
    let mask_y = if squared_length > y_threshold { mask_bits } else { 0 };
    let mask_x = if squared_length > x_threshold { mask_bits } else { 0 };

    let numerator = f32::from_bits(unsafe {
        lf_checker_rt::global::<u32>(NORMALIZATION_NUMERATOR).read_unaligned()
    });
    let length = squared_length.sqrt();
    let scale_for_yz = vector_div(numerator, length);
    let scale_for_x = vector_div(numerator, length);
    let normalized_x = vector_mul(scale_for_x, x).to_bits();
    let normalized_y = vector_mul(scale_for_yz, y).to_bits();
    let normalized_z = vector_mul(scale_for_yz, z).to_bits();

    let output_x = (!mask_x & source_words[SOURCE_X]) | (normalized_x & mask_x);
    let output_y = (!mask_y & source_words[SOURCE_Y]) | (normalized_y & mask_y);
    let output_z = (!mask_z & source_words[SOURCE_Z]) | (normalized_z & mask_z);
    unsafe {
        (this.wrapping_add(VECTOR_X) as *mut u32).write_unaligned(output_x);
        (this.wrapping_add(VECTOR_Y) as *mut u32).write_unaligned(output_y);
        (this.wrapping_add(VECTOR_Z) as *mut u32).write_unaligned(output_z);
    }
    source
});
