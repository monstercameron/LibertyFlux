// original: 0x006998a0 rage::crAnimChannelQuantizeFloat::vf9
/// Quantized-float sampler: dequantizes the stored integers at index i and
/// i+1 (unsigned int via a signed convert plus the 2^32 table fixup, scaled
/// by the gain and bias at +0x14/+0x18) and writes the linear blend at
/// parameter t to the output. Returns the output pointer.
lf_k2_rt::export!(thiscall, rw_006998a0(this: *mut u8, i: u32, t_bits: u32, out: *mut u32) -> u32 {
    unsafe {
        let tab = lf_k2_rt::global::<f64>(0x00FE8F50);
        let scale = f32::from_bits(*((this.add(0x14)) as *const u32));
        let bias = f32::from_bits(*((this.add(0x18)) as *const u32));
        let quant = (this as u32).wrapping_add(8);
        let q0: u32 = lf_k2_rt::callee_thiscall!(1, u32, quant, i);
        let v0 = ((q0 as i32) as f64 + *tab.add((q0 >> 31) as usize)) as f32 * scale + bias;
        let q1: u32 = lf_k2_rt::callee_thiscall!(2, u32, quant, i.wrapping_add(1));
        let v1 = ((q1 as i32) as f64 + *tab.add((q1 >> 31) as usize)) as f32 * scale + bias;
        let t = f32::from_bits(t_bits);
        *out = ((v1 - v0) * t + v0).to_bits();
        out as u32
    }
});
