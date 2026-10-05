// original: 0x00c3e6f0 train_scale_vec3_and_submit (proposed)
/// Scale a three-word vector by a constant and submit it to helper id 1.
///
/// `this` (ECX) is passed through untouched (the original never writes
/// ECX, so the helper receives the entry value), `vec` points at three
/// float words. Multiplies each by the constant 0.01745329252 (loaded
/// from the binary; operand order pinned) into scratch, then calls
/// helper id 1 (thiscall/1: scratch pointer). The contract skips the
/// scratch address but snapshots the three words, so the products are
/// compared. Returns the helper's answer.
///
/// Original: 0x00c3e6f0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3e6f0(this: u32, vec: u32) -> u32 {
    unsafe {
        const SCALE_BITS: u32 = 0x3c8e_fa35; // 0.01745329252
        const SUBMIT: u32 = 1;
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        let k = f32::from_bits(SCALE_BITS);
        let mut s = [0u32; 3];
        s[0] = mul(f32::from_bits((vec as *const u32).read_unaligned()), k).to_bits();
        s[1] = mul(f32::from_bits(((vec + 4) as *const u32).read_unaligned()), k).to_bits();
        s[2] = mul(f32::from_bits(((vec + 8) as *const u32).read_unaligned()), k).to_bits();
        lf_checker_rt::callee_thiscall!(SUBMIT, u32, this, s.as_mut_ptr() as u32)
    }
});
