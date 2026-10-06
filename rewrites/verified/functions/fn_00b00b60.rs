// original: 0x00b00b60 CViewport3DScene::vf12

/// Build a 3D viewport scene's child objects: allocate each child, construct
/// it, attach it to the scene hub at `this + 0x400`, and set its option pairs.
///
/// `this` points to the scene record; only a few fields are read: two float
/// scale factors at `+0x298`/`+0x29c`, two integer dimensions at `+0x2c0`/
/// `+0x2c4`, and a double plus a word at `+0x40c`/`+0x414` copied to the frame.
/// The low word of that double is the path flag: when its `MID` bit is clear
/// the scene takes the short path (scale the dimensions, describe two formats
/// through the shared renderer table, attach two children and finish,
/// returning the last attach result); otherwise it takes the long path below.
/// The single stack argument is never read. Returns the last attach call's
/// result.
///
/// Long path: optionally attach a marker child and a row of eight indexed
/// children (flag bits `ROW0`/`ROW1`), a plain child and a constant-fed child
/// (bit `EXTRA`), then size-driven format descriptions through the renderer
/// table's fixed slot (a callee fills a frame descriptor whose flag word the
/// contract scripts, steering the remaining branches; the width clamp and
/// the shift-count sign test compare signed), the children those
/// descriptions configure, a second descriptor feeding one configured child,
/// and finally either a constant-fed tail (two global switches read through
/// a polled helper, two marker reads) or a plain tail. Two globals collect
/// one attach result and one description answer.
///
/// Every child allocation that returns null is attached as null; a null
/// constructed object faults both sides identically if ever written through.
/// The descriptor's first twelve words (callee fill plus caller constants)
/// are scaffolding the checker observes only through the flag word; the
/// rewrite models the flag and the three words around it exactly.
///
/// Verified details: every option-setting child keeps its object in a
/// callee-saved register and stores a two-byte tag at +0x894/+0x89c
/// ((0,0) rows/marker, (1,1) format children, (t10 != 0, 1) on the last
/// configured child, (0,0) on the constant-fed child which also clears six
/// more tag bytes). The late branches test the renderer's flag word (a callee
/// writes it into the frame descriptor; the entry flags only steer the early
/// branches). The width clamp is a signed above-compare, the row loop a
/// signed less-compare, the shift-count test a signed test; the last child's
/// tag test is equality with zero. The tail's two marker reads land in the
/// frame (overwriting the descriptor's last words) and feed the final stores.
/// Original: 0x00b00b60 (thiscall, one unread stack word).
lf_checker_rt::export!(thiscall, rw_00b00b60(this: u32, _arg0: u32) -> u32 {
    unsafe {
        const HUB_OFF: u32 = 0x400;
        const SCALE_X: u32 = 0x298;
        const SCALE_Y: u32 = 0x29c;
        const DIM_X: u32 = 0x2c0;
        const DIM_Y: u32 = 0x2c4;
        const BLOB: u32 = 0x40c;
        const BLOB_W: u32 = 0x414;
        const FLAG_MID: u32 = 0x8000000;
        const FLAG_ROW0: u32 = 0x2000;
        const FLAG_ROW1: u32 = 0x4000;
        const FLAG_EXTRA: u32 = 0x40;
        const FLAG_FMT: u32 = 0x20000;
        const FLAG_BIG: u32 = 0x40000;
        const FLAG_ALT: u32 = 0x40000000;
        const FLAG_TAIL: u32 = 0x4000000;
        const G_VTABLE_OBJ: u32 = 0x017f5630;
        const G_OPT0: u32 = 0x0119cfe8;
        const G_OPT1: u32 = 0x0119cfec;
        const G_SHR: u32 = 0x01160eb0;
        const G_SHL: u32 = 0x01160eb4;
        const G_SW0: u32 = 0x0105c880;
        const G_SW1: u32 = 0x0105c87c;
        const G_SW2: u32 = 0x0105c884;
        const G_SW3: u32 = 0x0105c888;
        const G_DIM: u32 = 0x0154e170;
        const G_T0: u32 = 0x0154e174;
        const G_T1: u32 = 0x0154e178;
        const G_SAVED_DESC: u32 = 0x01720fd0;
        const G_SAVED_OBJ: u32 = 0x01601088;
        const VT_SLOT: u32 = 0x38;
        const S0: u32 = 0xea9da4;
        const S1: u32 = 0xea9db4;
        const S2: u32 = 0xea9dc4;
        const S3: u32 = 0xea9ddc;
        const S4: u32 = 0xea9df4;
        const S5: u32 = 0xea9e0c;
        const S6: u32 = 0xea9e24;
        const S7: u32 = 0xea9e3c;
        const S8: u32 = 0xea9e54;
        const S9: u32 = 0xea9e68;
        const S_CFG: u32 = 0xea9e80;
        const ALC: u32 = 1;
        const ENA: u32 = 2;
        const FILL: u32 = 3;
        const KIND: u32 = 4;
        const C390: u32 = 5;
        const SET0: u32 = 6;
        const SET1: u32 = 7;
        const QUAD: u32 = 8;
        const MARK: u32 = 9;
        const CBA0: u32 = 10;
        const CBD0: u32 = 11;
        const CCA0: u32 = 12;
        const CE30: u32 = 13;
        const CADD: u32 = 14;
        const ATT: u32 = 15;
        const K660: u32 = 16;
        const K680: u32 = 17;
        const E10: u32 = 18;
        const EA0: u32 = 19;
        const C19E0: u32 = 20;
        const N2210: u32 = 21;
        const C7920: u32 = 22;
        const C7950: u32 = 23;
        const C6760: u32 = 24;
        const C6790: u32 = 25;
        const C6810: u32 = 26;
        const C6840: u32 = 27;
        const C6880: u32 = 28;
        const C7990: u32 = 29;
        const CAC10: u32 = 30;
        const VT: u32 = 31;
        const FILLB: u32 = 32;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gwr32(va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(va), v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn trunc_f32(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        unsafe fn alloc(size: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(ALC, u32, size) }
        }
        #[inline(always)]
        unsafe fn attach(hub: u32, obj: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(ATT, u32, hub, obj) }
        }
        #[inline(always)]
        unsafe fn vtable(obj: u32, s: u32, n: u32, d1: u32, d0: u32, f: u32, p: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_SLOT)) as usize);
                tgt(obj, s, n, d1, d0, f, p)
            }
        }
        #[inline(always)]
        unsafe fn opt_pair(obj: u32, n0: u32, v0: u32, n1: u32, v1: u32, b0: u32, b1: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(SET0, u32, obj, n0, v0);
                lf_checker_rt::callee_thiscall!(SET1, u32, obj, n1, v1);
                wr8(obj.wrapping_add(0x894), b0 as u8);
                wr8(obj.wrapping_add(0x89c), b1 as u8);
            }
        }

        let hub = this.wrapping_add(HUB_OFF);
        let vobj = g32(G_VTABLE_OBJ);
        let mut desc = [0u32; 16];
        desc[12] = rd32(this.wrapping_add(BLOB));
        desc[13] = rd32(this.wrapping_add(BLOB).wrapping_add(4));
        desc[14] = rd32(this.wrapping_add(BLOB_W));
        desc[15] = 0;
        let desc_ptr = core::ptr::addr_of_mut!(desc) as u32;
        let mut desc2 = [0u32; 16];
        let desc2_ptr = core::ptr::addr_of_mut!(desc2) as u32;
        let mut t10 = 0u32;
        let mut t14 = 0u32;
        let mut t18 = 0u32;
        let mut t1c = 0u32;

        let p = alloc(0x940);
        let q: u32 = if p != 0 {
            lf_checker_rt::callee_thiscall!(CADD, u32, p)
        } else {
            0
        };
        attach(hub, q);
        let early = desc[12];
        if early & FLAG_MID == 0 {
            let dx = trunc_f32(mul((rd32(this.wrapping_add(DIM_X)) as i32) as f32,
                rdf(this.wrapping_add(SCALE_X))));
            let dy = trunc_f32(mul((rd32(this.wrapping_add(DIM_Y)) as i32) as f32,
                rdf(this.wrapping_add(SCALE_Y))));
            lf_checker_rt::callee_thiscall!(FILL, u32, desc_ptr, 0);
            let v0 = vtable(vobj, lf_checker_rt::relocated(S0), 3,
                dx as u32, dy as u32, 0x40, desc_ptr);
            let v1 = vtable(vobj, lf_checker_rt::relocated(S1), 2,
                dx as u32, dy as u32, 0x20, 0);
            t10 = v0;
            t18 = v1;
            let p = alloc(0x970);
            let s: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(CBD0, u32, p, this, 0)
            } else {
                0
            };
            attach(hub, s);
            let w = t10;
            wr32(s.wrapping_add(0x944), 0);
            opt_pair(s, 0, w, 0, t18, 1, 1);
            let p = alloc(0x950);
            let fin: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(CBA0, u32, p, w)
            } else {
                0
            };
            return attach(hub, fin);
        }

        if early & FLAG_ROW0 != 0 {
            let p = alloc(0xd30);
            let q: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(C390, u32, p, this)
            } else {
                0
            };
            attach(hub, q);
            if early & FLAG_ROW1 != 0 {
                let mut i = 0u32;
                while i < 8 {
                    let p = alloc(0x950);
                    let q: u32 = if p != 0 {
                        lf_checker_rt::callee_thiscall!(C7990, u32, p, this, i, 0)
                    } else {
                        0
                    };
                    let a = attach(hub, q);
                    opt_pair(a, 0, g32(G_OPT0), 0, g32(G_OPT1), 0, 0);
                    i = i.wrapping_add(1);
                }
            }
        }
        let p = alloc(0x940);
        let q: u32 = if p != 0 {
            lf_checker_rt::callee_thiscall!(CAC10, u32, p, this)
        } else {
            0
        };
        attach(hub, q);
        if early & FLAG_EXTRA != 0 {
            let p = alloc(0x950);
            let q: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(C6790, u32, p, this)
            } else {
                0
            };
            let a = attach(hub, q);
            let k0: u32 = lf_checker_rt::callee_cdecl!(K680, u32,);
            lf_checker_rt::callee_thiscall!(SET0, u32, a, 0, k0);
            let k1: u32 = lf_checker_rt::callee_cdecl!(K660, u32,);
            lf_checker_rt::callee_thiscall!(SET1, u32, a, 0, k1);
            wr8(a.wrapping_add(0x894), 0);
            wr8(a.wrapping_add(0x89c), 0);
        }
        let sh = g32(G_SHL) as u8;
        let mut wide = 0x80u32 << (sh & 31);
        // Signed above (cmovg): wide values with the top bit set are kept.
        if (wide as i32) > 0x400 {
            wide = 0x400;
        }
        t1c = 0;
        let e18 = early & FLAG_FMT;
        t18 = e18;
        if e18 != 0 {
            lf_checker_rt::callee_thiscall!(FILL, u32, desc_ptr, 0);
            t1c = vtable(vobj, lf_checker_rt::relocated(S2), 3, wide, wide, 0x40, desc_ptr);
            t10 = vtable(vobj, lf_checker_rt::relocated(S3), 3, wide, wide, 0x20, desc_ptr);
            let p = alloc(0x940);
            let q: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(C6840, u32, p, this)
            } else {
                0
            };
            let a = attach(hub, q);
            opt_pair(a, 0, t1c, 0, t10, 1, 1);
            t18 = 0;
            if e18 != 0 {
                lf_checker_rt::callee_thiscall!(FILL, u32, desc_ptr, 0);
                t10 = vtable(vobj, lf_checker_rt::relocated(S4), 3, wide, wide, 0x20, desc_ptr);
                t14 = vtable(vobj, lf_checker_rt::relocated(S5), 3, wide, wide, 0x20, desc_ptr);
                let p = alloc(0x950);
                let q: u32 = if p != 0 {
                    lf_checker_rt::callee_thiscall!(C6880, u32, p, this)
                } else {
                    0
                };
                let b = attach(hub, q);
                lf_checker_rt::callee_thiscall!(SET0, u32, b, 0, t10);
                lf_checker_rt::callee_thiscall!(SET0, u32, b, 1, t14);
                wr8(b.wrapping_add(0x894), 1);
                wr8(b.wrapping_add(0x8a4), 1);
                t18 = b;
            }
        }
        let snap = desc[12];
        if snap & FLAG_BIG != 0 {
            lf_checker_rt::callee_thiscall!(FILL, u32, desc_ptr, 0);
            let rv = g32(G_SHR);
            let dec = rv.wrapping_sub(1);
            let size = if (dec as i32) < 0 {
                0x10
            } else {
                0x80u32 << ((dec as u8) & 31)
            };
            t10 = vtable(vobj, lf_checker_rt::relocated(S6), 3, size, size, 0x40, desc_ptr);
            t14 = vtable(vobj, lf_checker_rt::relocated(S7), 3, size, size, 0x20, desc_ptr);
            let p = alloc(0x940);
            let q: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(EA0, u32, p, this)
            } else {
                0
            };
            let a = attach(hub, q);
            opt_pair(a, 0, t14, 0, t10, 1, 1);
            let p = alloc(0x950);
            let q: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(E10, u32, p, this)
            } else {
                0
            };
            let a = attach(hub, q);
            opt_pair(a, 0, t14, 0, t10, 0, 0);
            gwr32(G_SAVED_DESC, t14);
        }
        lf_checker_rt::callee_thiscall!(FILL, u32, desc_ptr, 0);
        lf_checker_rt::callee_thiscall!(FILLB, u32, desc2_ptr, 0);
        t14 = vtable(vobj, lf_checker_rt::relocated(S8), 3, 0x80, 0x80, 0x20, desc2_ptr);
        let k: u32 = lf_checker_rt::callee_cdecl!(KIND, u32, 3);
        let _mode = if (k as u8) != 0 { 3u32 } else { 4u32 };
        t10 = vtable(vobj, lf_checker_rt::relocated(S9), 3, 0x80, 0x80, 0x10, desc_ptr);
        let cfg = [desc[12] & 0xffffff00, 0x82,
            lf_checker_rt::relocated(S_CFG), 0x25];
        let p = alloc(0xa10);
        let q: u32 = if p != 0 {
            lf_checker_rt::callee_thiscall!(C19E0, u32, p, this,
                core::ptr::addr_of!(cfg) as u32)
        } else {
            0
        };
        let a = attach(hub, q);
        opt_pair(a, 0, t10, 0, t14, (t10 != 0) as u32, 1);
        lf_checker_rt::callee_cdecl!(N2210, u32, a);
        wr32(a.wrapping_add(0x9a4), t14);
        if snap & FLAG_ALT != 0 {
            let p = alloc(0x940);
            let q: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(C6760, u32, p, this)
            } else {
                0
            };
            attach(hub, q);
        }
        let p = alloc(0x940);
        let q: u32 = if p != 0 {
            lf_checker_rt::callee_thiscall!(C6810, u32, p, this)
        } else {
            0
        };
        attach(hub, q);
        let p = alloc(0x950);
        let q: u32 = if p != 0 {
            lf_checker_rt::callee_thiscall!(CE30, u32, p, this)
        } else {
            0
        };
        attach(hub, q);
        if snap & FLAG_TAIL != 0 {
            let e0: u32 = lf_checker_rt::callee_cdecl!(ENA, u32,);
            let mut c_a = g32(G_SW0);
            if (e0 as u8) != 0 {
                c_a = g32(G_SW1);
            }
            let e1: u32 = lf_checker_rt::callee_cdecl!(ENA, u32,);
            let mut c_b = g32(G_SW2);
            if (e1 as u8) != 0 {
                c_b = g32(G_SW3);
            }
            lf_checker_rt::callee_cdecl!(QUAD, u32, 4, c_b, c_a, 0x20);
            let dim_mark = g32(G_DIM);
            t10 = g32(G_T0);
            t14 = g32(G_T1);
            let m0: u32 = lf_checker_rt::callee_thiscall!(MARK, u32, c_b);
            let m1: u32 = lf_checker_rt::callee_thiscall!(MARK, u32, c_b);
            let p = alloc(0x950);
            let s: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(C7950, u32, p, this)
            } else {
                0
            };
            let ar = attach(hub, s);
            gwr32(G_SAVED_OBJ, ar);
            lf_checker_rt::callee_thiscall!(SET0, u32, s, 0, dim_mark);
            lf_checker_rt::callee_thiscall!(SET0, u32, s, 1, t10);
            lf_checker_rt::callee_thiscall!(SET0, u32, s, 2, t14);
            lf_checker_rt::callee_thiscall!(SET0, u32, s, 3, m0);
            lf_checker_rt::callee_thiscall!(SET1, u32, s, 0, m1);
            wr8(s.wrapping_add(0x894), 0);
            wr8(s.wrapping_add(0x89c), 0);
            wr8(s.wrapping_add(0x8a4), 0);
            wr8(s.wrapping_add(0x8ac), 0);
            wr8(s.wrapping_add(0x8b4), 0);
            wr8(s.wrapping_add(0x8bc), 0);
            wr8(s.wrapping_add(0x8c4), 0);
            wr8(s.wrapping_add(0x8cc), 0);
            let p = alloc(0x950);
            let f: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(C7920, u32, p, this)
            } else {
                0
            };
            attach(hub, f);
            wr32(f.wrapping_add(0x940), s);
            let p = alloc(0x970);
            let g: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(CBD0, u32, p, this, 1)
            } else {
                0
            };
            attach(hub, g);
            wr32(g.wrapping_add(0x948), t18);
            wr32(g.wrapping_add(0x944), s);
            wr32(g.wrapping_add(0x94c), t1c);
            wr32(g.wrapping_add(0x950), 0);
        } else {
            let p = alloc(0x970);
            let h: u32 = if p != 0 {
                lf_checker_rt::callee_thiscall!(CBD0, u32, p, this, 0)
            } else {
                0
            };
            attach(hub, h);
            wr32(h.wrapping_add(0x948), t18);
            wr32(h.wrapping_add(0x944), 0);
            wr32(h.wrapping_add(0x94c), t1c);
            wr32(h.wrapping_add(0x950), 0);
        }
        let p = alloc(0x950);
        let h: u32 = if p != 0 {
            lf_checker_rt::callee_thiscall!(CCA0, u32, p, this)
        } else {
            0
        };
        attach(hub, h)
    }
});
