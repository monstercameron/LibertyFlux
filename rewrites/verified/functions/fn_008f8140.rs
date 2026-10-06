// original: 0x008f8140 input_sampler_update (proposed)

/// Refresh the sampler through five callees when enabled.
///
/// The entry ECX is threaded into the first two callees (thiscall, no stack
/// words). When the global enable byte is zero the function returns with
/// EAX untouched, so the contract compares no return channel. Otherwise
/// three scratch buffers are handed out: the fill callee takes one
/// scratch pointer; the query callee takes a second pointer with the
/// constant 8, then a third pointer with 0xB, and its second answer (a
/// float pointer) is read once into a dead stack slot; the commit callee
/// takes (0, first pointer, second pointer). The scratch pointers differ
/// between the sides, so the contract skips them and snapshots the
/// pointed-to words instead, with scripted callee fills.
///
/// Thiscall: thread-through ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f8140(this: u32) -> u32 {
    unsafe {
        const C_A: u32 = 1;
        const C_B: u32 = 2;
        const C_FILL: u32 = 3;
        const C_QUERY: u32 = 4;
        const C_COMMIT: u32 = 5;
        const G_ENABLE: u32 = 0x11609f6;
        if ((lf_checker_rt::global::<u8>(G_ENABLE)) as *const u8).read() == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(C_A, u32, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_B, u32, this);
        let mut buf_a = [0u32; 2];
        let mut buf_b = [0u32; 2];
        let mut buf_c = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_cdecl!(C_FILL, u32, buf_a.as_mut_ptr() as u32);
        let _: u32 =
            lf_checker_rt::callee_cdecl!(C_QUERY, u32, buf_b.as_mut_ptr() as u32, 8);
        let fptr: u32 =
            lf_checker_rt::callee_cdecl!(C_QUERY, u32, buf_c.as_mut_ptr() as u32, 0xb);
        let _dead = (fptr as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(
            C_COMMIT, u32, 0, buf_a.as_ptr() as u32, buf_b.as_ptr() as u32);
        0
    }
});
