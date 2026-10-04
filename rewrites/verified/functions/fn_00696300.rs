// original: 0x00696300 rage::crAnimChannel::vf10
/// Forward an indexed sample to virtual slot 10 after float conversion.
export!(thiscall, rs80_696300(this: *const u8, n: u32, fbits: u32, extra: u32) -> u32 {
    unsafe {
        let f = (n as f32) + f32::from_bits(fbits);
        let vtbl = *((this).add(0) as *const u32) as *const u8;
        ({
        let __a = *((vtbl.add(0x14)) as *const u32);
        let __f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(__a as usize);
        __f(this as u32, f.to_bits(), extra)
    })
    }
});
