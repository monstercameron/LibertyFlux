// original: 0x0097C1E0 audio_tune_push_sum
/// Push a tuning packet holding base plus adjustment through the interface.
///
/// Builds an 8-byte packet (sum float, zero word, tag byte) in a local and
/// passes its address; the spare byte is zero, matching the checker's defined
/// stack fill on the original side. A null object faults, like the original.
/// cdecl(ignored, obj).
export!(cdecl, rw_s103_97c1e0(_a0: u32, a1: *const u8) -> u32 {
    unsafe {
        let base = f32::from_bits(*global::<u32>(0x1038DF4));
        let adj = f32::from_bits(*global::<u32>(0xFE88E8));
        let tag = *a1.add(0x210);
        let mut pkt = [0u32; 2];
        pkt[0] = (base + adj).to_bits();
        pkt[1] = (tag as u32) << 16;
        callee_thiscall!(1, u32, (a1 as usize as u32).wrapping_add(0x570), pkt.as_ptr() as usize as u32)
    }
});
