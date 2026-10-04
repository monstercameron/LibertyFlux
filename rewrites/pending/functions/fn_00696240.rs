// original: 0x00696240 rage::crAnimChannel::vf7
/// Forward an indexed sample to virtual slot 7 after float conversion.
///
/// Converts the unsigned sample index to float, adds the bias, and forwards
/// the result plus the trailing argument to the next override in the chain.
export!(thiscall, rs80_696240(this: *const u8, n: u32, fbits: u32, extra: u32) -> u32 {
    unsafe {
        let f = (n as f32) + f32::from_bits(fbits);
        let vtbl = *((this).add(0) as *const u32) as *const u8;
        ({
        let __a = *((vtbl.add(0x08)) as *const u32);
        let __f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(__a as usize);
        __f(this as u32, f.to_bits(), extra)
    })
    }
});
