// original: 0x00b53440 euphoria_object_construct
//! Constructor for the large blend-control object: initialise every
//! sub-object in a fixed order and return the object pointer.
//!
//! Specification. The object (about 6.7KB at `this`) is built bottom-up:
//! each sub-object is first passed to its init routine, then stamped with
//! its method table, then driven through that table's slot 1. Two
//! dispatcher chains resolve through the source object `a1`: when the
//! source answers, the resolved handle's slot 56 runs and the word past it
//! becomes the active token stored at `this+0x1a3c`; otherwise the token
//! comes from the source's fallback handle. A three-word signature on the
//! token gates an optional validation round (two alternate token shapes
//! accepted). Three late sub-objects each combine one init answer with the
//! peer object at `this+0x1a04` through their table slot 8. When the peer
//! handle argument `a2` is non-null a second phase runs (peer use checks,
//! two nullable service acquisitions with fallback to null, and teardown
//! of the phase's temporaries); when null the phase is skipped. The frame
//! ends by clearing the low object flag and tearing down the remaining
//! temporaries. Scalar constants, the startup float, and the callee
//! answers are the only value sources; all multi-word temporaries live in
//! the frame and never escape.
//!
//! Proof notes. Direct callees are intercepted by patch; indirect ones by
//! fabrication (heap tables for stub-stamped objects, planted slots for
//! the three constant tables). Frame pointers are never compared (their
//! contents are scratch on both sides); every scalar argument, heap
//! argument, call in the sequence, stored word and the returned pointer
//! is compared.


const T_PRIMARY: u32 = 0x00EA_F370;
const T_AUX: u32 = 0x00EA_F350;
const T_TOKEN: u32 = 0x00EA_F2FC;
const T_SERVICE: u32 = 0x00EA_F3C0;
const G_FLOAT: u32 = 0x0117_35BC;
const G_SVC_A: u32 = 0x0166_9D68;
const G_SVC_B: u32 = 0x0166_9D74;
const ONE_F: u32 = 0x3F80_0000;
const HALF_F: u32 = 0x3F00_0000;
const RATE_F: u32 = 0x3F7D_70A4;
const PEER_MAGIC: u32 = 0x0166_D9C0;
const SVC_MAGIC: u32 = 0x0110_DB10;

#[inline(always)]
unsafe fn rd(obj: u32, off: u32) -> u32 {
    unsafe { ((obj.wrapping_add(off)) as *const u32).read() }
}

#[inline(always)]
unsafe fn wr(obj: u32, off: u32, v: u32) {
    unsafe { ((obj.wrapping_add(off)) as *mut u32).write(v) }
}

#[inline(always)]
unsafe fn rd16(obj: u32, off: u32) -> u16 {
    unsafe { ((obj.wrapping_add(off)) as *const u16).read() }
}

#[inline(always)]
unsafe fn and8(obj: u32, mask: u8) {
    unsafe {
        let p = obj as *mut u8;
        p.write(p.read() & mask);
    }
}

/// Indirect call through table slot 1 (`thiscall/0` on both sides).
#[inline(always)]
unsafe fn thru_slot1(obj: u32) -> u32 {
    unsafe {
        let tbl = rd(obj, 0);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd(tbl, 4) as usize);
        f(obj)
    }
}

/// Indirect call through table slot 8 with three stack words.
#[inline(always)]
unsafe fn thru_slot8(obj: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        let tbl = rd(obj, 0);
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd(tbl, 0x20) as usize);
        f(obj, a, b, c)
    }
}

/// Dispatcher probe: slot 40 of the source's table (`thiscall/0`).
#[inline(always)]
unsafe fn probe40(obj: u32) -> u32 {
    unsafe {
        let tbl = rd(obj, 0);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd(tbl, 0xa0) as usize);
        f(obj)
    }
}

/// Dispatcher follow-up: slot 56 of the answer's table (`thiscall/0`).
#[inline(always)]
unsafe fn follow56(obj: u32) -> u32 {
    unsafe {
        let tbl = rd(obj, 0);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd(tbl, 0xe0) as usize);
        f(obj)
    }
}

