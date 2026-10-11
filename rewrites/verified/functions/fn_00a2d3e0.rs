// original: 0x00a2d3e0 CPlayerPed::vf80

/// Copy four 32-bit words from the supplied transform into ped offsets
/// `+0xec0..+0xecc`. The two float-shaped words are copied without arithmetic,
/// so signed zeros and NaN payloads remain unchanged. ECX is `this`; one stack
/// pointer is callee-cleaned, and EAX receives the fourth source word.
lf_checker_rt::export!(thiscall, rw_a2d3e0(this: u32, source: u32) -> u32 {
    const TRANSFORM: u32 = 0x0ec0;
    let source_words = source as *const u32;
    let destination = (this + TRANSFORM) as *mut u32;
    unsafe {
        let first = source_words.read_unaligned();
        let second = source_words.add(1).read_unaligned();
        let third = source_words.add(2).read_unaligned();
        let fourth = source_words.add(3).read_unaligned();
        destination.write_unaligned(first);
        destination.add(1).write_unaligned(second);
        destination.add(2).write_unaligned(third);
        destination.add(3).write_unaligned(fourth);
        fourth
    }
});
