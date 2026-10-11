// original: 0x00B38740 ped_task_candidate_callback_stage

/// Candidate callback stage for the second global object list. The routine
/// scans entries in reverse order, skips flagged slots, and conditionally
/// filters by the candidate's signed 16-bit type index. It probes each item
/// with a helper; when that helper answers false, the item's +0xd0 object
/// method and a follow-up state helper run. This stage covers one unflagged
/// item with the global type-filter byte clear and the probe answering false.
/// The helper-true queue path and later child-object cleanup are not covered.
/// The cdecl entry receives four stack words; the second is a float whose bit
/// pattern is forwarded to the probe helper.
lf_checker_rt::export!(cdecl, rw_00b38740(
    probe_context: u32,
    probe_value: f32,
    _unused: u32,
    probe_tag: u32
) -> u32 {
    unsafe { run_candidate_callback_00b38740(probe_context, probe_value.to_bits(), probe_tag, false) }
});

const LIST_SECONDARY_00B38740: u32 = 0x018B_6F10;
const TYPE_TABLE_00B38740: u32 = 0x0129_5CD8;
const TYPE_FILTER_00B38740: u32 = 0x0104_598C;
const LIST_DATA_00B38740: u32 = 0;
const LIST_FLAGS_00B38740: u32 = 4;
const LIST_COUNT_00B38740: u32 = 8;
const LIST_STRIDE_00B38740: u32 = 12;
const TYPE_INDEX_00B38740: u32 = 0x2E;
const REQUIRED_KIND_00B38740: u32 = 2;
const VTABLE_UPDATE_00B38740: u32 = 0xD0;

#[inline(always)]
unsafe fn read_u8_00b38740(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn read_u16_00b38740(address: u32) -> u16 {
    unsafe { (address as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32_00b38740(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

unsafe fn run_candidate_callback_00b38740(
    probe_context: u32,
    probe_value_bits: u32,
    probe_tag: u32,
    mutant: bool,
) -> u32 {
    unsafe {
        let list = read_u32_00b38740(lf_checker_rt::global::<u32>(LIST_SECONDARY_00B38740) as u32);
        let mut index = read_u32_00b38740(list + LIST_COUNT_00B38740);
        let flags = read_u32_00b38740(list + LIST_FLAGS_00B38740);
        let data = read_u32_00b38740(list + LIST_DATA_00B38740);
        let stride = read_u32_00b38740(list + LIST_STRIDE_00B38740);
        let type_filter = read_u8_00b38740(lf_checker_rt::global::<u8>(TYPE_FILTER_00B38740) as u32);

        while index != 0 {
            index -= 1;
            if read_u8_00b38740(flags.wrapping_add(index)) & 0x80 != 0 {
                continue;
            }
            let item = data.wrapping_add(stride.wrapping_mul(index));
            if item == 0 {
                continue;
            }
            if type_filter != 0 {
                let signed_index = read_u16_00b38740(item + TYPE_INDEX_00B38740) as i16;
                let slot = (signed_index as i32).wrapping_mul(4) as u32;
                let descriptor = read_u32_00b38740(
                    lf_checker_rt::global::<u32>(TYPE_TABLE_00B38740) as u32 + slot,
                );
                if read_u32_00b38740(descriptor + 0x90) == REQUIRED_KIND_00B38740 {
                    continue;
                }
            }

            let helper_item = if mutant { item.wrapping_add(4) } else { item };
            let helper_answer = lf_checker_rt::callee_cdecl!(
                1,
                u32,
                helper_item,
                probe_context,
                probe_value_bits,
                probe_tag
            );
            if helper_answer as u8 == 0 {
                let vtable = read_u32_00b38740(item);
                let update_target = read_u32_00b38740(vtable + VTABLE_UPDATE_00B38740);
                let update: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(update_target as usize);
                let result = update(item, 0);
                let _ = lf_checker_rt::callee_thiscall!(3, u32, result);
            }
        }
        0
    }
}
