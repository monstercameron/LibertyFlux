// original: 0x00C1F9A0 streaming_check_asset_record_flags

/// Query the record returned by the object's third vtable slot. When the mode
/// byte is zero, record flags select one of three threshold rules; successful
/// timed rules write either the remaining fraction or the current time through
/// the output pointer. A record of subtype two also enables the global-density
/// override. Comparisons preserve COMISS behavior for unordered values: `jb`
/// accepts unordered operands, while `ja` and `cmova` reject them. The function
/// takes three cdecl word slots and returns the status in AL.
lf_checker_rt::export!(cdecl, rw_00c1f9a0(object: u32, mode_word: u32, output: u32) -> u32 {
    unsafe {
        const VTABLE_OFFSET: u32 = 0;
        const REQUIRED_BITS: u32 = 0x20;
        const STARTED_BIT: u8 = 0x01;
        const READY_BIT: u8 = 0x40;
        const RECORD_FLAGS: u32 = 0x4c;
        const RECORD_KIND: u32 = 0x58;
        const TIME_VA: u32 = 0x012DDEF4;
        const DENSITY_VA: u32 = 0x012DDE98;
        const NEAR_ONE_VA: u32 = 0x00FE88DC;
        const ONE_VA: u32 = 0x00FE88E8;
        const MIN_TIME_VA: u32 = 0x00FE86B4;
        const QUARTER_VA: u32 = 0x00FE87E4;
        const HALF_VA: u32 = 0x00FE8830;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        fn comiss_jb(left: f32, right: f32) -> bool {
            left.is_nan() || right.is_nan() || left < right
        }

        #[inline(always)]
        fn comiss_ja(left: f32, right: f32) -> bool {
            !left.is_nan() && !right.is_nan() && left > right
        }

        let vtable = read_u32(object.wrapping_add(VTABLE_OFFSET));
        let slot = read_u32(vtable.wrapping_add(8));
        let lookup: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
        let record = lookup(object);
        let flags = read_u32(record.wrapping_add(RECORD_FLAGS));
        let low_flags = flags as u8;
        let category_flags = flags & REQUIRED_BITS;
        let time = read_f32(lf_checker_rt::relocated(TIME_VA));
        let mut selected = false;

        if mode_word as u8 == 0 {
            if category_flags != 0 && low_flags & READY_BIT != 0 {
                selected = true;
            } else if low_flags & STARTED_BIT != 0 {
                if category_flags != 0 {
                    let near_one = read_f32(lf_checker_rt::relocated(NEAR_ONE_VA));
                    if comiss_jb(near_one, time) {
                        let min_time = read_f32(lf_checker_rt::relocated(MIN_TIME_VA));
                        if low_flags & READY_BIT != 0 && !comiss_jb(time, min_time) {
                            (output as *mut u32).write_unaligned(time.to_bits());
                            selected = true;
                        }
                    } else {
                        let one = read_f32(lf_checker_rt::relocated(ONE_VA));
                        let remaining = core::hint::black_box(one) - core::hint::black_box(time);
                        (output as *mut u32).write_unaligned(remaining.to_bits());
                        selected = true;
                    }
                } else if low_flags & READY_BIT != 0 {
                    let min_time = read_f32(lf_checker_rt::relocated(MIN_TIME_VA));
                    if !comiss_jb(time, min_time) {
                        (output as *mut u32).write_unaligned(time.to_bits());
                        selected = true;
                    }
                }
            } else {
                let quarter = read_f32(lf_checker_rt::relocated(QUARTER_VA));
                if category_flags != 0 && comiss_ja(quarter, time) {
                    selected = true;
                } else if low_flags & READY_BIT != 0 && !comiss_jb(time, quarter) {
                    selected = true;
                }
            }
        }

        let kind = unsafe { (record.wrapping_add(RECORD_KIND) as *const u8).read_unaligned() };
        if kind == 2 {
            let density = read_f32(lf_checker_rt::relocated(DENSITY_VA));
            let half = read_f32(lf_checker_rt::relocated(HALF_VA));
            if comiss_ja(density, half) {
                selected = true;
            }
        }
        u32::from(selected)
    }
});
