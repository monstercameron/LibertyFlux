// original: 0x00D71C20 replay_block_state_gate (proposed)

lf_checker_rt::export!(thiscall, rw_00D71C20(this: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x104;
        const HOME_OFF: u32 = 0xA0;
        const NEXT_OFF: u32 = 0x04;
        const ENTRY_F1: u32 = 0x18;
        const ENTRY_F2: u32 = 0x1C;
        const G_INDEX: u32 = 0x0118_EC8C;
        const G_TABLE: u32 = 0x0118_E7F8;
        const G_FACTOR_A0: u32 = 0x0105_C884;
        const G_FACTOR_A1: u32 = 0x0105_C888;
        const G_FACTOR_B0: u32 = 0x0105_C880;
        const G_FACTOR_B1: u32 = 0x0105_C87C;
        const G_FLAG: u32 = 0x011F_70E0;
        const G_VALUE: u32 = 0x011F_70E4;
        const ST_DETACHED: u32 = 4;
        const ST_ARMED: u32 = 3;
        const ST_IDLE: u32 = 0;
        const HOME_INVALID: u32 = 0xFFFF_FFFF;
        const FLAG_HIGH: u32 = 6;
        const FLAG_LOW: u32 = 0x0E;
        const CAL_GUARD: u32 = 1;
        const CAL_POLL: u32 = 2;
        const CAL_RANGE: u32 = 3;
        const CAL_GATE_A: u32 = 4;
        const CAL_GATE_B: u32 = 5;
        const CAL_CONVERT: u32 = 6;
        const CAL_FETCH: u32 = 7;
        const CAL_LIMIT: u32 = 8;
        const CAL_FLAG: u32 = 9;
        const CAL_TAIL: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        /// Exact signed integer to float conversion (`cvtdq2ps`).
        #[inline(always)]
        fn cvt(v: u32) -> f32 {
            (v as i32) as f32
        }
        /// Multiply in the original's operand order, kept uncommuted.
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn flag_and_record(value: u32) {
            unsafe {
                let fp = lf_checker_rt::global::<u32>(G_FLAG);
                fp.write(fp.read() | 1);
                lf_checker_rt::global::<u32>(G_VALUE).write(value);
            }
        }

        let guard: u32 = lf_checker_rt::callee_thiscall!(CAL_GUARD, u32, this);
        if (guard as u8) == 0 {
            return guard;
        }
        if rd32(this.wrapping_add(STATE_OFF)) == ST_DETACHED {
            // The return register still holds the guard answer here.
            return guard;
        }
        let index = rd32(lf_checker_rt::global::<u32>(G_INDEX) as u32);
        let table = lf_checker_rt::global::<u32>(G_TABLE) as u32;
        let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
        let first = rdf(entry.wrapping_add(ENTRY_F1));
        let second = rdf(entry.wrapping_add(ENTRY_F2));

        let pick_a: u32 = lf_checker_rt::callee_thiscall!(CAL_POLL, u32, this);
        let factor_a = if (pick_a as u8) != 0 {
            rd32(lf_checker_rt::global::<u32>(G_FACTOR_A1) as u32)
        } else {
            rd32(lf_checker_rt::global::<u32>(G_FACTOR_A0) as u32)
        };
        let scaled_first = mul(cvt(factor_a), first);

        // The original reloads ecx with the selected first factor before
        // this call (mov/cmovne), so the callee observes the factor in ecx,
        // not the object pointer. Mirror that threading exactly.
        let pick_b: u32 = lf_checker_rt::callee_thiscall!(CAL_POLL, u32, factor_a);
        let factor_b = if (pick_b as u8) != 0 {
            rd32(lf_checker_rt::global::<u32>(G_FACTOR_B1) as u32)
        } else {
            rd32(lf_checker_rt::global::<u32>(G_FACTOR_B0) as u32)
        };
        // Computed and spilled exactly like the original; its slot is never
        // read back, so the value is discarded after forcing its evaluation.
        let scaled_second = mul(cvt(factor_b), second);
        core::hint::black_box(scaled_second);

        let mut probe: f32 = scaled_first;
        let in_range: u32 = lf_checker_rt::callee_thiscall!(
            CAL_RANGE,
            u32,
            this,
            core::ptr::addr_of_mut!(probe) as u32
        );
        if (in_range as u8) == 0 && rd32(this.wrapping_add(STATE_OFF)) != ST_ARMED {
            return in_range;
        }

        let gate_a: u32 = lf_checker_rt::callee_thiscall!(CAL_GATE_A, u32, this);
        if (gate_a as u8) == 0 {
            let gate_b: u32 = lf_checker_rt::callee_thiscall!(CAL_GATE_B, u32, this);
            if (gate_b as u8) == 0 {
                return gate_b;
            }
            if rd32(this.wrapping_add(STATE_OFF)) != ST_ARMED {
                return gate_b;
            }
            let next = rd32(this.wrapping_add(NEXT_OFF));
            wr32(this.wrapping_add(STATE_OFF), ST_IDLE);
            return lf_checker_rt::callee_thiscall!(CAL_TAIL, u32, next);
        }

        let mut state = rd32(this.wrapping_add(STATE_OFF));
        if state == ST_IDLE {
            wr32(this.wrapping_add(STATE_OFF), ST_ARMED);
            wr32(this.wrapping_add(HOME_OFF), HOME_INVALID);
            state = ST_ARMED;
        }
        if state != ST_ARMED {
            return gate_a;
        }

        let converted: u32 = lf_checker_rt::callee_thiscall!(
            CAL_CONVERT,
            u32,
            this,
            scaled_first.to_bits(),
            1
        );
        let fetched: u32 =
            lf_checker_rt::callee_thiscall!(CAL_FETCH, u32, this, converted);
        let limit: u32 = lf_checker_rt::callee_thiscall!(CAL_LIMIT, u32, this);
        if fetched > limit {
            let r: u32 = lf_checker_rt::callee_cdecl!(CAL_FLAG, u32, FLAG_HIGH);
            flag_and_record(fetched);
            return r;
        }
        if fetched < limit {
            let r: u32 = lf_checker_rt::callee_cdecl!(CAL_FLAG, u32, FLAG_LOW);
            flag_and_record(fetched);
            return r;
        }
        limit
    }
});
