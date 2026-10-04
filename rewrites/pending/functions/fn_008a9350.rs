// original: 0x008a9350 audeffect_advance_slot
/// Advance the effect slot: follow the chain link or bump the counter.
///
/// When the link at `this+8` is non-null the value comes from the chained
/// call (thiscall/0 on the link, stubbed by the checker); otherwise it is
/// the counter at `this+0x24` plus one. Either way the value is stored at
/// `this+0x28` and returned.
export!(thiscall, rw_008a9350(this: *mut u8) -> u32 {
    unsafe {
        let next = *(this.add(0x08) as *const u32);
        let value = if next != 0 {
            callee_thiscall!(1, u32, next)
        } else {
            (*(this.add(0x24) as *const u32)).wrapping_add(1)
        };
        *(this.add(0x28) as *mut u32) = value;
        value
    }
});
