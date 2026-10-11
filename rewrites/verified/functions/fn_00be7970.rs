// original: 0x00BE7970 CTaskSimpleWaitForBus::vf17

/// Search the task's sixteen candidate entities and select the first active
/// candidate whose position is within the small squared-distance limit and
/// whose signed result is below its one-byte threshold. The candidate list
/// pointer is stored at `argument + 0x224`; each candidate has a virtual
/// function table at offset zero, an active bit at `+0xF15`, and a threshold
/// byte at `+0x1070`. The position query is made through the candidate's
/// virtual slot at `+0xEC`; the second helper supplies a signed comparison
/// value. On success this writes the selected candidate pointer to `this +
/// 0x14` and returns one; otherwise it returns zero. The original method is
/// thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00BE7970(this: u32, argument: u32) -> u32 {
    const CANDIDATE_LIST: u32 = 0x224;
    const CANDIDATE_ARRAY: u32 = 0x10C;
    const ACTIVE_FLAGS: u32 = 0xF15;
    const ACTIVE_BIT: u8 = 0x08;
    const THRESHOLD_BYTE: u32 = 0x1070;
    const POSITION_LIMIT_VA: u32 = 0x00EB9500;
    const CANDIDATE_CAPACITY: u32 = 16;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        let candidates = read_u32(argument + CANDIDATE_LIST) + CANDIDATE_ARRAY;
        let limit = f32::from_bits(lf_checker_rt::global::<u32>(POSITION_LIMIT_VA).read_unaligned());

        for index in 0..CANDIDATE_CAPACITY {
            let candidate = read_u32(candidates + index * 4);
            if candidate == 0 || read_u8(candidate + ACTIVE_FLAGS) & ACTIVE_BIT == 0 {
                continue;
            }

            let mut position_out = [0u32; 3];
            let position_query = lf_checker_rt::callee_thiscall!(1, u32, candidate, position_out.as_mut_ptr() as u32);
            let x = f32::from_bits(read_u32(position_query));
            let y = f32::from_bits(read_u32(position_query + 4));
            let z = f32::from_bits(read_u32(position_query + 8));
            let x_squared = multiply(x, x);
            let y_squared = multiply(y, y);
            let z_squared = multiply(z, z);
            let squared_distance = add(add(x_squared, y_squared), z_squared);

            if !(core::hint::black_box(limit) > core::hint::black_box(squared_distance)) {
                continue;
            }

            let signed_result = lf_checker_rt::callee_thiscall!(2, u32, candidate);
            let candidate_bound = (read_u8(candidate + THRESHOLD_BYTE) as u32).wrapping_sub(1) as i32;
            if (signed_result as i32) < candidate_bound {
                write_u32(this + 0x14, candidate);
                return 1;
            }
        }
        0
    }
});
