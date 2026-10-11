// original: 0x00B2F940 ped_task_spawn_or_update

/// Update or create a task record for a pedestrian.
///
/// A set global stop flag sends the current stack context directly to the
/// finalizer. Otherwise the kind selects a row in the task table, optional
/// kind-specific checks can also finish early, and the helper chain prepares
/// a candidate before comparing it with the fixed shared record span. A
/// winning candidate updates the sequence counter, invokes the record
/// helpers, and is finalized. The function keeps the caller's six-word cdecl
/// argument order and returns the finalizer's value.
///
/// The row stride is 0x34 bytes and the shared record stride is 0x50 bytes.
/// The kind-three resolver can replace the incoming kind. Kind twenty-six
/// first checks a fixed world-space region against nine candidate points.
/// Float differences, squares, additions, and comparisons follow the
/// original scalar order; comparisons with unordered inputs take the same
/// failing side as the original.
///
/// Original: cdecl with six stack arguments; returns EAX.
lf_checker_rt::export!(cdecl, rw_00B2F940(
    kind: u32,
    vector: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    scale_bits: u32,
) -> u32 {
    unsafe {
        const STOP_FLAG_VA: u32 = 0x01661400;
        const ROWS_VA: u32 = 0x01661410;
        const RECORDS_VA: u32 = 0x01661a60;
        const RECORDS_END_VA: u32 = 0x01662460;
        const TICK_VA: u32 = 0x011735b4;
        const COUNTER_ONE_VA: u32 = 0x01661401;
        const SEQUENCE_VA: u32 = 0x01661404;
        const COUNTER_TWO_VA: u32 = 0x01661408;
        const COOKIE_VA: u32 = 0x01057fb4;
        const KIND_ROW_SIZE: u32 = 0x34;
        const RECORD_SIZE: u32 = 0x50;

        #[repr(C, align(16))]
        struct Frame {
            words: [u32; 0xb8 / 4],
        }

        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn read_global32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }

        #[inline(always)]
        unsafe fn write_global32(file_va: u32, value: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), value) }
        }

        #[inline(always)]
        unsafe fn read_global8(file_va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(file_va)) }
        }

        #[inline(always)]
        unsafe fn write_global8(file_va: u32, value: u8) {
            unsafe { wr8(lf_checker_rt::relocated(file_va), value) }
        }

        #[inline(always)]
        fn ordered_sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        #[inline(always)]
        fn ordered_square(value: f32) -> f32 {
            core::hint::black_box(value) * core::hint::black_box(value)
        }

        #[inline(always)]
        fn ordered_sum(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        unsafe fn finish(frame_context: u32) -> u32 {
            let answer = lf_checker_rt::callee_thiscall!(10, u32, frame_context);
            let _ = lf_checker_rt::callee_cdecl!(11, u32,);
            answer
        }

        let mut kind = kind;
        let mut frame = Frame { words: [0; 0xb8 / 4] };
        let frame_base = frame.words.as_mut_ptr() as u32;
        frame.words[0] = arg3;
        frame.words[2] = arg2;
        let frame_context = frame_base.wrapping_add(0x18);

        if read_global8(STOP_FLAG_VA) != 0 {
            let _ = lf_checker_rt::callee_cdecl!(11, u32,);
            return arg3;
        }

        if !(1..=30).contains(&kind) {
            let _ = lf_checker_rt::callee_cdecl!(11, u32,);
            return kind.wrapping_sub(1);
        }

        let row = lf_checker_rt::relocated(
            ROWS_VA.wrapping_add(kind.wrapping_mul(KIND_ROW_SIZE)),
        );
        if rd32(row) == 0 {
            let _ = lf_checker_rt::callee_cdecl!(11, u32,);
            return kind.wrapping_sub(1);
        }
        let row_value = rd32(row.wrapping_add(8));

        if kind == 0x1a {
            let point_x = f32::from_bits(rd32(vector));
            let point_y = f32::from_bits(rd32(vector.wrapping_add(4)));
            let point_z = f32::from_bits(rd32(vector.wrapping_add(8)));
            if point_x > -1142.0 && point_y > -243.0 && point_z > 40.0 {
                const NEAR_POINTS: [[u32; 3]; 9] = [
                    [0xc48e_d666, 0xc38c_9852, 0x4296_570a],
                    [0xc4bf_247b, 0xc417_2148, 0x428b_6148],
                    [0xc4bd_0000, 0xc41b_2f5c, 0x429f_570a],
                    [0xc4ac_247b, 0xc380_b5c3, 0x425d_eb85],
                    [0xc4b9_170a, 0xc41b_6ae1, 0x42c8_0000],
                    [0xc4ab_0f0a, 0xc383_ef5c, 0x4298_9eb8],
                    [0xc4f5_519a, 0xc374_028f, 0x4297_0a3d],
                    [0xc4eb_575c, 0xc38c_9333, 0x42a1_e148],
                    [0xc4eb_f6b8, 0xc373_deb8, 0x42bb_28f6],
                ];
                let radius_squared = f32::from_bits(read_global32(0x00fe8b48));
                for candidate in NEAR_POINTS {
                    let dx = ordered_sub(point_x, f32::from_bits(candidate[0]));
                    let dy = ordered_sub(point_y, f32::from_bits(candidate[1]));
                    let dz = ordered_sub(point_z, f32::from_bits(candidate[2]));
                    let xx = ordered_square(dx);
                    let yy = ordered_square(dy);
                    let zz = ordered_square(dz);
                    let xy = ordered_sum(yy, xx);
                    let distance_squared = ordered_sum(xy, zz);
                    if radius_squared > distance_squared {
                        return finish(frame_context);
                    }
                }
            }
        }

        if kind == 3 {
            kind = lf_checker_rt::callee_cdecl!(1, u32, arg4);
        }

        let row_integer = rd32(row.wrapping_add(0x0c)) as i32 as f32;
        let scale = f32::from_bits(scale_bits);
        let scaled_row = ordered_sum(row_integer, scale).to_bits();
        let tick = read_global32(TICK_VA);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, frame_context);
        let _ = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            frame_context,
            kind,
            vector,
            scaled_row,
            0u32,
            arg3,
            tick,
            row_value,
            0u32,
        );

        let candidate = lf_checker_rt::callee_cdecl!(4, u32, frame_context);
        if (candidate & 0xff) == 0 {
            return finish(frame_context);
        }

        let admitted = lf_checker_rt::callee_cdecl!(
            5,
            u32,
            kind,
            row_value,
            vector,
            0u32,
            arg3,
            arg4,
        );
        if (admitted & 0xff) != 0 {
            return finish(frame_context);
        }

        let count_two = read_global8(COUNTER_TWO_VA);
        write_global8(COUNTER_TWO_VA, count_two.wrapping_add(1));

        let records = lf_checker_rt::relocated(RECORDS_VA);
        let records_end = lf_checker_rt::relocated(RECORDS_END_VA);
        let mut best = 0u32;
        let mut slot = records;
        while slot < records_end {
            if rd32(slot) != 0 {
                if best == 0 {
                    best = slot;
                } else {
                    let prefer = lf_checker_rt::callee_cdecl!(6, u32, slot, best, 1u32);
                    if (prefer & 0xff) == 0 {
                        best = slot;
                    }
                }
            }
            slot = slot.wrapping_add(RECORD_SIZE);
        }

        if best == 0 {
            best = records;
        }

        let selected = lf_checker_rt::callee_cdecl!(7, u32, frame_context, best, 1u32);
        if (selected & 0xff) == 0 {
            return finish(frame_context);
        }

        if rd32(best) == 0 {
            let count_one = read_global8(COUNTER_ONE_VA);
            write_global8(COUNTER_ONE_VA, count_one.wrapping_add(1));
        }

        let _ = lf_checker_rt::callee_thiscall!(8, u32, best);
        let sequence = read_global32(SEQUENCE_VA).wrapping_add(1);
        write_global32(SEQUENCE_VA, sequence);
        let _ = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            best,
            kind,
            vector,
            scaled_row,
            0u32,
            arg3,
            tick,
            row_value,
            sequence,
        );
        let _ = lf_checker_rt::callee_cdecl!(9, u32, best);
        let answer = finish(frame_context);
        answer
    }
});
