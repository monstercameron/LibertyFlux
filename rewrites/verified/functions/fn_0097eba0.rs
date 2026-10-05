// original: 0x0097EBA0 ped_task_dual_vtable_resolve (proposed)

/// Resolve two ranged-table entries through virtual calls, then post audio.
///
/// `this` is the task object and `arg0` a selector handed to both lookup
/// calls. After the shared audio-ready guards, callee 1 resolves a table
/// descriptor; a zero descriptor ends the call. From the descriptor an
/// entry index is computed in floating point: `trunc(0.0 / count)` (zero
/// for any nonzero count, INT_MIN through the hardware convert for a zero
/// count, which addresses/drives the same entry 0 the rewrite reaches via
/// saturation) clamped below by `limit - 1`, so entry 0 is used; the entry
/// count times the index, converted back to float and sign-flipped, is the
/// virtual call's float argument. The virtual target is loaded from the
/// entry's object (`[obj] + 0x14`) and called with the float and an
/// out-word; unless bits 0x3000000 come back set, the call ends. The same
/// runs a second time through callee 2 with its own descriptor chain and
/// an out-word the original places on its incoming argument slot.
///
/// Then the shared scratch buffer is built (word 3 = `[this+0x120]` biased
/// by 0x780, word 6 = a global word, word 8 = `[this+8]`), callee 6
/// arbitrates with the second out-word first, a zero `[this+0x1C]` ends
/// the call, otherwise callee 7 posts with a (0, -1, 0x33) descriptor,
/// callee 8 derives a parameter from the post's answer, both land in the
/// `[this+0x1C]` object at `+0xA4`/`+0xA8` (with `+0xAC` cleared), and
/// callee 9 finishes with three zero arguments.
///
/// Original: 0x0097EBA0 (thiscall, one stack argument, no return value).
lf_checker_rt::export!(thiscall, rw_0097EBA0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const G_QUIT: u32 = 0x011F7060;
        const G_SESS_A: u32 = 0x012088B4;
        const G_SESS_B: u32 = 0x00F1C040;
        const G_MODE: u32 = 0x01037720;
        const G_WORD6: u32 = 0x01231314;
        const SKIP_MODE: u32 = 0x12;
        const SEL_A: u32 = 0x82;
        const SEL_B: u32 = 0x83;
        const VT_SLOT: u32 = 0x14;
        const SIGN_FLIP: u32 = 0x8000_0000;
        const CONT_BITS: u32 = 0x0300_0000;
        const OFF_NESTED: u32 = 0x1C;
        const OFF_SUB: u32 = 0x08;
        const OFF_PED: u32 = 0x120;
        const PED_BIAS: u32 = 0x780;
        const DESC_COUNT: u32 = 0x04;
        const DESC_TABLE: u32 = 0x08;
        const DESC_LIMIT: u32 = 0x0C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn gget(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        /// Entry object and sign-flipped float argument from a descriptor.
        #[inline(always)]
        unsafe fn resolve(desc: u32) -> (u32, u32) {
            unsafe {
                let count = rd16(desc.wrapping_add(DESC_COUNT));
                let limit = rd16(desc.wrapping_add(DESC_LIMIT));
                let q = core::hint::black_box(0.0f32) / core::hint::black_box(count as f32);
                let qi = q as i32;
                let mut idx = limit.wrapping_sub(1);
                if qi < idx as i32 {
                    idx = qi as u32;
                }
                let table = rd32(desc.wrapping_add(DESC_TABLE));
                let entry = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                let prod = count.wrapping_mul(idx);
                let f = core::hint::black_box(prod as i32) as f32;
                let bits = f.to_bits() ^ SIGN_FLIP;
                (rd32(entry.wrapping_add(4)), bits)
            }
        }
        #[inline(always)]
        unsafe fn vcall(vid: u32, obj: u32, bits: u32, out: u32) -> u32 {
            unsafe {
                let _ = vid;
                let slot = rd32(rd32(obj).wrapping_add(VT_SLOT));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj, bits, out)
            }
        }

        if gget(G_QUIT) == 1 {
            return 0;
        }
        if gget(G_SESS_A) != gget(G_SESS_B) {
            return 0;
        }
        if gget(G_MODE) == SKIP_MODE {
            return 0;
        }
        let desc_a: u32 = lf_checker_rt::callee_thiscall!(1, u32, arg0, SEL_A, 0);
        if desc_a == 0 {
            return 0;
        }
        let (obj_a, bits_a) = resolve(desc_a);
        let mut out_a = 0u32;
        vcall(3, obj_a, bits_a, &mut out_a as *mut u32 as u32);
        if out_a & CONT_BITS == 0 {
            return 0;
        }
        let desc_b: u32 = lf_checker_rt::callee_thiscall!(2, u32, arg0, SEL_B, 0);
        if desc_b == 0 {
            return 0;
        }
        let (obj_b, bits_b) = resolve(desc_b);
        // The original uses its incoming argument slot for this out-word;
        // a Rust rewrite cannot address that slot, so a local stands in.
        // The slot is above incoming ESP (invisible to the stack check on
        // the original side); the value is observed through the snapshot
        // and the later calls that consume it. The local starts as the
        // slot does, holding the incoming argument, so the pre-write
        // snapshot matches too.
        let mut out_b = arg0;
        vcall(4, obj_b, bits_b, &mut out_b as *mut u32 as u32);
        if out_b == 0 {
            return 0;
        }
        let nested_slot = this.wrapping_add(OFF_NESTED);
        let mut buf = [0u32; 16];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, buf_ptr);
        buf[8] = rd32(this.wrapping_add(OFF_SUB));
        buf[3] = rd32(this.wrapping_add(OFF_PED)).wrapping_add(PED_BIAS);
        buf[6] = gget(G_WORD6);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            6, u32, this, out_b, nested_slot, buf_ptr, 0xFFFF_FFFF, 0, 0
        );
        let nested = rd32(nested_slot);
        if nested == 0 {
            return 0;
        }
        let mut desc = [0u32, 0xFFFF_FFFF, 0x33];
        let desc_ptr = desc.as_mut_ptr() as u32;
        let h: u32 = lf_checker_rt::callee_cdecl!(
            7,
            u32,
            out_b,
            0,
            1,
            1,
            buf_ptr,
            desc_ptr,
            rd32(this.wrapping_add(OFF_PED)),
            0xFFFF_FFFF
        );
        let p: u32 = lf_checker_rt::callee_cdecl!(8, u32, h);
        (nested.wrapping_add(0xA4) as *mut u32).write_unaligned(h);
        (nested.wrapping_add(0xA8) as *mut u32).write_unaligned(p);
        (nested.wrapping_add(0xAC) as *mut u32).write_unaligned(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, nested, 0, 0, 0);
        0
    }
});
