// original: 0x00b357c0 copy16_then_call_with_old_and_count
/// Copy 16 bytes from `a` to `b - 16`, then call the worker with
/// `(a, 0, (b - a - 16) / 16, old0..old3, c)` where `old0..old3` are the
/// previous words at `b - 16` (signed division for the count).
lf_checker_rt::export!(cdecl, rw_b357c0(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        let src = a as *const u32;
        let dst = b.wrapping_sub(16) as *mut u32;
        let o0 = dst.read();
        let o1 = dst.add(1).read();
        let o2 = dst.add(2).read();
        let o3 = dst.add(3).read();
        dst.write(src.read());
        dst.add(1).write(src.add(1).read());
        dst.add(2).write(src.add(2).read());
        dst.add(3).write(src.add(3).read());
        let n = (b.wrapping_sub(a).wrapping_sub(16) as i32 >> 4) as u32;
        lf_checker_rt::callee_cdecl!(2, u32, a, 0, n, o0, o1, o2, o3, c)
    }
});
