// original: 0x009C0D80 ped_task_check_nearby_event
/// Scans 32 task records for events associated with `target_vector`. Events
/// with a low type nibble of at least 2 and a matching vector pointer are
/// compared by squared Euclidean distance against 0.25. The three single
/// precision differences are squared and added in y, x, z order. A distance
/// strictly below the threshold calls the scripted proximity helper with the
/// task-record base, option 1, descriptor address, and zero. The routine has
/// no semantic return value.
lf_checker_rt::export!(cdecl, rw_009c0d80(target_vector: u32) -> () {
    const TABLE_FIRST_EVENT_VA: u32 = 0x0129_2460;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_COUNT: u32 = 32;
    const EVENT_TYPE_OFFSET: u32 = 0x1E2;
    const EVENT_VECTOR_ID_OFFSET: u32 = 0x1BC;
    const EVENT_COORDINATE_POINTER_OFFSET: u32 = 0x20;
    const INLINE_COORDINATES_OFFSET: u32 = 0x10;
    const EXTERNAL_COORDINATES_OFFSET: u32 = 0x30;
    const NEARBY_DISTANCE_SQUARED: f32 = f32::from_bits(0x3E80_0000);
    const PROXIMITY_DESCRIPTOR_VA: u32 = 0x0110_DB70;

    #[inline(never)]
    fn subtract_single(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(never)]
    fn multiply_single(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(never)]
    fn add_single(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    for record_index in 0..RECORD_COUNT {
        let event_slot = TABLE_FIRST_EVENT_VA.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
        let event = unsafe { lf_checker_rt::global::<u32>(event_slot).read() };
        if event == 0 {
            continue;
        }

        let event_type = unsafe { ((event + EVENT_TYPE_OFFSET) as *const u8).read() } & 0x0F;
        let event_vector_id = if event_type >= 2 {
            unsafe { ((event + EVENT_VECTOR_ID_OFFSET) as *const u32).read() }
        } else {
            0
        };
        if event_vector_id != target_vector {
            continue;
        }

        let coordinate_pointer = unsafe {
            ((event + EVENT_COORDINATE_POINTER_OFFSET) as *const u32).read()
        };
        let coordinates = if coordinate_pointer == 0 {
            event.wrapping_add(INLINE_COORDINATES_OFFSET)
        } else {
            coordinate_pointer.wrapping_add(EXTERNAL_COORDINATES_OFFSET)
        };
        let event_x = unsafe { ((coordinates) as *const f32).read() };
        let event_y = unsafe { ((coordinates + 4) as *const f32).read() };
        let event_z = unsafe { ((coordinates + 8) as *const f32).read() };
        let target_x = unsafe { ((target_vector) as *const f32).read() };
        let target_y = unsafe { ((target_vector + 4) as *const f32).read() };
        let target_z = unsafe { ((target_vector + 8) as *const f32).read() };

        let y_delta = subtract_single(event_y, target_y);
        let x_delta = subtract_single(event_x, target_x);
        let z_delta = subtract_single(event_z, target_z);
        let y_squared = multiply_single(y_delta, y_delta);
        let x_squared = multiply_single(x_delta, x_delta);
        let z_squared = multiply_single(z_delta, z_delta);
        let xy_squared = add_single(y_squared, x_squared);
        let distance_squared = add_single(xy_squared, z_squared);
        if !(NEARBY_DISTANCE_SQUARED > distance_squared) {
            continue;
        }

        let record_base = event_slot.wrapping_sub(0x10);
        let _ = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            lf_checker_rt::relocated(record_base),
            1u32,
            lf_checker_rt::relocated(PROXIMITY_DESCRIPTOR_VA),
            0u32,
        );
    }
});
