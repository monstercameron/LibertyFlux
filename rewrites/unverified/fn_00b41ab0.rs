// original: 0x00B41AB0 task_spatial_list_intersects

/// Test a scaled sphere bounds against the receiver's linked spatial nodes.
///
/// The receiver stores the node head at +0xC50. Each node stores its next
/// pointer at +0x250, signed 16-bit minima and maxima at +0x260..+0x26A, and
/// flags at +0x26C. The point argument addresses three f32 coordinates; the
/// second stack argument is an f32 radius. Each low/high coordinate is
/// computed in source order, scaled, truncated toward zero like CVTTSS2SI,
/// then compared using its low signed 16 bits. Two intercepted helpers
/// bracket a two-word temporary object on the caller's stack.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00B41AB0(
    this_ptr: u32,
    point_ptr: u32,
    radius_bits: u32,
) -> u32 {
    const NODE_HEAD: u32 = 0xC50;
    const NODE_NEXT: u32 = 0x250;
    const NODE_MIN_X: u32 = 0x260;
    const NODE_MAX_X: u32 = 0x262;
    const NODE_MIN_Y: u32 = 0x264;
    const NODE_MAX_Y: u32 = 0x266;
    const NODE_MIN_Z: u32 = 0x268;
    const NODE_MAX_Z: u32 = 0x26A;
    const NODE_FLAGS: u32 = 0x26C;
    const SKIP_FILTER: u32 = 0x0179FD49;
    const SCALE: u32 = 0x00FE8AFC;
    const HELPER_CREATE: u32 = 1;
    const HELPER_DESTROY: u32 = 2;
    const HELPER_ARGUMENT: u32 = 0x016C6710;

    let read_word = |base: u32, offset: u32| -> u32 {
        // SAFETY: the checker contract maps the receiver, point and node data.
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    };
    let read_i16 = |base: u32, offset: u32| -> i16 {
        // SAFETY: node bounds are 16-bit fields at the documented offsets.
        unsafe { (base.wrapping_add(offset) as *const i16).read_unaligned() }
    };
    let add = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    };
    let sub = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    };
    let mul = |left: f32, right: f32| -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    };
    let cvttss2si = |value: f32| -> i32 {
        if !value.is_finite() || value < -2_147_483_648.0 || value >= 2_147_483_648.0 {
            i32::MIN
        } else {
            // The preceding finite-range guard bounds this truncation to signed 32-bit values.
            value as i32
        }
    };

    let radius = f32::from_bits(radius_bits);
    let point_x = f32::from_bits(read_word(point_ptr, 0));
    let point_y = f32::from_bits(read_word(point_ptr, 4));
    let point_z = f32::from_bits(read_word(point_ptr, 8));
    let scale_pointer = lf_checker_rt::global::<u32>(SCALE) as u32;
    let scale = f32::from_bits(read_word(scale_pointer, 0));
    let bounds = [
        cvttss2si(mul(sub(point_x, radius), scale)),
        cvttss2si(mul(add(point_x, radius), scale)),
        cvttss2si(mul(sub(point_y, radius), scale)),
        cvttss2si(mul(add(point_y, radius), scale)),
        cvttss2si(mul(sub(point_z, radius), scale)),
        cvttss2si(mul(add(point_z, radius), scale)),
    ];

    let mut helper_object = [0_u32; 2];
    let helper_object_ptr = helper_object.as_mut_ptr() as u32;
    let helper_argument = lf_checker_rt::global::<u32>(HELPER_ARGUMENT) as u32;
    let _ = lf_checker_rt::callee_thiscall!(
        HELPER_CREATE,
        u32,
        helper_object_ptr,
        helper_argument,
    );

    let skip_filtered = unsafe {
        (lf_checker_rt::global::<u8>(SKIP_FILTER) as *const u8).read_unaligned()
    } != 0;
    let mut node = read_word(this_ptr, NODE_HEAD);
    let mut intersects = false;
    while node != 0 {
        let flags = read_word(node, NODE_FLAGS);
        let filtered = skip_filtered && flags & 0x800 != 0;
        let overlaps = (bounds[0] as i16) <= read_i16(node, NODE_MAX_X)
            && (bounds[2] as i16) <= read_i16(node, NODE_MAX_Y)
            && (bounds[4] as i16) <= read_i16(node, NODE_MAX_Z)
            && (bounds[1] as i16) >= read_i16(node, NODE_MIN_X)
            && (bounds[3] as i16) >= read_i16(node, NODE_MIN_Y)
            && (bounds[5] as i16) >= read_i16(node, NODE_MIN_Z);
        if !filtered && flags & 1 != 0 && overlaps {
            intersects = true;
            break;
        }
        node = read_word(node, NODE_NEXT);
    }

    let _ = lf_checker_rt::callee_thiscall!(HELPER_DESTROY, u32, helper_object_ptr);
    u32::from(intersects)
}
