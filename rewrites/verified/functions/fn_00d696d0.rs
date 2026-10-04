// original: 0x00d696d0 status_byte_or_zero
// s16f08: read the linked record's status byte, or zero (thiscall/0).
//
// Returns the byte zero-extended in the low 8 bits. The upper 24 bits of
// EAX keep the link pointer's own high bytes (a partial-register merge in
// the original), reproduced exactly.
// (v2 port: black_box defeats a rustc 1.96.1 -O fold of the plain
// merge into a zero-extending byte load; verified by asm inspection.)
export!(thiscall, rw_s16f08(this: *const u8) -> u32 {
    unsafe {
        let linked = *((this.add(0xC)) as *const u32);
        if linked == 0 {
            return 0;
        }
        let status = *((linked.wrapping_add(0xFC)) as *const u8);
        core::hint::black_box(linked & 0xFFFFFF00) | status as u32
    }
});
