// original: 0x00C6F780 NativeImpl_SET_ANIM_GROUP_FOR_CHAR_2

/// Scan the shared animation-group table in index order and compare each record's leading name pointer with the requested name. Return the first matching zero-based index, or -1 when the signed group count is nonpositive or no name matches.
lf_checker_rt::export!(stdcall, rw_0x00c6f780(requested_name: u32) -> u32 {
    const GROUP_COUNT_GLOBAL:u32=0x016D_D638; const GROUP_TABLE_GLOBAL:u32=0x016D_D640; const GROUP_STRIDE:u32=0x58;
    unsafe { let count=lf_checker_rt::global::<u32>(GROUP_COUNT_GLOBAL).read_unaligned() as i32; if count<=0{return u32::MAX;} let table=lf_checker_rt::global::<u32>(GROUP_TABLE_GLOBAL).read_unaligned(); let mut i=0i32; while i<count { let candidate=table.wrapping_add((i as u32).wrapping_mul(GROUP_STRIDE)); if lf_checker_rt::callee_cdecl!(1,u32,requested_name,candidate)==0{return i as u32;} i+=1;} u32::MAX }
});
