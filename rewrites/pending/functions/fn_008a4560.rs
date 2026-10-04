// original: 0x008a4560 audSound_voice_forward_or_neg1
/// Forward to the voice callee, or -1 when this sound has no voice.
///
/// Original 0x008A4560 (`cdecl(obj, arg)`): resolves the voice pointer
/// like rw_008a3b70; returns 0xFFFFFFFF when the selector is 0xFF or the
/// resolved pointer is null, else the callee's answer.
export!(cdecl, rw_008a4560(obj: u32, arg: u32) -> u32 {    let id = unsafe { ((obj + 0x48) as *const u8).read_unaligned() } as u32;
    if id == 0xFF {
        return 0xFFFFFFFF;
    }
    let bank = unsafe { ((obj + 0x40) as *const u8).read_unaligned() } as u32;
    let stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
    let base = unsafe { global::<u32>(0x115d988).read_unaligned() };
    let entry = unsafe {
        (base
            .wrapping_add(bank.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10) as *const u32)
            .read_unaligned()
    };
    let slot = entry.wrapping_add(stride.wrapping_mul(id));
    if slot == 0 {
        return 0xFFFFFFFF;
    }
    callee_thiscall!(1, u32, slot, arg)
});
