// original: 0x0097E8D0 ped_task_write_indexed_table_value

/// ped task write indexed table value using the ped's type and group bytes.
///
/// The object stores an index byte at `+0x04` and a group byte at
/// `+0x40`. Two relocated globals provide the element stride and the
/// base of a two-dimensional table. Each row is spaced by `0x6f40`;
/// the dword at row offset `0x6f14` is added to the index-scaled
/// value. The first function writes the stack word at result `+0xcc`;
/// the second passes that result and its stack float to a helper.
/// The `0xff` index selects a null destination; in the setter it
/// reproduces the original's faulting write.
lf_checker_rt::export!(thiscall, rw_0097E8D0(this: u32, value: u32) -> u32 {
    const PED_ID: u32 = 0x04;
    const PED_GROUP: u32 = 0x40;
    const PED_VALUE: u32 = 0xCC;
    const GROUP_STRIDE: u32 = 0x6F40;
    const TABLE_ROW: u32 = 0x6F14;
    const TARGET_INDEX_SCALE: u32 = 4;
    const SCALE_GLOBAL_VA: u32 = 0x0115D968;
    const TABLE_GLOBAL_VA: u32 = 0x0115D988;
    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    let ped_index = unsafe { read_u8(this.wrapping_add(PED_ID)) };
    let target_base = if ped_index == u8::MAX {
        0
    } else {
        let group = unsafe { read_u8(this.wrapping_add(PED_GROUP)) } as u32;
        let scale = unsafe { lf_checker_rt::global::<u32>(SCALE_GLOBAL_VA).read() };
        let table = unsafe { lf_checker_rt::global::<u32>(TABLE_GLOBAL_VA).read() };
        let row = table.wrapping_add(group.wrapping_mul(GROUP_STRIDE)).wrapping_add(TABLE_ROW);
        let row_base = unsafe { read_u32(row) };
        let target_index = (ped_index as u32).wrapping_mul(scale);
        row_base.wrapping_add(target_index)
    };
    let destination = target_base.wrapping_add(PED_VALUE);
    unsafe { (destination as *mut u32).write_volatile(value); }
    value
});
