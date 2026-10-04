// original: 0x00696280 rage::crAnimChannel::vf8
/// Forward an indexed sample to virtual slot 8 after float conversion.
export!(thiscall, rs80_696280(this: *const u8, n: u32, fbits: u32, extra: u32) -> u32 {
    unsafe {
        let f = (n as f32) + f32::from_bits(fbits);
        let vtbl = *((this).add(0) as *const u32) as *const u8;
        ({
        let __a = *((vtbl.add(0x0C)) as *const u32);
        let __f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(__a as usize);
        __f(this as u32, f.to_bits(), extra)
    })
    }
});
