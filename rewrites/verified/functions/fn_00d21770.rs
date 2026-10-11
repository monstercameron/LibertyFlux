// original: 0x00D21770 entity_fx_position_fraction
/// Combines an entity position with a sample vector, takes the absolute value,
/// divides by the supplied scale, then returns the fractional remainder after
/// a signed truncating conversion. The entity flag selects its 16-bit index
/// path; otherwise its optional transform or inline position supplies the base.
/// All component additions and the division preserve the original f32 order.
lf_checker_rt::export!(stdcall, rw_00d21770(entity: u32, sample: u32, scale_bits: u32) -> f32 {
    const ENTITY_FLAGS: u32 = 0x24;
    const TRANSFORM_POINTER: u32 = 0x20;
    const INDEX_OFFSET: u32 = 0x2c;
    const INLINE_POSITION: u32 = 0x10;
    const TRANSFORM_POSITION: u32 = 0x30;
    const ABSOLUTE_VALUE_MASK: u32 = 0x7fff_ffff;

    #[inline(never)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(never)]
    fn div_f32(numerator: f32, denominator: f32) -> f32 {
        core::hint::black_box(numerator) / core::hint::black_box(denominator)
    }
    #[inline(never)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(never)]
    fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(never)]
    fn read_f32(address: u32) -> f32 { f32::from_bits(read_u32(address)) }
    #[inline(never)]
    fn cvttss2si(value: f32) -> i32 {
        // Mirrors x86 CVTTSS2SI: NaN, infinity, values >= 2^31, and values < -2^31 return i32::MIN.
        const I32_UPPER_EXCLUSIVE: f32 = 2_147_483_648.0;
        const I32_LOWER_INCLUSIVE: f32 = -2_147_483_648.0;
        if !value.is_finite() || value >= I32_UPPER_EXCLUSIVE || value < I32_LOWER_INCLUSIVE {
            i32::MIN
        } else {
            value.trunc() as i32
        }
    }

    let sample_x = read_f32(sample.wrapping_add(4));
    let sample_y = read_f32(sample.wrapping_add(8));
    let sample_z = read_f32(sample.wrapping_add(12));
    let flags = read_u32(entity.wrapping_add(ENTITY_FLAGS));

    let sum = if (flags >> 25) & 1 != 0 {
        let index = unsafe { (entity.wrapping_add(INDEX_OFFSET) as *const u16).read_unaligned() };
        let with_x = add_f32(index as f32, sample_x);
        let with_y = add_f32(with_x, sample_y);
        add_f32(with_y, sample_z)
    } else {
        let transform = read_u32(entity.wrapping_add(TRANSFORM_POINTER));
        let base = if transform == 0 {
            entity.wrapping_add(INLINE_POSITION)
        } else {
            transform.wrapping_add(TRANSFORM_POSITION)
        };
        let y = add_f32(read_f32(base.wrapping_add(4)), sample_y);
        let x = add_f32(read_f32(base), sample_x);
        let xy = add_f32(y, x);
        let z = add_f32(read_f32(base.wrapping_add(8)), sample_z);
        add_f32(xy, z)
    };

    let magnitude = f32::from_bits(sum.to_bits() & ABSOLUTE_VALUE_MASK);
    let ratio = div_f32(magnitude, f32::from_bits(scale_bits));
    let whole = cvttss2si(ratio);
    let whole_as_f32 = whole as f32;
    sub_f32(ratio, whole_as_f32)
});
