// original: 0x008eb8c0 bounded_record_decode
/// Bounded decode of packed records into caller buffers.
///
/// Runs a callee-built query over an input table (`bound` outer rounds),
/// then decodes up to `cap` packed record ids per round: each id selects a
/// record through a global table, and the record's packed x/y/z words are
/// scaled into the float buffer `a1` while a callee-derived tag byte lands
/// in `a2`. `a4` points at the running output count (zeroed first, also the
/// capacity check against `cap`). Returns the round count. Only whole words
/// and the low tag byte are significant.
export!(thiscall, rw_008eb8c0(obj: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const TAB: u32 = 0x011A2EC0; // opaque table cookie, never dereferenced
        const MAGIC: u32 = 0x497423FE;
        const F3: u32 = 0x40400000; // 3.0f
        const DECODE_TAB: u32 = 0x01178284;

        *(a4 as *mut u32) = 0;
        let tab = lf_checker_rt::relocated(TAB);
        let mut out0 = 0u32;
        callee_thiscall!(1, u32, obj, &mut out0 as *mut u32 as u32,
            tab, MAGIC, 0, 0, 0, 0, 0, 0, 0, 0, F3, 0xFFFF_FFFF, 0);

        let ac = *global::<i32>(0x011A2EAC);
        let b0 = *global::<i32>(0x011A2EB0);
        let bound = ac.wrapping_sub(b0);
        let mut scratch = 0u32;
        let mut arr = [0xFFFF_FFFFu32; 4];
        let mut bnd = 0u32;
        let mut tmp = 0u32;
        if bound > 0 {
            let mut esi_saved = out0;
            let mut cursor = tab;
            let mut c0: i32 = 0;
            let mut c1: i32 = 0;
            loop {
                c1 = c0.wrapping_add(1);
                let rem1 = c1 % ac;
                let tp1 =
                    tab.wrapping_add((rem1 as u32).wrapping_mul(16));
                let ret2: u32 = callee_thiscall!(1, u32, obj,
                    &mut scratch as *mut u32 as u32, tp1, MAGIC,
                    0, 0, 0, 0, 0, 0, 0, 0, F3, 0xFFFF_FFFF, 0);
                let edi2 = *(ret2 as *const u32);
                if edi2 != esi_saved {
                    let rem2 = c1 % ac;
                    let tp2 =
                        tab.wrapping_add((rem2 as u32).wrapping_mul(16));
                    let g = *global::<u32>(0x010330DC);
                    tmp = edi2;
                    callee_thiscall!(2, u32, obj, cursor, esi_saved, tp2,
                        arr.as_mut_ptr() as u32, &mut bnd as *mut u32 as u32,
                        0x200, 0, MAGIC, &mut tmp as *mut u32 as u32,
                        MAGIC, 0, g, 0, 0, 1, 1, 0, 0);
                    let mut i = if c0 != 0 { 1i32 } else { 0i32 };
                    if i < bnd as i32 {
                        loop {
                            let cnt = *(a4 as *const u32);
                            if (cnt as i32) < a3 as i32 {
                                let e = *(arr.as_ptr().add(i as usize));
                                let lo = e & 0xFFFF;
                                let hi = e >> 16;
                                let cell = *global::<u32>(
                                    DECODE_TAB.wrapping_add(lo.wrapping_mul(4)),
                                );
                                let rec = cell
                                    .wrapping_add(hi.wrapping_mul(32));
                                let rx = *((rec.wrapping_add(0x14))
                                    as *const i16);
                                let ry = *((rec.wrapping_add(0x16))
                                    as *const i16);
                                let rz = *((rec.wrapping_add(0x18))
                                    as *const i16);
                                let base =
                                    a1.wrapping_add(cnt.wrapping_mul(16));
                                *((base) as *mut f32) =
                                    (rx as f32) * 0.125;
                                *((base.wrapping_add(4)) as *mut f32) =
                                    (ry as f32) * 0.125;
                                *((base.wrapping_add(8)) as *mut f32) =
                                    (rz as f32) * 0.015625;
                                let tag: u32 = callee_thiscall!(3, u32, rec);
                                let cnt3 = *(a4 as *const u32);
                                *((a2.wrapping_add(cnt3)) as *mut u8) =
                                    tag as u8;
                                let b1f = *((rec.wrapping_add(0x1F))
                                    as *const u8);
                                if (b1f & 2) != 0 && (b1f & 1) != 0 {
                                    let c = *(a4 as *const u32);
                                    let p =
                                        (a2.wrapping_add(c)) as *mut u8;
                                    *p |= 0x80;
                                }
                                let n = *(a4 as *const u32);
                                *(a4 as *mut u32) = n.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                            if i >= bnd as i32 {
                                break;
                            }
                        }
                    }
                }
                cursor = cursor.wrapping_add(0x10);
                esi_saved = edi2;
                c0 = c1;
                if c1 >= bound {
                    break;
                }
            }
        }
        bound as u32
    }
});
