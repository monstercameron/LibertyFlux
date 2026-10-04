// original: 0x00948360 forward_sample_packet
/// Forward a seven-sample packet plus a tag through the sample sink.
///
/// The samples travel as bits (copied, never arithmetic): six are shuffled
/// into a scratch block handed to the sink by pointer, and the seventh rides
/// along as the fourth stack word. A sink answer above 31 (unsigned) fails
/// with -1; otherwise the answer and a fixed mode go to the follow-up call
/// whose result is returned.
lf_checker_rt::export!(
    cdecl,
    rw_00948360(f0: u32, f1: u32, f2: u32, f3: u32, f4: u32, f5: u32, f6: u32, a7: u32) -> u32 {
        unsafe {
            let scratch: [u32; 8] = [f3, f4, f5, 0, f0, f1, f2, 0];
            let p1 = (&scratch[0] as *const u32) as u32;
            let p2 = (&scratch[4] as *const u32) as u32;
            let r = lf_checker_rt::callee_cdecl!(1, u32, a7, p2, p1, f6);
            if r > 0x1f {
                0xffff_ffff
            } else {
                lf_checker_rt::callee_cdecl!(2, u32, r, 3)
            }
        }
    }
);
