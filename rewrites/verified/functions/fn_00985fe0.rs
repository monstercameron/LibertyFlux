// original: 0x00985fe0 audio_query_entity_flag
/// Original 0x00985fe0 (unnamed): look up an entity and test two flag bits.
///
/// Looks up the record keyed by the dword at +0x20 through the global audio
/// manager; returns 1 when the record exists and bits 6..7 of the byte at
/// +0x5 equal `01`, else 0.
export!(stdcall, rw_00985fe0(obj: u32) -> u32 {
    let key = unsafe { ((obj + 0x20) as *const u32).read() };
    let ent = callee_thiscall!(1, u32, relocated(0x0115D9A0), key);
    if ent == 0 {
        return 0;
    }
    let w = unsafe { ((ent + 5) as *const u32).read_unaligned() };
    if (w as u8) & 0xc0 == 0x40 { 1 } else { 0 }
});
