// original: 0x00d2b000 task_free_pair (proposed)
/// Destructor: stamp the two vtable slots, free the handles at `+0x50` and
/// `+0x54` when set (clearing each), then tail-jump to the base destructor,
/// forwarding `this` and its result.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2b000(this: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee224c;
        const VT14V: u32 = 0x00ee22a4;
        const FREE: u32 = 1;
        const TAIL: u32 = 2;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VT14V));
        for off in [0x50u32, 0x54] {
            let h = ((this + off) as *const u32).read_unaligned();
            if h != 0 {
                lf_checker_rt::callee_cdecl!(FREE, u32, h);
                ((this + off) as *mut u32).write_unaligned(0);
            }
        }
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
