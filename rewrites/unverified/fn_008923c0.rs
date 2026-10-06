// original: 0x008923C0 rage::audSound::vf6
/// Virtual slot 6: always answers false (zero byte).
///
/// Writes a zero byte to a scratch stack slot and returns it in AL. Only the
/// low byte of the return is meaningful; the upper 24 bits keep the caller's
/// entry EAX, so only AL is compared. No arguments, no memory effects.
/// Original: 0x008923C0 (thiscall, no stack arguments).
export!(thiscall, rw_008923C0(this: *mut u8) -> u32 {
    let _ = this;
    0
});
