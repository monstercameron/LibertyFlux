// original: 0x0059EB10 benchmark_run_hw_probes

/// Run the four machine-capacity probes in order and publish each float sample.
///
/// The cdecl routine reads two 32-bit stack parameters. It calls the first,
/// third, and fourth probes with those values in ECX and EDX, and calls the
/// second probe without stack arguments. Each helper returns a single-precision
/// sample in XMM0; the routine writes each sample's bits to the shared sample
/// global, then returns the final sample through the x87 float return channel.
/// The helpers are independent callees and their internal behavior is outside
/// this function's proof.
lf_checker_rt::export!(cdecl, rw_0059eb10(first_input: u32, second_input: u32) -> f32 {
    const SAMPLE_GLOBAL: u32 = 0x018B6E90;

    unsafe {
        let sample_slot = lf_checker_rt::global::<u32>(SAMPLE_GLOBAL);

        let first_sample_bits = lf_checker_rt::callee_fastcall!(
            1,
            u32,
            first_input,
            second_input
        );
        sample_slot.write_unaligned(first_sample_bits);

        let second_sample_bits = lf_checker_rt::callee_cdecl!(2, u32,);
        sample_slot.write_unaligned(second_sample_bits);

        let third_sample_bits = lf_checker_rt::callee_fastcall!(
            3,
            u32,
            first_input,
            second_input
        );
        sample_slot.write_unaligned(third_sample_bits);

        let final_sample_bits = lf_checker_rt::callee_fastcall!(
            4,
            u32,
            first_input,
            second_input
        );
        sample_slot.write_unaligned(final_sample_bits);
        f32::from_bits(final_sample_bits)
    }
});
