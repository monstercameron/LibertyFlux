// original: 0x00a93020 stream_visit_slots_filtered (proposed)

/// Visit live set slots selected by a two-stage filter.
///
/// Each slot index below the set count at `set+0x08` (set from its global)
/// whose select byte (`base[i]`, base at `set+0x04`) has bit 7 clear and
/// whose address (`set+0x00 + stride*i`, stride at `set+0x0c`) is nonzero
/// is probed (callee 1, cdecl/1 with the index); a zero answer skips it. A
/// nonzero argument low byte visits it at once (callee 4, cdecl/1 with the
/// index); a zero one runs the second probe (callee 2, cdecl/1) and, when
/// that is zero too, the classifier (callee 3, cdecl/2 with the index and
/// a global flag), visiting only when the classifier's answer has none of
/// the bits 0xc6 set.
///
/// Returns the last callee answer, the select base or 0 for the skip paths,
/// or 0 when no slot ran (entry eax is pinned to 0 by the contract).
/// Cdecl, one argument (only its low byte is read).
lf_checker_rt::export!(cdecl, rw_00a93020(flag: u32) -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const CLASS_GLOBAL: u32 = 0x0103e89c;
        const BASE_OFF: u32 = 0x00;
        const SELECT_OFF: u32 = 0x04;
        const COUNT_OFF: u32 = 0x08;
        const STRIDE_OFF: u32 = 0x0c;
        const SKIP_BIT: u8 = 0x80;
        const VETO_BITS: u32 = 0xc6;
        let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
        if ((set + COUNT_OFF) as *const i32).read_unaligned() <= 0 {
            return 0;
        }
        let mut ret = 0u32;
        let mut i = 0i32;
        while i < ((set + COUNT_OFF) as *const i32).read_unaligned() {
            let selbase = ((set + SELECT_OFF) as *const u32).read_unaligned();
            if (((selbase + (i as u32)) as *const u8).read() & SKIP_BIT) != 0 {
                ret = selbase;
            } else {
                let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
                let base = ((set + BASE_OFF) as *const u32).read_unaligned();
                let slot = base.wrapping_add(stride.wrapping_mul(i as u32));
                if slot == 0 {
                    ret = 0;
                } else {
                    let a1 = lf_checker_rt::callee_cdecl!(1, u32, i as u32);
                    ret = a1;
                    if a1 != 0 {
                        if (flag & 0xff) as u8 != 0 {
                            ret = lf_checker_rt::callee_cdecl!(4, u32, i as u32);
                        } else {
                            let a2 = lf_checker_rt::callee_cdecl!(2, u32, i as u32);
                            ret = a2;
                            if a2 == 0 {
                                let g = lf_checker_rt::global::<u32>(CLASS_GLOBAL)
                                    .read_unaligned();
                                let a3 =
                                    lf_checker_rt::callee_cdecl!(3, u32, i as u32, g);
                                ret = a3;
                                if (a3 & VETO_BITS) == 0 {
                                    ret = lf_checker_rt::callee_cdecl!(4, u32, i as u32);
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        ret
    }
});
