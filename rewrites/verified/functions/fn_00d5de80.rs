// original: 0x00D5DE80 prt_sys_entity_build_bounds

/// Produce an eight-float bounds record from either helper output or entity data.
///
/// `this` holds a pointer at byte offset `0x70` to three source coordinates at
/// offsets `0xd0`, `0xd4`, and `0xd8`. The output pointer is the single stack
/// argument and receives eight `f32` values: three source-minus-origin values,
/// the helper's fourth word, three origin-plus-source values, and that fourth
/// word again. A direct helper receives the data pointer in ECX and a pointer
/// to four temporary words. Its low return byte selects helper coordinates;
/// otherwise the coordinates come from the entity data and an image constant.
/// The mask and constant are read from relocated image data. The call's stack
/// pointer argument is skipped in the call log and its four pointed-to words
/// are snapshotted instead. The method returns the output pointer and uses the
/// 32-bit MSVC thiscall convention.
lf_checker_rt::export!(thiscall, rw_prt_sys_entity_build_bounds(
    this_ptr: u32,
    output_ptr: u32
) -> u32 {
    const DATA_POINTER_OFFSET: u32 = 0x70;
    const FIRST_COORDINATE_OFFSET: u32 = 0xD0;
    const SECOND_COORDINATE_OFFSET: u32 = 0xD4;
    const THIRD_COORDINATE_OFFSET: u32 = 0xD8;
    const ORIGIN_CONSTANT_VA: u32 = 0x00FE879C;
    const ORIGIN_MASK_VA: u32 = 0x00FE8F80;
    const BOUNDS_HELPER: u32 = 1;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_word(address) })
    }

    #[inline(always)]
    unsafe fn write_f32(address: u32, value: f32) {
        unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
    }

    #[inline(always)]
    fn add_in_source_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    #[inline(always)]
    fn subtract_in_source_order(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    let data_ptr = unsafe { read_word(this_ptr.wrapping_add(DATA_POINTER_OFFSET)) };
    let mut helper_words = [0u32; 4];
    let helper_used = lf_checker_rt::callee_thiscall!(
        BOUNDS_HELPER,
        u8,
        data_ptr,
        helper_words.as_mut_ptr() as u32
    ) != 0;

    let mask = unsafe { lf_checker_rt::global::<u32>(ORIGIN_MASK_VA).read_unaligned() };
    let (source_x, source_y, source_z, origin_bits) = if helper_used {
        (
            f32::from_bits(helper_words[0]),
            f32::from_bits(helper_words[1]),
            f32::from_bits(helper_words[2]),
            helper_words[3],
        )
    } else {
        (
            unsafe { read_f32(data_ptr.wrapping_add(FIRST_COORDINATE_OFFSET)) },
            unsafe { read_f32(data_ptr.wrapping_add(SECOND_COORDINATE_OFFSET)) },
            unsafe { read_f32(data_ptr.wrapping_add(THIRD_COORDINATE_OFFSET)) },
            unsafe { lf_checker_rt::global::<u32>(ORIGIN_CONSTANT_VA).read_unaligned() },
        )
    };
    let origin = f32::from_bits(origin_bits & mask);
    let fourth_word = f32::from_bits(helper_words[3]);

    let bounds = [
        subtract_in_source_order(source_x, origin),
        subtract_in_source_order(source_y, origin),
        subtract_in_source_order(source_z, origin),
        fourth_word,
        add_in_source_order(origin, source_x),
        add_in_source_order(origin, source_y),
        add_in_source_order(origin, source_z),
        fourth_word,
    ];
    for (index, value) in bounds.into_iter().enumerate() {
        unsafe { write_f32(output_ptr.wrapping_add((index as u32) * 4), value) };
    }
    output_ptr
});
