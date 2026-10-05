// original: 0x00a62fe0 ped_task_target_select (proposed)

/// Pick a task target id and flag for a ped task object.
///
/// `out_id` starts at -1 and `out_flag` at 0. The function scans the
/// linked list at `this+0x2e0` (nodes: id at `+4`, key field at `+8`,
/// next at `+0xc`) for the id `WANT`: it walks while the 3-bit key
/// `([node+8]>>1)&7` does not rise, gives up early when the key rises
/// from 2 or more, and stops at the first node whose id matches.
/// - Found: callee 2 (thiscall on `this+0x44`, arg `WANT`) resolves the
///   target object; a null answer returns 1 leaving the outputs as they
///   are. Otherwise `*out_id` becomes the object's id at `+0x38`, and
///   when the master object at `this+0x40` has a non-null link at `+0xd68`
///   the chain of callees 3, 4, 5 (thiscall on the target) and 6 (cdecl,
///   five args, two of them frame pointers) runs; a non-zero answer from
///   callee 6 sets `*out_flag` to 1. The return is 1 in all found cases.
///   Callee 4's middle argument is an uninitialised stack word of the
///   original (modelled as 0; see below).
/// - Not found: when the master's link is non-null, callee 0 (thiscall on
///   `this+0x2e0`, args `0x414, 0`) gates the search, a tag check on the
///   link's low 3 bits (1 rejects) follows, and callee 1 (thiscall on the
///   link, a frame out-pointer and 0) yields a candidate point; the
///   function returns 1 when the squared distance from the anchor at
///   `[[this+0x40]+0x20]+0x30` to that point is strictly below the
///   squared constant at `G_DIST2`, else 0.
///
/// The original stores one frame byte that it never reads (omitted here)
/// and pushes one uninitialised frame word to callee 4; the checker runs
/// both sides with a defined scratch fill of 0, so the rewrite pushes 0.
/// Any proof of this function is narrower than the default by that word.
///
/// Float operation order is the original's SSE order. Original: 0x00a62fe0
/// (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a62fe0(this: u32, out_id: u32, out_flag: u32) -> u32 {
    unsafe {
        const LIST: u32 = 0x2e0;
        const NODE_ID: u32 = 0x04;
        const NODE_KEY: u32 = 0x08;
        const NODE_NEXT: u32 = 0x0c;
        const CTX: u32 = 0x44;
        const MASTER: u32 = 0x40;
        const MASTER_LINK: u32 = 0xd68;
        const MASTER_POS: u32 = 0x20;
        const ANCHOR: u32 = 0x30;
        const TARGET_ID: u32 = 0x38;
        const WANT: u32 = 0x41e;
        const GATE_TAG: u32 = 0x414;
        const G_DIST2: u32 = 0x0103cd54;
        const CALLEE_GATE: u32 = 0;
        const CALLEE_CAND: u32 = 1;
        const CALLEE_RESOLVE: u32 = 2;
        const CALLEE_CK3: u32 = 3;
        const CALLEE_CK4: u32 = 4;
        const CALLEE_CK5: u32 = 5;
        const CALLEE_FINAL: u32 = 6;

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        wr32(out_id, 0xffffffff);
        wr8(out_flag, 0);
        // List scan for WANT.
        let mut found = false;
        let mut edx = rd32(this + LIST);
        if edx != 0 {
            let mut ecx = (rd32(edx + NODE_KEY) >> 1) & 7;
            loop {
                let eax = (rd32(edx + NODE_KEY) >> 1) & 7;
                if ecx < eax && ecx >= 2 {
                    break;
                }
                if rd32(edx + NODE_ID) == WANT {
                    found = true;
                    break;
                }
                ecx = eax;
                edx = rd32(edx + NODE_NEXT);
                if edx == 0 {
                    break;
                }
            }
        }
        if found {
            let tgt: u32 = lf_checker_rt::callee_thiscall!(CALLEE_RESOLVE, u32, this + CTX, WANT);
            if tgt == 0 {
                return 1;
            }
            wr32(out_id, rd32(tgt + TARGET_ID));
            let master = rd32(this + MASTER);
            if rd32(master + MASTER_LINK) == 0 {
                return 1;
            }
            let a3: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CK3, u32, tgt, master, 0);
            let a4: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CK4, u32, tgt, master, 0, a3 & 0xff);
            let a5: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CK5, u32, tgt, master, a4 & 0xff);
            let mut p1 = [0u32; 4];
            let mut p2 = [0u32; 4];
            let fr: u32 = lf_checker_rt::callee_cdecl!(
                CALLEE_FINAL, u32, master, p1.as_mut_ptr() as u32, p2.as_mut_ptr() as u32, 8,
                a5 & 0xff
            );
            if (fr & 0xff) != 0 {
                wr8(out_flag, 1);
            }
            return 1;
        }
        let master = rd32(this + MASTER);
        let link = rd32(master + MASTER_LINK);
        if link == 0 {
            return 0;
        }
        let g: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GATE, u32, this + LIST, GATE_TAG, 0);
        if (g & 0xff) == 0 {
            return 0;
        }
        if rd8(link) & 7 == 1 {
            return 0;
        }
        let mut pt = [0u32; 4];
        let cr: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CAND, u32, link, pt.as_mut_ptr() as u32, 0);
        if (cr & 0xff) == 0 {
            return 0;
        }
        let pa = rd32(rd32(this + MASTER) + MASTER_POS).wrapping_add(ANCHOR);
        let dx = sub(rdf(pa), f32::from_bits(pt[0]));
        let dy = sub(rdf(pa + 4), f32::from_bits(pt[1]));
        let dd = add(mul(dy, dy), mul(dx, dx));
        let gl = f32::from_bits(rd32(lf_checker_rt::relocated(G_DIST2)));
        let gs = mul(gl, gl);
        if gs > dd { 1 } else { 0 }
    }
});
