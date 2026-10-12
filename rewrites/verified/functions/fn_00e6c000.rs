// original: 0x00E6C000 veh_f64_convert_store
/// Read one image double, pass it to the helper, convert the helper's double
/// answer to single precision, and store that result in the output word.
/// The helper's input and answer use the low 64 bits of XMM0. The function
/// takes no arguments and has no observable return value.
use lf_checker_rt::{callee_addr, export, global};

export!(cdecl, rw_00e6c000() -> u32 {
    unsafe {
        const SOURCE_F64_VA: u32 = 0x00E9B9E0;
        const OUTPUT_F32_VA: u32 = 0x0171BC08;

        let input_bits = core::ptr::read_unaligned(global::<u64>(SOURCE_F64_VA));
        let helper: extern "cdecl" fn(u32, u32) -> u64 =
            core::mem::transmute(callee_addr(1) as usize);
        let answer_bits = helper(input_bits as u32, (input_bits >> 32) as u32);
        let answer = f64::from_bits(answer_bits);
        let output = {
            use core::arch::x86::{_mm_cvtss_f32, _mm_cvtsd_ss, _mm_set_sd, _mm_setzero_ps};
            _mm_cvtss_f32(_mm_cvtsd_ss(_mm_setzero_ps(), _mm_set_sd(answer)))
        };
        core::ptr::write_unaligned(global::<u32>(OUTPUT_F32_VA), output.to_bits());
        0
    }
});
