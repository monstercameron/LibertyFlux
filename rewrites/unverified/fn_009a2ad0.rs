// original: 0x009a2ad0 entity_release_effects
/// Conditionally notifies, then releases two effect handles.
///
/// When the mode globals select the active path, callee 1 is notified with
/// the linked object at offset 8. The handles at offsets 0x60 and 0x64 are
/// then released through callee 2 whenever set.
export!(thiscall, rw_009a2ad0(this: u32) -> () {
    unsafe {
        const MODE: u32 = 0x11f7060;
        const CUR: u32 = 0x12088b4;
        const WANT: u32 = 0xf1c040;
        const KIND: u32 = 0x1037720;
        if *global::<u32>(MODE) != 1
            && *global::<u32>(CUR) == *global::<u32>(WANT)
            && *global::<u32>(KIND) != 0x12
        {
            let inner = *((this.wrapping_add(8)) as *const u32);
            callee_cdecl!(1, u32, inner, 1);
        }
        let p = *((this.wrapping_add(0x60)) as *const u32);
        if p != 0 {
            callee_thiscall!(2, u32, p, 0);
        }
        let q = *((this.wrapping_add(0x64)) as *const u32);
        if q != 0 {
            callee_thiscall!(2, u32, q, 0);
        }
    }
});
