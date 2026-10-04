// original: 0x00c69360 dispatch_by_category
// Dispatch on the id object's category (vtable slot +0xC): category 6 runs
// one handler, category 5 another, anything else returns the category
// answer itself.
export!(thiscall, rw_00c69360(obj: u32, id: u32) -> u32 {
    unsafe {
        let tab = relocated(0x1295cd8);
        let ent = *(tab.wrapping_add(id.wrapping_mul(4)) as *const u32);
        let vt = *(ent as *const u32);
        let slot = *((vt as *const u8).add(0xc) as *const u32);
        let vf: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let a1 = vf(ent);
        if a1 & 0xff == 6 {
            return callee_thiscall!(2, u32, obj, id);
        }
        let a2 = vf(ent);
        if a2 & 0xff == 5 {
            return callee_thiscall!(3, u32, obj, id);
        }
        a2
    }
});
