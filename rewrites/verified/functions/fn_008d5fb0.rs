// original: 0x008D5FB0 cached_record_key_equal
/// Compare a cached key or the key returned by the property lookup against
/// the value reached through the supplied pointer chain. The Boolean occupies
/// AL while the upper EAX bytes retain the cached or helper pointer value.
lf_checker_rt::export!(cdecl, rw_008d5fb0(key_root: u32, owner: u32) -> u32 {
    const PROPERTY_LOOKUP: u32 = 1;
    unsafe {
        let expected = ((key_root as *const u32).read_unaligned() as *const u32)
            .wrapping_add(3).read_unaligned();
        let cache = (owner.wrapping_add(0x2C4) as *const u32).read_unaligned();
        let (candidate, compared) = if cache != 0 {
            (cache, cache)
        } else {
            let lookup_context = (owner.wrapping_add(0x224) as *const u32).read_unaligned()
                .wrapping_add(0x44);
            let first = lf_checker_rt::callee_thiscall!(PROPERTY_LOOKUP, u32, lookup_context, 0x13B);
            if first == 0 {
                (0, 0)
            } else {
                let second = lf_checker_rt::callee_thiscall!(PROPERTY_LOOKUP, u32, lookup_context, 0x13B);
                let value = (second.wrapping_add(0x14) as *const u32).read_unaligned();
                (second, value)
            }
        };
        (candidate & 0xFFFF_FF00) | u32::from(compared == expected)
    }
});
