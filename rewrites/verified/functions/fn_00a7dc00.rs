// original: 0x00a7dc00 CSimpleClimbTaskInfo::CSimpleClimbTaskInfo
/// Construct a climb task info: four flag bytes plus a 3-word vector
/// copied from the given source.
export!(thiscall, rw_00a7dc00(this: u32, flag_a: u32, flag_b: u32, flag_c: u32, flag_d: u32, src_vec: u32) -> u32 {
    const VTABLE: u32 = 0x00EA170C;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        *obj.add(0x18) = flag_a as u8;
        *obj.add(0x19) = flag_b as u8;
        *obj.add(0x1a) = flag_c as u8;
        *obj.add(0x1b) = flag_d as u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        let src = src_vec as *const u32;
        let dst = obj.add(0x20) as *mut u32;
        dst.write_unaligned(src.read_unaligned());
        dst.add(1).write_unaligned(src.add(1).read_unaligned());
        dst.add(2).write_unaligned(src.add(2).read_unaligned());
    }
    this
});
