// original: 0x008a3b70 audSound_voice_query
/// Look up this sound's voice slot and ask the callee about it.
///
/// Original 0x008A3B70 (`cdecl(obj, arg)`): unless the selector byte at
/// `obj+0x48` is 0xFF, resolves the voice pointer from the audio bank
/// table (`base[idx*0x6F40+0x6F10] + stride*id`, with the bank index from
/// `obj+0x40`) and calls the callee with it as `this`; returns the
/// callee's answer with only the low byte tested (`(ans & ~0xFF) |
/// (low != 0)`).
export!(cdecl, rw_008a3b70(obj: u32, arg: u32) -> u32 {    let id = unsafe { ((obj + 0x48) as *const u8).read_unaligned() } as u32;
    let slot = if id == 0xFF {
        0
    } else {
        let bank = unsafe { ((obj + 0x40) as *const u8).read_unaligned() } as u32;
        let stride = unsafe { global::<u32>(0x115d964).read_unaligned() };
        let base = unsafe { global::<u32>(0x115d988).read_unaligned() };
        let entry = unsafe {
            (base
                .wrapping_add(bank.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10) as *const u32)
                .read_unaligned()
        };
        entry.wrapping_add(stride.wrapping_mul(id))
    };
    let ans = callee_thiscall!(1, u32, slot, arg);
    (ans & 0xFFFFFF00) | (((ans & 0xFF) != 0) as u32)
});
