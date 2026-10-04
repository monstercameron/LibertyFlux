// original: 0x00abb9d0 construct_and_resolve
/// Allocate a 0x60-byte object, construct it via callee 3, then resolve.
///
/// Forwards all seven arguments plus the fresh allocation to the
/// constructor (callee 3); the constructed object is validated twice
/// through vtable slot 8 and a mixing step folds the two answers into
/// `[obj+4]`: `t = r1 % 16; s = (16 - t) % 16; m = ((r2 + s) / 16 * 0x4000
/// ^ [obj+4]) & 0x1FFC000; [obj+4] ^= m`, returning `m`. All divisions
/// are truncating signed operations matching the original's shift idiom.
/// A null allocation skips construction and faults on the vtable read,
/// exactly like the original.
lf_checker_rt::export!(cdecl, rw_00abb9d0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let m = lf_checker_rt::callee_cdecl!(1, u32, 0x60, 0);
    let edi = if m != 0 {
        lf_checker_rt::callee_thiscall!(3, u32, m, a0, a1, a2, a3, a4, a5, a6)
    } else {
        0
    };
    // SAFETY: the object and vtable are checker-backed heap on every
    // trial (a null object faults here on both sides, as designed).
    let vtab = unsafe { (edi as *const u32).read_unaligned() };
    let slot = unsafe { (vtab.wrapping_add(8) as *const u32).read_unaligned() };
    let query: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
    let t1 = (query(edi) as i32) % 16;
    let s = (0x10i32 - t1) % 16;
    let sum = (query(edi) as i32).wrapping_add(s);
    let e4 = unsafe { (edi.wrapping_add(4) as *const u32).read_unaligned() };
    let masked = (((sum / 16).wrapping_mul(0x4000) as u32) ^ e4) & 0x01FF_C000;
    unsafe { (edi.wrapping_add(4) as *mut u32).write_unaligned(e4 ^ masked) };
    masked
});

