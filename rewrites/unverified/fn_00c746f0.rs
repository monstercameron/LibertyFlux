// original: 0x00c746f0 ped_task_init_and_flag_scan (proposed)

/// Initialise a ped task object and accumulate its flag masks.
///
/// `this` is a task object (thiscall, no stack arguments, no return value).
/// The object carries a vtable pointer at `+0x00`, an alternate key source at
/// `+0x08`, an info block at `+0x0c`, four flag-mask words at `+0x60..0x6c`, an
/// option byte at `+0x8c`, two lazily built worker objects at `+0x128`/`+0x12c`,
/// a resolved id at `+0x130` and 36 probe bytes at `+0x134`.
///
/// When the worker-A slot starts empty, a setup block runs first: worker A
/// is built (`0x14` bytes through the allocator and constructor callees),
/// then the object is asked for an index through vtable slot `+0x34`, which
/// is resolved through a global registry (a validity bitset plus a row table
/// reached from one global pointer). An index whose validity bit is set
/// resolves to null, which the key load then faults on, exactly as the
/// original does; otherwise the row's key goes through two lookup callees. An
/// info block whose second word is set triggers a refresh callee. A second
/// index query (same vtable slot, this time with the object passed as a stack
/// argument under the cdecl convention, so the callee cleans nothing on
/// either call shape) feeds two more lookup callees whose answer is stored
/// into the info block, and the option byte gates one more worker-A callee. A
/// task that already has worker A skips this whole block.
///
/// When the worker-B slot starts empty, a second block runs: worker B
/// (`0xa8` bytes) is built the same lazy way, a key word taken from the
/// alternate source when present, else from the info block, is handed to a
/// consuming callee, and an id queried through vtable slot `+0x30` (queried
/// twice when the first answer is not -1) is handed to a worker-B callee
/// whose answer is stored as the resolved id.
///
/// Then 36 probe arguments from a global table are each offered, together with
/// a stack out-slot, to a probe callee, and the returned byte is kept in the
/// probe area. Finally 24 flag words from a second global table are each
/// offered through vtable slot `+0x38`; each answer selects one bit of a
/// 128-bit mask: answers below `0x40` set bits in the first pair of mask
/// words, the rest in the second pair, the low six bits choosing the bit.
///
/// Edge cases: a null allocator answer skips its constructor and leaves the
/// worker slot null (later worker callees then run with a null `this`, which
/// the stubs accept); a null info block skips the refresh but still faults on
/// the later unconditional info store; an id answer of -1 skips the second
/// query and the id store.
///
/// Original: 0x00c746f0 (thiscall, no stack words, void).
lf_checker_rt::export!(thiscall, rw_00C746F0(this: u32) -> u32 {
    unsafe {
        const VT_SLOT_INDEX: u32 = 0x34;
        const VT_SLOT_ID: u32 = 0x30;
        const VT_SLOT_FLAG: u32 = 0x38;
        const OFF_ALT: u32 = 0x08;
        const OFF_INFO: u32 = 0x0c;
        const OFF_MASK0: u32 = 0x60;
        const OFF_MASK1: u32 = 0x64;
        const OFF_MASK2: u32 = 0x68;
        const OFF_MASK3: u32 = 0x6c;
        const OFF_OPT_IN: u32 = 0x8c;
        const OFF_KEY_INNER: u32 = 0xb4;
        const OFF_KEY_WORD: u32 = 0x0c;
        const OFF_WORKER_A: u32 = 0x128;
        const OFF_WORKER_B: u32 = 0x12c;
        const OFF_ID: u32 = 0x130;
        const OFF_PROBE_OUT: u32 = 0x134;
        const N_PROBES: u32 = 0x24;
        const NEW_A: u32 = 1;
        const CTOR_A: u32 = 2;
        const RESOLVE_NAME: u32 = 3;
        const WORKER_A_USE: u32 = 4;
        const MAYBE_REFRESH: u32 = 5;
        const LOOKUP_B: u32 = 6;
        const WORKER_A_SET: u32 = 7;
        const WORKER_A_TOUCH: u32 = 8;
        const NEW_B: u32 = 9;
        const CTOR_B: u32 = 10;
        const CONSUME_KEY: u32 = 11;
        const WORKER_B_SET: u32 = 12;
        const PROBE: u32 = 13;
        const REGISTRY: u32 = 0x015f_8ba4;
        const PROBE_TABLE: u32 = 0x0103_daf0;
        const FLAG_TABLE: u32 = 0x0104_b758;
        const FLAG_TABLE_END: u32 = 0x0104_b7b8;
        const INVALID_BIT: u8 = 0x80;
        const NONE: u32 = 0xffff_ffff;
        const FLAG_SPLIT: u32 = 0x40;
        const BIT_MASK: u32 = 0x3f;
        const HALF_BITS: u32 = 0x20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn key_object(this: u32) -> u32 {
            unsafe {
                let alt = rd32(this.wrapping_add(OFF_ALT));
                if alt != 0 {
                    rd32(alt.wrapping_add(OFF_KEY_INNER))
                } else {
                    rd32(rd32(this.wrapping_add(OFF_INFO)))
                }
            }
        }

        // Worker-A setup block: runs only when the slot starts empty; a
        // task that already has worker A skips everything up to worker B.
        let vtable = rd32(this);
        if rd32(this.wrapping_add(OFF_WORKER_A)) == 0 {
            // Lazily built worker A.
            let mem: u32 = lf_checker_rt::callee_cdecl!(NEW_A, u32, 0x14u32);
            let built = if mem == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(CTOR_A, u32, mem)
            };
            wr32(this.wrapping_add(OFF_WORKER_A), built);
            // Index lookup through the global registry.
            let index_fn: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(VT_SLOT_INDEX)) as usize);
            let index = index_fn(this);
            let reg = rd32(lf_checker_rt::relocated(REGISTRY));
            let bit_base = rd32(reg.wrapping_add(4));
            let resolved = if (bit_base.wrapping_add(index) as *const u8).read() & INVALID_BIT != 0 {
                0
            } else {
                let stride = rd32(reg.wrapping_add(0x0c));
                let row_base = rd32(reg);
                stride.wrapping_mul(index).wrapping_add(row_base)
            };
            let key = rd32(resolved.wrapping_add(OFF_KEY_WORD));
            if key != NONE {
                let named: u32 = lf_checker_rt::callee_cdecl!(RESOLVE_NAME, u32, key);
                lf_checker_rt::callee_thiscall!(
                    WORKER_A_USE,
                    u32,
                    rd32(this.wrapping_add(OFF_WORKER_A)),
                    named
                );
            }
            // Conditional refresh, then the second index query.
            let info = rd32(this.wrapping_add(OFF_INFO));
            if info != 0 && rd32(info.wrapping_add(4)) != 0 {
                lf_checker_rt::callee_thiscall!(MAYBE_REFRESH, u32, this);
            }
            // Same vtable slot as above, now with one stack argument; the callee
            // is cdecl so it cleans nothing in either shape.
            let index2_fn: extern "cdecl" fn(u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(VT_SLOT_INDEX)) as usize);
            let index2 = index2_fn(this);
            // Stdcall: one stack argument with callee cleanup; the entry ecx
            // is whatever the index callee left, so it is not part of the
            // call on either side.
            let look: u32 = lf_checker_rt::callee_stdcall!(LOOKUP_B, u32, index2);
            wr32(
                rd32(this.wrapping_add(OFF_INFO)).wrapping_add(4),
                lf_checker_rt::callee_thiscall!(
                    WORKER_A_SET,
                    u32,
                    rd32(this.wrapping_add(OFF_WORKER_A)),
                    look
                ),
            );
            if (this.wrapping_add(OFF_OPT_IN) as *const u8).read() != 0 {
                lf_checker_rt::callee_thiscall!(
                    WORKER_A_TOUCH,
                    u32,
                    rd32(this.wrapping_add(OFF_WORKER_A))
                );
            }
        }
        // Worker-B block: runs only when that slot starts empty, together
        // with the key handoff and the resolved id.
        if rd32(this.wrapping_add(OFF_WORKER_B)) == 0 {
            let mem: u32 = lf_checker_rt::callee_cdecl!(NEW_B, u32, 0xa8u32);
            let built = if mem == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(CTOR_B, u32, mem)
            };
            wr32(this.wrapping_add(OFF_WORKER_B), built);
            lf_checker_rt::callee_stdcall!(CONSUME_KEY, u32, rd32(key_object(this).wrapping_add(OFF_KEY_WORD)));
            let id_fn: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vtable.wrapping_add(VT_SLOT_ID)) as usize);
            if id_fn(this) != NONE {
                let id2 = id_fn(this);
                wr32(
                    this.wrapping_add(OFF_ID),
                    lf_checker_rt::callee_thiscall!(
                        WORKER_B_SET,
                        u32,
                        rd32(this.wrapping_add(OFF_WORKER_B)),
                        id2
                    ),
                );
            }
        }
        // Probe sweep over the global probe table.
        let probe_table = lf_checker_rt::relocated(PROBE_TABLE) as *const u32;
        let mut i = 0u32;
        while i < N_PROBES {
            let probe_arg = probe_table.add(i as usize).read_unaligned();
            // The out-slot is pre-filled with 0xff before the call.
            let mut slot: u32 = 0xff;
            lf_checker_rt::callee_thiscall!(
                PROBE,
                u32,
                rd32(key_object(this).wrapping_add(OFF_KEY_WORD)),
                probe_arg,
                core::ptr::addr_of_mut!(slot) as u32
            );
            (this.wrapping_add(OFF_PROBE_OUT).wrapping_add(i) as *mut u8).write(slot as u8);
            i += 1;
        }
        // Flag scan over the global flag table.
        let flag_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_SLOT_FLAG)) as usize);
        let mut flag_ptr = lf_checker_rt::relocated(FLAG_TABLE) as *const u32;
        let flag_end = lf_checker_rt::relocated(FLAG_TABLE_END) as *const u32;
        while flag_ptr < flag_end {
            let bit = flag_fn(this, flag_ptr.read_unaligned());
            flag_ptr = flag_ptr.add(1);
            let masked = bit & BIT_MASK;
            // The split uses a signed comparison: negative answers land in
            // the first mask pair.
            let (word, shift) = if (bit as i32) < FLAG_SPLIT as i32 {
                if masked < HALF_BITS {
                    (OFF_MASK0, masked)
                } else {
                    (OFF_MASK1, masked & 31)
                }
            } else if masked < HALF_BITS {
                (OFF_MASK2, masked)
            } else {
                (OFF_MASK3, masked & 31)
            };
            wr32(
                this.wrapping_add(word),
                rd32(this.wrapping_add(word)) | (1u32 << shift),
            );
        }
        0
    }
});
