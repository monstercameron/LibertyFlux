// original: 0x00ddc710 UITextContainer::vf103 (merged symbol name)

/// Refresh a text container against the current UI state: re-resolve its
/// content, measure it, and lay it out.
///
/// `this` points to the container. The routine first reads the shared UI
/// state object (reached through a global) and asks the UI manager (a global
/// object) to re-resolve several named resources; when the resolved content
/// differs from the container's current content it returns early with no
/// further work. Otherwise it checks the container's visibility flag through
/// its own virtual table, clears a stale-layout byte while notifying one
/// child object, then gathers four width measurements (two through virtual
/// calls on the shared state and the container's string child, two through
/// repeated calls on one slot) and combines them with a global scale factor
/// and two per-object floats into a clamped layout value. The tail forwards
/// that value plus a few derived words through a chain of small helper calls
/// and finally marks the resolved content object.
///
/// Layout arithmetic, in the original's order: with `w0` the shared-state
/// width, `w1`/`w2`/`w3` the string-child widths, `g` the global scale, `m`
/// the member float at `+STR_FLOAT` and `p` the content float at
/// `+CONTENT_FLOAT`: `x = (w0 - (w1 - w2 * g)) / w3`; unless `m` is negative,
/// `x -= p * g / m` (a zero `m` takes this path and yields an infinity, a NaN
/// `m` does too); then `x` is clamped to zero from below, except that a NaN
/// survives (the original's take-if-below-or-unordered compare keeps it);
/// the forwarded value is `m * x`. A NaN `m` likewise skips the correction
/// rather than taking it.
///
/// Calling details that matter: the incoming stack word is never read (it is
/// only popped by the the callee pops 4 bytes); one helper pops nothing while taking a stack
/// word and another pops eight words while taking three, so the frame holds
/// deliberate residue on return — both are reproduced exactly; two helpers
/// take out-pointers into the frame whose addresses are skipped in the proof
/// while their words are snapshotted. The return value is the last resolved
/// content pointer, or the mismatch witness on the early path.
///
/// Original: thiscall, one (unread) stack word, u32 return.
lf_checker_rt::export!(thiscall, rw_00ddc710(this: u32, _arg: u32) -> u32 {
    unsafe {
        const STATE_GLOBAL: u32 = 0x18b6c8c;
        const SCALE_GLOBAL: u32 = 0xfe8830;
        const UI_MANAGER: u32 = 0x1981a4c;
        const LAYOUT_OWNER: u32 = 0x1176888;
        const NAME_A: u32 = 0xefc374;
        const NAME_B: u32 = 0xefc380;
        const NAME_C: u32 = 0xefc394;
        const NAME_D: u32 = 0xefc3a4;
        const NAME_E: u32 = 0xefc3c4;
        const NAME_F: u32 = 0xefc3d0;
        const STATE_CONTENT: u32 = 0x200;
        const STATE_WIDTH: u32 = 0xc8;
        const MEMBER_STALE: u32 = 0x1e0;
        const MEMBER_CHILD: u32 = 0x1e8;
        const MEMBER_STR: u32 = 0x1f8;
        const STR_WIDTH_A: u32 = 0xc8;
        const STR_WIDTH_B: u32 = 0xb8;
        const STR_FLOAT: u32 = 0x1ec;
        const CONTENT_FLOAT: u32 = 0x210;
        const CONTENT_DONE: u32 = 0x20b;
        const VT_CHECK: u32 = 0x4c;
        const VT_SYNC: u32 = 0x1b0;
        const VT_VISIBLE: u32 = 0x1c;
        const VT_RESTORE: u32 = 0x1ac;
        const VT_SHOW: u32 = 0x18;
        const VT_NOTIFY: u32 = 0x120;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let manager = lf_checker_rt::relocated(UI_MANAGER);
        let state = rd32(lf_checker_rt::relocated(STATE_GLOBAL) as u32);
        let scale = rdf(lf_checker_rt::relocated(SCALE_GLOBAL) as u32);
        let current = rd32(state.wrapping_add(STATE_CONTENT));
        let worker: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager, current);
        let first: u32 = lf_checker_rt::callee_thiscall!(2, u32, manager, lf_checker_rt::relocated(NAME_A));
        let check: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(rd32(first).wrapping_add(VT_CHECK)) as usize) };
        let got = check(first);
        if current != got {
            return got;
        }
        let second: u32 = lf_checker_rt::callee_thiscall!(2, u32, manager, lf_checker_rt::relocated(NAME_B));
        let sync: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(rd32(second).wrapping_add(VT_SYNC)) as usize) };
        sync(second);
        let this_vtable = rd32(this);
        let visible: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(this_vtable.wrapping_add(VT_VISIBLE)) as usize) };
        if (visible(this) as u8) == 0 {
            let third: u32 =
                lf_checker_rt::callee_thiscall!(2, u32, manager, lf_checker_rt::relocated(NAME_C));
            let sync3: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(rd32(third).wrapping_add(VT_SYNC)) as usize) };
            sync3(third);
            let restore: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(this_vtable.wrapping_add(VT_RESTORE)) as usize) };
            restore(this);
            let show: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(this_vtable.wrapping_add(VT_SHOW)) as usize) };
            show(this, 1);
        }
        if rd8(this.wrapping_add(MEMBER_STALE)) != 0 {
            let child = rd32(this.wrapping_add(MEMBER_CHILD));
            wr8(this.wrapping_add(MEMBER_STALE), 0);
            let notify: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(rd32(child).wrapping_add(VT_NOTIFY)) as usize) };
            notify(child, 0);
        }
        lf_checker_rt::callee_thiscall!(
            3,
            u32,
            lf_checker_rt::relocated(LAYOUT_OWNER),
            lf_checker_rt::relocated(NAME_D)
        );
        let content: u32 =
            lf_checker_rt::callee_thiscall!(2, u32, manager, lf_checker_rt::relocated(NAME_E));
        let state_width: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(rd32(state).wrapping_add(STATE_WIDTH)) as usize) };
        let w0 = state_width(state);
        let str_obj = rd32(this.wrapping_add(MEMBER_STR));
        let str_vtable = rd32(str_obj);
        let width_a: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(str_vtable.wrapping_add(STR_WIDTH_A)) as usize) };
        let width_b: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(rd32(str_vtable.wrapping_add(STR_WIDTH_B)) as usize) };
        let w1 = width_a(str_obj);
        let w2 = width_b(str_obj);
        let t3 = sub(w0, sub(w1, mul(w2, scale)));
        let w3 = width_b(str_obj);
        let mut x = div(t3, w3);
        let m = rdf(str_obj.wrapping_add(STR_FLOAT));
        let p = rdf(content.wrapping_add(CONTENT_FLOAT));
        // Both compares are take-if-below-or-unordered, which keeps a NaN
        // where a plain ordered compare would drop it: `m >= 0.0` and
        // `x <= 0.0` are false for NaN, matching the original exactly.
        if m >= 0.0 {
            x = sub(x, div(mul(p, scale), m));
        }
        if x <= 0.0 {
            x = 0.0;
        }
        let forwarded = mul(m, x);
        let mut out_word: u32 = 0;
        let out_ptr = core::ptr::addr_of_mut!(out_word) as u32;
        let fetched: u32 = lf_checker_rt::callee_thiscall!(4, u32, worker, out_ptr);
        let tag: u32 = lf_checker_rt::callee_thiscall!(5, u32, worker);
        let mut scratch = [0u32; 2];
        let scratch_ptr = scratch.as_mut_ptr() as u32;
        let fetch_word = rd32(fetched);
        let combine: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(lf_checker_rt::callee_addr(6) as usize) };
        combine(worker, scratch_ptr, 0, 0, fetch_word, tag);
        let rate: f32 = lf_checker_rt::callee_thiscall!(7, f32, worker);
        let apply: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(lf_checker_rt::callee_addr(8) as usize) };
        let applied = apply(worker, rate.to_bits());
        let finish: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(lf_checker_rt::callee_addr(9) as usize) };
        finish(this, forwarded.to_bits(), p.to_bits(), applied);
        let done: u32 = lf_checker_rt::callee_thiscall!(2, u32, manager, lf_checker_rt::relocated(NAME_F));
        wr8(done.wrapping_add(CONTENT_DONE), 1);
        done
    }
});
