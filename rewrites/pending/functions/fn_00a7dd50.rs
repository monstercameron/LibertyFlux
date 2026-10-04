// original: 0x00a7dd50 CSimpleNMExplosionTaskInfo::CSimpleNMExplosionTaskInfo
/// Construct an explosion task info holding a 3-word vector copied from
/// the given source.
export!(thiscall, rw_00a7dd50(this: u32, src_vec: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1C64;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        let src = src_vec as *const u32;
        let dst = obj.add(0x20) as *mut u32;
        dst.write_unaligned(src.read_unaligned());
        dst.add(1).write_unaligned(src.add(1).read_unaligned());
        dst.add(2).write_unaligned(src.add(2).read_unaligned());
    }
    this
});
