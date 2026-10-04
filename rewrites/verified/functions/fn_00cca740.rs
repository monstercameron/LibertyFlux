// original: 0x00cca740 effect_submit_gated (proposed)
/// Predicate-gated submission that builds three parameter blocks, invokes a
/// handler, and reports a status bit.
///
/// `obj` points to a large owner object; `param` is an opaque word forwarded
/// into the middle parameter block. The owner layout read here is: flags word
/// at `+0x28` (enable is bit 21), a flag byte copied into the handler
/// descriptor at `+0x210`, the handler object at `+0x570`, and a force-on
/// state byte at `+0xa60`.
///
/// Algorithm: return 0 unless enable bit 21 of the flags word is set, then
/// resolve a session object through the global registry (callee 1, called
/// with the registry base and `obj`; a null answer returns 0). The go/no-go
/// predicate is forced true when the state byte equals 1, otherwise it reads
/// an index from a global slot: index -1 or a null table entry means no-go,
/// else the table entry is asked (callee 2) and no-go is reported exactly
/// when its low byte is non-zero (inverted sense). On no-go the function
/// returns 0.
/// On go it constructs block C (callee 3, arguments 0, a global word and the
/// tag 5), forms the scaled rate `global_float * 25.0` (the second factor is
/// a read-only constant), fills the shared block A/B (callee 4, arguments the
/// session word at `+0x44`, the scaled rate bits, `param`, 0, 0), issues the
/// request (callee 5, arguments `obj` and a status buffer) reusing block A/B
/// as its object, submits the handler descriptor (callee 6 on `obj + 0x570`,
/// one argument: the words 20.0 and `3 | flag_byte << 16`), then destroys
/// block B (callee 7) and block C (callee 8). The result is 1 when bits 2-3
/// of status byte 4 are set, else 0.
///
/// Edge cases: every early exit returns full `eax` with only the low byte
/// forced (the upper 24 bits keep whatever the last value held: the shifted
/// flags word, or a scripted callee answer with its low byte cleared); the
/// deep path returns the destroy answer's upper bits or-ed with the status
/// bit. `param` is never inspected, only forwarded. The enable test reads bit
/// 21 via a shift, so the whole flags word feeds the early-exit residue.
///
/// Original: 0x00CCA740 (cdecl, two stack words), returns a byte in `al`.
lf_checker_rt::export!(cdecl, rw_00cca740(obj: u32, param: u32) -> u32 {
    unsafe {
        const FLAG_WORD: u32 = 0x28;
        const ENABLE_BIT: u32 = 21;
        const COPY_BYTE: u32 = 0x210;
        const HANDLER_OBJ: u32 = 0x570;
        const STATE_BYTE: u32 = 0xa60;
        const SESSION_EXTRA: u32 = 0x44;
        const REGISTRY: u32 = 0x012e2420;
        const INDEX_GLOBAL: u32 = 0x01036f14;
        const TABLE: u32 = 0x011a8808;
        const NO_INDEX: u32 = 0xffff_ffff;
        const CTOR_WORD_GLOBAL: u32 = 0x011735b4;
        const RATE_GLOBAL: u32 = 0x011735bc;
        const RATE_SCALE: u32 = 0x00fe8b40;
        const CTOR_TAG: u32 = 5;
        const DESC_FLOAT: f32 = 20.0;
        const DESC_KIND: u32 = 3;
        const STATUS_WORD: usize = 1;
        const STATUS_MASK: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        // Gate 1: enable bit 21 of the flags word.
        let mut eax: u32 = rd32(obj + FLAG_WORD) >> ENABLE_BIT;
        if eax & 1 == 0 {
            return eax & 0xffff_ff00;
        }
        // Resolve the session object through the global registry.
        eax = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTRY), obj);
        if eax == 0 {
            return 0;
        }
        let session = eax;
        // Gate 2: forced on by the state byte, else the global slot's check.
        let go: bool;
        if rd8(obj + STATE_BYTE) == 1 {
            go = true;
        } else {
            let idx = rd32(lf_checker_rt::relocated(INDEX_GLOBAL));
            if idx == NO_INDEX {
                eax &= 0xffff_ff00;
                go = false;
            } else {
                let slot =
                    rd32(lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)));
                if slot == 0 {
                    eax &= 0xffff_ff00;
                    go = false;
                } else {
                    eax = lf_checker_rt::callee_thiscall!(2, u32, slot);
                    if eax & 0xff == 0 {
                        go = true;
                    } else {
                        eax &= 0xffff_ff00;
                        go = false;
                    }
                }
            }
        }
        if !go {
            return eax & 0xffff_ff00;
        }
        // Block C: constructed here, destroyed at the end.
        let mut block_c = [0u32; 8];
        let ctor_word = rd32(lf_checker_rt::relocated(CTOR_WORD_GLOBAL));
        lf_checker_rt::callee_thiscall!(
            3,
            u32,
            block_c.as_mut_ptr() as u32,
            0,
            ctor_word,
            CTOR_TAG
        );
        // Scaled rate in the original's operand order.
        let rate = f32::from_bits(rd32(lf_checker_rt::relocated(RATE_GLOBAL)));
        let scale = f32::from_bits(rd32(lf_checker_rt::relocated(RATE_SCALE)));
        let scaled = mul(rate, scale);
        // Shared block A/B: filled, then used as the request object.
        let mut block_ab = [0u32; 8];
        let extra = rd32(session + SESSION_EXTRA);
        lf_checker_rt::callee_thiscall!(
            4,
            u32,
            block_ab.as_mut_ptr() as u32,
            extra,
            scaled.to_bits(),
            param,
            0,
            0
        );
        let mut status = [0u32; 4];
        lf_checker_rt::callee_thiscall!(
            5,
            u32,
            block_ab.as_mut_ptr() as u32,
            obj,
            status.as_mut_ptr() as u32
        );
        // Handler descriptor: 20.0, kind 3, copied flag byte at byte 6.
        let flag = rd8(obj + COPY_BYTE);
        let mut desc = [0u32; 2];
        desc[0] = DESC_FLOAT.to_bits();
        desc[1] = DESC_KIND | ((flag as u32) << 16);
        lf_checker_rt::callee_thiscall!(
            6,
            u32,
            obj.wrapping_add(HANDLER_OBJ),
            desc.as_ptr() as u32
        );
        let bl: u32 = if status[STATUS_WORD] & 0xff & STATUS_MASK != 0 {
            1
        } else {
            0
        };
        lf_checker_rt::callee_thiscall!(7, u32, block_ab.as_mut_ptr() as u32);
        let last = lf_checker_rt::callee_thiscall!(8, u32, block_c.as_mut_ptr() as u32);
        (last & 0xffff_ff00) | bl
    }
});
