// original: 0x00D3EA20 task_simple_sideways_dive_update
/// Compares the task's height against an actor-relative height and selects a
/// scripted scale from one of two vtable queries. The upper path multiplies
/// the queried scale by the global launch factor and notifies the update
/// manager when the result is strictly above its limit; the lower path scales
/// the height difference by a separate global. The result is passed through
/// two temporary task records, then all three stack records are finalized.
/// The original reuses the incoming actor stack word as scratch for the
/// computed float; the value is observed in the checked record-constructor
/// call arguments. Float arithmetic keeps the original left-to-right order.
lf_checker_rt::export!(thiscall, rw_d3ea20(task: u32, actor: u32) -> () {
    const QUERY_SCALE_MODE: u32 = 1;
    const QUERY_ACTOR_SCALE: u32 = 2;
    const NOTIFY_UPDATE_MANAGER: u32 = 3;
    const BUILD_PRIMARY_RECORD: u32 = 4;
    const BUILD_SCALE_RECORD: u32 = 5;
    const BUILD_RESULT_RECORD: u32 = 6;
    const DESTROY_SCALE_RECORD: u32 = 7;
    const DESTROY_PRIMARY_RECORD: u32 = 8;
    const HEIGHT_OFFSET: usize = 0x34;
    const ACTOR_TRANSFORM_LINK: usize = 0x20;
    const TRANSFORM_HEIGHT: usize = 0x38;
    const MODE_THRESHOLD_A: u32 = 0x00ECA868;
    const MODE_THRESHOLD_B: u32 = 0x00ECA86C;
    const UPPER_SCALE: u32 = 0x00FE8BB0;
    const UPPER_LIMIT: u32 = 0x00FE8628;
    const LOWER_SCALE: u32 = 0x00FE8AB8;
    const REQUEST_TAG: u32 = 0x011735B4;
    const UPDATE_MANAGER: u32 = 0x0172BD20;
    const RECORD_KIND: u32 = 0x36;

    let mut primary_record = [0u32; 15];
    let mut scale_record = [0u32; 5];
    let mut result_record = [0u32; 4];
    let use_second_threshold = lf_checker_rt::callee_thiscall!(
        QUERY_SCALE_MODE, u8, actor
    ) != 0;
    let threshold_bits = unsafe {
        lf_checker_rt::global::<u32>(if use_second_threshold {
            MODE_THRESHOLD_B
        } else {
            MODE_THRESHOLD_A
        })
        .read_unaligned()
    };
    let threshold = f32::from_bits(threshold_bits);
    let task_height = f32::from_bits(unsafe {
        ((task + HEIGHT_OFFSET as u32) as *const u32).read_unaligned()
    });
    let actor_transform = unsafe {
        ((actor + ACTOR_TRANSFORM_LINK as u32) as *const u32).read_unaligned()
    };
    let actor_height = f32::from_bits(unsafe {
        ((actor_transform + TRANSFORM_HEIGHT as u32) as *const u32).read_unaligned()
    });
    let height_delta = core::hint::black_box(task_height)
        - core::hint::black_box(actor_height);

    let scaled_height = if height_delta > threshold {
        let actor_scale = lf_checker_rt::callee_thiscall!(
            QUERY_ACTOR_SCALE, f32, actor
        );
        let upper_factor = f32::from_bits(unsafe {
            lf_checker_rt::global::<u32>(UPPER_SCALE).read_unaligned()
        });
        let scaled = core::hint::black_box(actor_scale)
            * core::hint::black_box(upper_factor);
        let upper_limit = f32::from_bits(unsafe {
            lf_checker_rt::global::<u32>(UPPER_LIMIT).read_unaligned()
        });
        if scaled > upper_limit {
            let manager = lf_checker_rt::relocated(UPDATE_MANAGER);
            let _ = lf_checker_rt::callee_thiscall!(
                NOTIFY_UPDATE_MANAGER, u32, manager, actor
            );
        }
        scaled
    } else {
        let lower_factor = f32::from_bits(unsafe {
            lf_checker_rt::global::<u32>(LOWER_SCALE).read_unaligned()
        });
        core::hint::black_box(height_delta) * core::hint::black_box(lower_factor)
    };
    let scaled_bits = scaled_height.to_bits();
    let request_tag = unsafe {
        lf_checker_rt::global::<u32>(REQUEST_TAG).read_unaligned()
    };
    let primary_pointer = primary_record.as_mut_ptr() as u32;
    let scale_pointer = scale_record.as_mut_ptr() as u32;
    let result_pointer = result_record.as_mut_ptr() as u32;

    let _ = lf_checker_rt::callee_thiscall!(
        BUILD_PRIMARY_RECORD, u32, primary_pointer, 0, request_tag, RECORD_KIND
    );
    let _ = lf_checker_rt::callee_thiscall!(
        BUILD_SCALE_RECORD, u32, scale_pointer, 0, scaled_bits, RECORD_KIND, 0, 0
    );
    let _ = lf_checker_rt::callee_thiscall!(
        BUILD_RESULT_RECORD, u32, scale_pointer, actor, result_pointer
    );
    let _ = lf_checker_rt::callee_thiscall!(DESTROY_SCALE_RECORD, u32, scale_pointer);
    let _ = lf_checker_rt::callee_thiscall!(DESTROY_PRIMARY_RECORD, u32, primary_pointer);
});
