// original: 0x00CF4210 ped_task_test_control_input

/// Answers a task-control query using a two-word stdcall interface: the first
/// word points to a task object and the second is an unsigned query index.
/// The object must have byte fields at `+0x218` and `+0x219`; successful
/// queries also acquire a controller through the first intercepted helper.
/// Indices 0 through 4 select separate checks, and every other unsigned value
/// returns false. Query zero uses a signed score comparison against 120;
/// query two compares the scripted float against the image's `-0.5` limit,
/// query three compares it against the image's `0.5` limit, and query four
/// uses the `-0.5` limit again. The ordered comparisons return false for NaN.
/// The result is a boolean in AL; upper EAX bytes are left over on several
/// branches. This rewrite makes the same intercepted calls in branch order.
lf_checker_rt::export!(stdcall, rw_00cf4210(object: u32, query: u32) -> u32 {
    unsafe {
        const OBJECT_READY: u32 = 0x218;
        const OBJECT_ENABLED: u32 = 0x219;
        const CHILD_HAS_CONTROL: u32 = 0x328d;
        const CHILD_CONTROL_SLOT: u32 = 0x26c8;
        const CHILD_STATE_SLOT: u32 = 0x26b8;
        const CALLEE_ACQUIRE: u32 = 1;
        const CALLEE_CONTROL_QUERY: u32 = 2;
        const CALLEE_SCORE: u32 = 3;
        const CALLEE_SCORE_LIMIT: u32 = 4;
        const CALLEE_VALUE: u32 = 5;
        const CALLEE_FLOAT: u32 = 6;
        const LAST_QUERY: u32 = 4;
        const SCORE_LIMIT: i32 = 120;
        const LESS_THAN_HALF: u32 = 0x00fe8d7c;
        const HALF: u32 = 0x00fe8830;

        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_float_at(file_va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(file_va).read_unaligned() }
        }

        #[inline(always)]
        fn ordered_greater(left: f32, right: f32) -> bool {
            left.partial_cmp(&right) == Some(core::cmp::Ordering::Greater)
        }

        if unsafe { read_byte(object + OBJECT_READY) } != 0
            || unsafe { read_byte(object + OBJECT_ENABLED) } == 0
        {
            return 0;
        }

        let controller = lf_checker_rt::callee_thiscall!(CALLEE_ACQUIRE, u32, object);
        if controller == 0 || query > LAST_QUERY {
            return 0;
        }

        match query {
            0 => {
                let has_control = lf_checker_rt::callee_thiscall!(
                    CALLEE_CONTROL_QUERY,
                    u32,
                    controller + CHILD_CONTROL_SLOT
                ) as u8;
                if has_control != 0 {
                    return 1;
                }
                if unsafe { read_byte(controller + CHILD_HAS_CONTROL) } == 0 {
                    return 0;
                }
                let score = lf_checker_rt::callee_thiscall!(CALLEE_SCORE, u32, controller);
                let accepted = lf_checker_rt::callee_cdecl!(CALLEE_SCORE_LIMIT, u32, score);
                u32::from((accepted as i32) > SCORE_LIMIT)
            }
            1 => {
                let state = lf_checker_rt::callee_thiscall!(
                    CALLEE_CONTROL_QUERY,
                    u32,
                    controller + CHILD_STATE_SLOT
                ) as u8;
                u32::from(state != 0)
            }
            2 => {
                let value = lf_checker_rt::callee_thiscall!(CALLEE_VALUE, u32, controller);
                let measured = lf_checker_rt::callee_cdecl!(CALLEE_FLOAT, f32, value);
                let limit = unsafe { read_float_at(LESS_THAN_HALF) };
                u32::from(ordered_greater(limit, measured))
            }
            3 => {
                let value = lf_checker_rt::callee_thiscall!(CALLEE_VALUE, u32, controller);
                let measured = lf_checker_rt::callee_cdecl!(CALLEE_FLOAT, f32, value);
                let limit = unsafe { read_float_at(HALF) };
                u32::from(ordered_greater(measured, limit))
            }
            4 => {
                let value = lf_checker_rt::callee_thiscall!(CALLEE_SCORE, u32, controller);
                let measured = lf_checker_rt::callee_cdecl!(CALLEE_FLOAT, f32, value);
                let limit = unsafe { read_float_at(LESS_THAN_HALF) };
                u32::from(ordered_greater(limit, measured))
            }
            _ => 0,
        }
    }
});
