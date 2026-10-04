// original: 0x00985ef0 audio_eval_skinned_position
/// Original 0x00985ef0 (unnamed): evaluate a skinned emitter position.
///
/// Transforms the `idx`-th stored direction by the optional matrix: with no
/// matrix, forwards to the fallback evaluator; otherwise computes the three
/// row dot-products in SSE order (bit-exact) and stores them plus a zero
/// fourth lane (the original reads an uninitialised stack word there, which
/// the contract defines as zero). Void; the stores are the behaviour.
export!(thiscall, rw_00985ef0(this_: u32, idx: u32, out: u32) -> u32 {
    let base = idx.wrapping_mul(0x60);
    let v = this_.wrapping_add(0x8240).wrapping_add(base);
    let h = unsafe {
        (this_.wrapping_add(0x8264).wrapping_add(base) as *const u32).read()
    };
    let m = unsafe { ((h + 0x20) as *const u32).read() };
    if m == 0 {
        callee_cdecl!(1, u32, out, h.wrapping_add(0x10), v);
        return 0;
    }
    let x = unsafe { (v as *const f32).read() };
    let y = unsafe { ((v + 4) as *const f32).read() };
    let z = unsafe { ((v + 8) as *const f32).read() };
    let r = |o: u32| unsafe { ((m + o) as *const f32).read() };
    // Exact accumulate order of the original SSE sequence.
    let mut o0 = r(0x10) * y;
    o0 += r(0x00) * x;
    o0 += r(0x20) * z;
    o0 += r(0x30);
    let mut o1 = r(0x14) * y;
    o1 += r(0x04) * x;
    o1 += r(0x24) * z;
    o1 += r(0x34);
    let mut o2 = r(0x18) * y;
    o2 += r(0x08) * x;
    o2 += r(0x28) * z;
    o2 += r(0x38);
    unsafe {
        (out as *mut f32).write(o0);
        ((out + 4) as *mut f32).write(o1);
        ((out + 8) as *mut f32).write(o2);
        ((out + 12) as *mut f32).write(0.0);
    }
    0
});
