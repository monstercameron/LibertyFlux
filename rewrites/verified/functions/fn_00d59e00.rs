// original: 0x00d59e00 ccam_cinematic_veh_offset_init
/// Initialise a cinematic vehicle-offset camera: mark the two extra slots
/// empty and zero the whole parameter block.
///
/// Writes -1 to the dwords at `+0x180` and `+0x184` (empty-slot markers) and
/// zero to thirteen parameter dwords (`+0x140`, `+0x144`, `+0x148`, `+0x150`,
/// `+0x154`, `+0x158`, `+0x160`, `+0x164`, `+0x168`, `+0x170`, `+0x174`,
/// `+0x178`, `+0x188`) and the flag byte at `+0x18c`. Returns 1.
///
/// Original: thiscall, no stack arguments, returns 1 in `al`.
lf_checker_rt::export!(thiscall, rw_00d59e00 (this: u32) -> u32 {
    unsafe {
        const EMPTY0: u32 = 0x180;
        const EMPTY1: u32 = 0x184;
        const FLAG: u32 = 0x18c;
        for off in [0x140u32, 0x144, 0x148, 0x150, 0x154, 0x158, 0x160, 0x164, 0x168, 0x170, 0x174, 0x178, 0x188] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + EMPTY0) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + EMPTY1) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + FLAG) as *mut u8).write(0);
        1
    }
});
