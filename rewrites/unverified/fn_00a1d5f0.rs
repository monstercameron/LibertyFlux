// original: 0x00A1D5F0 task_probe_surroundings

/// Query nearby records for a task, updating the shared probe state as the
/// records are examined.
///
/// The first argument is the task object and the second points to a position
/// vector. Unless the task's mode is already six, the predicate stored at
/// `+0x224` can keep the function on its short path, which returns whether the
/// shared state word is nonzero. The full path prepares two related vectors
/// and sixty-four records, asks the task helpers to fill them, and checks the
/// resulting records in order. A flagged record whose virtual distance is
/// greater than twenty restores the shared state and stops the scan; every
/// other resolved record becomes the new shared state. A task distance above
/// 450 under the high probe gate returns zero immediately.
///
/// The helper calls and their observable return conventions are preserved.
/// The record area is 0x60 bytes per entry; its fields used by the helpers are
/// initialized from the three shared coordinate constants and fixed flags.
///
/// Original: stdcall with two stack arguments; returns a freshly computed
/// boolean in EAX.
lf_checker_rt::export!(stdcall, rw_00A1D5F0(task: u32, position: u32) -> u32 {
    unsafe {
        const STATE_VA: u32 = 0x012dd2a4;
        const COORD_X_VA: u32 = 0x01b4b320;
        const COORD_Y_VA: u32 = 0x01b4b324;
        const COORD_Z_VA: u32 = 0x01b4b328;
        const GATE_VA: u32 = 0x011d6fd4;
        const DISTANCE_LIMIT_VA: u32 = 0x00e8d714;
        const RECORD_LIMIT_VA: u32 = 0x00fe8b38;
        const ONE_VA: u32 = 0x00fe88e8;
        const RECORD_COUNT: usize = 64;
        const RECORD_SIZE: u32 = 0x60;

        #[repr(C, align(16))]
        struct Frame {
            words: [u32; 0x1848 / 4],
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
        unsafe fn read_global(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }

        #[inline(always)]
        unsafe fn write_global(file_va: u32, value: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), value) }
        }

        let _ = lf_checker_rt::callee_cdecl!(1, u32);

        if rd32(task.wrapping_add(0x7b8)) != 6 {
            let predicate_this = rd32(task.wrapping_add(0x224));
            let predicate = lf_checker_rt::callee_thiscall!(2, u32, predicate_this);
            if (predicate & 0xff) == 0 {
                let result = u32::from(read_global(STATE_VA) != 0);
                let _ = lf_checker_rt::callee_cdecl!(4, u32);
                return result;
            }
        }

        let x = rd32(position);
        let y = rd32(position.wrapping_add(4));
        let z = rd32(position.wrapping_add(8));
        let z_plus_one = (core::hint::black_box(f32::from_bits(z))
            + core::hint::black_box(f32::from_bits(read_global(ONE_VA))))
        .to_bits();

        let mut frame = Frame { words: [0; 0x1848 / 4] };
        let context = frame.words.as_mut_ptr() as u32;
        let record_count_address = context.wrapping_add(4);
        let first_vector = context.wrapping_add(0x0c);
        let second_vector = context.wrapping_add(0x1c);
        let records_address = context.wrapping_add(0x2c);

        wr32(first_vector, x);
        wr32(first_vector.wrapping_add(4), y);
        wr32(first_vector.wrapping_add(8), z);
        wr32(second_vector, x);
        wr32(second_vector.wrapping_add(4), y);
        wr32(second_vector.wrapping_add(8), z_plus_one);
        let coord_x = read_global(COORD_X_VA);
        let coord_y = read_global(COORD_Y_VA);
        let coord_z = read_global(COORD_Z_VA);
        for index in 0..RECORD_COUNT {
            let record = records_address.wrapping_add((index as u32).wrapping_mul(RECORD_SIZE));
            wr32(record, 0);
            wr32(record.wrapping_add(8), coord_x);
            wr32(record.wrapping_add(12), coord_y);
            wr32(record.wrapping_add(16), coord_z);
            wr32(record.wrapping_add(0x28), 0);
            wr32(record.wrapping_add(0x2c), 0);
            wr32(record.wrapping_add(0x30), 0);
            wr32(record.wrapping_add(0x34), 0xffff);
            wr32(record.wrapping_add(0x38), 0);
        }

        let list_handle = lf_checker_rt::callee_cdecl!(
            5,
            u32,
            context,
            second_vector,
            task,
            records_address,
            0x98u32,
            0x40u32,
            4u32,
        );

        wr32(record_count_address, list_handle);
        let _ = lf_checker_rt::callee_cdecl!(
            6,
            u32,
            records_address,
            record_count_address,
        );

        let record_count = rd32(record_count_address);
        let mut result_record = 0u32;
        write_global(STATE_VA, 0);
        let gate = read_global(GATE_VA) as i32;
        if gate >= 2 {
            let task_owner = rd32(task.wrapping_add(0x20));
            let task_distance = f32::from_bits(rd32(task_owner.wrapping_add(0x38)));
            let limit = f32::from_bits(read_global(DISTANCE_LIMIT_VA));
            if task_distance > limit {
                let _ = lf_checker_rt::callee_cdecl!(3, u32);
                return 0;
            }
        }

        let signed_count = record_count as i32;
        if signed_count > 0 {
            for index in 0..(signed_count as usize).min(RECORD_COUNT) {
                if result_record != 0 {
                    break;
                }

                let record_address = records_address
                    .wrapping_add((index as u32).wrapping_mul(RECORD_SIZE));
                let record_object = rd32(record_address);
                if record_object == 0 {
                    continue;
                }

                result_record = lf_checker_rt::callee_cdecl!(7, u32, record_object);
                let flags = rd32(result_record.wrapping_add(0x24));
                if (flags & 0x2000_0000) != 0 {
                    let level = rd32(record_object.wrapping_add(4));
                    let virtual_this = rd32(rd32(level.wrapping_add(0x0c)));
                    let distance = lf_checker_rt::callee_thiscall!(
                        8,
                        f32,
                        virtual_this,
                    );
                    let threshold = f32::from_bits(read_global(RECORD_LIMIT_VA));
                    if distance > threshold {
                        result_record = read_global(STATE_VA);
                        break;
                    }
                }
                write_global(STATE_VA, result_record);
            }
        }

        let _ = lf_checker_rt::callee_cdecl!(4, u32);
        u32::from(result_record != 0)
    }
});
