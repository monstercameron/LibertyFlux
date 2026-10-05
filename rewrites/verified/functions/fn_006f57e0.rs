// original: 0x006F57E0 input_device_reset (proposed)

/// Reset the input device `this`, draining its queues and reseeding its
/// sequence words, returning a mix of the time source.
///
/// Drains up to one entry from each of two queues (the pop callee clears the
/// pending flag, so each loop runs at most once): each entry's stamp word is
/// noted through the stamp callee and the entry is released twice through
/// the device's virtual slot `+0xC`. Then walks the `+0x50` link chain of the
/// attach object to its tail, notes it, and releases the tail through
/// virtual slot `+4`. A null attach object with work pending faults reading
/// address 0, like the original. Constant timing parameters are stored at
/// `+0x30..+0x40` and several state words are zeroed. The time source is a
/// global function pointer plus a global query pointer when the former is
/// set, else the system millisecond clock; the result is mixed with the
/// object address and a multiply-add chain (`STEP = 0x5CDCFAA7`) into the
/// sequence words at `+0x80..+0x86`, with a divide remainder when the first
/// word is zero and shift-or merges otherwise.
/// Thiscall with no stack words.
lf_checker_rt::export!(thiscall, rw_006F57E0(this: u32) -> u32 {
    unsafe {
        const ID_POP: u32 = 1;
        const ID_STAMP: u32 = 2;
        const ID_RELEASE: u32 = 3;
        const ID_ATTACH: u32 = 4;
        const ID_TAIL: u32 = 5;
        const ID_CLOCK_FN: u32 = 6;
        const ID_CLOCK_Q: u32 = 7;
        const ID_TICK: u32 = 8;
        const STEP: u64 = 0x5CDC_FAA7;
        const G_CLOCK_FN: u32 = 0x017A_CD20;
        const G_CLOCK_Q: u32 = 0x017A_CD00;
        const VT_RELEASE: u32 = 0x0C;
        const VT_TAIL: u32 = 0x04;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// One 64-bit multiply-add step: `x * STEP + y` as (lo, hi).
        #[inline(always)]
        fn mac(x: u32, y: u32) -> (u32, u32) {
            let p = (x as u64).wrapping_mul(STEP).wrapping_add(y as u64);
            (p as u32, (p >> 32) as u32)
        }
        /// Release call through virtual slot `+0xC` of the device object.
        #[inline(always)]
        unsafe fn release(dev: u32, arg: u32) {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(dev).wrapping_add(VT_RELEASE)) as usize);
                slot(dev, arg);
            }
        }

        // Drain queue one (at most one entry: the pop clears the flag).
        if rd32(this + 0x4c) != 0 {
            let e = rd32(this + 0x44);
            lf_checker_rt::callee_thiscall!(ID_POP, u32, this.wrapping_add(0x44), e);
            let mut stamp_in = [0u32; 2];
            stamp_in[0] = rd16(e.wrapping_add(0x30)) as u32;
            lf_checker_rt::callee_thiscall!(
                ID_STAMP, u32, this.wrapping_add(0x5c), stamp_in.as_mut_ptr() as u32
            );
            let dev = rd32(this + 0x1c);
            release(dev, rd32(e));
            release(dev, e);
        }
        // Drain queue two, noting the stamp only for kind-bit entries.
        if rd32(this + 0x58) != 0 {
            let e = rd32(this + 0x50);
            lf_checker_rt::callee_thiscall!(ID_POP, u32, this.wrapping_add(0x50), e);
            if (rd8(rd32(e).wrapping_add(1)) >> 5) & 1 != 0 {
                let mut stamp_in = [0u32; 2];
                stamp_in[0] = rd16(e.wrapping_add(0x30)) as u32;
                lf_checker_rt::callee_thiscall!(
                    ID_STAMP, u32, this.wrapping_add(0x5c), stamp_in.as_mut_ptr() as u32
                );
            }
            let dev = rd32(this + 0x1c);
            release(dev, rd32(e));
            release(dev, e);
        }
        // Release the tail of the attach chain.
        if rd32(this + 0x6c) != 0 {
            let mut e = rd32(this + 0x68);
            if e == 0 {
                unsafe {
                    core::ptr::read_volatile(0 as *const u8);
                }
                unreachable!("null attach faults");
            }
            loop {
                let next = rd32(e.wrapping_add(0x50));
                if next == 0 {
                    break;
                }
                e = next;
            }
            let mut attach_out = [0u32; 2];
            lf_checker_rt::callee_thiscall!(
                ID_ATTACH, u32, this.wrapping_add(0x68), attach_out.as_mut_ptr() as u32, e
            );
            let tail: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(e).wrapping_add(VT_TAIL)) as usize);
            tail(e);
            let _ = ID_TAIL;
        }
        wr32(this + 0x30, 0xfa);
        wr32(this + 0x34, 0x20);
        wr32(this + 0x38, 0x3e8);
        wr32(this + 0x3c, 0x1f40);
        wr32(this + 0x40, 1);
        wr32(this + 0x20, 0);
        wr32(this + 0x28, 0);
        wr32(this + 0x24, 0);
        wr32(this + 0x2c, 0);
        wr32(this + 0x7c, 0);
        let tick = if rd32(lf_checker_rt::relocated(G_CLOCK_FN)) != 0 {
            lf_checker_rt::callee_cdecl!(ID_CLOCK_FN, u32,);
            let mut q = [0u32; 2];
            let mut qi = [0u32; 1];
            lf_checker_rt::callee_cdecl!(
                ID_CLOCK_Q, u32, q.as_mut_ptr() as u32, qi.as_mut_ptr() as u32
            );
            // The original reads back the second frame, not the first.
            qi[0]
        } else {
            lf_checker_rt::callee_stdcall!(ID_TICK, u32,)
        };
        let edx = tick ^ this;
        let mut eax = (edx == 0) as u32;
        let si = rd16(this + 0x80);
        let ecx0 = edx.rotate_left(16) ^ edx;
        eax = eax.wrapping_add(edx);
        let (t1_lo, t1_hi) = mac(eax, ecx0);
        let (t_lo, t_hi) = mac(t1_lo, t1_hi);
        if si == 0 {
            let r = (t_lo & 0x7FFF_FFFF) % 0xFF01;
            wr16(this + 0x80, r.wrapping_add(0xFF) as u16);
        } else {
            wr16(this + 0x80, (((si as u32) << 8) | (t_lo & 0xFF)) as u16);
        }
        let (mut cx3, mut bx3) = (t_lo, t_hi);
        let si2 = rd16(this + 0x82);
        if si2 != 0 {
            let (lo, hi) = mac(cx3, bx3);
            cx3 = lo;
            bx3 = hi;
            wr16(this + 0x82, (((si2 as u32) << 8) | (lo & 0xFF)) as u16);
        }
        let (t3_lo, t3_hi) = mac(cx3, bx3);
        wr16(this + 0x84, t3_lo as u16);
        let fin = t3_lo.wrapping_mul(0x559);
        let dhi = t3_hi.wrapping_sub(fin);
        wr8(this + 0x88, rd8(this + 0x88) & 0xF1);
        wr16(this + 0x86, dhi as u16);
        fin
    }
});
