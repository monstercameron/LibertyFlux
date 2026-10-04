// original: 0x0089cc20 audio_resolver_list_walk
/// Keyed entry walk over a resolver list (original 0x0089CC20).
///
/// Resolves the voice for the object's sub-slot, then walks the resolver
/// list counting down `a3` by each entry's weight until the entry that
/// owns the remainder is found. That entry's key is hashed and stored
/// back into the voice. Returns 1 on success, 0 when the walk runs out.
/// The upper return bytes repeat the triggering value, as in the
/// original.
///
/// Note: the two frame buffers the original passes to callees are
/// uninitialized stack there; the contract defines that fill as zero,
/// so the rewrite passes an explicit zeroed buffer.
export!(thiscall, rw_89cc20(this: *const u8, a1: u32, a2: u32, a3: u32, _a4: u32) -> u32 {
    unsafe {
        let base: u32 = *global::<u32>(0x115d988);
        let sel = (*this.add(0x40)) as u32;
        let stride2: u32 = *global::<u32>(0x115d968);
        let sub = (*this.add(0xb4)) as u32;
        let row2 = *(((base.wrapping_add(sel.wrapping_mul(0x6f40))).wrapping_add(0x6f14))
            as *const u32);
        let voice = row2.wrapping_add(stride2.wrapping_mul(sub));
        if voice == 0 {
            return base & 0xffffff00;
        }
        if a3 == 0 {
            return base & 0xffffff00;
        }
        let g1: u32 = *global::<u32>(0x115f82c);
        let g2: u32 = *global::<u32>(0x115f830);
        let r1: u32 = callee_cdecl!(1, u32, a1, g2, g1);
        if r1 == 0 {
            return 0;
        }
        let r2: u32 = callee_cdecl!(2, u32, a2, 0);
        let r3: u32 = callee_cdecl!(3, u32, r1, r2);
        if r3 == 0 {
            return 0;
        }
        let node_p = r1 as *const u8;
        let mut edx = r3;
        let mut esi = a3;
        loop {
            if esi != 0 {
                let lim = *(((edx as *const u8).add(0xd)) as *const u8) as u32;
                if esi <= lim {
                    break;
                }
            }
            let lim = *(((edx as *const u8).add(0xd)) as *const u8) as u32;
            esi = esi.wrapping_sub(lim);
            let cnt = *(((node_p.add(8)) as *const u16)) as u32;
            let edi = (edx as u32).wrapping_add(0xe);
            let bound = (*(node_p as *const u32)).wrapping_add(cnt.wrapping_mul(14));
            if edi >= bound {
                return bound & 0xffffff00;
            }
            let k1 = *((((edi as *const u8).add(8))) as *const u32);
            let k2 = *((((edx as *const u8).add(8))) as *const u32);
            if k1 != k2 {
                return k1 & 0xffffff00;
            }
            edx = edi;
        }
        let r4: u32 = callee_cdecl!(4, u32, *(edx as *const u32));
        *((voice.wrapping_add(4)) as *mut u32) = r4 & 0xFFFF;
        let mut buf = [0u32; 8];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let _: u32 = callee_cdecl!(5, u32, buf_ptr, relocated(0xe79e58), a2, a3);
        let r6: u32 = callee_cdecl!(6, u32, buf_ptr, 0);
        *(voice as *mut u32) = r6;
        (r6 & 0xffffff00) | 1
    }
});
