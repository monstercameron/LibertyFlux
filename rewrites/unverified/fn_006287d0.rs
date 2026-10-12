// original: 0x006287D0 rage::ptxgpuRenderSettings::ptxgpuRenderSettings (proposed)
/// Constructor for the GPU particle render-settings object.
///
/// Installs the class virtual table, zeroes the integer/pointer fields and
/// sets the float fields to their defaults: scale factors of 1.0, two
/// half-weight factors of 0.5, one factor of 2.0, and an enable byte of 1.
/// Takes the object pointer in `ecx` (thiscall, no stack arguments) and
/// returns it unchanged.
lf_checker_rt::export!(thiscall, rw_006287d0(this_ptr: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EC5E0C;
        const ONE: u32 = 0x3F80_0000;
        const HALF: u32 = 0x3F00_0000;
        const TWO: u32 = 0x4000_0000;
        let obj = this_ptr as *mut u8;
        (obj as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        (obj.add(0x04) as *mut u32).write(0);
        (obj.add(0x08) as *mut u32).write(ONE);
        (obj.add(0x0C) as *mut u32).write(0);
        (obj.add(0x10) as *mut u32).write(TWO);
        (obj.add(0x14) as *mut u32).write(0);
        (obj.add(0x18) as *mut u32).write(0);
        (obj.add(0x1C) as *mut u32).write(0);
        obj.add(0x20).write(1);
        (obj.add(0x24) as *mut u32).write(0);
        (obj.add(0x28) as *mut u32).write(0);
        (obj.add(0x30) as *mut u32).write(ONE);
        (obj.add(0x34) as *mut u32).write(ONE);
        (obj.add(0x38) as *mut u32).write(ONE);
        (obj.add(0x3C) as *mut u32).write(HALF);
        (obj.add(0x40) as *mut u32).write(ONE);
        (obj.add(0x44) as *mut u32).write(ONE);
        (obj.add(0x48) as *mut u32).write(ONE);
        (obj.add(0x4C) as *mut u32).write(HALF);
        (obj.add(0x50) as *mut u32).write(0);
        (obj.add(0x54) as *mut u32).write(0);
        (obj.add(0x58) as *mut u32).write(ONE);
    }
    this_ptr
});
