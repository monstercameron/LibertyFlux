// Wrong version: vtable slots swapped. Must fail calls.
export!(cdecl, rw_00901bd0() -> u32 {
    unsafe {
        if (global::<u32>(0x011F7060)).read_unaligned() == 1 {
            return 0;
        }
        let gen = (global::<u32>(0x012088B4)).read_unaligned();
        if gen != (global::<u32>(0x00F1C040)).read_unaligned() {
            return gen;
        }
        if (global::<u32>(0x01037720)).read_unaligned() == 0x12 {
            return gen;
        }
        let dev = (global::<u32>(0x0118D808)).read_unaligned();
        let vt = (dev as *const u32).read_unaligned();
        let a1 = ((vt.wrapping_add(4)) as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(a1 as usize);
        let _: u32 = f1(dev);
        let a2 = ((vt.wrapping_add(8)) as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(a2 as usize);
        f2(dev)
    }
});
