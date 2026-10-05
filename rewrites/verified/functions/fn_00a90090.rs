// original: 0x00a90090 stream_vector_publish

/// Publishes a 4-float vector and a tag into streaming globals.
///
/// Copies `vec[0..4]` to the globals at file VA 0x12FB240..0x12FB24C,
/// raises the ready byte at 0x12FB21D, stores `n`'s low byte at 0x12FB21E
/// and 0x14 at 0x12FB224, then calls the consumer (callee 1, cdecl) with
/// (`vec`, `n`). When the consumer's low answer byte is non-zero the
/// ready byte is cleared again. Returns the consumer's answer. One call.
/// Original: 0x00A90090 (cdecl, two stack words), 112 bytes.
lf_checker_rt::export!(cdecl, rw_00a90090(vec: u32, n: u32) -> u32 {
    unsafe {
        const READY: u32 = 0x12FB21D;
        const TAG: u32 = 0x12FB21E;
        const KIND: u32 = 0x12FB224;
        const VEC_BASE: u32 = 0x12FB240;
        const KIND_VALUE: u32 = 0x14;
        const CONSUME: u32 = 1;
        (lf_checker_rt::global::<u8>(READY) as *mut u8).write(1);
        for i in 0..4u32 {
            let w = (vec.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            (lf_checker_rt::global::<u32>(VEC_BASE).wrapping_add(i as usize))
                .write_unaligned(w);
        }
        (lf_checker_rt::global::<u8>(TAG) as *mut u8).write(n as u8);
        (lf_checker_rt::global::<u32>(KIND) as *mut u32).write_unaligned(KIND_VALUE);
        let ans: u32 = lf_checker_rt::callee_cdecl!(CONSUME, u32, vec, n);
        let flag = (lf_checker_rt::global::<u8>(READY) as *const u8).read();
        let kept = if (ans & 0xFF) != 0 { 0 } else { flag };
        (lf_checker_rt::global::<u8>(READY) as *mut u8).write(kept);
        ans
    }
});
