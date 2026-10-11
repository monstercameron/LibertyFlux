// original: 0x009BB040 CCamScriptInstruction_SetTelescopeCamAngleLimits::vf2

/// Store the six telescope angle limits from this instruction.
///
/// `this` contains six single precision values at offsets `+0x08` through
/// `+0x1C`. The first four values are multiplied by the global angle scale;
/// the last two are copied bit for bit. The products are formed in the
/// original input-times-scale operand order. This method has no value result;
/// EAX is left as incidental incoming state by the original.
lf_checker_rt::export!(thiscall, rw_009bb040(this: u32) -> () {
    unsafe {
        const SCALE_VA: u32 = 0x00fe_8728;
        const OUTPUT_VAS: [u32; 6] = [
            0x0104_9080, 0x0104_9084, 0x0104_9088,
            0x0104_908c, 0x0104_9090, 0x0104_9094,
        ];
        let scale_bits = lf_checker_rt::global::<u32>(SCALE_VA).read_unaligned();
        let scale = f32::from_bits(scale_bits);
        for (index, input_offset) in [8u32, 12, 16, 20].into_iter().enumerate() {
            let input_bits = (this.wrapping_add(input_offset) as *const u32).read_unaligned();
            let input = f32::from_bits(input_bits);
            let product = core::hint::black_box(input) * core::hint::black_box(scale);
            lf_checker_rt::global::<u32>(OUTPUT_VAS[index]).write_unaligned(product.to_bits());
        }
        for index in 4..6 {
            let input_bits = (this.wrapping_add(8 + index as u32 * 4) as *const u32).read_unaligned();
            lf_checker_rt::global::<u32>(OUTPUT_VAS[index]).write_unaligned(input_bits);
        }
    }
});
