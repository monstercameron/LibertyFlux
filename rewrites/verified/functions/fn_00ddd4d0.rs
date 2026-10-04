// original: 0x00ddd4d0 uitextfield_format_text (proposed)

/// Build the field's display strings from a new value and refresh it.
///
/// `this` is the field object: a child control pointer at `+0x1e8` and a
/// limit byte at `+0x208`. `s` is the new NUL-terminated text, `flags` a
/// detail word whose low byte tunes the middle copy.
///
/// Behaviour: fetch a prefix string through virtual slot `+0x20c` of the
/// child and copy it (at most 0xff bytes) into the first scratch buffer
/// with the copy helper; append the new text after the limit byte; measure
/// the text length; fetch a second string, copy it shifted by the limit
/// into the buffer after the text; fold the buffer onto itself with an
/// offset derived from the limit byte versus the flags low byte (their
/// positive difference, else 0); fetch a third string into the second
/// scratch buffer; hand the first buffer to virtual slot `+0x1e0` with 0.
/// Then poll the field: on a nonzero answer store the text length minus the
/// flags low byte (wrapping byte arithmetic) through the set callee and
/// return 1; otherwise hand the second buffer to slot `+0x1e0` with 1,
/// poll once more, and return 0. The copy helper takes (destination,
/// source, byte count) at every site: the count word is pushed before the
/// string fetch, so it sits deepest in the argument list. The original
/// measures the text length twice with no call in between; one measurement
/// is enough.
///
/// Edge cases: an empty text copies nothing but still runs every step; a
/// limit byte at or below the flags byte zeroes the fold offset; only the
/// poll answer's low byte decides the branch.
///
/// Original: 0x00ddd4d0 (thiscall, two stack words: text pointer, flags).
lf_checker_rt::export!(thiscall, rw_00ddd4d0(this: u32, s: u32, flags: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x1e8;
        const LIMIT: u32 = 0x208;
        const VT_GET: u32 = 0x20c;
        const VT_APPLY: u32 = 0x1e0;
        const FULL: u32 = 0xff;
        const BUF: usize = 0x208;
        const CALLEE_GET: u32 = 1;
        const CALLEE_COPY1: u32 = 2;
        const CALLEE_COPY2: u32 = 3;
        const CALLEE_COPY3: u32 = 4;
        const CALLEE_COPY4: u32 = 5;
        const CALLEE_COPY5: u32 = 6;
        const CALLEE_POLL: u32 = 8;
        const CALLEE_SET: u32 = 9;
        const CALLEE_COOKIE: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let obj = rd32(this + CHILD);
        let vt = rd32(obj);
        let get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_GET) as usize);
        let apply: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_APPLY) as usize);

        let mut buf1 = [0u8; BUF];
        let mut buf2 = [0u8; BUF];
        let b1 = buf1.as_mut_ptr() as u32;
        let b2 = buf2.as_mut_ptr() as u32;

        let v1: u32 = get(obj);
        lf_checker_rt::callee_cdecl!(CALLEE_COPY1, u32, b1, v1, FULL);
        let lim = rd8(this + LIMIT) as u32;
        lf_checker_rt::callee_cdecl!(CALLEE_COPY2, u32, b1.wrapping_add(lim), s, FULL.wrapping_sub(lim));

        let mut len = 0u32;
        while rd8(s.wrapping_add(len)) != 0 {
            len = len.wrapping_add(1);
        }

        let room = FULL.wrapping_sub(lim).wrapping_sub(len);
        let v2: u32 = get(obj);
        lf_checker_rt::callee_cdecl!(
            CALLEE_COPY3, u32,
            b1.wrapping_add(lim).wrapping_add(len),
            v2.wrapping_add(lim),
            room
        );

        let cl = flags as u8;
        let dl = rd8(this + LIMIT);
        let off = if dl > cl { dl.wrapping_sub(cl) } else { 0 };
        lf_checker_rt::callee_cdecl!(
            CALLEE_COPY4, u32,
            b1.wrapping_add(off as u32),
            b1.wrapping_add(dl as u32),
            FULL.wrapping_sub(off as u32)
        );

        let v3: u32 = get(obj);
        lf_checker_rt::callee_cdecl!(CALLEE_COPY5, u32, b2, v3, FULL);

        apply(obj, b1, 0);
        let p: u32 = lf_checker_rt::callee_thiscall!(CALLEE_POLL, u32, this);
        if (p as u8) != 0 {
            let share = (len as u8).wrapping_sub(cl);
            lf_checker_rt::callee_thiscall!(CALLEE_SET, u32, this, share as u32);
            lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
            1
        } else {
            apply(obj, b2, 1);
            lf_checker_rt::callee_thiscall!(CALLEE_POLL, u32, this);
            lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
            0
        }
    }
});
