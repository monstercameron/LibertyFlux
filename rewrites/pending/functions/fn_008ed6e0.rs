// original: 0x008ed6e0 packed_flags_forwarder
/// Unpack two flag bytes into a loader call.
///
/// Splits `this[5]` into two 3-bit fields and `this[6]` into a 4-bit
/// height plus a top-bit flag, forwarding all four to the engine
/// routine (stubbed by the checker) and returning its answer.
export!(thiscall, rw_008ed6e0(this: *const u8) -> u32 {
    unsafe {
        let u = *(this.add(5) as *const u8);
        let v = *(this.add(6) as *const u8);
        let h = (v & 0xF) as u32 as f32;
        callee_cdecl!(
            1,
            u32,
            (u & 7) as u32,
            ((u >> 3) & 7) as u32,
            h.to_bits(),
            (v >> 7) as u32
        )
    }
});
