// original: 0x00B696E0 heli_get_linked_field_1434

/// Return the word at `this + 0x1434` when the two link words at `+0x38` and
/// `+0x1430` are equal; otherwise return zero. The object uses a 32-bit game
/// layout, and the original takes only its object pointer in ECX.
lf_checker_rt::export!(thiscall, rw_00B696E0(this: u32) -> u32 {
    unsafe {
        const LINK_LEFT: u32 = 0x38;
        const LINK_RIGHT: u32 = 0x1430;
        const LINKED_VALUE: u32 = 0x1434;
        let left = (this.wrapping_add(LINK_LEFT) as *const u32).read_unaligned();
        let right = (this.wrapping_add(LINK_RIGHT) as *const u32).read_unaligned();
        if left == right {
            (this.wrapping_add(LINKED_VALUE) as *const u32).read_unaligned()
        } else {
            0
        }
    }
});
