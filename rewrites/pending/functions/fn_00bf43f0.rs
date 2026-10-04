// original: 0x00bf43f0 build_emitter_plain
/// Same as build_emitter_full without the float properties.
export!(thiscall, rw_bf43f0(this: *const u8, obj: u32) -> u32 {
    unsafe {
        let w8 = *((this.add(8)) as *const u32);
        let edi: u32 = callee_thiscall!(1, u32, relocated(0x1394D60), w8, 0, 0);
        if edi == 0 {
            return 0;
        }
        let vtable = *(obj as *const u32);
        let target = *((vtable as *const u8).add(0xA0) as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let first = probe(obj);
        let base = if first == 0 {
            *((obj as *const u8).add(0x100) as *const u32)
        } else {
            let second = probe(obj);
            let v2 = *(second as *const u32);
            let t2 = *((v2 as *const u8).add(0xE0) as *const u32);
            let resolve: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(t2 as usize);
            resolve(second)
        };
        let w = *((base as *const u8).add(4) as *const u32);
        let idx: u32 = callee_cdecl!(4, u32, w, 0x4B5);
        if idx == 0xFFFFFFFF {
            return idx;
        }
        let m: u32 = callee_thiscall!(5, u32, obj, idx);
        if m == 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(6, u32, edi, m);
        let _: u32 = callee_thiscall!(7, u32, edi);
        let t: u32 = callee_cdecl!(8, u32,);
        let g1 = *global::<u32>(0x11F702C);
        let g2 = *global::<u32>(0x11F70C4);
        let v = if g1 == t { g2.wrapping_add(1) } else { g2 };
        *((edi as *mut u8).add(0x1D4) as *mut u32) = v;
        v
    }
});
