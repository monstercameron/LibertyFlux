// original: 0x00c6c740 rage::pgDictionary<rage::crAnimation>::vf0 (symbols)

/// Dictionary deleting destructor: destroy the contents, then free the
/// object itself when the low flag bit is set.
///
/// Runs the shared dictionary teardown on `this`; when `flags & 1` the
/// object is passed to the freeing callee. Returns `this` either way.
///
/// Original: thiscall with one stack word, two calls.
lf_checker_rt::export!(thiscall, rw_00c6c740(this: u32, flags: u32) -> u32 {
    unsafe {
        const TEARDOWN: u32 = 1;
        const FREE: u32 = 2;

        lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
        if flags & 1 != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, this);
        }
        this
    }
});
