// original: 0x008d58c0 build_arc_vertex_records
/// Arc vertex-record builder: allocates n 24-byte records on the stack,
/// fills them from a source point plus sin/cos-driven offsets, and hands
/// the block to the triangle dispatcher.
///
/// Original: cdecl/5 (src ptr to 2 dwords, float scale, count n, dispatch
/// word, dispatch float). The allocator call runs NATIVELY on the
/// original side (a stub cannot move ESP, and the function restores ESP
/// itself); the rewrite models the buffer as a zeroed local, matching
/// because the contract zeroes uninitialized stack. Exit EAX is the
/// scripted cookie answer. All float order pinned (NaN payloads bit for
/// bit). Record i (6 words): [x, y, x+t(i-1), y+u(i-1), x+t(i), y+u(i)]
/// with t(i),u(i) the scaled sin/cos of i*step (t(-1),u(-1) from the
/// pre-loop angle).
export!(cdecl, rw_008d58c0(src: u32, scale_bits: u32, n: u32, p3: u32, f4_bits: u32) -> u32 {
    unsafe {
        let bb = core::hint::black_box;
        let scale = f32::from_bits(scale_bits);
        let x = *(src as *const f32);
        let y = *((src + 4) as *const f32);
        // Native-alloca model: 3 records x 6 words, zeroed (n <= 3 and
        // stack_fill 0 make the original's unwritten tail zeros too).
        let mut buf = [0u32; 18];
        let step = bb(*global::<f32>(0xFE8AEC)) / bb((n as i32) as f32);
        let a0 = bb(step) * bb(*global::<f32>(0xFE8628));
        let s0 = f32::from_bits(callee_cdecl!(1, u32, bb(a0).to_bits()));
        let mut t = bb(s0) * bb(scale);
        let c0 = f32::from_bits(callee_cdecl!(2, u32, bb(a0).to_bits()));
        let mut u = bb(c0) * bb(scale);
        if (n as i32) > 0 {
            for i in 1..=n {
                let rec = ((i - 1) * 6) as usize;
                buf[rec] = x.to_bits();
                buf[rec + 1] = y.to_bits();
                buf[rec + 2] = (bb(x) + bb(t)).to_bits();
                buf[rec + 3] = (bb(y) + bb(u)).to_bits();
                let ang = bb((i as i32) as f32) * bb(step);
                let s = f32::from_bits(callee_cdecl!(1, u32, bb(ang).to_bits()));
                let t_new = bb(s) * bb(scale);
                buf[rec + 4] = (bb(x) + bb(t_new)).to_bits();
                let c = f32::from_bits(callee_cdecl!(2, u32, bb(ang).to_bits()));
                let u_new = bb(c) * bb(scale);
                buf[rec + 5] = (bb(y) + bb(u_new)).to_bits();
                t = t_new;
                u = u_new;
            }
        }
        let _: u32 = callee_cdecl!(3, u32, buf.as_ptr() as u32, n, p3, f4_bits);
        callee_cdecl!(4, u32,)
    }
});
