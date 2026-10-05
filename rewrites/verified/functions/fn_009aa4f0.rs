// original: 0x009aa4f0 table_id_of_mapped
/// Map `key` through the id mapper, then find its Stride25 row.
///
/// The key mapper (stubbed, cdecl/2 with a zero second argument) turns
/// `key` into a row id, which is passed to the row search (stubbed,
/// thiscall/1 on this object) whose answer is returned. Thiscall, one
/// stack word, dword result.
export!(thiscall, rw_009AA4F0(this: u32, key: u32) -> u32 {
    unsafe {
        let id: u32 = callee_cdecl!(1, u32, key, 0);
        callee_thiscall!(2, u32, this, id)
    }
});
