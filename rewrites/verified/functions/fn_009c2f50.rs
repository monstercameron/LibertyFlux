// original: 0x009C2F50 stream_find_nearest_context

/// Check the candidate object through its first virtual method, then inspect
/// active streaming records through their second virtual method. Return the
/// first accepted record whose three-dimensional distance is within the
/// configured threshold; return null when either method rejects every
/// candidate. Signed count comparisons and single-precision operand order
/// match the original control flow.
lf_checker_rt::export!(cdecl, rw_009c2f50(candidate: u32, _ignored: u32, origin: u32) -> u32 {
    unsafe {
        const ACTIVE_COUNT: u32 = 0x0129_5734;
        const POSITION_TABLE: u32 = 0x0129_3e98;
        const NESTED_VECTOR: u32 = 0x20;
        const NESTED_VECTOR_OFFSET: u32 = 0x30;
        const DIRECT_VECTOR: u32 = 0x10;
        const DISTANCE_BIAS: u32 = 0x00fe_88e8;
        const DISTANCE_LIMIT: u32 = 0x00e9_55f8;
        const COORDINATE_WORDS: usize = 3;

        let accepts_candidate = lf_checker_rt::callee_thiscall!(1, u32, candidate);
        if accepts_candidate as u8 == 0 {
            return 0;
        }

        let count = lf_checker_rt::global::<i32>(ACTIVE_COUNT).read_unaligned();
        let mut index = 0i32;
        while index < count {
            let slot = POSITION_TABLE.wrapping_add((index as u32).wrapping_mul(4));
            let record = lf_checker_rt::global::<u32>(slot).read_unaligned();
            if record != 0 {
                let accepts_record = lf_checker_rt::callee_thiscall!(2, u32, record);
                if accepts_record as u8 != 0 {
                    let origin_nested =
                        (origin.wrapping_add(NESTED_VECTOR) as usize as *const u32)
                            .read_unaligned();
                    let origin_vector = if origin_nested != 0 {
                        origin_nested.wrapping_add(NESTED_VECTOR_OFFSET)
                    } else {
                        origin.wrapping_add(DIRECT_VECTOR)
                    };
                    let record_nested =
                        (record.wrapping_add(NESTED_VECTOR) as usize as *const u32)
                            .read_unaligned();
                    let record_vector = if record_nested != 0 {
                        record_nested.wrapping_add(NESTED_VECTOR_OFFSET)
                    } else {
                        record.wrapping_add(DIRECT_VECTOR)
                    };
                    let record_object = record;

                    let origin = origin_vector as usize as *const u32;
                    let record = record_vector as usize as *const u32;
                    let mut squared = [0.0f32; COORDINATE_WORDS];
                    for coordinate in 0..COORDINATE_WORDS {
                        let origin_bits = origin.add(coordinate).read_unaligned();
                        let record_bits = record.add(coordinate).read_unaligned();
                        let origin_value = core::hint::black_box(f32::from_bits(origin_bits));
                        let record_value = core::hint::black_box(f32::from_bits(record_bits));
                        let difference = core::hint::black_box(origin_value - record_value);
                        squared[coordinate] =
                            core::hint::black_box(difference * core::hint::black_box(difference));
                    }

                    let y_then_x = core::hint::black_box(
                        core::hint::black_box(squared[1]) + core::hint::black_box(squared[0]),
                    );
                    let sum = core::hint::black_box(
                        y_then_x + core::hint::black_box(squared[2]),
                    );
                    let bias = lf_checker_rt::global::<f32>(DISTANCE_BIAS).read_unaligned();
                    let distance = core::hint::black_box(
                        core::hint::black_box(sum) - core::hint::black_box(bias),
                    );
                    let limit = lf_checker_rt::global::<f32>(DISTANCE_LIMIT).read_unaligned();
                    if distance.partial_cmp(&limit) == Some(core::cmp::Ordering::Greater) {
                        return 0;
                    }
                    return record_object as u32;
                }
            }
            index += 1;
        }
        0
    }
});
