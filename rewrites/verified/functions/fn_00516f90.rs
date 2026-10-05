// original: 0x00516f90 leaderboard_leaf_init (proposed)
/// Leaderboard row initialiser: zero the key slots, link the payload, set live.
///
/// Zeroes 16 bytes at `this+0x18` (two 8-byte stores plus a redundant word
/// store the original keeps), points the slot at `this+0x28` at `this+0x30`,
/// sets bit 0 of the flag byte 0x100 past that payload, and returns `this`.
/// Thiscall, no stack arguments, no calls.
lf_checker_rt::export!(thiscall, rw_00516f90(this: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x18;
        const LINK_OFF: u32 = 0x28;
        const PAYLOAD_OFF: u32 = 0x30;
        const LIVE_OFF: u32 = 0x100;
        (this.wrapping_add(KEY_OFF) as *mut u64).write_unaligned(0);
        let payload = this.wrapping_add(PAYLOAD_OFF);
        (this.wrapping_add(KEY_OFF + 8) as *mut u64).write_unaligned(0);
        (this.wrapping_add(KEY_OFF + 8) as *mut u16).write_unaligned(0);
        let flag = payload.wrapping_add(LIVE_OFF) as *mut u8;
        flag.write(flag.read() | 1);
        (this.wrapping_add(LINK_OFF) as *mut u32).write_unaligned(payload);
        this
    }
});
