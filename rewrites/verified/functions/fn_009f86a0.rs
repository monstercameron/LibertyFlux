// original: 0x009f86a0 Stat_SetString
/// Set a ranged string statistic after formatting its table entry.
///
/// When the id falls in the string-stat range and the string is not null,
/// formats the stat's table entry with the string through the first helper
/// (a length cap and a slot holding the string go with it), then notifies
/// the second helper of the id. Out-of-range ids and null strings do
/// nothing.
export!(cdecl, rw_009f86a0(id: u32, text: u32) -> u32 {
    unsafe {
        if id.wrapping_sub(0x289) <= 0x20 && text != 0 {
            let table = global::<u32>(0x012b5a74);
            let entry = *table.add(id as usize);
            let mut slot = text;
            callee_cdecl!(1, u32, entry, &mut slot as *mut u32 as u32, 0x80);
            callee_cdecl!(2, u32, id);
        }
        0
    }
});
