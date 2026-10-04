// original: 0x009a46d0 audio_store_record_tag
/// Original 0x009a46d0 (unnamed): resolve a record and store its tag byte.
///
/// Looks up the record for `arg`; when found, stores the tag byte at +0x1918
/// into the slot at +0x80 of `this` and returns it, else returns 0.
export!(thiscall, rw_009a46d0(this_: u32, arg: u32) -> u32 {
    let rec = callee_cdecl!(1, u32, arg);
    if rec == 0 {
        return 0;
    }
    let tag = unsafe { ((rec + 0x1918) as *const u8).read() };
    callee_thiscall!(2, u32, this_, tag as u32)
});
