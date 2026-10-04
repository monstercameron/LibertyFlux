// original: 0x00C67010 cutscene_actor_setup (proposed)
//
// thiscall (ecx = this, three stack words: value arg0, byte-pointer arg1,
// byte-pointer arg2; callee pops 12). Configures three flag bytes on the
// object at +0x2a9/+0x2aa/+0x2ab from the two pointed-to bytes, then resolves
// and tunes actor handles through scripted callees:
//
// - flag_a (+0x2a9) = (*arg1 != 0); flag_b (+0x2aa) = (*arg2 != 0).
// - When flag_a is set: look up a record id from (arg0, arg1) via callee 1/2
//   (cdecl table lookup), resolve it to a record via callee 3 (cdecl), and
//   when the record exists with bit 0x10 in its byte at +6, set flag_c
//   (+0x2ab) and seed the option word with 0xe00.
// - When flag_a is set: build a record through callee 4 (thiscall on the
//   object at [this+0x78], five stack words: lookup result, arg1, options,
//   0, 8.0f) and, when it succeeds, tune it through callee 5 (thiscall with
//   1.0f). The original leaves callee 2's last four argument words on the
//   stack so they become callee 4's trailing arguments; only the call log
//   matters here, so the rewrite passes them explicitly.
// - When flag_b is set: same build/tune pair with (arg0, arg2, 0x40000, 7,
//   8.0f).
//
// Returns whatever the last executed step left in eax (arg0 when neither
// flag is set, otherwise the last callee result).
lf_checker_rt::export!(thiscall, rw_00C67010(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x2a9;
        const FLAG_B: u32 = 0x2aa;
        const FLAG_C: u32 = 0x2ab;
        const CHILD: u32 = 0x78;
        const RECORD_BIT: u8 = 0x10;
        const RECORD_BIT_OFF: u32 = 6;
        const OPT_SEED: u32 = 0xe00;
        const OPT_TOP: u32 = 0x100000;
        const EIGHT: u32 = 0x41000000;
        const ONE: u32 = 0x3f800000;
        const ALT0: u32 = 0x40000;
        const ALT1: u32 = 7;
        const LOOKUP2: u32 = 1;
        const LOOKUP5: u32 = 2;
        const RESOLVE: u32 = 3;
        const BUILD: u32 = 4;
        const TUNE: u32 = 5;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        wr8(this + FLAG_C, 0);
        wr8(this + FLAG_A, u8::from(rd8(arg1) != 0));
        wr8(this + FLAG_B, u8::from(rd8(arg2) != 0));

        let mut eax = arg0;
        if rd8(this + FLAG_A) != 0 {
            let id = lf_checker_rt::callee_cdecl!(LOOKUP2, u32, arg0, arg1);
            let rec = lf_checker_rt::callee_cdecl!(RESOLVE, u32, id, arg1);
            eax = rec;
            let mut opts = 0u32;
            if rec != 0 && rd8(rec + RECORD_BIT_OFF) & RECORD_BIT != 0 {
                opts = OPT_SEED;
                wr8(this + FLAG_C, 1);
            } else {
                wr8(this + FLAG_C, 0);
            }
            if rd8(this + FLAG_A) != 0 {
                opts |= OPT_TOP;
                let child = rd32(this + CHILD);
                let key = lf_checker_rt::callee_cdecl!(LOOKUP5, u32, arg0, arg1, opts, 0, EIGHT);
                let obj = lf_checker_rt::callee_thiscall!(BUILD, u32, child, key, arg1, opts, 0, EIGHT);
                eax = obj;
                if obj != 0 {
                    eax = lf_checker_rt::callee_thiscall!(TUNE, u32, obj, ONE);
                }
            }
        }
        if rd8(this + FLAG_B) != 0 {
            let child = rd32(this + CHILD);
            let key = lf_checker_rt::callee_cdecl!(LOOKUP5, u32, arg0, arg2, ALT0, ALT1, EIGHT);
            let obj = lf_checker_rt::callee_thiscall!(BUILD, u32, child, key, arg2, ALT0, ALT1, EIGHT);
            eax = obj;
            if obj != 0 {
                eax = lf_checker_rt::callee_thiscall!(TUNE, u32, obj, ONE);
            }
        }
        eax
    }
});
