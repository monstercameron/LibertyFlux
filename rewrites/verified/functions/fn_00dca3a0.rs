// original: 0x00dca3a0 CTaskComplexWalkRoundEntity::vf9

/// Update a task position when the squared distance to the supplied point is
/// at least the global squared-distance threshold. The object stores its
/// position at `+0x0c..+0x14`, the copied fourth point word at `+0x18`, and a
/// state bit at `+0x88`. The first and third stack words are unused. This is
/// a thiscall routine with three stack words and callee cleanup of 12 bytes.
lf_checker_rt::export!(thiscall, rw_00dca3a0(this_ptr: u32, _unused_first: u32, point_ptr: u32, _unused_third: u32) -> () {
    unsafe {
        const POSITION_X: u32 = 0x0c;
        const POSITION_Y: u32 = 0x10;
        const POSITION_Z: u32 = 0x14;
        const POSITION_W: u32 = 0x18;
        const STATE_FLAGS: u32 = 0x88;
        const UPDATED_FLAG: u32 = 1;
        const DISTANCE_LIMIT_VA: u32 = 0x00fe8778;
        let point_x = f32::from_bits((point_ptr as *const u32).read_unaligned());
        let point_y = f32::from_bits((point_ptr.wrapping_add(4) as *const u32).read_unaligned());
        let point_z = f32::from_bits((point_ptr.wrapping_add(8) as *const u32).read_unaligned());
        let object_x = f32::from_bits((this_ptr.wrapping_add(POSITION_X) as *const u32).read_unaligned());
        let object_y = f32::from_bits((this_ptr.wrapping_add(POSITION_Y) as *const u32).read_unaligned());
        let object_z = f32::from_bits((this_ptr.wrapping_add(POSITION_Z) as *const u32).read_unaligned());
        let dx = core::hint::black_box(point_x) - core::hint::black_box(object_x);
        let dy = core::hint::black_box(point_y) - core::hint::black_box(object_y);
        let dz = core::hint::black_box(point_z) - core::hint::black_box(object_z);
        let dx_squared = core::hint::black_box(dx) * core::hint::black_box(dx);
        let dy_squared = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dz_squared = core::hint::black_box(dz) * core::hint::black_box(dz);
        let xy_squared = core::hint::black_box(dx_squared) + core::hint::black_box(dy_squared);
        let distance_squared = core::hint::black_box(xy_squared) + core::hint::black_box(dz_squared);
        let limit = f32::from_bits((lf_checker_rt::relocated(DISTANCE_LIMIT_VA) as *const u32).read_unaligned());
        if limit > distance_squared { return; }
        for (offset, value) in [(POSITION_X, point_x), (POSITION_Y, point_y), (POSITION_Z, point_z)] {
            (this_ptr.wrapping_add(offset) as *mut u32).write_unaligned(value.to_bits());
        }
        let point_w = (point_ptr.wrapping_add(12) as *const u32).read_unaligned();
        (this_ptr.wrapping_add(POSITION_W) as *mut u32).write_unaligned(point_w);
        let flags = (this_ptr.wrapping_add(STATE_FLAGS) as *const u32).read_unaligned();
        (this_ptr.wrapping_add(STATE_FLAGS) as *mut u32).write_unaligned(flags | UPDATED_FLAG);
    }
});
