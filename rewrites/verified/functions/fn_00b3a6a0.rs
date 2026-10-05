// original: 0x00B3A6A0 ped_task_candidate_eval (proposed)

/// Evaluate scanner candidates around a point, returning 1 only if the
/// scanner exhausts without any candidate failing, else 0.
///
/// `obj` is an object whose function table's slot 0x54 yields a probe
/// callee; `center` points at three floats; `radius` is a float threshold;
/// `tag` is an opaque word forwarded to the scanner callee.
///
/// Behaviour: the probe callee's return value supplies a kind byte (at
/// +0x14). A lookup callee maps the kind to a handle, then a setup callee
/// takes (kind, two out-slots) and returns a flag; the out-slots receive a
/// float bound and a small count limit. When the kind is 0x29, a chain of
/// three further callees must all succeed (non-null, non-null, returning
/// more than 1) for an override: flag 1, limit 2, bound 250.0. The working
/// radius is the larger of `radius` and the bound (unordered comparisons
/// keep the bound). The center is copied aside; when the handle's word at
/// +0x14 is 0 or 5 the copy's z grows by 1.0 and the distance limit becomes
/// 0.25, otherwise the limit is the squared radius. An entity-scanner
/// callee is then primed and its next-item callee polled: a null item ends
/// the scan with 1. Each item's position (inline at +0x10 when its word at
/// +0x20 is null, else through that word at +0x30) gives a squared distance
/// `((dx^2+dy^2)+dz^2)` in that operation order; a distance below the limit
/// ends the scan with 0. When the setup flag is set, the item's slot-0xD0
/// callee is invoked; a null answer skips the item, else a state callee
/// takes (answer, item, 0): equalling the kind counts the item, else the
/// handle's word at +0x20 decides (equal counts, -1 or anything else falls
/// through to a marker check requiring the override flag, a value of 2 at
/// the answer's +0x12c, and item bits that are 6, or 3 with bit 2 of its
/// +0x26c byte clear). Reaching the count limit ends the scan with 0. Both
/// exits invoke a teardown callee first.
///
/// Original: 0x00B3A6A0 (cdecl, four stack words, returns 0/1 in al).
lf_checker_rt::export!(cdecl, rw_00B3A6A0(obj: u32, center: u32, radius: u32, tag: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 2;
        const SETUP_CALLEE: u32 = 3;
        const CHAIN_A_CALLEE: u32 = 4;
        const CHAIN_B_CALLEE: u32 = 5;
        const CHAIN_C_CALLEE: u32 = 6;
        const SCANNER_CALLEE: u32 = 7;
        const NEXT_CALLEE: u32 = 8;
        const STATE_CALLEE: u32 = 10;
        const TEARDOWN_CALLEE: u32 = 11;
        const SPECIAL_KIND: u32 = 0x29;
        const F250_ADDR: u32 = 0xE8E300;
        const Z_BUMP_ADDR: u32 = 0xFE88E8;
        const ADJ_LIMIT_ADDR: u32 = 0xFE87E4;

        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn vcall0(slot_addr: u32, this: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(slot_addr) as usize);
                f(this)
            }
        }

        let rad = f32::from_bits(radius);
        let vt = rd32(obj);
        let probe = vcall0(vt.wrapping_add(0x54), obj);
        let kind = ((probe.wrapping_add(0x14)) as *const u8).read() as u32;
        let handle: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, kind);
        let mut bound = 0u32;
        let mut limit = 999u32;
        let mut flag17 = 0u8;
        let setup: u32 = lf_checker_rt::callee_cdecl!(
            SETUP_CALLEE,
            u32,
            kind,
            &mut bound as *mut u32 as u32,
            &mut limit as *mut u32 as u32
        );
        let mut flag16 = (setup as u8) != 0;
        if kind == SPECIAL_KIND {
            let a: u32 = lf_checker_rt::callee_cdecl!(CHAIN_A_CALLEE, u32, 0u32);
            if a != 0 {
                let b: u32 = lf_checker_rt::callee_cdecl!(CHAIN_B_CALLEE, u32,);
                if b != 0 {
                    let cc: u32 = lf_checker_rt::callee_cdecl!(CHAIN_B_CALLEE, u32,);
                    let d: u32 = lf_checker_rt::callee_thiscall!(CHAIN_C_CALLEE, u32, cc);
                    if (d as i32) > 1 {
                        flag16 = true;
                        limit = 2;
                        bound = rd32(lf_checker_rt::relocated(F250_ADDR));
                        flag17 = 1;
                    }
                }
            }
        }
        let xmax = f32::from_bits(bound);
        let work = if rad > xmax { rad } else { xmax };
        let mut cc = [rdf(center), rdf(center.wrapping_add(4)), rdf(center.wrapping_add(8))];
        let hw = rd32(handle.wrapping_add(0x14));
        let dist_lim: f32;
        if hw == 0 || hw == 5 {
            cc[2] = fadd(cc[2], rdf(lf_checker_rt::relocated(Z_BUMP_ADDR)));
            dist_lim = rdf(lf_checker_rt::relocated(ADJ_LIMIT_ADDR));
        } else {
            dist_lim = fmul(rad, rad);
        }
        let mut scan = [0u32; 16];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SCANNER_CALLEE,
            u32,
            scan.as_mut_ptr() as u32,
            3u32,
            tag,
            cc.as_mut_ptr() as u32,
            work.to_bits()
        );
        let mut count = 0u32;
        let mut item: u32 =
            lf_checker_rt::callee_thiscall!(NEXT_CALLEE, u32, scan.as_mut_ptr() as u32);
        if item == 0 {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, scan.as_mut_ptr() as u32);
            return 1;
        }
        loop {
            let p20 = rd32(item.wrapping_add(0x20));
            let pos = if p20 == 0 {
                item.wrapping_add(0x10)
            } else {
                p20.wrapping_add(0x30)
            };
            let dx = fsub(cc[0], rdf(pos));
            let dy = fsub(cc[1], rdf(pos.wrapping_add(4)));
            let dz = fsub(cc[2], rdf(pos.wrapping_add(8)));
            let d2 = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
            if dist_lim > d2 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    TEARDOWN_CALLEE,
                    u32,
                    scan.as_mut_ptr() as u32
                );
                return 0;
            }
            let mut counted = false;
            if flag16 {
                let ivt = rd32(item);
                let h2 = vcall0(ivt.wrapping_add(0xD0), item);
                if h2 != 0 {
                    let e: u32 =
                        lf_checker_rt::callee_thiscall!(STATE_CALLEE, u32, h2, item, 0u32);
                    if e == kind {
                        counted = true;
                    } else {
                        let h20 = rd32(handle.wrapping_add(0x20));
                        if h20 != 0xFFFF_FFFF && h20 == e {
                            counted = true;
                        } else if flag17 != 0 && rd32(h2.wrapping_add(0x12C)) == 2 {
                            let z = (rd32(item.wrapping_add(0x28)) >> 6) & 0xF;
                            if z == 6 {
                                counted = true;
                            } else if z == 3 {
                                let mb: u8 =
                                    ((item.wrapping_add(0x26C)) as *const u8).read();
                                if mb & 4 == 0 {
                                    counted = true;
                                }
                            }
                        }
                    }
                }
            }
            if counted {
                count = count.wrapping_add(1);
                if (count as i32) >= (limit as i32) {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        TEARDOWN_CALLEE,
                        u32,
                        scan.as_mut_ptr() as u32
                    );
                    return 0;
                }
            }
            item = lf_checker_rt::callee_thiscall!(NEXT_CALLEE, u32, scan.as_mut_ptr() as u32);
            if item == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    TEARDOWN_CALLEE,
                    u32,
                    scan.as_mut_ptr() as u32
                );
                return 1;
            }
        }
    }
});

///
