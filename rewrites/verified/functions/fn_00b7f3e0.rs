// original: 0x00B7F3E0 trigger_boat_horizon_tracking

/// Builds the boat horizon tracking request when the first object's status
/// value is nonnegative and ordered. It reads two position records, scales the
/// first record's x and y by the image's single-precision tracking factor, adds
/// those offsets to the second record's x and y, then dispatches a request with
/// a three-word vector on its stack. The status test treats a not-a-number as
/// rejected, matching COMISS followed by `jb`. This cdecl routine takes two
/// object pointers and returns no meaningful value.
lf_checker_rt::export!(cdecl, rw_b7f3e0(first: u32, second: u32) -> () {
    unsafe {
        const STATUS_OFFSET: u32 = 0x1078;
        const POSITION_PTR_OFFSET: u32 = 0x20;
        const FIRST_POSITION_X: u32 = 0x10;
        const FIRST_POSITION_Y: u32 = 0x14;
        const SECOND_POSITION_X: u32 = 0x30;
        const SECOND_POSITION_Y: u32 = 0x34;
        const TRACKING_SCALE_VA: u32 = 0x00FE8B68;
        const EVENT_VTABLE_VA: u32 = 0x00EB3110;
        const DISPATCH_OFFSET: u32 = 0x0BB0;
        const DISPATCHER: u32 = 1;

        let status = (first.wrapping_add(STATUS_OFFSET) as *const f32).read_unaligned();
        if !(status >= 0.0) {
            return;
        }

        let first_position = (first.wrapping_add(POSITION_PTR_OFFSET) as *const u32).read_unaligned();
        let second_position = (second.wrapping_add(POSITION_PTR_OFFSET) as *const u32).read_unaligned();
        let scale = (lf_checker_rt::relocated(TRACKING_SCALE_VA) as *const f32).read_unaligned();
        let first_x = (first_position.wrapping_add(FIRST_POSITION_X) as *const f32).read_unaligned();
        let first_y = (first_position.wrapping_add(FIRST_POSITION_Y) as *const f32).read_unaligned();
        let second_x = (second_position.wrapping_add(SECOND_POSITION_X) as *const f32).read_unaligned();
        let second_y = (second_position.wrapping_add(SECOND_POSITION_Y) as *const f32).read_unaligned();

        let scaled_x = core::hint::black_box(first_x) * core::hint::black_box(scale);
        let scaled_y = core::hint::black_box(first_y) * core::hint::black_box(scale);
        let result_x = core::hint::black_box(second_x) + core::hint::black_box(scaled_x);
        let result_y = core::hint::black_box(second_y) + core::hint::black_box(scaled_y);
        let mut vector = [result_x, result_y, 0.0_f32];
        let vector_ptr = vector.as_mut_ptr() as u32;
        let request_vtable = lf_checker_rt::relocated(EVENT_VTABLE_VA);

        let _ = lf_checker_rt::callee_thiscall!(
            DISPATCHER, u32,
            second.wrapping_add(DISPATCH_OFFSET),
            request_vtable, 0_u32, 0_u32, 1000_u32, u32::MAX,
            vector_ptr, 0_u32, 500_u32, 500_u32, 1_u32,
        );
    }
});
