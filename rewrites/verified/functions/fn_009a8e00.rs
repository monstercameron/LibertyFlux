// original: 0x009a8e00 flag_fetch_or_default
/// Fetch the flag object for `id`, building a default when null.
///
/// When `id` is null the default builder (stubbed, cdecl/1) supplies
/// the object; otherwise `id` is used directly. Either way the object
/// is passed in ECX to the flag fetcher (stubbed, thiscall/1) with
/// `slot`, whose answer is returned. Stdcall, two stack words.
export!(stdcall, rw_009A8E00(id: u32, slot: u32) -> u32 {
    unsafe {
        let obj = if id == 0 { callee_cdecl!(1, u32, 0) } else { id };
        callee_thiscall!(2, u32, obj, slot)
    }
});
