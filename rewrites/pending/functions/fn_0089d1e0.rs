// original: 0x0089d1e0 audio_descriptor_merge_pass
/// Merge pass over an array of 10-byte descriptors (original 0x0089D1E0).
///
/// For each of `count` descriptors at `obj`, resolves two record lists
/// through the resolver callee, then either merges them by their sort keys
/// into the destination buffer at `base` (both lists present) or forwards
/// the single present list to the writer callee. Writes back the merged
/// item count into each descriptor and returns the total.
///
/// Record layout is 14 bytes: an 8-byte payload, a 4-byte sort key,
/// and a 2-byte tag.
export!(cdecl, rw_89d1e0(obj: *mut u8, count: u32, b: u32, c: u32, base: u32) -> u32 {
    unsafe {
        if count == 0 {
            return 0;
        }
        let g1: u32 = *global::<u32>(0x115f82c);
        let g2: u32 = *global::<u32>(0x115f830);
        const REC: u32 = 14;
        let mut acc: u32 = 0;
        let mut cur = obj;
        let mut dst_base = base;
        let mut remaining = count;
        loop {
            let key: u32 = *((cur.add(4)) as *const u32);
            let r1: u32 = callee_cdecl!(1, u32, key, g2, g1);
            let r2: u32 = callee_cdecl!(2, u32, key, b, c);
            *(cur as *mut u32) = dst_base;
            let n: u32;
            if r1 == 0 {
                let cnt = *(((r2 as *const u8).add(8)) as *const u16) as u32;
                let arr = *(r2 as *const u32);
                let _: u32 = callee_cdecl!(3, u32, dst_base, arr, cnt.wrapping_mul(REC));
                n = cnt;
            } else if r2 == 0 {
                let cnt = *(((r1 as *const u8).add(8)) as *const u16) as u32;
                let arr = *(r1 as *const u32);
                let _: u32 = callee_cdecl!(3, u32, dst_base, arr, cnt.wrapping_mul(REC));
                n = cnt;
            } else {
                let cnt_a = *(((r1 as *const u8).add(8)) as *const u16) as u32;
                let cnt_b = *(((r2 as *const u8).add(8)) as *const u16) as u32;
                let arr_a = *(r1 as *const u32);
                let arr_b = *(r2 as *const u32);
                let dst0 = *(cur as *const u32);
                let (mut i, mut j) = (0u32, 0u32);
                let (mut off_a, mut off_b, mut off_d) = (0u32, 0u32, 0u32);
                let mut merged = 0u32;
                while i < cnt_a || j < cnt_b {
                    let src: u32;
                    if j >= cnt_b {
                        src = arr_a.wrapping_add(off_a);
                        i += 1;
                        off_a += REC;
                    } else if i >= cnt_a {
                        src = arr_b.wrapping_add(off_b);
                        j += 1;
                        off_b += REC;
                    } else {
                        let ka = *((((arr_a.wrapping_add(off_a)) as *const u8).add(8)) as *const u32);
                        let kb = *((((arr_b.wrapping_add(off_b)) as *const u8).add(8)) as *const u32);
                        if ka > kb {
                            src = arr_b.wrapping_add(off_b);
                            j += 1;
                            off_b += REC;
                        } else {
                            src = arr_a.wrapping_add(off_a);
                            i += 1;
                            off_a += REC;
                        }
                    }
                    core::ptr::copy_nonoverlapping(
                        src as *const u8,
                        (dst0.wrapping_add(off_d)) as *mut u8,
                        REC as usize,
                    );
                    off_d += REC;
                    merged += 1;
                }
                n = merged;
            }
            *((cur.add(8)) as *mut u16) = n as u16;
            dst_base = dst_base.wrapping_add(n.wrapping_mul(REC));
            acc = acc.wrapping_add(n);
            cur = cur.add(10);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
        acc
    }
});
