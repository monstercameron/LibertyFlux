// original: 0x00B40EC0 task_geometry_record_init

/// Initialize the task geometry record at `this` from a descriptor.
///
/// This is a 32-bit thiscall routine with one callee-cleaned descriptor
/// pointer. The descriptor contains a nested point pointer at `+0x20` and
/// mode bits at `+0x28`; the point is read from nested `+0x30` when present,
/// otherwise the inline point starts at descriptor `+0x10`. Four words from
/// that point are copied to the record. The routine then calls its geometry
/// helpers, computes the record's center and two vertical limits, and writes
/// the two fixed axis vectors. The helper calls remain observable through the
/// checker runtime; their contract supplies their answers and effects.
///
/// Float operations retain the source order. Values at the documented
/// offsets are single-precision fields; mode tests use unsigned bit fields.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00B40EC0(this_ptr: u32, descriptor_ptr: u32) {
    const NESTED_POINT: u32 = 0x20;
    const MODE_BITS: u32 = 0x28;
    const INLINE_POINT: u32 = 0x10;
    const POINT_INNER: u32 = 0x30;
    const GEOMETRY_MODE: u32 = 2;
    const LOWER_LIMIT_MODE: u32 = 0x80;
    const MODE_MASK: u32 = 0x3C0;
    const COOKIE_VALUE: u32 = 0x01057FB4;
    const HALF_VALUE: u32 = 0x00FE87E4;
    const GEOMETRY_SCALE: u32 = 0x00FE880C;
    const LOWER_LIMIT_SCALE: u32 = 0x00FE8830;
    const CENTER_SCALE: u32 = 0x00FE8D1C;

    const CALLEE_TASK_GEOMETRY: u32 = 1;
    const CALLEE_VOLUME_BUILD: u32 = 2;
    const CALLEE_COPY_AXIS: u32 = 3;
    const CALLEE_SET_EXTENTS: u32 = 4;
    const CALLEE_COPY_VOLUME: u32 = 5;
    const CALLEE_COPY_NORMAL: u32 = 6;
    const CALLEE_COOKIE_GUARD: u32 = 7;

    const FRAME_BYTES: u32 = 0x118;
    const FRAME_CONTEXT: u32 = 0x28;
    const FRAME_LOCAL_POINT: u32 = 0xE8;
    const FRAME_COPY_VOLUME: u32 = 0xA8;
    const FRAME_COPY_NORMAL: u32 = 0x08;

    let read_word = |base: u32, offset: u32| -> u32 {
        // SAFETY: every accessed object and local buffer is supplied by the
        // checker contract, and the offsets are within the described layouts.
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    };
    let write_word = |base: u32, offset: u32, value: u32| {
        // SAFETY: output fields are within the task geometry record or the
        // local scratch buffer owned by this call.
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    };
    let read_float = |base: u32, offset: u32| -> f32 {
        f32::from_bits(read_word(base, offset))
    };
    let write_float = |base: u32, offset: u32, value: f32| {
        write_word(base, offset, value.to_bits())
    };
    let add_float = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    };
    let sub_float = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    };
    let mul_float = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    };

    let mut local_frame = [0_u32; (FRAME_BYTES / 4) as usize];
    let frame_ptr = local_frame.as_mut_ptr() as u32;

    let nested_point = read_word(descriptor_ptr, NESTED_POINT);
    let point_ptr = if nested_point != 0 {
        nested_point.wrapping_add(POINT_INNER)
    } else {
        descriptor_ptr.wrapping_add(INLINE_POINT)
    };
    for offset in [0_u32, 4, 8, 12] {
        write_word(this_ptr, offset, read_word(point_ptr, offset));
    }

    let mode_word = read_word(descriptor_ptr, MODE_BITS);
    let mode = (mode_word >> 6) & 0xF;
    let selected_scale = if mode == GEOMETRY_MODE {
        read_word(lf_checker_rt::global::<u32>(GEOMETRY_SCALE) as u32, 0)
    } else {
        read_word(lf_checker_rt::global::<u32>(HALF_VALUE) as u32, 0)
    };
    write_word(frame_ptr, 4, selected_scale);

    let geometry_result = if mode == GEOMETRY_MODE {
        lf_checker_rt::callee_thiscall!(
            CALLEE_TASK_GEOMETRY,
            u32,
            this_ptr,
            descriptor_ptr,
            frame_ptr.wrapping_add(FRAME_LOCAL_POINT),
        )
    } else {
        0
    };

    let context_ptr = frame_ptr.wrapping_add(FRAME_CONTEXT);
    let point_height = read_word(this_ptr, 8);
    lf_checker_rt::callee_thiscall!(
        CALLEE_VOLUME_BUILD,
        u32,
        context_ptr,
        descriptor_ptr,
        point_height,
        selected_scale,
        1,
        0,
        geometry_result,
        frame_ptr.wrapping_add(FRAME_LOCAL_POINT),
        0,
        0,
    );

    lf_checker_rt::callee_stdcall!(
        CALLEE_COPY_AXIS,
        u32,
        this_ptr.wrapping_add(0x40),
        this_ptr.wrapping_add(0xE0),
    );
    lf_checker_rt::callee_thiscall!(
        CALLEE_SET_EXTENTS,
        u32,
        context_ptr,
        this_ptr.wrapping_add(0xA0),
        this_ptr.wrapping_add(0xF8),
    );
    lf_checker_rt::callee_thiscall!(
        CALLEE_COPY_VOLUME,
        u32,
        context_ptr,
        frame_ptr.wrapping_add(FRAME_COPY_VOLUME),
    );

    let half = f32::from_bits(read_word(
        lf_checker_rt::global::<u32>(HALF_VALUE) as u32,
        0,
    ));
    let sum_x = add_float(read_float(frame_ptr, 0xB8), read_float(frame_ptr, 0xA8));
    let sum_y = add_float(read_float(frame_ptr, 0xBC), read_float(frame_ptr, 0xAC));
    let sum_z = add_float(read_float(frame_ptr, 0xC0), read_float(frame_ptr, 0xB0));
    let near_z = add_float(read_float(frame_ptr, 0xC8), sum_x);
    let near_y = add_float(read_float(frame_ptr, 0xCC), sum_y);
    let near_x = add_float(read_float(frame_ptr, 0xD0), sum_z);
    let far_z = add_float(read_float(frame_ptr, 0xD8), near_z);
    let far_y = add_float(read_float(frame_ptr, 0xDC), near_y);
    let far_x = add_float(read_float(frame_ptr, 0xE0), near_x);

    write_float(this_ptr, 0x10, mul_float(far_z, half));
    write_float(this_ptr, 0x14, mul_float(far_y, half));
    write_float(this_ptr, 0x18, mul_float(far_x, half));
    write_float(this_ptr, 0x1C, read_float(frame_ptr, 0xF4));

    lf_checker_rt::callee_thiscall!(
        CALLEE_COPY_NORMAL,
        u32,
        context_ptr,
        frame_ptr.wrapping_add(FRAME_COPY_NORMAL),
    );

    let normal_offset = read_word(frame_ptr, 0x18);
    write_float(this_ptr, 0x108, f32::from_bits(normal_offset));
    let lower_limit_scale = if mode_word & MODE_MASK == LOWER_LIMIT_MODE {
        read_float(
            lf_checker_rt::global::<u32>(LOWER_LIMIT_SCALE) as u32,
            0,
        )
    } else {
        half
    };
    let lower_limit = read_float(frame_ptr, 0x98);
    let upper_limit = read_float(frame_ptr, 0x9C);
    write_float(this_ptr, 0x110, lower_limit);
    write_float(this_ptr, 0x10C, sub_float(upper_limit, lower_limit_scale));

    write_word(this_ptr, 0x80, 0);
    write_word(this_ptr, 0x84, 0);
    write_word(this_ptr, 0x88, 0x3F800000);
    write_float(this_ptr, 0x8C, read_float(frame_ptr, 0xF4));

    let center = mul_float(
        add_float(read_float(this_ptr, 0x14), read_float(this_ptr, 0x10)),
        f32::from_bits(read_word(
            lf_checker_rt::global::<u32>(CENTER_SCALE) as u32,
            0,
        )),
    );
    write_float(this_ptr, 0xF0, sub_float(center, read_float(this_ptr, 0x110)));

    write_float(this_ptr, 0x9C, read_float(frame_ptr, 0xF4));
    write_word(this_ptr, 0x90, 0);
    write_word(this_ptr, 0x94, 0);
    write_word(this_ptr, 0x98, 0xBF800000);
    let upper_center = mul_float(
        add_float(read_float(this_ptr, 0x14), read_float(this_ptr, 0x10)),
        f32::from_bits(read_word(
            lf_checker_rt::global::<u32>(CENTER_SCALE) as u32,
            0,
        )),
    );
    write_float(this_ptr, 0xF4, add_float(upper_center, read_float(this_ptr, 0x10C)));

    let cookie = read_word(
        lf_checker_rt::global::<u32>(COOKIE_VALUE) as u32,
        0,
    );
    lf_checker_rt::callee_thiscall!(CALLEE_COOKIE_GUARD, (), cookie);
}
