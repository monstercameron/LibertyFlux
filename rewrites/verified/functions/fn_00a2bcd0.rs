// original: 0x00a2bcd0 CPlayerPed::vf29

/// Player-ped per-frame update: gate on game state, decay timers, refresh
/// targeting and movement state. Always returns 1 (low byte).
///
/// `obj` (ECX) is the player ped. The update runs only when the ped is not
/// already leaving (bytes `+0x218`/`+0x219`), the game is not in state 1
/// (dword at file `0x11f7060`), the two run flags at files `0x12088b4` and
/// `0xf1c040` agree, and the mode at file `0x1037720` is not `0x12`;
/// otherwise only the shared tail (callee 0) runs. The running update:
///
/// * sets the active bit (`+0x28`, `0x10000000`) and clears `+0xe61`;
/// * ticks the weapon/task record (`obj[0x228]+0x70`, or address 0 when the
///   record is missing, which faults exactly as the original does): the
///   cooldown at `+0x3bc` counts down and zeroes `+0x3cc` at rest, the draw
///   flag at `+0x415` follows `+0x416` while leaving, and the burst pair at
///   `+0x3dc`/`+0x3dd` counts down;
/// * picks a speed cap (60.0 or 300.0 at `+0xb28`) from the squared speed
///   against the constant at file `0xe9bd18` when the flag at `+0x118` is
///   set;
/// * decays the timers at `+0xed4` (minus the delta at file `0x11735bc`)
///   and `+0xed0` (minus delta times the constant at file `0xfe879c`);
/// * runs the shared tail (callee 0), sets `+0x29c` bit `0x20`, resolves
///   the current item (callee 1), and, for a matching vehicle occupant,
///   notifies through callee 2 unless muted by the byte at file
///   `0x1173604`;
/// * pushes the anchor to callee 3, refreshes movement (callee 4), and when
///   the record at `+0x398` exists walks the extra task chain (callees 5-11,
///   passing frame scratch to callees 7, 8 and 10);
/// * when the item, the seat (`+0x38` equals `+0x7b4`) and the move record
///   (`+0x2c4` with a driver at `+0x25c`, approved by callee 12) all agree
///   and the item heat tops `0x7f`, either skips (current task head from
///   callee 13 identifying as `0x841`) or re-seats through callees 14-15
///   (passing frame scratch);
/// * scales the move blend through callee 16 (0.3 when the mode at `+0xb80`
///   is 3, 1.0 when it is 0 with a driver present, or any other non-4 mode);
/// * when the state at `+0xa74` is 1 or 2 delegates to callee 20 and
///   returns; otherwise measures the planar speed from the virtual slot
///   `+0xec` answer: under the constant at file `0xfe879c` it latches the
///   table pointer from file `0x11735b4` plus `0x1f4` (or sets the hold bit
///   when already latched past it), over it clears the latch and the bit;
/// * when the extra task chain reports state 1 stores the table pointer at
///   `+0x3f8`;
/// * unless suppressed (byte at file `0x18b6ed7` set while callee 17
///   reports idle), refreshes the wanted state (callee 18);
/// * decays `+0xed8` (respawning through callee 19 with tag 2 at 2.0 when
///   it crosses zero from a live vehicle seat) and `+0xedc` (through callee
///   19 with tag `0x15`), then counts `+0xee4` down.
///
/// Callee 8 takes three stack words with no register setup, so it is read
/// as stdcall; every other direct callee with register setup is thiscall
/// and every one with caller cleanup is cdecl (see the contract). The two
/// virtual slots are called through the objects exactly as the original
/// does. The original reads relocated game data (run flags, delta, table) from the mapped image, while the
/// state/mode/mute/suppress gates stay unrelocated and read identically and read identically on both sides.
///
/// Original: 0x00a2bcd0 (thiscall, no stack words; returns low byte).
lf_checker_rt::export!(thiscall, rw_00a2bcd0(obj: u32) -> u32 {
    unsafe {
        const LEAVING: u32 = 0x218;
        const LEAVING2: u32 = 0x219;
        const ACTIVE_BIT: u32 = 0x10000000;
        const WREC: u32 = 0x228;
        const WREC_INNER: u32 = 0x70;
        const G_STATE: u32 = 0x11f7060;
        const G_RUN_A: u32 = 0x12088b4;
        const G_RUN_B: u32 = 0xf1c040;
        const G_MODE: u32 = 0x1037720;
        const MODE_SKIP: u32 = 0x12;
        const MATRIX: u32 = 0x20;
        const SPEED_FLAG: u32 = 0x118;
        const K_SPEED2: u32 = 0xe9bd18;
        const CAP_SLOW_BITS: u32 = 0x42700000; // 60.0
        const CAP_FAST_BITS: u32 = 0x43960000; // 300.0
        const G_DT: u32 = 0x11735bc;
        const K_DT_SCALE: u32 = 0xfe879c;
        const G_TABLE: u32 = 0x11735b4;
        const G_MUTE: u32 = 0x1173604;
        const G_SUPPRESS: u32 = 0x18b6ed7;
        const VT_SPEED_SLOT: u32 = 0xec;
        const VT_ID_SLOT: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn g32(ph: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(ph) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn g8(ph: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(ph) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn gf(ph: u32) -> f32 {
            unsafe { f32::from_bits(g32(ph)) }
        }
        /// The weapon/task inner record, or 0 when missing (the original
        /// keeps going and faults on the access; so does this).
        #[inline(always)]
        unsafe fn w(obj: u32) -> u32 {
            unsafe {
                let p = rd32(obj.wrapping_add(WREC));
                if p != 0 { p.wrapping_add(WREC_INNER) } else { 0 }
            }
        }

        if rd8(obj.wrapping_add(LEAVING)) != 0 && rd8(obj.wrapping_add(LEAVING2)) != 0 {
            lf_checker_rt::callee_thiscall!(0, u32, obj);
            return 1;
        }
        if g32(G_STATE) == 1 || g32(G_RUN_A) != g32(G_RUN_B) || g32(G_MODE) == MODE_SKIP {
            lf_checker_rt::callee_thiscall!(0, u32, obj);
            return 1;
        }
        wr32(obj.wrapping_add(0x28), rd32(obj.wrapping_add(0x28)) | ACTIVE_BIT);
        wr8(obj.wrapping_add(0xe61), 0);

        // Cooldown / draw / burst ticks on the inner record.
        if rd8(w(obj).wrapping_add(0x3bc)) != 0 {
            let a = w(obj);
            wr8(a.wrapping_add(0x3bc), rd8(a.wrapping_add(0x3bc)).wrapping_sub(1));
        }
        if rd8(w(obj).wrapping_add(0x3bc)) == 0 {
            wr32(w(obj).wrapping_add(0x3cc), 0);
        }
        if rd8(obj.wrapping_add(LEAVING2)) != 0 {
            wr8(w(obj).wrapping_add(0x415), 0);
            if rd8(w(obj).wrapping_add(0x416)) != 0 {
                wr8(w(obj).wrapping_add(0x415), 1);
            }
        }
        if rd8(w(obj).wrapping_add(0x3dd)) != 0 {
            // Zero both at 0 or 1 (the original decrements then branches
            // signed less-or-equal), otherwise count the first down.
            let v = rd8(w(obj).wrapping_add(0x3dc));
            if v <= 1 {
                wr8(w(obj).wrapping_add(0x3dc), 0);
                wr8(w(obj).wrapping_add(0x3dd), 0);
            } else {
                let a = w(obj);
                wr8(a.wrapping_add(0x3dc), rd8(a.wrapping_add(0x3dc)).wrapping_sub(1));
            }
        }

        if rd8(obj.wrapping_add(SPEED_FLAG)) & 1 != 0 {
            let m = rd32(obj.wrapping_add(MATRIX));
            let sx = mul(rdf(m.wrapping_add(0x30)), rdf(m.wrapping_add(0x30)));
            let sy = mul(rdf(m.wrapping_add(0x34)), rdf(m.wrapping_add(0x34)));
            let sz = mul(rdf(m.wrapping_add(0x38)), rdf(m.wrapping_add(0x38)));
            wr32(
                obj.wrapping_add(0xb28),
                if add(add(sx, sy), sz) > gf(K_SPEED2) { CAP_SLOW_BITS } else { CAP_FAST_BITS },
            );
        }
        let t0 = rdf(obj.wrapping_add(0xed4));
        if t0 > 0.0 {
            wrf(obj.wrapping_add(0xed4), sub(t0, gf(G_DT)));
        }
        let t1 = rdf(obj.wrapping_add(0xed0));
        if t1 > 0.0 {
            wrf(obj.wrapping_add(0xed0), sub(t1, mul(gf(G_DT), gf(K_DT_SCALE))));
        }

        lf_checker_rt::callee_thiscall!(0, u32, obj);
        wr32(obj.wrapping_add(0x29c), rd32(obj.wrapping_add(0x29c)) | 0x20);
        let item = lf_checker_rt::callee_thiscall!(1, u32, obj);

        // Occupant notify.
        let d = rd32(obj.wrapping_add(0xab0));
        if d != 0
            && rd32(d.wrapping_add(0x28)) & 0x3c0 == 0x80
            && rd32(d.wrapping_add(0x1304)) == 0
            && rd8(d.wrapping_add(0xf1f)) & 0x40 != 0
        {
            let pw = rd32(d.wrapping_add(0xf50));
            if pw != 0
                && rd32(rd32(pw.wrapping_add(0x21c)).wrapping_add(0x12c)) == 2
                && g8(G_MUTE) & 0x3f == 0
            {
                lf_checker_rt::callee_cdecl!(2, u32, 0xa, d, obj);
            }
        }

        let anchor_sel = if rd8(obj.wrapping_add(0x26c)) & 4 != 0 { rd32(obj.wrapping_add(0xb30)) } else { 0 };
        lf_checker_rt::callee_thiscall!(
            3, u32, w(obj),
            rd32(obj.wrapping_add(MATRIX)).wrapping_add(0x30),
            anchor_sel
        );
        lf_checker_rt::callee_thiscall!(4, u32, obj);

        // Extra task chain.
        if rd32(obj.wrapping_add(0x398)) != 0 {
            let r = lf_checker_rt::callee_thiscall!(5, u32, obj.wrapping_add(0x2b0));
            if r != 0 {
                let r = lf_checker_rt::callee_thiscall!(5, u32, obj.wrapping_add(0x2b0));
                let s = lf_checker_rt::callee_cdecl!(6, u32, rd32(r.wrapping_add(0x18)));
                if rd32(s.wrapping_add(0x0c)) != 1 {
                    let mut tmp7 = [0u32; 8];
                    let mut tmp8 = [0u32; 8];
                    let p7 = tmp7.as_mut_ptr() as u32;
                    let p8 = tmp8.as_mut_ptr() as u32;
                    lf_checker_rt::callee_thiscall!(7, u32, p7, obj, rd32(obj.wrapping_add(0x398)), 0);
                    let mid = lf_checker_rt::callee_stdcall!(8, u32, p8, 0, 1);
                    lf_checker_rt::callee_thiscall!(9, u32, mid);
                    lf_checker_rt::callee_thiscall!(10, u32, p8);
                }
            }
            lf_checker_rt::callee_thiscall!(11, u32, obj);
        }

        // Re-seat check.
        let seat = rd32(obj.wrapping_add(0x38));
        let mv = rd32(obj.wrapping_add(0x2c4));
        let mut reseat = item != 0
            && seat != 0
            && seat == rd32(obj.wrapping_add(0x7b4))
            && mv != 0
            && rd32(mv.wrapping_add(0x25c)) != 0
            && lf_checker_rt::callee_thiscall!(12, u32, rd32(mv.wrapping_add(0x25c))) as u8 != 0
            && (rd8(item.wrapping_add(0x26de)) ^ rd8(item.wrapping_add(0x26dc))) > 0x7f;
        if reseat {
            let head = lf_checker_rt::callee_thiscall!(13, u32, rd32(obj.wrapping_add(0x224)));
            if head != 0 {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(head).wrapping_add(VT_ID_SLOT)) as usize);
                if f(head) == 0x841 {
                    reseat = false;
                }
            }
            if reseat {
                let mut o1 = [0u32; 8];
                let mut o2 = [0u32; 8];
                let mv = rd32(obj.wrapping_add(0x2c4));
                lf_checker_rt::callee_thiscall!(
                    14, u32,
                    rd32(mv.wrapping_add(0x25c)),
                    mv,
                    rd32(mv.wrapping_add(0x20)),
                    o2.as_mut_ptr() as u32,
                    o1.as_mut_ptr() as u32
                );
                lf_checker_rt::callee_thiscall!(
                    15, u32,
                    rd32(rd32(obj.wrapping_add(0x2c4)).wrapping_add(0x25c)),
                    obj,
                    rd32(rd32(obj.wrapping_add(0x2c4)).wrapping_add(0x20)),
                    0, 0, 0, 0, 0, 0,
                    0xbf800000
                );
            }
        }

        // Move blend scale.
        match rd32(obj.wrapping_add(0xb80)) {
            3 => {
                lf_checker_rt::callee_thiscall!(16, u32, obj, 0, 0x3e99999a);
            }
            4 => {}
            0 => {
                if rd8(obj.wrapping_add(0x26c)) & 4 != 0 && rd32(obj.wrapping_add(0xb30)) != 0 {
                    lf_checker_rt::callee_thiscall!(16, u32, obj, 0, 0x3f800000);
                }
            }
            _ => {
                lf_checker_rt::callee_thiscall!(16, u32, obj, 0, 0x3f800000);
            }
        }

        // Planar speed latch.
        let st = rd32(obj.wrapping_add(0xa74));
        if st == 1 || st == 2 {
            lf_checker_rt::callee_thiscall!(20, u32, obj);
            return 1;
        }
        let mut vout = [0u32; 8];
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_SPEED_SLOT)) as usize);
        let ans = f(obj, vout.as_mut_ptr() as u32);
        let ax = rdf(ans);
        let ay = rdf(ans.wrapping_add(4));
        let speed = add(mul(ay, ay), mul(ax, ax)).sqrt();
        if gf(K_DT_SCALE) > speed {
            if rd32(w(obj).wrapping_add(0x3c0)) == 0 {
                wr32(w(obj).wrapping_add(0x3c0), g32(G_TABLE).wrapping_add(0x1f4));
            } else if g32(G_TABLE) > rd32(w(obj).wrapping_add(0x3c0)) {
                let a = w(obj);
                wr32(a.wrapping_add(0x3d0), rd32(a.wrapping_add(0x3d0)) | 1);
            }
        } else {
            wr32(w(obj).wrapping_add(0x3c0), 0);
            let a = w(obj);
            wr32(a.wrapping_add(0x3d0), rd32(a.wrapping_add(0x3d0)) & 0xfffffffe);
        }

        let e = lf_checker_rt::callee_thiscall!(5, u32, obj.wrapping_add(0x2b0));
        if e != 0 {
            let e = lf_checker_rt::callee_thiscall!(5, u32, obj.wrapping_add(0x2b0));
            if rd32(e.wrapping_add(0x1c)) == 1 {
                wr32(w(obj).wrapping_add(0x3f8), g32(G_TABLE));
            }
        }

        if g8(G_SUPPRESS) == 0
            || lf_checker_rt::callee_cdecl!(17, u32,) as u8 != 0
        {
            lf_checker_rt::callee_thiscall!(18, u32, obj);
        }

        if rd32(obj.wrapping_add(0xb0)) != 0 && (rd32(obj.wrapping_add(0xb8)) as i32) > 0 {
            let g = rd32(obj.wrapping_add(0xab0));
            if g != 0 && rd32(g.wrapping_add(0x28)) & 0x3c0 == 0x100 {
                let u = sub(rdf(obj.wrapping_add(0xed8)), gf(G_DT));
                wrf(obj.wrapping_add(0xed8), u);
                // jb after comiss(0, u) skips exactly when u > 0 or NaN.
                if u <= 0.0 {
                    lf_checker_rt::callee_cdecl!(
                        19, u32, 2,
                        rd32(obj.wrapping_add(MATRIX)).wrapping_add(0x30),
                        obj, 0, 0, 0
                    );
                    wrf(obj.wrapping_add(0xed8), f32::from_bits(0x40000000));
                }
            }
        }
        let z = rdf(obj.wrapping_add(0xedc));
        if z > 0.0 {
            let u = sub(z, gf(G_DT));
            wrf(obj.wrapping_add(0xedc), u);
            if u <= 0.0 {
                lf_checker_rt::callee_cdecl!(
                    19, u32, 0x15,
                    rd32(obj.wrapping_add(MATRIX)).wrapping_add(0x30),
                    obj, 0, 0, 0
                );
            }
        }
        let c = rd32(obj.wrapping_add(0xee4));
        if (c as i32) > 0 {
            wr32(obj.wrapping_add(0xee4), c.wrapping_sub(1));
        }
        1
    }
});
