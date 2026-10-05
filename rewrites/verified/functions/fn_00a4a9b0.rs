// original: 0x00a4a9b0 vehicle_pair_nonzero
/// True when either of the two head words of the object at `this` is nonzero.
///
/// The original compares `[this]` and `[this+4]` against zero and returns 1
/// in AL when either differs, 0 otherwise (thiscall, no stack arguments).
/// Only AL is compared: the upper bytes of EAX keep their entry value.
export!(thiscall, rw_00a4a9b0(this: u32) -> u32 {
    unsafe {
        let a = (this as *const u32).read_unaligned();
        let b = (this.wrapping_add(4) as *const u32).read_unaligned();
        (a != 0 || b != 0) as u32
    }
});
