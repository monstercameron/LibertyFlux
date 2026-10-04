// original: 0x008eb600 flag_query_decode
/// Flag-gated record query with bounded decode into caller buffers.
///
/// Runs a chain of flag tests (callees 1-5) selecting a float parameter,
/// builds a query through two callee rounds (6), fills an id table through
/// one of two writer rounds (7, with 8 as the empty-table fallback), stores
/// the hit count through `a5`, then decodes each packed id through a global
/// table: scaled x/y/z into the float buffer reached through callee 6's
/// answer, and a tag byte from callee 9 into `a2`. `a4` bounds a callee
/// size argument, `a6` (low byte) picks the float parameter. The return
/// value is leftover and not significant.
export!(thiscall, rw_008eb600(obj: u32, a1: u32, a2: u32, a3: u32, a4: u32,
        a5: u32, a6: u32) -> u32 {
    unsafe {
        const MAGIC: u32 = 0x497423FE;
        const DECODE_TAB: u32 = 0x01178284;
        let mut qa = [0u32; 3];
        callee_cdecl!(1, u32, qa.as_mut_ptr() as u32);
        let r2: u32 = callee_cdecl!(2, u32, a1);
        let flag: u32;
        if (r2 as u8) != 0 {
            let r4: u32 = callee_cdecl!(4, u32, qa.as_mut_ptr() as u32);
            if (r4 as u8) != 0 {
                flag = 1;
            } else {
                let r5: u32 = callee_cdecl!(5, u32, qa.as_mut_ptr() as u32);
                flag = u32::from((r5 as u8) != 0);
            }
        } else {
            let r3: u32 = callee_cdecl!(3, u32, a1);
            if (r3 as u8) != 0 {
                let r4: u32 = callee_cdecl!(4, u32, qa.as_mut_ptr() as u32);
                if (r4 as u8) != 0 {
                    flag = 1;
                } else {
                    let r5: u32 =
                        callee_cdecl!(5, u32, qa.as_mut_ptr() as u32);
                    flag = u32::from((r5 as u8) != 0);
                }
            } else {
                flag = 0;
            }
        }
        let fb: u32 = if (a6 as u8) == 0 { 0x40400000 } else { 0 };
        let mut s1 = 0u32;
        let retb: u32 = callee_thiscall!(6, u32, obj,
            &mut s1 as *mut u32 as u32, a1, MAGIC,
            0, 0, 0, 0, 0, 0, 0, 0, fb, 0xFFFF_FFFF, 1);
        s1 = *(retb as *const u32);
        let mut s3c = 0u32;
        let retc: u32 = callee_thiscall!(6, u32, obj,
            &mut s3c as *mut u32 as u32, qa.as_mut_ptr() as u32, MAGIC,
            0, 0, 0, 0, 0, 0, 0, 0, fb, 0xFFFF_FFFF, 1);
        let retc0 = *(retc as *const u32);
        let g = *global::<u32>(0x010330DC);
        let edi = if (a4 as i32) < 0x200 { a4 } else { 0x200 };
        let mut arr = [0xFFFF_FFFFu32; 4];
        let mut bnd = fb;
        callee_thiscall!(7, u32, obj, qa.as_mut_ptr() as u32, retc0, a1,
            arr.as_mut_ptr() as u32, &mut bnd as *mut u32 as u32,
            edi, 0, MAGIC, &mut s1 as *mut u32 as u32, MAGIC,
            1, g, 0, 0, 1, 1, flag, 1);
        let mut count = bnd as i32;
        if count == 0 {
            callee_thiscall!(8, u32, obj, qa.as_mut_ptr() as u32, g, a1,
                arr.as_mut_ptr() as u32, &mut bnd as *mut u32 as u32,
                edi, 0, MAGIC, &mut s1 as *mut u32 as u32, MAGIC,
                0, g, 0, 0, 1, 1, 0, 1);
            count = bnd as i32;
        }
        *(a5 as *mut u32) = count as u32;
        if count > 0 {
            let mut i: i32 = 0;
            while i < count {
                let e = *(arr.as_ptr().add(i as usize));
                let lo = e & 0xFFFF;
                let hi = e >> 16;
                let cell = *global::<u32>(
                    DECODE_TAB.wrapping_add(lo.wrapping_mul(4)),
                );
                let rec = cell.wrapping_add(hi.wrapping_mul(32));
                let rx = *((rec.wrapping_add(0x14)) as *const i16);
                let ry = *((rec.wrapping_add(0x16)) as *const i16);
                let rz = *((rec.wrapping_add(0x18)) as *const i16);
                let base =
                    a2.wrapping_add((i as u32).wrapping_mul(16));
                *(base as *mut f32) = (rx as f32) * 0.125;
                *((base.wrapping_add(4)) as *mut f32) = (ry as f32) * 0.125;
                *((base.wrapping_add(8)) as *mut f32) = (rz as f32) * 0.015625;
                let tag: u32 = callee_thiscall!(9, u32, rec);
                *((a3.wrapping_add(i as u32)) as *mut u8) = tag as u8;
                let b1f = *((rec.wrapping_add(0x1F)) as *const u8);
                if (b1f & 2) != 0 && (b1f & 1) != 0 {
                    *((a3.wrapping_add(i as u32)) as *mut u8) =
                        (tag as u8) | 0x80;
                }
                i = i.wrapping_add(1);
            }
        }
        0u32
    }
});
