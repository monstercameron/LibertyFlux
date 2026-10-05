// original: 0x00A71DB0 query_and_select_flagged_result (proposed)

/// Run a spatial query around a position and return the first result whose
/// flags select it.
///
/// `obj` points to a task object whose word at `+0x20` points to an inner
/// struct; the query centre is the three floats at inner `+0x30` (x, y, z)
/// with the `dy` argument added to z (scalar single-precision add, operand
/// order centre-then-argument). Three more single floats come from data
/// globals (`G_X`, `G_Y`, `G_Z` below); in the file image they are zero.
///
/// The query passed to the callee is two entry templates of `0x60` bytes on
/// the stack, each word zero except: result slot `+0x00` (zero, filled by the
/// callee), three `[G_X, G_Y, G_Z]` triples at `+0x10`, `+0x20`, `+0x30`
/// (the fourth word of each group is left as the stack fill), three zero
/// words at `+0x40`, `0xFFFF` at `+0x4C`, and zero byte/word tails at
/// `+0x50`/`+0x52`. The callee (cdecl, seven stack words: source-position
/// pointer, query-centre pointer, `obj`, entry base, 8, 2, 4) fills the
/// result slots and returns a signed count.
///
/// Each counted entry's word 0 points to a handle whose word at `+0x0C`
/// points to the result record (a null record skips the entry); the first
/// record with `(flags at +0x28) & 0x3C0 == 0x80` is returned, else null.
/// A non-positive count returns null without scanning.
///
/// Original: 0x00A71DB0 (stdcall, two stack words: object pointer, float
/// bits). Returns a pointer or null in `eax`.
lf_checker_rt::export!(stdcall, rw_00A71DB0(obj: u32, dy_bits: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x20;
        const POS_OFF: u32 = 0x30;
        const ENTRY_STRIDE: u32 = 0x60;
        const HANDLE_REC: u32 = 0x0c;
        const REC_FLAGS: u32 = 0x28;
        const FLAG_MASK: u32 = 0x3c0;
        const WANT: u32 = 0x80;
        const TAIL_MARK: u32 = 0xffff;
        // Data globals holding the template floats (file VAs; zero in the image).
        const G_X: u32 = 0x1b4b320;
        const G_Y: u32 = 0x1b4b324;
        const G_Z: u32 = 0x1b4b328;
        const CALLEE_QUERY: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let gx = rd32(lf_checker_rt::relocated(G_X));
        let ga = rd32(lf_checker_rt::relocated(G_Y));
        let gb = rd32(lf_checker_rt::relocated(G_Z));

        let base = rd32(obj + INNER_OFF);
        let src = base.wrapping_add(POS_OFF);
        let px = f32::from_bits(rd32(src));
        let py = f32::from_bits(rd32(src + 4));
        let pz = f32::from_bits(rd32(src + 8));
        let qz = add(pz, f32::from_bits(dy_bits));

        let mut centre = [0u32; 3];
        centre[0] = px.to_bits();
        centre[1] = py.to_bits();
        centre[2] = qz.to_bits();

        // Two zeroed entries; gaps stay zero like the original's zero fill.
        let mut ent = [0u32; 48];
        for e in 0..2usize {
            let b = e * 24;
            for g in 0..3usize {
                ent[b + 4 + g * 4] = gx;
                ent[b + 5 + g * 4] = ga;
                ent[b + 6 + g * 4] = gb;
            }
            ent[b + 16] = 0;
            ent[b + 17] = 0;
            ent[b + 18] = 0;
            ent[b + 19] = TAIL_MARK;
            let bb = ent.as_mut_ptr() as u32 + (b as u32) * 4;
            ((bb + 0x50) as *mut u8).write(0);
            ((bb + 0x52) as *mut u16).write_unaligned(0);
        }
        let eb = ent.as_mut_ptr() as u32;

        let n = lf_checker_rt::callee_cdecl!(
            CALLEE_QUERY,
            u32,
            src,
            centre.as_ptr() as u32,
            obj,
            eb,
            8,
            2,
            4
        );
        let count = n as i32;
        if count <= 0 {
            return 0;
        }
        let mut i = 0i32;
        while i < count {
            let t = rd32(eb + (i as u32) * ENTRY_STRIDE);
            let rec = rd32(t + HANDLE_REC);
            if rec != 0 && rd32(rec + REC_FLAGS) & FLAG_MASK == WANT {
                return rec;
            }
            i += 1;
        }
        0
    }
});
