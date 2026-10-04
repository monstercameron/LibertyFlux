// original: 0x009a47e0 audio_detach_source
/// Original 0x009a47e0 (unnamed): detach a source from its owner slot.
///
/// Notifies the helper about `arg` when it is nonzero; when `arg` still
/// matches the owner slot at +0x2c, runs the global reset helper if the mode
/// byte is clear, zeroes the gate dword, and clears the owner slot. The
/// original leaves entry EAX on some paths, so no return value is compared.
export!(thiscall, rw_009a47e0(this_: u32, arg: u32) -> u32 {
    if arg != 0 {
        callee_cdecl!(1, u32, arg.wrapping_add(0xa20));
    }
    let owner = unsafe { ((this_ + 0x2c) as *const u32).read() };
    if arg == owner {
        let mode = unsafe { (relocated(0x012845C8) as *const u8).read() };
        if mode == 0 {
            callee_cdecl!(2, u32,);
            unsafe { (relocated(0x012845C4) as *mut u32).write(0) };
        }
        unsafe { ((this_ + 0x2c) as *mut u32).write(0) };
    }
    0
});
