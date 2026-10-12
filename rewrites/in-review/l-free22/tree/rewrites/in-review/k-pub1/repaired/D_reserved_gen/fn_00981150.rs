// original: 0x00981150 audio_bank_resolve (proposed)

/// Resolve an audio bank id: hash the bank names on first use, find the
/// requested bank, and run it.
///
/// `arg0` is the bank id to find; `arg1` points to a context whose word at
/// `+0x224` locates a list head. The call runs only when three gates pass:
/// the audio-stop flag is not 1, the two generation counters agree, and the
/// audio state is not `0x12`. The gate exits return the incoming EAX (fixed
/// by the contract), or the first generation counter, respectively.
///
/// Two one-time hash chains fill the bank table, guarded by flag bits in
/// the table's flag word: chain A hashes five names into the five search
/// slots, chain B hashes twenty names into the twenty entry slots in three
/// sub-chains (8, 8, 4). Each chain stores each hash result into the slot
/// named before the next call, so slot `j` holds call `j`'s answer. The
/// checker answers the hash callee from a per-call sequence.
///
/// `arg0` is then compared against the five search slots in order. When it
/// matches none, the function returns `arg0`. On a match at index `i`, a
/// getter callee is tried with code `0xfe`: when it answers null, or the
/// word at `+0x64` of its answer is null, it is retried with code `0xd2`,
/// whose null answer returns 0 and whose null word at `+0x34` returns that
/// answer. Otherwise the final handle (word `+0x68` or `+0x38`) is tested
/// for null (returning 0) and resolved through a lookup callee; its null
/// answer also returns 0. A tag byte at `+0x63` of the looked-up object
/// selects a small mode (1 to 3, 2 to 2, 3 to 1, anything else 0).
///
/// When the match index is 1, a fast-path callee runs with the context and
/// returns its answer. Otherwise a prepare callee (whose frame-pointer
/// argument and effects are unobserved: it only hands an area to later
/// intercepted calls, which never read it back), a context callee and a
/// query callee run; the query takes `(i == 4) ? 2 : 0` (a value the
/// original stashes in a frame slot well ahead) plus another unobserved
/// frame area. Two zero-argument callees are polled for small integers;
/// both run with whatever the previous stub left in ECX, so the contract
/// models them as registerless. An emit callee then takes a table entry
/// indexed by `i + mode * 5`, an unobserved frame area, and the two polled
/// integers; its low byte decides the end: zero runs a tail callee and the
/// finish callee, non-zero runs a build callee (eight arguments, two of
/// them unobserved frame areas) and the finish callee. The finish callee
/// takes the lookup result, the getter's object word, and zero, with the
/// context in ECX; its answer is the return value.
///
/// Original: 0x00981150 (cdecl, two stack arguments, EAX return).
lf_checker_rt::export!(cdecl, rw_00981150(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const ENTRY_EAX: u32 = 0x12345678;
        const GATE_STOP: u32 = 0x11F7060;
        const GATE_GEN_A: u32 = 0x12088B4;
        const GATE_GEN_B: u32 = 0xF1C040;
        const GATE_STATE: u32 = 0x1037720;
        const FLAGS: u32 = 0x12316E4;
        const SEARCH: u32 = 0x12316D0;
        const ENTRIES: u32 = 0x12316E8;
        const LOOKUP_OBJ: u32 = 0x115D9A0;
        const HASH: u32 = 1;
        const GET: u32 = 2;
        const LOOKUP: u32 = 3;
        const FAST: u32 = 4;
        const PREP: u32 = 5;
        const CTX: u32 = 6;
        const QUERY: u32 = 7;
        const POLL_A: u32 = 8;
        const POLL_B: u32 = 9;
        const EMIT: u32 = 10;
        const BUILD: u32 = 11;
        const TAIL2: u32 = 12;
        const FIN: u32 = 13;
        // (slot, name) tables: slot j holds call j's answer.
        const CHAIN_A_SLOTS: [u32; 5] =
            [0x12316D0, 0x12316D4, 0x12316D8, 0x12316DC, 0x12316E0];
        const CHAIN_A_NAMES: [u32; 5] = [0xE8C454, 0xE8C474, 0xE8C4B0, 0xE8C4E4, 0xE8C504];
        const CHAIN_B1_SLOTS: [u32; 8] = [
            0x12316E8, 0x12316EC, 0x12316F0, 0x12316F4, 0x12316F8, 0x12316FC,
            0x1231700, 0x1231704,
        ];
        const CHAIN_B1_NAMES: [u32; 8] = [
            0xE8C544, 0xE8C574, 0xE8C59C, 0xE8C5CC, 0xE8C5FC, 0xE8C634,
            0xE8C65C, 0xE8C678,
        ];
        const CHAIN_B2_SLOTS: [u32; 8] = [
            0x1231708, 0x123170C, 0x1231710, 0x1231714, 0x1231718, 0x123171C,
            0x1231720, 0x1231724,
        ];
        const CHAIN_B2_NAMES: [u32; 8] = [
            0xE8C694, 0xE8C6B4, 0xE8C6D8, 0xE8C700, 0xE8C728, 0xE8C750,
            0xE8C778, 0xE8C798,
        ];
        const CHAIN_B3_SLOTS: [u32; 4] = [0x1231728, 0x123172C, 0x1231730, 0x1231734];
        const CHAIN_B3_NAMES: [u32; 4] = [0xE8C7C4, 0xE8C7F0, 0xE8C81C, 0xE8C840];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g(v: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(v).read() }
        }
        #[inline(always)]
        unsafe fn wgl(v: u32, x: u32) {
            unsafe { lf_checker_rt::global::<u32>(v).write(x) }
        }
        #[inline(always)]
        unsafe fn hash(name: u32) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(HASH, u32, lf_checker_rt::relocated(name), 0) }
        }

        if g(GATE_STOP) == 1 {
            return ENTRY_EAX;
        }
        let gen = g(GATE_GEN_A);
        if gen != g(GATE_GEN_B) {
            return gen;
        }
        if g(GATE_STATE) == 0x12 {
            return gen;
        }
        let mut flags = g(FLAGS);
        if flags & 1 == 0 {
            flags |= 1;
            wgl(FLAGS, flags);
            let mut prev = hash(CHAIN_A_NAMES[0]);
            for j in 0..4 {
                wgl(CHAIN_A_SLOTS[j], prev);
                prev = hash(CHAIN_A_NAMES[j + 1]);
            }
            wgl(CHAIN_A_SLOTS[4], prev);
        }
        flags = g(FLAGS);
        if flags & 2 == 0 {
            flags |= 2;
            wgl(FLAGS, flags);
            let mut prev = hash(CHAIN_B1_NAMES[0]);
            for j in 0..7 {
                wgl(CHAIN_B1_SLOTS[j], prev);
                prev = hash(CHAIN_B1_NAMES[j + 1]);
            }
            wgl(CHAIN_B1_SLOTS[7], prev);
            prev = hash(CHAIN_B2_NAMES[0]);
            for j in 0..7 {
                wgl(CHAIN_B2_SLOTS[j], prev);
                prev = hash(CHAIN_B2_NAMES[j + 1]);
            }
            wgl(CHAIN_B2_SLOTS[7], prev);
            prev = hash(CHAIN_B3_NAMES[0]);
            for j in 0..3 {
                wgl(CHAIN_B3_SLOTS[j], prev);
                prev = hash(CHAIN_B3_NAMES[j + 1]);
            }
            wgl(CHAIN_B3_SLOTS[3], prev);
        }
        let mut idx = 0u32;
        loop {
            if arg0 == g(SEARCH + idx * 4) {
                break;
            }
            idx += 1;
            if idx >= 5 {
                return arg0;
            }
        }
        let ctx = rd32(arg1 + 0x224).wrapping_add(0x44);
        let v1: u32 = lf_checker_rt::callee_thiscall!(GET, u32, ctx, 0xFE);
        let (fin, edi_v) = if v1 != 0 && rd32(v1 + 0x64) != 0 {
            (rd32(v1 + 0x68), rd32(v1 + 0x64))
        } else {
            let v2: u32 = lf_checker_rt::callee_thiscall!(GET, u32, ctx, 0xD2);
            if v2 == 0 {
                return 0;
            }
            let e = rd32(v2 + 0x34);
            if e == 0 {
                return v2;
            }
            (rd32(v2 + 0x38), e)
        };
        if fin == 0 {
            return 0;
        }
        let t: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(LOOKUP_OBJ),
            fin
        );
        if t == 0 {
            return 0;
        }
        let b = rd8(t + 0x63);
        let mode = if b == 1 {
            3u32
        } else if b == 2 {
            2u32
        } else if b == 3 {
            1u32
        } else {
            0u32
        };
        let t2 = arg1.wrapping_add(0x3C0);
        if idx == 1 {
            return lf_checker_rt::callee_thiscall!(FAST, u32, t2, 0, 1);
        }
        // One scratch area stands in for every unobserved frame-pointer
        // argument below; the stubs neither read nor write through them.
        let mut scratch = [0u32; 16];
        let sp = scratch.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(PREP, u32, sp);
        let _: u32 = lf_checker_rt::callee_thiscall!(CTX, u32, t2);
        let k = if idx == 4 { 2u32 } else { 0u32 };
        let _: u32 = lf_checker_rt::callee_thiscall!(QUERY, u32, t2, k, sp);
        let r952: u32 = lf_checker_rt::callee_stdcall!(POLL_A, u32,);
        let r953: u32 = lf_checker_rt::callee_stdcall!(POLL_B, u32,);
        let tab = g(ENTRIES + idx.wrapping_add(mode.wrapping_mul(5)).wrapping_mul(4));
        let re: u32 = lf_checker_rt::callee_thiscall!(EMIT, u32, t2, tab, sp, r952, r953, 0);
        if re as u8 == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(TAIL2, u32,);
            return lf_checker_rt::callee_thiscall!(FIN, u32, t2, t, edi_v, 0);
        }
        let tab2 = g(ENTRIES + r952.wrapping_mul(4));
        // Eight words: the seven pushed for this call plus the earlier pushed
        // copy of the first polled integer, still outstanding above them.
        let _: u32 =
            lf_checker_rt::callee_cdecl!(BUILD, u32, tab2, 0, 0, 1, sp, sp, arg1, r952);
        lf_checker_rt::callee_thiscall!(FIN, u32, t2, t, edi_v, 0)
    }
});
