// original: 0x00bf4a50 build_emitter_tabled
/// Look the entry up in the global type table, attach it to a fresh emitter,
/// push two float properties, and stamp the sequence number.
/// A null object returns entry EAX in the original, so the channel is none.
export!(thiscall, rw_bf4a50(this: *const u8, obj: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        let idx = *((obj as *const u8).add(0x2E) as *const i16) as i32;
        let edx = *this.add(0x18) as usize;
        let v1 = *global::<u32>(0x1295CD8).offset(idx as isize);
        let v2 = *((v1 as *const u8).add(0xCC) as *const u32);
        let val = *((v2 as *const u8).add(edx.wrapping_mul(4).wrapping_add(0xA4)) as *const u32);
        if (val as i32) <= -1 {
            return val;
        }
        let m: u32 = callee_thiscall!(1, u32, obj, val);
        if m == 0 {
            return 0;
        }
        let w8 = *((this.add(8)) as *const u32);
        let edi: u32 = callee_thiscall!(2, u32, relocated(0x1394D60), w8, 0, 0);
        if edi == 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(3, u32, edi, m);
        let _: u32 = callee_thiscall!(4, u32, relocated(0x1394D60), edi, obj, 0);
        let f10 = *((this.add(0x10)) as *const u32);
        let _: u32 = callee_thiscall!(5, u32, edi, relocated(0xEBBF78), f10);
        let f14 = *((this.add(0x14)) as *const u32);
        let _: u32 = callee_thiscall!(5, u32, edi, relocated(0xEBBF80), f14);
        let _: u32 = callee_thiscall!(6, u32, edi);
        let t: u32 = callee_cdecl!(7, u32,);
        let g1 = *global::<u32>(0x11F702C);
        let g2 = *global::<u32>(0x11F70C4);
        let v = if g1 == t { g2.wrapping_add(1) } else { g2 };
        *((edi as *mut u8).add(0x1D4) as *mut u32) = v;
        v
    }
});
