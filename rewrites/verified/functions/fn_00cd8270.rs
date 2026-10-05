// original: 0x00CD8270 CTaskComplexFollowPedFootsteps::vf20

/// Original: 0x00CD8270 (thiscall, one stack word, callee pops 4).
export!(thiscall, rw_00cd8270(task: u32, ped: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x08;
        const TARGET: u32 = 0x14;
        const TRAIL: u32 = 0x20;
        const STAMP1: u32 = 0x1c;
        const STAMP2: u32 = 0x18;
        const MODEW: u32 = 0x24;
        const FLAGW: u32 = 0x26;
        const KIND_BYTE: u32 = 0x210;
        const KIND_STATE: u32 = 0xa74;
        const LINK: u32 = 0x224;
        const DONE_BYTE: u32 = 0x26c;
        const DONE_BIT: u8 = 4;
        const GRAND: u32 = 0x14;
        const STATE_SLOT: u32 = 0x0c;
        const POS: u32 = 0x20;
        const NAV: u32 = 0x224;
        const SEL_INIT: u32 = 0xc8;
        const SEL_BAIL: u32 = 0x516;
        const SEL_DONE: u32 = 0x38b;
        const SEL_LATE: u32 = 0x1f4;
        const SEL_ZERO: u32 = 0xcb;
        const SEL_SHORT: u32 = 0x384;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn hook0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let tgt = rd32(rd32(obj).wrapping_add(slot));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn child_ptr(task: u32) -> u32 {
            unsafe { rd32(task.wrapping_add(CHILD)) }
        }

        unsafe fn dispatch(task: u32, ped: u32, sel: u32) -> u32 {
            unsafe {
                let ch = child_ptr(task);
                if rd8(ch.wrapping_add(0x0c)) & 1 == 0 {
                    let tgt = rd32(rd32(ch).wrapping_add(0x14));
                    let admit: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    if admit(ch, ped, 1, 0) & 0xff == 0 {
                        return child_ptr(task);
                    }
                    let f = rd32(ch.wrapping_add(0x0c)) | 2;
                    (ch.wrapping_add(0x0c) as *mut u32).write_unaligned(f);
                }
                let tgt = rd32(rd32(task).wrapping_add(0x54));
                let mode: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                mode(task, sel, ped)
            }
        }

        let mut sel = SEL_INIT;
        let child = child_ptr(task);
        let t14 = rd32(task.wrapping_add(TARGET));
        let mut bail = false;
        if t14 == 0 {
            bail = true;
        } else if rd32(task.wrapping_add(TRAIL)) == 0 {
            bail = true;
        } else {
            if rd8(t14.wrapping_add(KIND_BYTE)) != 0 {
                let ks = rd32(t14.wrapping_add(KIND_STATE));
                if ks == 1 || ks == 2 {
                    bail = true;
                }
            }
            if !bail && rd32(t14.wrapping_add(LINK)) == 0 {
                bail = true;
            }
        }
        if bail {
            return dispatch(task, ped, SEL_BAIL);
        }
        if hook0(child, STATE_SLOT) == 0x11d {
            let g = rd32(child_ptr(task).wrapping_add(GRAND));
            if g != 0 && hook0(g, STATE_SLOT) == 0x38b {
                return child_ptr(task);
            }
        }
        let edx = rd32(task.wrapping_add(TARGET));
        if rd8(edx.wrapping_add(DONE_BYTE)) & DONE_BIT != 0 {
            return dispatch(task, ped, SEL_DONE);
        }
        let clk = rd32(rd32(ped.wrapping_add(NAV)).wrapping_add(0x264));
        if (clk as i32) > 30 {
            return dispatch(task, ped, SEL_LATE);
        }
        let g0: u32 = *global::<u32>(0x11735B4);
        if g0.wrapping_sub(rd32(task.wrapping_add(STAMP1))) > 0x1f4 {
            (task.wrapping_add(STAMP1) as *mut u32).write_unaligned(g0);
            let e1 = rd32(edx.wrapping_add(POS)).wrapping_add(0x30);
            let e2 = rd32(ped.wrapping_add(POS)).wrapping_add(0x30);
            let a: u32 = callee_cdecl!(3, u32, e2, e1, 0, 0x86, 0, 0);
            let w = rd16(task.wrapping_add(FLAGW));
            let nw = (w & 0xfffe) | ((a & 1) as u16);
            (task.wrapping_add(FLAGW) as *mut u16).write_unaligned(nw);
        }
        let la = rd32(rd32(task.wrapping_add(TARGET)).wrapping_add(POS));
        let fa = rd32(ped.wrapping_add(POS));
        let lx = rdf(la.wrapping_add(0x30));
        let ly = rdf(la.wrapping_add(0x34));
        let lz = rdf(la.wrapping_add(0x38));
        let dx = fsub(lx, rdf(fa.wrapping_add(0x30)));
        let dy = fsub(ly, rdf(fa.wrapping_add(0x34)));
        let dz = fsub(lz, rdf(fa.wrapping_add(0x38)));
        let n2 = add(mul(dz, dz), add(mul(dy, dy), mul(dx, dx)));
        let c1 = f32::from_bits(*global::<u32>(0xFE88E8));
        let rt = core::hint::black_box(add(mul(dy, dy), mul(dx, dx))).sqrt();
        if c1 > n2 {
            if hook0(child, STATE_SLOT) == SEL_ZERO {
                // rejoins with the same registers
            } else if rd8(task.wrapping_add(FLAGW)) & 1 == 0 {
                return child_ptr(task);
            } else {
                let fs0 = rd32(task.wrapping_add(TRAIL));
                (fs0 as *mut u32).write_unaligned(0);
                return dispatch(task, ped, SEL_ZERO);
            }
        }
        let c64 = f32::from_bits(*global::<u32>(0xFE8B88));
        if n2 > c64 {
            return dispatch(task, ped, SEL_DONE);
        }
        if c1 > rt {
            let m: u32 = *global::<u32>(0xFE8F80);
            let az = f32::from_bits(dz.to_bits() & m);
            let c2 = f32::from_bits(*global::<u32>(0xFE8A24));
            if az > c2 {
                return dispatch(task, ped, SEL_DONE);
            }
        }
        if rd8(task.wrapping_add(FLAGW)) & 1 != 0 {
            let fs0 = rd32(task.wrapping_add(TRAIL));
            (fs0 as *mut u32).write_unaligned(0);
            let bptr = rd32(rd32(task.wrapping_add(TARGET)).wrapping_add(POS)).wrapping_add(0x30);
            let _: u32 = callee_thiscall!(4, u32, fs0, bptr);
            let g2: u32 = *global::<u32>(0x11735B4);
            (task.wrapping_add(STAMP2) as *mut u32)
                .write_unaligned(g2.wrapping_add(0xffff_feb4));
            let nav = rd32(ped.wrapping_add(NAV)).wrapping_add(0x44);
            let c: u32 = callee_thiscall!(7, u32, nav);
            if c != 0 && hook0(child, STATE_SLOT) == 0x11d {
                let c14 = rd32(child.wrapping_add(GRAND));
                if c14 != 0 && hook0(c14, STATE_SLOT) == 0x384 {
                    let d: u32 = callee_thiscall!(8, u32, child, ped);
                    if d != 0 {
                        let da14 = d.wrapping_add(GRAND);
                        let tgt = rd32(rd32(da14).wrapping_add(0x1c));
                        let meas: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                            core::mem::transmute(tgt as usize);
                        let fword = [0u32; 1];
                        let fptr = (&fword[0] as *const u32) as u32;
                        let edx2 =
                            rd32(rd32(task.wrapping_add(TARGET)).wrapping_add(POS)).wrapping_add(0x30);
                        let ha: u32 = meas(da14, fptr, edx2, 0x3e00_0000);
                        let e: u32 = callee_thiscall!(14, u32, ha);
                        if e & 0xff == 0 {
                            sel = SEL_SHORT;
                        }
                    }
                }
            }
        }
        let g3: u32 = *global::<u32>(0x11735B4);
        if g3.wrapping_sub(rd32(task.wrapping_add(STAMP2))) < 0xa6 {
            // short-trail exit
            if hook0(child, STATE_SLOT) != SEL_ZERO {
                if sel != SEL_INIT {
                    return dispatch(task, ped, sel);
                }
                return child_ptr(task);
            }
            let fsx = rd32(task.wrapping_add(TRAIL));
            if (rd32(fsx) as i32) <= 1 {
                if sel != SEL_INIT {
                    return dispatch(task, ped, sel);
                }
                return child_ptr(task);
            }
            return dispatch(task, ped, SEL_SHORT);
        }
        let fs = rd32(task.wrapping_add(TRAIL));
        if (rd32(fs) as i32) >= 8 {
            if hook0(child, STATE_SLOT) != SEL_ZERO {
                if sel != SEL_INIT {
                    return dispatch(task, ped, sel);
                }
                return child_ptr(task);
            }
            let fsx = rd32(task.wrapping_add(TRAIL));
            if (rd32(fsx) as i32) <= 1 {
                if sel != SEL_INIT {
                    return dispatch(task, ped, sel);
                }
                return child_ptr(task);
            }
            return dispatch(task, ped, SEL_SHORT);
        }
        (task.wrapping_add(STAMP2) as *mut u32).write_unaligned(g3);
        let count = rd32(fs);
        if count == 0 {
            let lead = [lx.to_bits(), ly.to_bits(), lz.to_bits()];
            let lptr = (&lead[0] as *const u32) as u32;
            let _: u32 = callee_thiscall!(5, u32, fs, lptr);
        } else {
            let ex = rdf(fs.wrapping_add(count.wrapping_mul(16)));
            let ey = rdf(fs.wrapping_add(count.wrapping_mul(16).wrapping_add(4)));
            let ez = rdf(fs.wrapping_add(count.wrapping_mul(16).wrapping_add(8)));
            let ix = fsub(lx, ex);
            let iy = fsub(ly, ey);
            let iz = fsub(lz, ez);
            let n2b = add(add(mul(iy, iy), mul(ix, ix)), mul(iz, iz));
            let c00625 = f32::from_bits(*global::<u32>(0xFE8778));
            if n2b > c00625 {
                if (count as i32) >= 2 {
                    let px = rdf(fs.wrapping_add(count.wrapping_mul(16).wrapping_sub(16)));
                    let py = rdf(fs.wrapping_add(count.wrapping_mul(16).wrapping_sub(12)));
                    let pz = rdf(fs.wrapping_add(count.wrapping_mul(16).wrapping_sub(8)));
                    let ddx = fsub(ex, px);
                    let ddy = fsub(ey, py);
                    let ddz = fsub(ez, pz);
                    let mut buf40 = [ddx.to_bits(), ddy.to_bits(), ddz.to_bits()];
                    let mut buf20 = [ix.to_bits(), iy.to_bits(), iz.to_bits()];
                    let p40 = (&mut buf40[0] as *mut u32) as u32;
                    let p20 = (&mut buf20[0] as *mut u32) as u32;
                    let _: u32 = callee_thiscall!(15, u32, p40);
                    let _: u32 = callee_thiscall!(15, u32, p20);
                    let w40 = f32::from_bits(buf40[0]);
                    let w44 = f32::from_bits(buf40[1]);
                    let w48 = f32::from_bits(buf40[2]);
                    let w20 = f32::from_bits(buf20[0]);
                    let w24 = f32::from_bits(buf20[1]);
                    let w28 = f32::from_bits(buf20[2]);
                    let dot = add(add(mul(w44, w24), mul(w40, w20)), mul(w48, w28));
                    let c095 = f32::from_bits(*global::<u32>(0xFE88C4));
                    if !(dot < c095) {
                        let _: u32 =
                            callee_thiscall!(16, u32, fs, count.wrapping_sub(1));
                    }
                }
                let lead = [lx.to_bits(), ly.to_bits(), lz.to_bits()];
                let lptr = (&lead[0] as *const u32) as u32;
                let _: u32 = callee_thiscall!(5, u32, fs, lptr);
            }
        }
        let fsa = rd32(task.wrapping_add(TRAIL));
        let mut acc = 0.0f32;
        let cnt = rd32(fsa) as i32;
        let bx = rdf(fsa.wrapping_add(0x10));
        let by = rdf(fsa.wrapping_add(0x14));
        let bz = rdf(fsa.wrapping_add(0x18));
        if cnt > 1 {
            let mut i = 0i32;
            while i < cnt - 1 {
                let ex = rdf(fsa.wrapping_add(0x20).wrapping_add((i as u32).wrapping_mul(16)));
                let ey = rdf(fsa.wrapping_add(0x24).wrapping_add((i as u32).wrapping_mul(16)));
                let ez = rdf(fsa.wrapping_add(0x28).wrapping_add((i as u32).wrapping_mul(16)));
                let ddx = fsub(ex, bx);
                let ddy = fsub(ey, by);
                let ddz = fsub(ez, bz);
                let t = add(add(mul(ddy, ddy), mul(ddx, ddx)), mul(ddz, ddz));
                acc = add(acc, t);
                i += 1;
            }
        }
        let xa = rd32(ped.wrapping_add(POS));
        let xc = rd32(rd32(task.wrapping_add(TARGET)).wrapping_add(POS));
        let fx = fsub(rdf(xa.wrapping_add(0x30)), rdf(xc.wrapping_add(0x30)));
        let fy = fsub(rdf(xa.wrapping_add(0x34)), rdf(xc.wrapping_add(0x34)));
        let fz = fsub(rdf(xa.wrapping_add(0x38)), rdf(xc.wrapping_add(0x38)));
        let n4 = add(add(add(mul(fy, fy), mul(fx, fx)), mul(fz, fz)), acc);
        let c625 = f32::from_bits(*global::<u32>(0xEA6D40));
        let wv: u16 = if n4 > c625 { 3 } else { 2 };
        (task.wrapping_add(MODEW) as *mut u16).write_unaligned(wv);
        if hook0(child, STATE_SLOT) == 0x11d {
            let c14 = rd32(child.wrapping_add(GRAND));
            if c14 != 0 && hook0(c14, STATE_SLOT) == 0x384 {
                let d: u32 = callee_thiscall!(8, u32, child, ped);
                if d != 0 {
                    let da14 = d.wrapping_add(GRAND);
                    let tgt = rd32(rd32(da14).wrapping_add(0x0c));
                    let stride: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    let sx = rd16(task.wrapping_add(MODEW)) as i16 as i32 as u32;
                    let _: u32 = stride(da14, sx);
                }
            }
        }
        if sel != SEL_INIT {
            return dispatch(task, ped, sel);
        }
        child_ptr(task)
    }
});
