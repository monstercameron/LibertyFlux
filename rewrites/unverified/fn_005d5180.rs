// original: 0x005d5180 input_binding_attach (proposed)
//
// Attach an input binding to its device and action slots.
//
// `this` is the binding and the two stack words are a flags value and a
// cookie. Four virtual calls configure it first (slots `+0x30` with 0,
// `+0xc` with pi/2-ish 0x40490fdb, a direct mode call with 2, slot `+0xd0`
// whose answer receives the flags at `+0x12c`, slot `+0xf4` with
// (100.0, 0)),
// then a direct link call with (1, 0) and flag bit 0 of `+0x24` is set.
// Two allocator requests follow through TLS slot 0 (tags 0xa8 and 0x60):
// each live answer is adapted by a direct callee and stored (`+0xe68`,
// `+0xe60`); a null first answer stores null, while a null second answer
// faults writing through it, exactly as the original does. The device
// object at `+0x34` contributes two words (`+0xc` to a direct callee,
// `+4` to `+0xe64` and to the adapted answer's `+0x58`). A register
// callee then takes (`this`, `+0x21c` + 0x80, `+0x21c`, 0), a sub-object
// call takes `this + 0x2b0` with `this`, and a factory callee on a global
// object either yields the action record (stamped with a vtable, two
// all-ones words and the cookie) or null; the record, 4 and 0 go to a
// final direct call on `+0x224` + 0x44, and `+0x41` is set to 2.
//
// Original: 0x005d5180 (thiscall, two stack words; returns the final
// callee's answer).
lf_checker_rt::export!(thiscall, rw_005d5180(this: u32, flags: u32, cookie: u32) -> u32 {
    unsafe {
        const ID_MODE: u32 = 3;
        const ID_LINK: u32 = 6;
        const ID_ADAPT1: u32 = 8;
        const ID_DEVFN: u32 = 9;
        const ID_ADAPT2: u32 = 10;
        const ID_REGISTER: u32 = 11;
        const ID_SUBOBJ: u32 = 12;
        const ID_FACTORY: u32 = 13;
        const ID_ACTION: u32 = 14;
        const ID_FINAL: u32 = 15;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let vt = rd32(this);
        wr8(this.wrapping_add(0xe6c), 0);
        let v30: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(0x30)) as usize);
        let _: u32 = v30(this, 0);
        let v0c: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(0x0c)) as usize);
        let _: u32 = v0c(this, 0x40490fdb);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_MODE, u32, this, 2);
        let vd0: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(0xd0)) as usize);
        let holder = vd0(this);
        wr32(holder.wrapping_add(0x12c), flags);
        let vf4: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this).wrapping_add(0xf4)) as usize);
        let _: u32 = vf4(this, 0x42c80000, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_LINK, u32, this, 1, 0);
        wr32(this.wrapping_add(0x24), rd32(this.wrapping_add(0x24)) | 1);
        let tls0 = lf_checker_rt::tls_slot(0);
        let alloc = rd32(tls0.wrapping_add(8));
        let request: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(alloc).wrapping_add(8)) as usize);
        let r1 = request(alloc, 0xa8, 0x10, 0);
        let c1 = if r1 != 0 {
            lf_checker_rt::callee_thiscall!(ID_ADAPT1, u32, r1)
        } else {
            0
        };
        wr32(this.wrapping_add(0xe68), c1);
        let dev = rd32(this.wrapping_add(0x34));
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_DEVFN, u32, c1, rd32(rd32(dev).wrapping_add(0x0c)));
        let r2 = request(rd32(tls0.wrapping_add(8)), 0x60, 0x10, 0);
        let c2 = if r2 != 0 {
            lf_checker_rt::callee_thiscall!(ID_ADAPT2, u32, r2)
        } else {
            0
        };
        wr32(this.wrapping_add(0xe60), c2);
        let w = rd32(rd32(this.wrapping_add(0x34)).wrapping_add(4));
        wr32(this.wrapping_add(0xe64), w);
        wr32(c2.wrapping_add(0x58), w);
        let v = rd32(this.wrapping_add(0x21c));
        let _: u32 = lf_checker_rt::callee_cdecl!(
            ID_REGISTER, u32, this, v.wrapping_add(0x80), v, 0
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_SUBOBJ, u32, this.wrapping_add(0x2b0), this);
        wr32(this.wrapping_add(0xe70), cookie);
        let fac: u32 = lf_checker_rt::callee_thiscall!(
            ID_FACTORY, u32, rd32(lf_checker_rt::relocated(0x0167e2a0))
        );
        let rec = if fac != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(ID_ACTION, u32, fac);
            wr32(fac, lf_checker_rt::relocated(0x00e9f0bc));
            wr8(fac.wrapping_add(0x14), 0);
            wr32(fac.wrapping_add(0x18), 0xffff_ffff);
            wr32(fac.wrapping_add(0x1c), 0xffff_ffff);
            wr32(fac.wrapping_add(0x20), rd32(this.wrapping_add(0xe70)));
            fac
        } else {
            0
        };
        let fin: u32 = lf_checker_rt::callee_thiscall!(
            ID_FINAL, u32, rd32(this.wrapping_add(0x224)).wrapping_add(0x44), rec, 4, 0
        );
        wr8(this.wrapping_add(0x41), 2);
        fin
    }
});
