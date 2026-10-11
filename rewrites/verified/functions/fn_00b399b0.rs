// original: 0x00B399B0 ped_task_state_snapshot

/// Copies the selected task-state snapshot into caller-owned output records.
///
/// In the proven mode, the two global float inputs are widened to doubles,
/// the second input's sign is flipped, and both are passed to the double
/// heading helper. Its result is rounded to `f32` and passed to the smoothing
/// helper. The function then copies the primary vector, state word, secondary
/// vector and final vector into the output pointers and clears the output flag.
///
/// The proof fixes the mode selector to the nonzero path, fixes the follow-up
/// selector to the global-vector copy path, and scripts the smoothing result
/// to zero. The alternate selector paths and above-threshold path are not
/// claimed by this rewrite's contract. The double call compares the low eight
/// bytes of each input vector register because those bytes contain the two
/// arguments consumed by the callee.
lf_checker_rt::export!(cdecl, rw_00B399B0(
    out_vector: u32,
    out_heading: u32,
    out_state: u32,
    out_secondary: u32,
    out_final_vector: u32,
    out_flag: u32
) -> u32 {
    unsafe {
        const MODE_WORD: u32 = 0x0166_2478;
        const FINAL_MODE: u32 = 0x0166_2490;
        const INPUT_X: u32 = 0x0166_2520;
        const INPUT_Y: u32 = 0x0166_2524;
        const STATE_WORD: u32 = 0x0166_248C;
        const PRIMARY_VECTOR: u32 = 0x0166_2530;
        const SECONDARY_VECTOR: u32 = 0x0166_24F0;
        const FINAL_VECTOR: u32 = 0x0166_2500;
        const ATAN_ID: u32 = 1;
        const HEADING_ID: u32 = 2;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn copy_global_words(source_va: u32, destination: u32) {
            for word in 0..4u32 {
                let value = unsafe {
                    lf_checker_rt::global::<u32>(source_va.wrapping_add(word * 4)).read()
                };
                unsafe { write_u32(destination.wrapping_add(word * 4), value) };
            }
        }
        #[inline(always)]
        unsafe fn copy_output_words(source: u32, destination: u32) {
            for word in 0..4u32 {
                let value = unsafe {
                    (source.wrapping_add(word * 4) as *const u32).read_unaligned()
                };
                unsafe { write_u32(destination.wrapping_add(word * 4), value) };
            }
        }

        let mode = (lf_checker_rt::global::<u32>(MODE_WORD).read() >> 24) as u8;
        if mode == 0 {
            return 0;
        }

        let x_bits = lf_checker_rt::global::<u32>(INPUT_X).read();
        let y_bits = lf_checker_rt::global::<u32>(INPUT_Y).read() ^ 0x8000_0000;
        let x_double = (f32::from_bits(x_bits) as f64).to_bits();
        let y_double = (f32::from_bits(y_bits) as f64).to_bits();
        let atan_bits: u64 = lf_checker_rt::callee_cdecl!(
            ATAN_ID,
            u64,
            y_double as u32,
            (y_double >> 32) as u32,
            x_double as u32,
            (x_double >> 32) as u32
        );
        let atan_heading = core::hint::black_box(f64::from_bits(atan_bits) as f32);
        let smoothed_heading: f32 = lf_checker_rt::callee_cdecl!(
            HEADING_ID,
            f32,
            atan_heading.to_bits()
        );

        unsafe { write_u32(out_heading, smoothed_heading.to_bits()) };
        unsafe { copy_global_words(PRIMARY_VECTOR, out_vector) };
        unsafe {
            write_u32(
                out_state,
                lf_checker_rt::global::<u32>(STATE_WORD).read(),
            )
        };
        unsafe { copy_global_words(SECONDARY_VECTOR, out_secondary) };
        unsafe { copy_output_words(out_vector, out_final_vector) };
        if lf_checker_rt::global::<u32>(FINAL_MODE).read() as u8 != 0 {
            unsafe { copy_global_words(FINAL_VECTOR, out_final_vector) };
        }
        unsafe { (out_flag as *mut u8).write(0) };
        0
    }
});
