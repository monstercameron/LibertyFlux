// original: 0x00dff377 FindCompleteObject
/// MSVC RTTI helper: adjust a base pointer to its complete object.
///
/// Reads the vtable slot's unwind descriptor: subtract the first offset,
/// and when the second offset is nonzero also subtract the pointed-to
/// displacement read back through the adjusted base.
export!(cdecl, rw_00dff377(obj: u32) -> u32 {
    unsafe {
        let vftable = (obj as *const u32).read_unaligned();
        let desc = ((vftable.wrapping_sub(4)) as *const u32).read_unaligned();
        let first = ((desc.wrapping_add(4)) as *const u32).read_unaligned();
        let mut out = obj.wrapping_sub(first);
        let extra = ((desc.wrapping_add(8)) as *const u32).read_unaligned();
        if extra != 0 {
            let back = obj.wrapping_sub(extra);
            out = out.wrapping_sub((back as *const u32).read_unaligned());
        }
        out
    }
});
