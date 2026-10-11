// original: 0x00C6F630 anim_group_find_member

/// Search the selected group's signed member count in order. Each candidate name is compared case-insensitively with the requested name; on a match return the first word of that member's 12-byte result record, otherwise return -1.
lf_checker_rt::export!(stdcall, rw_0x00c6f630(group_index: u32, requested_name: u32) -> u32 {
    const GROUP_TABLE_GLOBAL:u32=0x016D_D640; const GROUP_STRIDE:u32=0x58; const COUNT:u32=0x38; const NAMES:u32=0x3c; const RESULTS:u32=0x44; const RESULT_STRIDE:u32=12;
    unsafe {
        let table=lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned();
        let group=table.wrapping_add(group_index.wrapping_mul(GROUP_STRIDE)) as *const u8;
        let count=group.add(COUNT as usize).cast::<i32>().read_unaligned();
        if count<=0 { return u32::MAX; }
        let name_table=group.add(NAMES as usize).cast::<u32>().read_unaligned() as *const u32;
        let result_table=group.add(RESULTS as usize).cast::<u32>().read_unaligned();
        let mut idx=0i32;
        while idx<count {
            let candidate=name_table.add(idx as usize).read_unaligned();
            if lf_checker_rt::callee_cdecl!(1,u32,requested_name,candidate)==0 {
                return (result_table.wrapping_add((idx as u32).wrapping_mul(RESULT_STRIDE)) as *const u32).read_unaligned();
            }
            idx+=1;
        }
        u32::MAX
    }
});
