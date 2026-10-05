// original: 0x009a8e20 flag_byte_set
/// Release the cached helper, then mark the flag block for `id` live.
///
/// When the helper pointer at `this+0x3ab0` is non-null it is released
/// (stubbed, thiscall/1 with a zero argument). Then the flag block for
/// `id` is used directly, or built by the default builder (stubbed,
/// cdecl/1) when `id` is null, and its marker byte at `+0xd30` is set
/// to 0xff. Thiscall, one stack word, no result.
export!(thiscall, rw_009A8E20(this: u32, id: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 0x3ab0;
        const MARKER: u32 = 0xd30;
        const LIVE: u8 = 0xff;
        let helper = ((this + HELPER) as *const u32).read_unaligned();
        if helper != 0 {
            let _: u32 = callee_thiscall!(1, u32, helper, 0);
        }
        let base = if id == 0 { callee_cdecl!(2, u32, 0) } else { id };
        ((base + MARKER) as *mut u8).write(LIVE);
        0
    }
});
