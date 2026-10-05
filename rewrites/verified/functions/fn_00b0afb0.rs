// original: 0x00b0afb0 net_player_score_publish (proposed)

/// Publish this network member's score, or refresh the fallback display.
///
/// `this` (ecx) is a network member object; the function takes no stack
/// arguments and returns nothing meaningful (the exit eax is
/// path-dependent residue, so the contract compares no return value).
///
/// Fast path: when the member is active (byte `+0x219`), the network gate
/// callee answers nonzero, the global enable byte is set, the auxiliary
/// object at `+0x6c` (if any) has a zero flag at its `+0x0e`, and the key at
/// `+0x1e4` is nonzero, the killer lookup runs on (`this`, key); a negative
/// answer ends the call. Otherwise two player references are resolved and
/// their `+0x580` generation fields are compared: equal generations end the
/// call unless both are -1 (unset). A second lookup on `this`, negative or
/// not, selects the score path: a negative answer publishes generation
/// `0 + 10`, otherwise the player's score token is stored twice through the
/// global score manager and the second store's answer plus 10 is published.
/// Publishing calls the emit callee with (`u32[this+0x20] + 0x30`, value).
///
/// Slow path (any gate above closed): bit 0x16 of `+0x260` decides. When
/// clear, the word at `[[this+0x21c]+0x12c]` equal to 2, 0x10 or 0x11 ends
/// the call, as does mode byte `+0xa60` equal to 2 with the bit clear, flag
/// bit 2 of `+0x26c`, or the counter at `+0xb88` below 10. Otherwise the
/// emit callee runs with (`u32[this+0x20] + 0x30`, counter) and the reset
/// callee runs on (`this`, 0).
///
/// Callee names are proposed except where the merged symbols already named
/// the target (killer lookup, player-by-index); conventions were read off
/// the call sites (a callee followed by `(an instruction of the original)` is cdecl, one without
/// is thiscall).
lf_checker_rt::export!(thiscall, rw_00b0afb0(this: u32) -> u32 {
    unsafe {
        const ACTIVE: u32 = 0x219;
        const GLOBAL_ENABLE: u32 = 0x0104_00ba;
        const AUX: u32 = 0x6c;
        const AUX_FLAG: u32 = 0x0e;
        const KEY: u32 = 0x1e4;
        const BASE: u32 = 0x20;
        const BASE_BIAS: u32 = 0x30;
        const GENERATION: u32 = 0x580;
        const UNSET: u32 = 0xffff_ffff;
        const PUBLISH_BIAS: u32 = 10;
        const SCORE_MGR: u32 = 0x0198_1a58;
        const GATE: u32 = 0;
        const FIND_KILLER: u32 = 1;
        const PLAYER_BY_INDEX: u32 = 2;
        const ACTIVE_PLAYER: u32 = 3;
        const LOOKUP_INDEX: u32 = 4;
        const SCORE_OF: u32 = 5;
        const MGR_STORE: u32 = 6;
        const EMIT: u32 = 7;
        const RESET_STATE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        /// The slow path: fallback display refresh.
        unsafe fn slow_path(this: u32) -> u32 {
            unsafe {
                const MODE_BIT: u32 = 0x16;
                const MODE_WORD: u32 = 0x260;
                const FALLBACK_PTR: u32 = 0x21c;
                const FALLBACK_STATE: u32 = 0x12c;
                const FALLBACK_MODE: u32 = 0xa60;
                const FALLBACK_FLAGS: u32 = 0x26c;
                const FALLBACK_FLAG_BUSY: u32 = 4;
                const FALLBACK_COUNT: u32 = 0xb88;
                const MIN_COUNT: u32 = 10;
                const BASE: u32 = 0x20;
                const BASE_BIAS: u32 = 0x30;
                const EMIT: u32 = 7;
                const RESET_STATE: u32 = 8;

                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn rd8(a: u32) -> u8 {
                    unsafe { (a as *const u8).read() }
                }

                let bit = rd32(this.wrapping_add(MODE_WORD)) >> MODE_BIT & 1;
                if bit == 0 {
                    let inner = rd32(rd32(this.wrapping_add(FALLBACK_PTR)).wrapping_add(FALLBACK_STATE));
                    if inner == 2 || inner == 0x10 || inner == 0x11 {
                        return 0;
                    }
                }
                if rd8(this.wrapping_add(FALLBACK_MODE)) == 2 && bit == 0 {
                    return 0;
                }
                if rd32(this.wrapping_add(FALLBACK_FLAGS)) & FALLBACK_FLAG_BUSY != 0 {
                    return 0;
                }
                let count = rd32(this.wrapping_add(FALLBACK_COUNT));
                if count < MIN_COUNT {
                    return 0;
                }
                let base = rd32(this.wrapping_add(BASE)).wrapping_add(BASE_BIAS);
                let _: u32 = lf_checker_rt::callee_cdecl!(EMIT, u32, base, count);
                let _: u32 = lf_checker_rt::callee_thiscall!(RESET_STATE, u32, this, 0);
                0
            }
        }

        if rd8(this.wrapping_add(ACTIVE)) == 0 {
            return slow_path(this);
        }
        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32,);
        if gate == 0 {
            return slow_path(this);
        }
        if rd8(lf_checker_rt::relocated(GLOBAL_ENABLE)) == 0 {
            return slow_path(this);
        }
        let aux = rd32(this.wrapping_add(AUX));
        if aux != 0 && rd8(aux.wrapping_add(AUX_FLAG)) != 0 {
            return slow_path(this);
        }
        let key = rd32(this.wrapping_add(KEY));
        if key == 0 {
            return 0;
        }
        let killer: u32 = lf_checker_rt::callee_cdecl!(FIND_KILLER, u32, this, key);
        if (killer as i32) < 0 {
            return 0;
        }
        let player: u32 = lf_checker_rt::callee_cdecl!(PLAYER_BY_INDEX, u32, killer);
        let active: u32 = lf_checker_rt::callee_cdecl!(ACTIVE_PLAYER, u32,);
        if player == 0 || active == 0 {
            return 0;
        }
        let gen_p = rd32(player.wrapping_add(GENERATION));
        let gen_a = rd32(active.wrapping_add(GENERATION));
        if gen_p == gen_a && gen_a != UNSET {
            return 0;
        }
        let base = rd32(this.wrapping_add(BASE)).wrapping_add(BASE_BIAS);
        let index: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_INDEX, u32, this);
        if (index as i32) < 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(EMIT, u32, base, PUBLISH_BIAS);
            return 0;
        }
        let p2: u32 = lf_checker_rt::callee_cdecl!(PLAYER_BY_INDEX, u32, index);
        let token: u32 = lf_checker_rt::callee_thiscall!(SCORE_OF, u32, p2);
        let mgr = lf_checker_rt::relocated(SCORE_MGR);
        let stored: u32 = lf_checker_rt::callee_thiscall!(MGR_STORE, u32, mgr, token);
        if (stored as i32) < 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(EMIT, u32, base, PUBLISH_BIAS);
            return 0;
        }
        let token2: u32 = lf_checker_rt::callee_thiscall!(SCORE_OF, u32, p2);
        let published: u32 = lf_checker_rt::callee_thiscall!(MGR_STORE, u32, mgr, token2);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            EMIT,
            u32,
            base,
            published.wrapping_add(PUBLISH_BIAS)
        );
        0
    }
});
