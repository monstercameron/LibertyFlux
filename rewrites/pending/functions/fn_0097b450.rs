// original: 0x0097b450 audio_voice_resolve_and_read
/// Resolve a voice handle and read one of its parameters.
///
/// Looks the handle up in the global voice table, substitutes the
/// override handle when the bank flag at +0x1a0 is set, and reads
/// parameter 4 through the bank reader. A null handle reads as 0.
export!(thiscall, rw_0097b450(this: *const u8, arg: *const u8) -> u32 {
    unsafe {
        let w = core::ptr::read_unaligned((arg.add(0x21)) as *const u32);
        let ans = callee_thiscall!(1, u32, relocated(0x115D9A0), w);
        let mut target = ans;
        if *this.add(0x1a0) != 0 {
            target = *global::<u32>(0x1231318);
        }
        if target == 0 {
            return 0;
        }
        let m78 = *((this.add(0x78)) as *const u32);
        callee_thiscall!(2, u32, this as u32, target, 4, m78)
    }
});
