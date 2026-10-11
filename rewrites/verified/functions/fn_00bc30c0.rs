// original: 0x00BC30C0 CTaskComplexMoveSeekEntity<CEntitySeekPosCalculatorRadiusAngleOffset>::vf19

/// Update a move-seek task for one request using x86 thiscall (`this` in ECX,
/// one request pointer on the stack, and the selected handler's EAX result).
/// The function copies the timing-state word into task offsets `0x5c` and,
/// when offset `0x40` is not `u32::MAX`, `0x68`; it copies the word at `0x44`
/// to `0x60`, writes byte `1` at `0x64`, and conditionally records the prior
/// target at `0x6c` and byte `1` at `0x70`. A null task target or a ready
/// request (`request+0x26c` bit `0x04` and nonzero `request+0xb30`) forwards
/// event `0x516`. Otherwise a position helper supplies four task words at
/// `0x30..0x3c`; the function compares planar squared distance from task and
/// request positions with the squared radius at `0x48`. When squared radius
/// is greater than squared distance, the function sets task flag bit `0x02`,
/// notifies the request, and forwards event `0x11e`. An unordered comparison
/// continues into threshold handling. Inside the radius, threshold `0x50`
/// is compared with zero: equality forwards `0x3ae`,
/// otherwise the squared threshold selects `0x387` when distance is not
/// greater, or `0x3ae` when it is greater. All scalar float operations keep
/// the original SSE operand order; unordered comparisons follow the original
/// `jbe`/`jnp` behavior.
lf_checker_rt::export!(thiscall, rw_00bc30c0(this: u32, request: u32) -> u32 {
    unsafe {
        const TIMER_COPY: u32 = 0x5C;
        const TASK_TARGET: u32 = 0x20;
        const TASK_POSITION: u32 = 0x30;
        const TASK_RADIUS: u32 = 0x48;
        const TASK_THRESHOLD: u32 = 0x50;
        const TASK_FLAGS: u32 = 0x80;
        const REQUEST_TARGET: u32 = 0x20;
        const REQUEST_FLAGS: u32 = 0x26C;
        const REQUEST_GATE: u32 = 0xB30;
        const TARGET_POSITION: u32 = 0x30;
        const FLAG_OUTSIDE_RADIUS: u8 = 0x02;
        const REQUEST_READY: u8 = 0x04;
        const ZERO: f32 = f32::from_bits(0x0000_0000);
        const TIMING_STATE_VA: u32 = 0x0117_35B4;
        const POSITION_HELPER: u32 = 3;
        const EVENT_516_EMPTY: u32 = 1;
        const EVENT_516_FLAGGED: u32 = 2;
        const EVENT_11E_OUTSIDE: u32 = 5;
        const EVENT_3AE_FAR: u32 = 6;
        const EVENT_387_NEAR: u32 = 7;
        const EVENT_3AE_ZERO_THRESHOLD: u32 = 8;
        const TASK_NOTIFY: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(base: u32, offset: u32) -> u32 {
            unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(base: u32, offset: u32) -> u8 {
            unsafe { (base.wrapping_add(offset) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u32(base: u32, offset: u32, value: u32) {
            unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u8(base: u32, offset: u32, value: u8) {
            unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn read_f32(base: u32, offset: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(base, offset) })
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

        let timer = lf_checker_rt::global::<u32>(TIMING_STATE_VA).read_unaligned();
        write_u32(this, TIMER_COPY, timer);
        write_u32(this, 0x60, read_u32(this, 0x44));
        write_u8(this, 0x64, 1);

        let stored_target = read_u32(this, 0x40);
        if stored_target != u32::MAX {
            write_u32(this, 0x68, timer);
            write_u32(this, 0x6C, stored_target);
            write_u8(this, 0x70, 1);
        }

        if read_u32(this, TASK_TARGET) == 0 {
            return lf_checker_rt::callee_thiscall!(EVENT_516_EMPTY, u32, this, 0x516, request);
        }
        if read_u8(request, REQUEST_FLAGS) & REQUEST_READY != 0
            && read_u32(request, REQUEST_GATE) != 0
        {
            return lf_checker_rt::callee_thiscall!(EVENT_516_FLAGGED, u32, this, 0x516, request);
        }

        let mut ignored_out_words = [0u32; 4];
        let position = lf_checker_rt::callee_thiscall!(
            POSITION_HELPER, u32, this,
            ignored_out_words.as_mut_ptr() as u32, request
        );
        for index in 0..4u32 {
            write_u32(this, TASK_POSITION + index * 4, read_u32(position, index * 4));
        }

        let target = read_u32(request, REQUEST_TARGET);
        let dx = sub_f32(read_f32(target, TARGET_POSITION), read_f32(this, TASK_POSITION));
        let dy = sub_f32(read_f32(target, TARGET_POSITION + 4), read_f32(this, TASK_POSITION + 4));
        let dx_squared = mul_f32(dx, dx);
        let dy_squared = mul_f32(dy, dy);
        let distance_squared = add_f32(dy_squared, dx_squared);
        let radius = read_f32(this, TASK_RADIUS);
        let radius_squared = mul_f32(radius, radius);

        if radius_squared > distance_squared {
            let flags = read_u8(this, TASK_FLAGS);
            write_u8(this, TASK_FLAGS, flags | FLAG_OUTSIDE_RADIUS);
            let _ = lf_checker_rt::callee_thiscall!(TASK_NOTIFY, u32, request, 1);
            return lf_checker_rt::callee_thiscall!(EVENT_11E_OUTSIDE, u32, this, 0x11E, request);
        }

        let threshold = read_f32(this, TASK_THRESHOLD);
        if threshold == ZERO {
            return lf_checker_rt::callee_thiscall!(EVENT_3AE_ZERO_THRESHOLD, u32, this, 0x3AE);
        }
        let threshold_squared = mul_f32(threshold, threshold);
        if !(distance_squared > threshold_squared) {
            return lf_checker_rt::callee_thiscall!(EVENT_387_NEAR, u32, this, 0x387);
        }
        lf_checker_rt::callee_thiscall!(EVENT_3AE_FAR, u32, this, 0x3AE)
    }
});
