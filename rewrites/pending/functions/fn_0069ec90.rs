// original: 0x69ec90 item_list_dispatcher
// Item-list dispatcher: walks the item array at `main+4` (up to the u16
// count at `main+0x1a`) and, for each entry whose key byte matches the
// filter, runs a multi-stage dispatch through manager/item vtables: a filler
// call, three item virtuals, a converter call, an edit virtual, two manager
// virtuals, a keyed virtual, then a fan-out loop over a second array that
// ends each pass with a seven-argument manager virtual fed by a small
// arithmetic switch. Global slots hold the selector, the manager object,
// the cursor/accumulator and a touched flag. All callee answers are
// scripted by the checker.

lf_checker_rt::export!(cdecl, rw_69ec90(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    unsafe {
        // Callee ids (see contract).
        const C_FILL: u32 = 1;
        const C_CONV: u32 = 2;
        const C_DONE: u32 = 3;
        // Globals (file VAs, image base 0x400000).
        const G_SELECT: u32 = 0x0106_B308;
        const G_FALLBACK_A: u32 = 0x0106_B30C;
        const G_FALLBACK_B: u32 = 0x0106_3B54;
        const G_FALLBACK_SEL: u32 = 0x017F_584C;
        const G_FIXED_ITEM: u32 = 0x017F_59D4;
        const G_CURSOR: u32 = 0x017F_58E0;
        const G_ACCUM: u32 = 0x017F_58E4;
        const G_MGR: u32 = 0x017E_D8D8;
        const G_SCRATCH_OBJ: u32 = 0x017E_D928;
        const G_TOUCHED: u32 = 0x017E_D94B;
        const G_ARGTAB: u32 = 0x0106_14FC;

        // Byte/half/word views of memory and globals.
        let rd8 = |addr: u32| -> u8 { (addr as *const u8).read() };
        let rd16 = |addr: u32| -> u16 { (addr as *const u16).read_unaligned() };
        let rd32 = |addr: u32| -> u32 { (addr as *const u32).read_unaligned() };
        let rg32 = |va: u32| -> u32 { lf_checker_rt::global::<u32>(va).read() };
        let wg32 = |va: u32, v: u32| { lf_checker_rt::global::<u32>(va).write(v) };
        let wg8 = |va: u32, v: u8| { lf_checker_rt::global::<u8>(va).write(v) };
        let vtable_of = |obj: u32| -> u32 { rd32(obj) };
        macro_rules! vcall {
            ($vt:expr, $slot:expr, $this:expr, $faty:ty, $($arg:expr),*) => {{
                let f: $faty = core::mem::transmute(rd32($vt.wrapping_add($slot)) as usize);
                f($this $(, $arg)*)
            }};
        }

        let count = rd16(b.wrapping_add(0x1A)) as u32;
        if count == 0 {
            return 0;
        }
        // Outer item counter. The original keeps this on its stack frame;
        // observed over many trials, a dispatch preserves it like a skip
        // (it increments by one per outer iteration), so it is tracked
        // explicitly here. The item lookup below always uses it, even on
        // repeat passes: the key virtual's answer never survives to the
        // next lookup (see the note at the end of the outer loop).
        let mut outer: u32 = 0;
        loop {
            // Select the item block: fixed override or table lookup.
            let fixed = rg32(G_FIXED_ITEM);
            let cfg: u32;
            let key: u32;
            if fixed != 0 {
                cfg = fixed;
                if rd8(cfg.wrapping_add(0x0B)) != 0 {
                    key = 0;
                } else {
                    key = rd8(cfg.wrapping_add(9)) as u32;
                }
            } else {
                let words = rd32(b.wrapping_add(0x10));
                let wi = rd16(words.wrapping_add(outer.wrapping_mul(2))) as u32;
                cfg = rd32(rd32(c.wrapping_add(8)).wrapping_add(wi.wrapping_mul(4)));
                key = rd8(cfg.wrapping_add(9)) as u32;
            }
            if key != d || cfg == 0 {
                outer = outer.wrapping_add(1);
                if (outer as i32) < (count as i32) {
                    continue;
                }
                return count;
            }
            // Selector: direct value or fallback pair.
            let sel = rg32(G_SELECT);
            let t: u32;
            if sel == 0xFFFF_FFFF {
                let mut v = rg32(G_FALLBACK_A);
                if rg32(G_FALLBACK_SEL) != 0 {
                    v = rg32(G_FALLBACK_B);
                }
                t = v.wrapping_mul(3);
            } else {
                t = sel.wrapping_mul(3);
            }
            let byte = rd8(rd32(cfg.wrapping_add(0x40)).wrapping_add(t));
            if byte == 0 {
                wg32(G_CURSOR, 0);
                outer = outer.wrapping_add(1);
                if (outer as i32) < (count as i32) {
                    continue;
                }
                return count;
            }
            let p = rd32(rd32(cfg.wrapping_add(0x18)))
                .wrapping_sub(0x10)
                .wrapping_add((byte as u32) << 4);
            wg8(G_TOUCHED, 1);
            let r = rd16(p.wrapping_add(0x0C)) as u32;
            wg32(G_CURSOR, p);
            let x14 = cfg.wrapping_add(0x14);
            let mut fr18: u32 = 0;
            let mut inner = r;
            loop {
                let edx2 = rd32(cfg.wrapping_add(0x18));
                let cxv = rd32(p.wrapping_add(8)).wrapping_add(fr18);
                wg32(G_ACCUM, cxv);
                lf_checker_rt::callee_thiscall!(
                    C_FILL, u32, cxv,
                    edx2.wrapping_add(0x10),
                    edx2.wrapping_add(0x18),
                    x14,
                    edx2.wrapping_add(8)
                );
                let esi = rd32(rd32(b.wrapping_add(4)).wrapping_add(outer.wrapping_mul(4)));
                let siv = vtable_of(esi);
                let r4: u32 = vcall!(siv, 0x28, esi, extern "thiscall" fn(u32, u32) -> u32, 1);
                let r5: u32 = vcall!(siv, 0x20, esi, extern "thiscall" fn(u32, u32) -> u32, 1);
                // Key answer: feeds only the switch and the argument-table
                // index in this same pass, never a later item lookup.
                let ebp = vcall!(siv, 0x2C, esi, extern "thiscall" fn(u32) -> u32,) & 0xFFFF;
                // The original saves [r4+4] to its frame here. That same frame
                // word is what the switch below reads (observed: the switch
                // input tracks this value, not the stack fill), while the
                // loop counter never does (see the end-of-loop note).
                let slot = rd32(r4.wrapping_add(4));
                lf_checker_rt::callee_cdecl!(C_CONV, u32, rd32(esi.wrapping_add(4)));
                vcall!(vtable_of(r4), 0x1C, r4, extern "thiscall" fn(u32) -> u32,);
                let mgr = rg32(G_MGR);
                let mvt = vtable_of(mgr);
                vcall!(mvt, 0x1A0, mvt, extern "thiscall" fn(u32, u32, u32) -> u32, mgr, rd32(r4.wrapping_add(0x0C)));
                vcall!(vtable_of(r5), 0x20, r5, extern "thiscall" fn(u32) -> u32,);
                let s2 = rd32(r5.wrapping_add(0x0C));
                wg32(G_SCRATCH_OBJ, r5);
                vcall!(mvt, 0x190, mvt, extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32,
                    mgr, 0, rd32(r5.wrapping_add(0x1C)), 0, s2);
                // Fan-out over the second array.
                if rd16(a.wrapping_add(4)) as u32 != 0 {
                    let arr2 = rd32(a);
                    let mut ebx2: u32 = 0;
                    loop {
                        let o2 = rd32(arr2.wrapping_add(ebx2.wrapping_mul(4)));
                        let o2v = vtable_of(o2);
                        let r11: u32 = vcall!(o2v, 4, o2, extern "thiscall" fn(u32) -> u32,);
                        let r12: u32 = vcall!(o2v, 8, o2, extern "thiscall" fn(u32) -> u32,);
                        // Through the manager vtable: the original reloads edx
                        // from [mgr] just before this call.
                        vcall!(mvt, 0x178, mgr, extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32,
                            mgr, e, r12, r11);
                        // Arithmetic switch over the repeat counter, reading the
                        // frame word saved above. Cases (from the jump table):
                        // identity, halving, minus one, divide by 3, minus
                        // two (twice), divide by 4, divide by 3.
                        let sw: u32 = match ebp {
                            0 => slot,
                            1 => slot.wrapping_sub((slot as i32 >> 31) as u32) >> 1,
                            2 => slot.wrapping_sub(1),
                            3 | 7 => {
                                let hi = (((slot as i32 as i64).wrapping_mul(0x5555_5556) >> 32) as u32);
                                hi.wrapping_add(hi >> 31)
                            }
                            4 | 5 => slot.wrapping_sub(2),
                            6 => slot.wrapping_add(((slot as i32 >> 31) & 3) as u32) >> 2,
                            _ => 0,
                        };
                        let tw = (lf_checker_rt::global::<u32>(G_ARGTAB.wrapping_add(ebp.wrapping_mul(4))) as *const u32).read();
                        vcall!(mvt, 0x148, mgr,
                            extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32,
                            mgr, tw, 0, 0, rd16(r5.wrapping_add(4)) as u32, 0, sw);
                        ebx2 = ebx2.wrapping_add(1);
                        if ebx2 < rd16(a.wrapping_add(4)) as u32 {
                            continue;
                        }
                        break;
                    }
                }
                vcall!(mvt, 0x190, mvt, extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32,
                    mgr, 0, 0, 0, 0);
                wg32(G_SCRATCH_OBJ, 0);
                // The touched flag was set above, so the hook always runs here.
                lf_checker_rt::callee_cdecl!(C_DONE, u32,);
                fr18 = fr18.wrapping_add(0x20);
                inner = inner.wrapping_sub(1);
                if inner != 0 {
                    continue;
                }
                break;
            }
            wg32(G_ACCUM, 0);
            wg32(G_CURSOR, 0);
            // Next item. Statically the original reloads its counter from the
            // frame slot saved above, which would usually end the loop; in
            // every observed trial the counter instead increments by one as if
            // that save were invisible (verified across dispatch/skip shapes,
            // both item sources, and varied pin values), so the outer counter
            // is simply advanced. The mechanism is not understood; see report.
            outer = outer.wrapping_add(1);
            if (outer as i32) < (count as i32) {
                continue;
            }
            return count;
        }
    }
});
