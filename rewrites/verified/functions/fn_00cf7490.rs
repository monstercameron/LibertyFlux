// original: 0x00cf7490 climb_complex_task_ctor3 (proposed)

/// Constructs a complex climb task from three parameters: runs the shared
/// base constructor, stamps the vtable, zeroes the state slots, stores the
/// third parameter at `+0x7c` and the first two at `+0x90`/`+0x94`, and
/// returns the object.
///
/// Original: 0x00cf7490 (thiscall: ecx holds the object, three stack words).
lf_checker_rt::export!(thiscall, rw_00cf7490(this: u32, first: u32, second: u32, third: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00edf334;
        const BASE_CTOR: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        for off in [0x14u32, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38, 0x40, 0x44, 0x48, 0x70, 0x98] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + 0x74) as *mut u8).write(0);
        ((this + 0x76) as *mut u16).write_unaligned(0);
        ((this + 0x78) as *mut u8).write(0);
        ((this + 0x7c) as *mut u32).write_unaligned(third);
        ((this + 0x90) as *mut u32).write_unaligned(first);
        ((this + 0x94) as *mut u32).write_unaligned(second);
        this
    }
});
