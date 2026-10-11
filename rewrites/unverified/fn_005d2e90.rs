// original: 0x005D2E90 blip_slot_alloc

/// Find a free slot in the requested blip group and advance its scan counter.
///
/// `group` is in ECX and `out_index` is in EDX; `u32::MAX` selects the full
/// 960-slot table, while other groups cover 30 slots at `group * 30`. The
/// table has 0x50-byte entries with an object pointer at offset 0 and a
/// 16-bit network key at offset 0x40. A slot is free only when both are zero.
/// The function returns the relocated slot address, stores the absolute slot
/// index when the optional output pointer is nonnull, and sets the global
/// scan counter to that index relative to the selected group plus one.
///
/// In full-table mode, the signed current index chooses a player bucket by
/// truncating its product with the game's 32-bit scale constant. That path is
/// reached only for indices from -1 through 959, so the float-to-i32 cast is
/// in range and matches the original `cvttss2si`. A missing player skips that
/// 30-slot bucket and retries; a present player scans from the current index
/// through the end of the table. The cdecl player lookup is intercepted by
/// the checker contract.
lf_checker_rt::export!(fastcall, rw_005d2e90(group: u32, out_index: u32) -> u32 {
    unsafe {
        const ALL_GROUPS: u32 = u32::MAX;
        const GROUP_WIDTH: u32 = 30;
        const SLOT_COUNT: u32 = 960;
        const SLOT_STRIDE: u32 = 0x50;
        const SLOT_KEY_OFFSET: u32 = 0x40;
        const SLOT_TABLE_VA: u32 = 0x019D5A10;
        const COUNTER_VA: u32 = 0x018B6F90;
        const GROUP_SCALE_BITS: u32 = 0x3D088889;

        let group_base = if group == ALL_GROUPS {
            0
        } else {
            group.wrapping_mul(GROUP_WIDTH)
        };
        let scan_limit = if group == ALL_GROUPS {
            SLOT_COUNT
        } else {
            group_base.wrapping_add(GROUP_WIDTH)
        };
        let counter = lf_checker_rt::global::<u32>(COUNTER_VA).read_unaligned();
        let mut scan_index = counter.wrapping_add(group_base);
        if (scan_index as i32) >= (scan_limit as i32) {
            return 0;
        }

        if group == ALL_GROUPS {
            while (scan_index as i32) >= -1 {
                // The initial signed bound check above limits this value to
                // [-1, 959], and each skipped bucket restarts below 960.
                let scaled = core::hint::black_box(scan_index as i32 as f32)
                    * core::hint::black_box(f32::from_bits(GROUP_SCALE_BITS));
                let player_number = scaled as i32;
                let next_bucket = if player_number == -1 {
                    SLOT_COUNT
                } else {
                    (player_number.wrapping_add(1) as u32).wrapping_mul(GROUP_WIDTH)
                };
                let player = lf_checker_rt::callee_cdecl!(1, u32, player_number as u32);
                if player != 0 {
                    break;
                }
                scan_index = next_bucket;
                if (scan_index as i32) >= (SLOT_COUNT as i32) {
                    return 0;
                }
            }
        }

        while (scan_index as i32) < (scan_limit as i32) {
            let slot_file_va = SLOT_TABLE_VA.wrapping_add(scan_index.wrapping_mul(SLOT_STRIDE));
            let slot = lf_checker_rt::relocated(slot_file_va) as *const u8;
            let object = (slot as *const u32).read_unaligned();
            let key = (slot.add(SLOT_KEY_OFFSET as usize) as *const u16).read_unaligned();
            if object == 0 && key == 0 {
                let relative_index = scan_index.wrapping_sub(group_base).wrapping_add(1);
                lf_checker_rt::global::<u32>(COUNTER_VA).write_unaligned(relative_index);
                if out_index != 0 {
                    (out_index as *mut u32).write_unaligned(scan_index);
                }
                return lf_checker_rt::relocated(slot_file_va);
            }
            scan_index = scan_index.wrapping_add(1);
        }
        0
    }
});