lf_checker_rt::export!(thiscall, rw_q19_fn(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        // One scratch word stands in for every frame temporary: their
        // addresses are skipped by the contract and their contents are
        // never read back on either side.
        let mut scratch = [0u32; 4];
        let fp = scratch.as_mut_ptr() as u32;

        // Block 1: two twin sub-objects (head + aux pair).
        lf_checker_rt::callee_thiscall!(37, u32, this.wrapping_add(4));
        wr(this, 4, lf_checker_rt::relocated(T_PRIMARY));
        lf_checker_rt::callee_thiscall!(6, u32, this.wrapping_add(0x8ac));
        wr(this, 0x8ac, lf_checker_rt::relocated(T_AUX));
        wr(this, 0x8b4, this.wrapping_add(0x82c));
        thru_slot1(this.wrapping_add(0x8ac));
        let twin = this.wrapping_add(0x8b8);
        lf_checker_rt::callee_thiscall!(37, u32, twin);
        wr(twin, 0, lf_checker_rt::relocated(T_PRIMARY));
        lf_checker_rt::callee_thiscall!(6, u32, twin.wrapping_add(0x8a8));
        wr(twin, 0x8a8, lf_checker_rt::relocated(T_AUX));
        wr(twin, 0x8b0, twin.wrapping_add(0x828));
        thru_slot1(twin.wrapping_add(0x8a8));

        // Block 2: token slots and the peer object header.
        lf_checker_rt::callee_thiscall!(13, u32, this.wrapping_add(0x116c));
        let tok = this.wrapping_add(0x117c);
        lf_checker_rt::callee_thiscall!(5, u32, tok);
        wr(tok, 0, lf_checker_rt::relocated(T_TOKEN));
        wr(tok, 0xc, 0);
        wr(tok, 8, 0xffff_ffff);
        lf_checker_rt::callee_thiscall!(13, u32, this.wrapping_add(0x118c));
        lf_checker_rt::callee_thiscall!(13, u32, this.wrapping_add(0x119c));
        lf_checker_rt::callee_thiscall!(7, u32, this.wrapping_add(0x11ac));
        lf_checker_rt::callee_thiscall!(7, u32, this.wrapping_add(0x11c4));
        lf_checker_rt::callee_thiscall!(37, u32, this.wrapping_add(0x11dc));
        let peer = this.wrapping_add(0x1a04);
        lf_checker_rt::callee_thiscall!(16, u32, peer);
        wr(this, 0x1a28, 0);
        wr(this, 0x1a2c, 0);
        wr(this, 0x1a30, 0);
        wr(this, 0x1a34, 0);
        wr(this, 0x1a38, 0);
        wr(this, 0x1a40, a2);
        wr(this, 0x1a4c, 0);
        let gf = lf_checker_rt::global::<u32>(G_FLOAT).read();
        and8(this, 0xf9);
        wr(this, 0x1a50, gf);
        lf_checker_rt::callee_thiscall!(41, u32, this.wrapping_add(0x1a28));
        lf_checker_rt::callee_thiscall!(40, u32, this.wrapping_add(0x1a34));

        // Block 3: first token resolution through the source object.
        let src = a1;
        if probe40(src) == 0 {
            wr(this, 0x1a3c, rd(rd(src, 0x100), 4));
        } else {
            let ans = probe40(src);
            wr(this, 0x1a3c, rd(follow56(ans), 4));
        }
        thru_slot1(this.wrapping_add(0x116c));
        thru_slot1(tok);
        thru_slot1(this.wrapping_add(0x118c));
        thru_slot1(this.wrapping_add(0x119c));

        // Block 4: second resolution, feeding the peer link.
        if rd(src, 0xd0) == 0 {
            let v: u32;
            if probe40(src) == 0 {
                v = rd(src, 0x100);
            } else {
                let ans = probe40(src);
                v = follow56(ans);
            }
            lf_checker_rt::callee_thiscall!(17, u32, peer, v, 0);
        } else {
            lf_checker_rt::callee_thiscall!(18, u32, peer, rd(src, 0xd0));
        }
        thru_slot1(peer);

        // Block 5: optional peer use check.
        let c18 = rd(this, 0x1a18);
        if c18 != 0 {
            let r = lf_checker_rt::callee_thiscall!(4, u32, c18, 1);
            if r != 0 {
                wr(this, 0x1a4c, r);
            }
        }

        // Block 6: token signature gate with the validation round.
        let qtok = rd(this, 0x1a3c);
        if rd16(qtok, 0x14) == 0x50 && rd16(qtok, 0x18) == 0xF0 {
            let w16 = rd16(qtok, 0x16);
            if w16 == 0x12 || w16 == 0x15 {
                let v: u32;
                if rd(src, 0xd0) == 0 {
                    let r1 = lf_checker_rt::callee_cdecl!(43, u32, qtok);
                    v = if r1 != 0 {
                        r1
                    } else {
                        lf_checker_rt::callee_cdecl!(39, u32, qtok);
                        lf_checker_rt::callee_cdecl!(43, u32, rd(this, 0x1a3c))
                    };
                } else {
                    let r1 = lf_checker_rt::callee_cdecl!(42, u32, rd(src, 0xd0));
                    v = if r1 != 0 {
                        r1
                    } else {
                        lf_checker_rt::callee_cdecl!(38, u32, rd(src, 0xd0));
                        lf_checker_rt::callee_cdecl!(42, u32, rd(src, 0xd0))
                    };
                }
                lf_checker_rt::callee_thiscall!(20, u32, peer, v);
            }
        }

        // Block 7: peer magic plus the channel pair setup.
        lf_checker_rt::callee_thiscall!(3, u32, peer, lf_checker_rt::relocated(PEER_MAGIC));
        lf_checker_rt::callee_thiscall!(35, u32, fp, 0, 0);
        lf_checker_rt::callee_thiscall!(35, u32, fp, 0, 0);
        let ch1 = this.wrapping_add(0x11ac);
        thru_slot1(ch1);
        lf_checker_rt::callee_thiscall!(8, u32, ch1, 0, 0);
        lf_checker_rt::callee_thiscall!(9, u32, ch1, 1);
        let ch2 = this.wrapping_add(0x11c4);
        thru_slot1(ch2);
        lf_checker_rt::callee_thiscall!(8, u32, ch2, ONE_F, ONE_F);
        lf_checker_rt::callee_thiscall!(9, u32, ch2, 0);
        lf_checker_rt::callee_thiscall!(22, u32, fp, fp, ch2);
        lf_checker_rt::callee_thiscall!(27, u32, fp, fp, fp, HALF_F, 0);
        lf_checker_rt::callee_thiscall!(30, u32, fp, ch1);
        lf_checker_rt::callee_thiscall!(29, u32, fp, ONE_F);
        lf_checker_rt::callee_thiscall!(22, u32, fp, fp, 0);
        lf_checker_rt::callee_thiscall!(19, u32, peer, fp, 0);

        // Block 8: init answers distributed into their slots.
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        lf_checker_rt::callee_thiscall!(14, u32, this.wrapping_add(0x116c), r);
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        lf_checker_rt::callee_thiscall!(14, u32, this.wrapping_add(0x118c), r);
        lf_checker_rt::callee_thiscall!(35, u32, fp, 0, 0);
        lf_checker_rt::callee_thiscall!(25, u32, fp, fp, ONE_F);
        lf_checker_rt::callee_thiscall!(2, u32, fp, ONE_F);
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        lf_checker_rt::callee_thiscall!(19, u32, peer, fp, r);

        // Block 9: first nullable service acquisition.
        let s = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::global::<u32>(G_SVC_A).read());
        let s = if s != 0 {
            lf_checker_rt::callee_thiscall!(11, u32, s);
            wr(s, 0, lf_checker_rt::relocated(T_SERVICE));
            s
        } else {
            0
        };
        lf_checker_rt::callee_thiscall!(12, u32, s, 0x4b2, 0, lf_checker_rt::relocated(SVC_MAGIC));
        lf_checker_rt::callee_thiscall!(31, u32, fp, s);
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        lf_checker_rt::callee_thiscall!(19, u32, peer, fp, r);
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        lf_checker_rt::callee_thiscall!(14, u32, this.wrapping_add(0x119c), r);

        // Block 10: three slot-8 combines over the peer.
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        thru_slot8(this.wrapping_add(0x11dc), peer, r, 1);
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        thru_slot8(this.wrapping_add(4), peer, r, 0);
        let r = lf_checker_rt::callee_thiscall!(10, u32, fp);
        thru_slot8(this.wrapping_add(0x8b8), peer, r, 0);

        // Block 11: second phase, present only with a peer handle.
        if rd(this, 0x1a40) != 0 {
            lf_checker_rt::callee_thiscall!(44, u32, rd(this, 0x1a40), rd(this, 0x1a3c));
            let c40 = rd(this, 0x1a40);
            lf_checker_rt::callee_thiscall!(33, u32, fp, RATE_F, c40, c40.wrapping_add(0x24), 0, 0);
            lf_checker_rt::callee_thiscall!(21, u32, fp);
            let t = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::global::<u32>(G_SVC_B).read());
            let t = if t != 0 {
                lf_checker_rt::callee_thiscall!(5, u32, t);
                wr(t, 0, lf_checker_rt::relocated(T_TOKEN));
                wr(t, 0xc, 5);
                wr(t, 8, 0xffff_fffa);
                t
            } else {
                0
            };
            lf_checker_rt::callee_thiscall!(24, u32, fp, fp, t);
            let e = lf_checker_rt::callee_thiscall!(15, u32, this.wrapping_add(4), 0, fp);
            lf_checker_rt::callee_thiscall!(14, u32, rd(this, 0x1a40).wrapping_add(0x58), e);
            lf_checker_rt::callee_thiscall!(35, u32, fp, 0, 0);
            wr(this, 0x82c, 8);
            lf_checker_rt::callee_thiscall!(15, u32, this.wrapping_add(0x8b8), 0, fp);
            lf_checker_rt::callee_thiscall!(36, u32, fp);
            lf_checker_rt::callee_thiscall!(23, u32, fp);
            lf_checker_rt::callee_thiscall!(34, u32, fp);
        }

        // Block 12: flag clear and teardown of the temporaries.
        and8(this, 0xfe);
        lf_checker_rt::callee_thiscall!(32, u32, fp);
        lf_checker_rt::callee_thiscall!(26, u32, fp);
        lf_checker_rt::callee_thiscall!(36, u32, fp);
        lf_checker_rt::callee_thiscall!(23, u32, fp);
        lf_checker_rt::callee_thiscall!(28, u32, fp);
        lf_checker_rt::callee_thiscall!(23, u32, fp);
        lf_checker_rt::callee_thiscall!(36, u32, fp);
        lf_checker_rt::callee_thiscall!(36, u32, fp);
        this
    }
});
