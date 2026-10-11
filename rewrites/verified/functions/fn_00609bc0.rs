// original: 0x00609BC0 rage::phJoint1Dof::phJoint1Dof

/// Initialize the one-degree-of-freedom joint after calling its scripted
/// base initializer. Store its relocated vtable, clear the active and range
/// state fields, set the four extreme range endpoints and two default scalar
/// values, then return `this`. Float fields are stored by their original bit
/// patterns. This is a 32-bit thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00609BC0(this: u32) -> u32 {
    unsafe {
        const BASE_INITIALIZER: u32 = 1;
        const VTABLE_VA: u32 = 0x00fe_19ec;
        const VTABLE: u32 = 0;
        let _ = lf_checker_rt::callee_thiscall!(BASE_INITIALIZER, u32, this);
        ((this + VTABLE) as *mut u32).write_unaligned(lf_checker_rt::global::<u8>(VTABLE_VA) as u32);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0x270) as *mut u32).write_unaligned(0xff7f_ffff);
        ((this + 0x274) as *mut u32).write_unaligned(0x7f7f_ffff);
        ((this + 0x278) as *mut u32).write_unaligned(0xff7f_ffff);
        ((this + 0x27c) as *mut u32).write_unaligned(0x7f7f_ffff);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        ((this + 0x2b0) as *mut u32).write_unaligned(0x4348_0000);
        ((this + 0x2b4) as *mut u32).write_unaligned(0x4120_0000);
        ((this + 0x2b8) as *mut u32).write_unaligned(0);
        ((this + 0x2bc) as *mut u32).write_unaligned(0);
        ((this + 0x2c0) as *mut u32).write_unaligned(0x4cbe_bc20);
        ((this + 0x2c4) as *mut u32).write_unaligned(0xccbe_bc20);
        ((this + 0x284) as *mut u32).write_unaligned(0);
        ((this + 0x280) as *mut u32).write_unaligned(0);
        this
    }
});
