// original: 0x00c3a8e0 train_count_cars_via_next (proposed)
/// Count the cars reachable through the `+0x14d4` link chain.
///
/// `this` (ECX) points to a train car; each car's dword at `+0x14d4`
/// points at the next car or is null. Returns the number of non-null
/// nodes starting at this car's link (0 when the link is null). The
/// chain must be null-terminated. No calls.
///
/// Original: 0x00c3a8e0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3a8e0(this: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x14d4;
        let mut cur = ((this + NEXT) as *const u32).read_unaligned();
        let mut n = 0u32;
        while cur != 0 {
            cur = ((cur + NEXT) as *const u32).read_unaligned();
            n += 1;
        }
        n
    }
});
