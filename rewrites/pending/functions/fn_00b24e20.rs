// original: 0x00b24e20 network_id_match
// s08_b24e20: network-aware id match. thiscall/1 (id: u32) -> u32.
// Asks the network-session helper (thiscall/0); when in a session and the
// stored id word at +0x11C is zero, ids 0x1F..=0x3E and 0x3F..=0x46 match
// directly, otherwise the low word must equal the stored word. The low byte
// is the boolean result; the upper three bytes are carried over from the
// helper's answer (or from id-0x3F on the range path), exactly as the
// original's byte-wide writes leave them.
export!(thiscall, rw_b24e20(this: *mut u8, id: u32) -> u32 {
    unsafe {
        let in_session: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ans = in_session(this as u32);
        let stored = *(this.add(0x11C) as *const u16);
        if ans & 0xFF != 0 && stored == 0 {
            let w = id as u16;
            if (0x1Fu16..=0x3E).contains(&w) {
                return (ans & 0xFFFF_FF00) | 1;
            }
            let t = id.wrapping_sub(0x3F);
            if (t & 0xFFFF) <= 7 {
                return (t & 0xFFFF_FF00) | 1;
            }
            let eq = u32::from(stored == w);
            return (t & 0xFFFF_FF00) | eq;
        }
        let eq = u32::from(stored == id as u16);
        (ans & 0xFFFF_FF00) | eq
    }
});
