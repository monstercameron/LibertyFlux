// original: 0x00b53e40 release_child_slot0
/// Releases the nullable child at +8 through its slot-0 virtual with
/// argument 1, then nulls the slot. The return value is the release
/// answer, or the untouched entry residue when there is no child, so the
/// return channel is not compared; the call and the store are.
export!(thiscall, rw_00b53e40(this: *mut u8) -> u32 {
    unsafe {
        let child = *(this.add(8) as *const u32);
        if child != 0 {
            let vt = *(child as *const u32);
            let tgt = *(vt as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            release(child, 1);
            *(this.add(8) as *mut u32) = 0;
        }
        0
    }
});
