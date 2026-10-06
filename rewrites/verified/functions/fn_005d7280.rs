// original: 0x005D7280 dispatch_by_stored_kind (proposed)

/// Ensures a shared helper object exists, then runs one of seven handlers
/// selected by a stored kind code.
///
/// `this` is the owner object; the single stack argument `node` is a work
/// node. The node points at `+0xDC` to a descriptor word that is passed to
/// the kind query. A global helper pointer (file VA `FLAG_VA`) is reused
/// when non-zero, otherwise it is built through the thread allocator
/// reached as `tls[0] -> [+8] -> vtable[+8]` (callee 1, thiscall:
/// allocator, `0x24, 0x10, 0`) and completed by callee 2 (thiscall:
/// fresh object), and the result is stored back to the global (0 when the
/// allocator answered null).
///
/// The kind code comes from callee 3 (thiscall: helper, descriptor word),
/// is stashed at `[node+0xD8]`, and callee 4 (thiscall: owner, node) runs.
/// When the word at `[node+0x14]` is `0x10` or `0x11` (unsigned equality),
/// six words are copied from the source object at `[node+8]` (offsets
/// `0x74, 0x80, 0x8C, 0x98, 0xBC, 0xC0`) over `[node+0x74..+0xC0]` in that
/// order. Callee 5 (thiscall: owner, node) runs next; its answer and
/// callee 4's are ignored.
///
/// Dispatch reads `kind - 2` as an UNSIGNED value: above `0x25` the
/// function returns `kind - 2` unchanged. Otherwise a 38-entry index map
/// (copied from the switch tables at file VAs 0x5D73F0/0x5D73CC and
/// embedded here because the rewrite cannot read the original's code
/// pages) selects the handler: 2 -> callee 6
/// (stdcall: node), 3 -> mark the global seen-byte and store the node at
/// `[this+4]`, returning 1; 23 -> callee 7 (thiscall: owner, node);
/// 21 -> callee 8 (stdcall: node); 26 -> callee 9 fills a 49-word scratch
/// buffer (thiscall, buffer address as `this`) which is then copied over
/// `[node+0x14]` (49 words), returning callee 9's answer; words the callee
/// leaves untouched read back the zeroed scratch; 38 -> callee 11
/// (stdcall: node); 39 -> callee 10 (thiscall: owner, node). All other
/// in-range codes fall through and return the index byte (2 or 8). The
/// range check is unsigned (`ja`): negative kinds and 0/1 take the
/// out-of-range path.
///
/// The trailing security-cookie check runs natively in the original and is
/// not intercepted: it always passes and writes nothing.
///
/// Original: 0x005D7280 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_005D7280(this: u32, node: u32) -> u32 {
    unsafe {
        const DC_OFF: u32 = 0xDC;
        const KIND_OFF: u32 = 0x14;
        const SRC_OFF: u32 = 8;
        const STASH_OFF: u32 = 0xD8;
        const FLAG_VA: u32 = 0x018B7430;
        const SEEN_VA: u32 = 0x018B6FA4;
        const TLS_SLOT: usize = 0;
        const HEAPOBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const SCRATCH_WORDS: u32 = 49;
        const MAKE_HELPER: u32 = 2;
        const KIND_OF: u32 = 3;
        const SETUP: u32 = 4;
        const PREPARE: u32 = 5;
        const H_B: u32 = 6;
        const H_C: u32 = 7;
        const H_D: u32 = 8;
        const H_E: u32 = 9;
        const H_F: u32 = 10;
        const H_G: u32 = 11;
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
        let desc = rd32(node.wrapping_add(DC_OFF));
        let desc_word = rd32(desc);
        let mut helper = rd32(lf_checker_rt::relocated(FLAG_VA));
        if helper == 0 {
            // tls_slot(0) already holds what the original loads with its
            // second dereference (fs:[0x2c] -> [slot0]); add the object offset.
            let tls0 = lf_checker_rt::tls_slot(TLS_SLOT);
            let heap_obj = rd32(tls0.wrapping_add(HEAPOBJ_OFF));
            let vtable = rd32(heap_obj);
            let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vtable.wrapping_add(ALLOC_SLOT)) as usize) };
            let made = alloc(heap_obj, 0x24, 0x10, 0);
            helper = if made != 0 {
                lf_checker_rt::callee_thiscall!(MAKE_HELPER, u32, made)
            } else {
                0
            };
            wr32(lf_checker_rt::relocated(FLAG_VA), helper);
        }
        let kind = lf_checker_rt::callee_thiscall!(KIND_OF, u32, helper, desc_word);
        wr32(node.wrapping_add(STASH_OFF), kind);
        let _ = lf_checker_rt::callee_thiscall!(SETUP, u32, this, node);
        let k = rd32(node.wrapping_add(KIND_OFF));
        if k == 0x11 || k == 0x10 {
            let src = rd32(node.wrapping_add(SRC_OFF));
            wr32(node.wrapping_add(0xBC), rd32(src.wrapping_add(0xBC)));
            wr32(node.wrapping_add(0xC0), rd32(src.wrapping_add(0xC0)));
            wr32(node.wrapping_add(0x74), rd32(src.wrapping_add(0x74)));
            wr32(node.wrapping_add(0x80), rd32(src.wrapping_add(0x80)));
            wr32(node.wrapping_add(0x8C), rd32(src.wrapping_add(0x8C)));
            wr32(node.wrapping_add(0x98), rd32(src.wrapping_add(0x98)));
        }
        let _ = lf_checker_rt::callee_thiscall!(PREPARE, u32, this, node);
        // Unsigned range check (original `ja`): kind-2 above 0x25 returns as is.
        let back = kind.wrapping_sub(2);
        if back > 0x25 {
            return back;
        }
        // Index map from the switch tables (file VAs 0x5D73F0/0x5D73CC).
        let idx: u32 = match kind {
            2 => 0,
            3 => 1,
            12 | 25 => 2,
            21 => 3,
            23 => 4,
            26 => 5,
            38 => 6,
            39 => 7,
            _ => 8,
        };
        match idx {
            0 => lf_checker_rt::callee_stdcall!(H_B, u32, node),
            1 => {
                wr8(lf_checker_rt::relocated(SEEN_VA), 1);
                wr32(this.wrapping_add(4), node);
                1
            }
            3 => lf_checker_rt::callee_stdcall!(H_D, u32, node),
            4 => lf_checker_rt::callee_thiscall!(H_C, u32, this, node),
            5 => {
                // Zeroed scratch like the original's frame buffer; the stub
                // writes the callee's words over the head of it.
                let mut buf = [0u32; SCRATCH_WORDS as usize];
                let ans = lf_checker_rt::callee_thiscall!(H_E, u32, buf.as_mut_ptr() as u32);
                let mut i: u32 = 0;
                while i < SCRATCH_WORDS {
                    wr32(
                        node.wrapping_add(KIND_OFF).wrapping_add(i.wrapping_mul(4)),
                        buf[i as usize],
                    );
                    i = i.wrapping_add(1);
                }
                ans
            }
            6 => lf_checker_rt::callee_stdcall!(H_G, u32, node),
            7 => lf_checker_rt::callee_thiscall!(H_F, u32, this, node),
            _ => idx,
        }
    }
});
