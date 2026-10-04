// original: 0x00cf7530 climb_complex_task_ctor1 (proposed)

/// Constructs a complex climb task from one parameter: runs the shared base
/// constructor, stamps the vtable, zeroes the state slots, stores the
/// parameter at `+0x7c` and the marker -1 at `+0x94`, and returns the object.
///
/// Original: 0x00cf7530 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf7530(this: u32, param: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00edf334;
        const BASE_CTOR: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        for off in [0x14u32, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38, 0x40, 0x44, 0x48, 0x70, 0x90, 0x98] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + 0x74) as *mut u8).write(0);
        ((this + 0x76) as *mut u16).write_unaligned(0);
        ((this + 0x78) as *mut u8).write(0);
        ((this + 0x7c) as *mut u32).write_unaligned(param);
        ((this + 0x94) as *mut u32).write_unaligned(0xffff_ffff);
        this
    }
});
