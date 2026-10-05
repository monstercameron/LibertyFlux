// original: 0x00A8A420 pool_guarded_lookup (proposed)

/// Look up the entry for the argument unless its kind selects the null one.
///
/// The session helper first runs on a frame slot with the global scope;
/// the kind bits (`+0x28` of `*arg`, shifted down 6, low four) 2 or 3
/// select the null entry: tear down and return 0. Otherwise the fetch
/// helper runs on the sub-object at `this+8` with the argument slot; a
/// non-zero answer tears down and is returned. On zero the list rooted at
/// `this+0x18` (ending at `this+8`) is walked for the first object whose
/// `+0x24` word has none of bits `0xA00` set, which is poked through its
/// function table slot `+0x44`; the fetch helper runs once more and its
/// answer (after tear-down) is returned.
///
/// Original: thiscall, one stack word, returns u32 in EAX. Four callee
/// shapes: session (thiscall, one stack word, frame object), fetch
/// (thiscall, one stack word), tear-down (thiscall, no stack words,
/// frame object), poke (thiscall through the object, no stack words),
/// the last intercepted by a planted stub address.
lf_checker_rt::export!(thiscall, rw_00A8A420(this: u32, arg: u32) -> u32 {
    unsafe {
        const SCOPE_FILE_VA: u32 = 0x12fb1dc;
        const SUB_OFF: u32 = 8;
        const HEAD_OFF: u32 = 0x18;
        const KIND_OFF: u32 = 0x28;
        const FLAG_OFF: u32 = 0x24;
        const FLAG_MASK: u32 = 0xa00;
        const POKE_SLOT: u32 = 0x44;
        const SESSION: u32 = 1;
        const FETCH: u32 = 2;
        const TEARDOWN: u32 = 3;
        let scope = lf_checker_rt::relocated(SCOPE_FILE_VA);
        let mut session_slot: [u32; 2] = [0, 0];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SESSION,
            u32,
            &mut session_slot as *mut u32 as u32,
            scope
        );
        let kind = ((((arg + KIND_OFF) as *const u32).read_unaligned() >> 6) & 0xf) as u8;
        // The tear-down takes the session slot; its contents are the
        // session helper's and unobserved here.
        let teardown = |slot: &mut [u32; 2]| {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                TEARDOWN,
                u32,
                slot as *mut [u32; 2] as u32
            );
        };
        if kind == 2 || kind == 3 {
            teardown(&mut session_slot);
            return 0;
        }
        let sub = this.wrapping_add(SUB_OFF);
        // The fetch helper takes the address of our argument slot, like
        // the original takes its incoming stack slot; both are S+4.
        let mut arg_copy = arg;
        let r: u32 = lf_checker_rt::callee_thiscall!(
            FETCH,
            u32,
            sub,
            &mut arg_copy as *mut u32 as u32
        );
        if r != 0 {
            teardown(&mut session_slot);
            return r;
        }
        let end = sub;
        let mut link = ((this + HEAD_OFF) as *const u32).read_unaligned();
        if link != end {
            loop {
                let obj = (link as *const u32).read_unaligned();
                if ((obj + FLAG_OFF) as *const u32).read_unaligned() & FLAG_MASK == 0 {
                    let slot = ((((obj as *const u32).read_unaligned()) + POKE_SLOT)
                        as *const u32)
                        .read_unaligned();
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    let _ = f(obj);
                    break;
                }
                link = ((link + 4) as *const u32).read_unaligned();
                if link == end {
                    break;
                }
            }
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(
            FETCH,
            u32,
            sub,
            &mut arg_copy as *mut u32 as u32
        );
        teardown(&mut session_slot);
        r2
    }
});
