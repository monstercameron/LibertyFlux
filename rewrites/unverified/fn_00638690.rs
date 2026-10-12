// original: 0x00638690 rage::ptxEffectRuleStd::init_effect_instance

/// Clear the cache words belonging to every active effect record, set the two
/// rule-parameter flags, clear the shared entry selected by the manager byte,
/// clear the shared table's final word, and return the low byte of the current
/// record count. The rule object stores its record list at `+0x130` and its
/// 16-bit count at `+0x134`. The parameter object supplies the per-record rows
/// at `+0x7c` and shared table at `+0x260`.

#[inline(always)]
unsafe fn read_u16(address: u32) -> u16 {
    unsafe { (address as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_word(file_va: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
}

unsafe fn clear_rule_state(this: u32, params: u32, omit_first_row_word: bool) -> u8 {
    const RECORDS: u32 = 0x130;
    const RECORD_COUNT: u32 = 0x134;
    const ACTIVE_FLAG: u32 = 0x14;
    const PARAM_ROWS: u32 = 0x7C;
    const FIRST_CACHE_WORD: u32 = 0x04;
    const FIRST_CACHE_NEXT_WORD: u32 = 0x08;
    const SECOND_CACHE_WORD: u32 = 0x14;
    const SECOND_CACHE_NEXT_WORD: u32 = 0x18;
    const ROW_STRIDE: u32 = 0x120;
    const PARAM_FLAGS_A: u32 = 0x1C4;
    const PARAM_FLAGS_B: u32 = 0x1C8;
    const SHARED_TABLE: u32 = 0x260;
    const SHARED_FINAL_WORD: u32 = 0x288;
    const MANAGER_GLOBAL_FILE_VA: u32 = 0x01BB_6678;
    const SHARED_ENTRY_BASE: u32 = 0x37;
    const SHARED_ENTRY_WIDTH: u32 = 3;

    let records = unsafe { read_u32(this.wrapping_add(RECORDS)) };
    let rows = unsafe { read_u32(params.wrapping_add(PARAM_ROWS)) };
    let mut index = 0_u32;
    while index < u32::from(unsafe { read_u16(this.wrapping_add(RECORD_COUNT)) }) {
        let record = unsafe { read_u32(records.wrapping_add(index.wrapping_mul(4))) };
        if unsafe { read_u32(record.wrapping_add(ACTIVE_FLAG)) } == 0 {
            let row_offset = index.wrapping_mul(ROW_STRIDE);
            if !omit_first_row_word {
                unsafe {
                    write_u32(rows.wrapping_add(FIRST_CACHE_WORD).wrapping_add(row_offset), 0);
                }
            }
            unsafe {
                write_u32(rows.wrapping_add(FIRST_CACHE_NEXT_WORD).wrapping_add(row_offset), 0);
                write_u32(rows.wrapping_add(SECOND_CACHE_WORD).wrapping_add(row_offset), 0);
                write_u32(rows.wrapping_add(SECOND_CACHE_NEXT_WORD).wrapping_add(row_offset), 0);
            }
        }
        index = index.wrapping_add(1);
    }

    unsafe {
        write_u32(params.wrapping_add(PARAM_FLAGS_B), 0);
        write_u32(params.wrapping_add(PARAM_FLAGS_A), 1);
    }
    let manager = unsafe { read_global_word(MANAGER_GLOBAL_FILE_VA) };
    let variant = unsafe { core::ptr::read(manager.wrapping_add(2) as *const u8) } as u32;
    let entry_index = variant
        .wrapping_add(SHARED_ENTRY_BASE)
        .wrapping_mul(SHARED_ENTRY_WIDTH)
        .wrapping_mul(4);
    let shared_table = unsafe { read_u32(params.wrapping_add(SHARED_TABLE)) };
    unsafe {
        write_u32(shared_table.wrapping_add(entry_index), 0);
        write_u32(shared_table.wrapping_add(SHARED_FINAL_WORD), 0);
        core::ptr::read(this.wrapping_add(RECORD_COUNT) as *const u8)
    }
}

lf_checker_rt::export!(thiscall, rw_00638690(this: u32, params: u32) -> u8 {
    unsafe { clear_rule_state(this, params, false) }
});