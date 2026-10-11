// original: 0x00A6CB30 refresh_blast_stream_descriptor

/// Refresh up to eight blast-stream slots owned by `items`.
///
/// `items` is the ECX receiver and points to eight 32-bit object pointers.
/// `target_position` points to four consecutive `f32` values. The remaining
/// stack arguments are `factor`, `distance_limit`, and `scale`. For each
/// non-null slot, the function asks the object's virtual position method for
/// three coordinates, obtains its radius as an x87 `f32`, and compares the
/// distance from the target with `distance_limit`. A passing
/// item is submitted to the global event queue with its position, limit,
/// clamped blend factor, and an enabled byte. When the global removal flag is
/// zero, the item is also detached and its slot is cleared. The method is
/// thiscall and consumes four stack arguments; its return value is void.
///
/// Float operations preserve the original single-precision operand order.
/// Unordered distance comparisons take the original `jbe` path and skip the
/// event. The event record's unwritten padding is zero under the contract's
/// defined stack fill.
lf_checker_rt::export!(thiscall, rw_00A6CB30(
    items: u32,
    target_position: u32,
    factor_bits: u32,
    distance_limit_bits: u32,
    scale_bits: u32,
) -> () {
    unsafe {
        const SLOT_COUNT: u32 = 8;
        const TARGET_X: u32 = 0x00;
        const TARGET_Y: u32 = 0x04;
        const TARGET_Z: u32 = 0x08;
        const TARGET_W: u32 = 0x0c;
        const ITEM_VTABLE: u32 = 0x00;
        const VTABLE_POSITION: u32 = 0x50;
        const VTABLE_RADIUS: u32 = 0x58;
        const EVENT_ITEM: usize = 0;
        const EVENT_TARGET: usize = 4;
        const EVENT_DISTANCE_LIMIT: usize = 8;
        const EVENT_FACTOR: usize = 9;
        const EVENT_ENABLED: usize = 10;
        const EVENT_WORDS: usize = 11;
        const EVENT_QUEUE_VA: u32 = 0x0130_5d30;
        const REMOVAL_FLAG_VA: u32 = 0x0103_cde0;
        const RATE_VA: u32 = 0x00e9_ddd0;
        const ADD_EVENT_CALLEE: u32 = 3;
        const REMOVE_NODE_CALLEE: u32 = 4;
        const ONE: f32 = 1.0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let target_x = read_f32(target_position + TARGET_X);
        let target_y = read_f32(target_position + TARGET_Y);
        let target_z = read_f32(target_position + TARGET_Z);
        let target_w = read_u32(target_position + TARGET_W);
        let factor = f32::from_bits(factor_bits);
        let distance_limit = f32::from_bits(distance_limit_bits);
        let scale = f32::from_bits(scale_bits);
        let global_rate = read_f32(lf_checker_rt::relocated(RATE_VA));
        let removal_flag = lf_checker_rt::global::<u8>(REMOVAL_FLAG_VA);
        let event_queue = lf_checker_rt::relocated(EVENT_QUEUE_VA);
        let mut position = [0u32; 3];
        let position_out = position.as_mut_ptr() as u32;

        for index in 0..SLOT_COUNT {
            let slot = items.wrapping_add(index * 4);
            let item = read_u32(slot);
            if item == 0 {
                continue;
            }

            let object_vtable = read_u32(item + ITEM_VTABLE);
            let position_method = read_u32(object_vtable + VTABLE_POSITION);
            let position_getter: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(position_method as usize);
            let _ = position_getter(item, position_out);

            let dx = sub_f32(f32::from_bits(position[0]), target_x);
            let dy = sub_f32(f32::from_bits(position[1]), target_y);
            let dz = sub_f32(f32::from_bits(position[2]), target_z);
            let dy_squared = mul_f32(dy, dy);
            let dx_squared = mul_f32(dx, dx);
            let dz_squared = mul_f32(dz, dz);
            let distance_squared = add_f32(add_f32(dy_squared, dx_squared), dz_squared);

            let radius_method = read_u32(object_vtable + VTABLE_RADIUS);
            let radius: f32 = {
                let method: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(radius_method as usize);
                method(item)
            };
            let separation = sub_f32(core::hint::black_box(distance_squared).sqrt(), radius);
            if distance_limit <= separation || distance_limit.is_nan() || separation.is_nan() {
                continue;
            }

            let product = mul_f32(mul_f32(factor, scale), global_rate);
            let blend_factor = if product > ONE { ONE } else { product };

            let mut event = [0u32; EVENT_WORDS];
            event[EVENT_ITEM] = item;
            event[EVENT_TARGET] = target_x.to_bits();
            event[EVENT_TARGET + 1] = target_y.to_bits();
            event[EVENT_TARGET + 2] = target_z.to_bits();
            event[EVENT_TARGET + 3] = target_w;
            event[EVENT_DISTANCE_LIMIT] = distance_limit.to_bits();
            event[EVENT_FACTOR] = blend_factor.to_bits();
            event[EVENT_ENABLED] = 1;
            let event_ptr = event.as_mut_ptr() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                ADD_EVENT_CALLEE,
                u32,
                event_queue,
                event_ptr,
                0,
            );

            if removal_flag.read() == 0 {
                let current = read_u32(slot);
                if current != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        REMOVE_NODE_CALLEE,
                        u32,
                        current,
                        slot,
                    );
                    (slot as *mut u32).write_unaligned(0);
                }
            }
        }
    }
});
