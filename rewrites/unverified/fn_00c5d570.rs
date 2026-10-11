// original: 0x00C5D570 CTaskComplexDestroyCarArmed::compute_attack_geometry

/// Copies two four-word vectors into the task object, stores their XYZ
/// difference and length, and derives two weapon-scale fields from the
/// selected weapon record. `this` is the destination object; the one cdecl
/// stack word is a context pointer. The object at `this+0x18` points to a
/// second vector source at `+0x20`; the context points to the first source
/// at `+0x20` and holds a counted, twelve-byte-stride weapon list at `+0x2b0`.
/// The weapon lookup is an intercepted cdecl call. The original also copies
/// the unwritten aligned stack slot at `+0x1c` to `this+0x4c`; this rewrite
/// uses the contract's defined zero fill for that slot. Float arithmetic is
/// single precision and keeps the original operand order.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_c5d570(this_address: u32, context_address: u32) -> u32 {
    const VECTOR_POINTER: u32 = 0x20;
    const POSITION: u32 = 0x30;
    const WEAPON_LIST: u32 = 0x2b0;
    const WEAPON_STRIDE_WORDS: u32 = 3;
    const WEAPON_LOOKAHEAD: u32 = 3;
    const WEAPON_VALUE: u32 = 0x18;
    const SCALE: f32 = f32::from_bits(0x3f00_0000);
    const MINIMUM: f32 = f32::from_bits(0x3f4c_cccd);
    const CLAMP_FACTOR: f32 = f32::from_bits(0x3f66_6666);
    const SCALE_FACTOR: f32 = f32::from_bits(0x3f8c_cccd);
    const VECTOR_WORD_OFFSETS: [u32; 4] = [0, 4, 8, 12];

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) };
    }
    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }
    #[inline(always)]
    unsafe fn write_f32(address: u32, value: f32) {
        unsafe { write_u32(address, value.to_bits()) };
    }
    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn sub(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    unsafe {
        let first_source = read_u32(context_address + VECTOR_POINTER);
        let path_node = read_u32(this_address + 0x18);
        let second_source = read_u32(path_node + VECTOR_POINTER);

        for word_offset in VECTOR_WORD_OFFSETS {
            write_u32(
                this_address + POSITION + word_offset - 0x10,
                read_u32(first_source + POSITION + word_offset),
            );
            write_u32(
                this_address + POSITION + word_offset,
                read_u32(second_source + POSITION + word_offset),
            );
        }

        let first_x = read_f32(this_address + POSITION - 0x10);
        let first_y = read_f32(this_address + POSITION - 0x0c);
        let first_z = read_f32(this_address + POSITION - 0x08);
        let second_x = read_f32(this_address + POSITION);
        let second_y = read_f32(this_address + POSITION + 4);
        let second_z = read_f32(this_address + POSITION + 8);
        let delta_x = sub(second_x, first_x);
        let delta_y = sub(second_y, first_y);
        let delta_z = sub(second_z, first_z);
        write_f32(this_address + 0x40, delta_x);
        write_f32(this_address + 0x44, delta_y);
        write_f32(this_address + 0x48, delta_z);

        // The original reads this word from an unwritten aligned stack slot.
        write_u32(this_address + 0x4c, 0);

        let squared_length = add(
            add(mul(delta_x, delta_x), mul(delta_y, delta_y)),
            mul(delta_z, delta_z),
        );
        write_f32(this_address + 0x60, core::hint::black_box(squared_length).sqrt());

        let count = read_u32(context_address + WEAPON_LIST);
        let selected_record = count
            .wrapping_add(WEAPON_LOOKAHEAD)
            .wrapping_mul(WEAPON_STRIDE_WORDS);
        let selected_pointer_address = context_address
            .wrapping_add(WEAPON_LIST)
            .wrapping_add(selected_record.wrapping_mul(4));
        let weapon_reference = read_u32(selected_pointer_address);
        let weapon_info = lf_checker_rt::callee_cdecl!(1, u32, weapon_reference);
        let weapon_value = read_f32(weapon_info.wrapping_add(WEAPON_VALUE));
        let half_scale = mul(weapon_value, SCALE);
        write_f32(this_address + 0x50, half_scale);
        let damped_scale = mul(half_scale, CLAMP_FACTOR);
        let scale_limit = if MINIMUM > damped_scale {
            MINIMUM
        } else {
            damped_scale
        };
        write_f32(this_address + 0x58, scale_limit);
        write_f32(this_address + 0x50, mul(half_scale, SCALE_FACTOR));
        weapon_info
    }
}
