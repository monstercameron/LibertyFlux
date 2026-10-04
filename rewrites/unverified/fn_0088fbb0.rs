// original: 0x0088FBB0 
// 0088FBB0 audSound broadcast: run every bound slot voice through its kind
// handler with the given tag.
export!(thiscall, rw_0088fbb0(this: *mut u8, kind: u32) -> () {
    unsafe {
        let stride = *global::<u32>(0x115D964);
        let base = *global::<u32>(0x115D988);
        let row = (*this.add(0x40) as u32).wrapping_mul(0x6F40);
        let entry = *((row.wrapping_add(base).wrapping_add(0x6F10)) as *const u32);
        for i in 0..8u32 {
            let sel = *this.add((0x48 + i) as usize);
            if sel == 0xFF {
                continue;
            }
            let obj = stride.wrapping_mul(sel as u32).wrapping_add(entry);
            if obj == 0 {
                continue;
            }
            let pick = *(obj as *const u8).add(0x3B);
            let target = *global::<u32>(0x115D6B4 + (pick as u32) * 4);
            let go: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            go(obj, kind);
        }
    }
});
