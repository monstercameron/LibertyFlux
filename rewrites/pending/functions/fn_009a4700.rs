// original: 0x009a4700 NativeImpl_RETUNE_RADIO_TO_STATION_INDEX
/// Original 0x009a4700 (unnamed): store the argument into the object.
///
/// Writes `val` to the dword slot at +0x80 of `this` and returns it.
export!(thiscall, rw_009a4700(this_: u32, val: u32) -> u32 {
    unsafe { ((this_ + 0x80) as *mut u32).write(val) };
    val
});
