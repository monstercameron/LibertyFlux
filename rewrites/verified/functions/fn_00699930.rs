// original: 0x00699930 rage::crAnimChannelQuantizeFloat::vf12
/// Quantized-float pair fetch: reads the stored integers at index i and
/// i+1 and writes them interleaved with zero words (four dwords total) to
/// the output record. The middle argument is unused. Returns the second
/// integer.
lf_k2_rt::export!(thiscall, rw_00699930(this: *mut u8, i: u32, _unused: u32, out: *mut u8) -> u32 {
    unsafe {
        let quant = (this as u32).wrapping_add(8);
        let v0: u32 = lf_k2_rt::callee_thiscall!(1, u32, quant, i);
        *(out as *mut u32) = v0;
        *((out.add(4)) as *mut u32) = 0;
        let v1: u32 = lf_k2_rt::callee_thiscall!(2, u32, quant, i.wrapping_add(1));
        *((out.add(8)) as *mut u32) = v1;
        *((out.add(0x0c)) as *mut u32) = 0;
        v1
    }
});
