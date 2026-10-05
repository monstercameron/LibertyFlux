// original: 0x00bf8910 task_handle_dispatch (proposed)

/// Look up a task target through a type table, ensure its handle, query it
/// through its virtual table, then dispatch a 16-argument call whose block
/// arguments mix a random seed, copied constants and a matrix combine.
///
/// `this` carries a type id at `+0x08` and a blend input float at `+0x18`;
/// `target` points at the target object (vtable pointer at `+0x00`, a matrix
/// pointer at `+0x20`, a table index word at `+0x2e`); `blend` is a float.
///
/// Behaviour: a null target returns immediately (the original returns whatever
/// was in eax, unobservable; the proof never takes that path). Five log calls
/// run in order until one's answer equals the type id (or all five run). The
/// index word (sign-extended) walks a two-level global table; a final value
/// of -1 returns -1. A parameter block is fetched for the value, three floats
/// are taken from it, and the manager object asks for the handle for
/// (`target+1`, type id) with two zero words, writing a flag byte through an
/// out-cell; a null handle returns 0. When `blend` is zero (ucomiss against
/// +0.0, so -0.0 counts and NaN does not), the `+0x18` float is pushed into
/// the handle. Otherwise a zero flag runs a check call and, when its low
/// byte is non-zero, the compute/lerp pair, whose float result lands in the
/// `+0x18` frame cell that already holds the `+0x18` input float.
/// The target's virtual slot `+0xec`
/// is then called with a zeroed scratch word; three dwords from its answer
/// are stored at handle `+0x190`, a second vector call and a register call
/// run, and a non-zero flag runs the stamp trio (vector push of a zero word
/// plus the two live parameter floats, post call, tick stamp at `+0x1d4`,
/// bumped when the tick-check answer equals the compare word).
///
/// Both paths then run a linear-congruential step over two global words
/// (multiplier 0x5CDCFAA7 with carry) and fold the low 23 bits into a random
/// float scaled into the global range. Two check calls select either the
/// range base or that random float; it is multiplied by the frame cell and
/// three accumulators combine the matrix rows with zero and the two live
/// parameter floats (the first parameter float is overwritten with zero
/// before any read).
/// The dispatch call takes three zero/-1/constant words, four block pointers
/// (three copied constants, aliasing the tail of the second block;
/// accumulators plus a zero read from an unwritten frame slot plus three
/// copied constants plus a zero; 0/1/0; 0/0/1), the scaled float, a zero, a
/// global word, a constant word, two zeros, -1 and two zeros. Its answer is
/// the return value. Unlike its sibling,
/// neither the set/check/apply section nor the stamp section exits: every
/// acquired handle runs the query, the stamp test and the dispatch tail.
///
/// Original: 0x00bf8910 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00bf8910(this: u32, target: u32, blend: u32) -> u32 {
    unsafe {
        const STR_LOG0: u32 = 0x00ebc458;
        const STR_LOG1: u32 = 0x00ebc46c;
        const STR_LOG2: u32 = 0x00ebc47c;
        const STR_LOG3: u32 = 0x00ebc490;
        const STR_LOG4: u32 = 0x00ebc4a4;
        const STR_SET_KEY: u32 = 0x00ebc4b8;
        const TASK_MGR: u32 = 0x01394d60;
        const TYPE_TABLE: u32 = 0x01295cd8;
        const VT_SLOT: u32 = 0xec;
        const TICK_CMP: u32 = 0x011f702c;
        const TICK_SRC: u32 = 0x011f70c4;
        const RNG_LO: u32 = 0x011101a0;
        const RNG_HI: u32 = 0x011101a4;
        const LCG_MUL: u32 = 0x5cdcfaa7;
        const RAND_MASK: u32 = 0x7fffff;
        const C_E8: u32 = 0x00ee19e8;
        const C_EC: u32 = 0x00ee19ec;
        const C_F4: u32 = 0x00ee19f4;
        const C_HI: u32 = 0x00ee1a00;
        const C_LO: u32 = 0x00ee19f8;
        const C_1A04: u32 = 0x00ee1a04;
        const C_SCALE: u32 = 0x00fe864c;
        const G_EXTRA: u32 = 0x0103eed4;
        const TYPE_ID: u32 = 0x08;
        const BLEND_IN: u32 = 0x18;
        const TGT_MATRIX: u32 = 0x20;
        const TGT_INDEX: u32 = 0x2e;
        const HANDLE_V3: u32 = 0x190;
        const HANDLE_STAMP: u32 = 0x1d4;
        const L1_NEXT: u32 = 0xcc;
        const L2_VALUE: u32 = 0x04;
        const ONE_BITS: u32 = 0x3f800000;
        const C_L0: u32 = 1;
        const C_L1: u32 = 2;
        const C_L2: u32 = 3;
        const C_L3: u32 = 4;
        const C_L4: u32 = 5;
        const C_GET: u32 = 6;
        const C_ACQUIRE: u32 = 7;
        const C_SETVAL: u32 = 8;
        const C_CHECK: u32 = 9;
        const C_COMPUTE: u32 = 10;
        const C_LERP: u32 = 11;
        const C_QUERY: u32 = 12;
        const C_VEC2: u32 = 13;
        const C_REG: u32 = 14;
        const C_SETVEC: u32 = 15;
        const C_POST: u32 = 16;
        const C_TICK2: u32 = 17;
        const C_CK1: u32 = 18;
        const C_CK2: u32 = 19;
        const C_BIG: u32 = 20;

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
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        if target == 0 {
            // Original returns incoming eax here; unobservable, never taken
            // by the proof (see narrowed). The value below is unreachable.
            return 0;
        }
        let type_id = rd32(this.wrapping_add(TYPE_ID));
        for (id, s) in [
            (C_L0, STR_LOG0),
            (C_L1, STR_LOG1),
            (C_L2, STR_LOG2),
            (C_L3, STR_LOG3),
            (C_L4, STR_LOG4),
        ] {
            let r = lf_checker_rt::callee_cdecl!(id, u32, lf_checker_rt::relocated(s), 0);
            if type_id == r {
                break;
            }
        }
        let blend_in = rd32(this.wrapping_add(BLEND_IN));
        let idx = (rd32(target.wrapping_add(TGT_INDEX)) & 0xffff) as u16 as i16 as i32;
        let entry = rd32(
            lf_checker_rt::relocated(TYPE_TABLE).wrapping_add((idx.wrapping_mul(4)) as u32),
        );
        let value = rd32(rd32(entry.wrapping_add(L1_NEXT)).wrapping_add(L2_VALUE));
        if value == 0xffffffff {
            return 0xffffffff;
        }
        let params = lf_checker_rt::callee_thiscall!(C_GET, u32, target, value);
        // The first parameter float is stored and then overwritten with zero
        // before any read, so only the other two are live.
        let p34 = rd32(params.wrapping_add(0x34));
        let p38 = rd32(params.wrapping_add(0x38));
        let mut flag: u32 = 0;
        let handle = lf_checker_rt::callee_thiscall!(
            C_ACQUIRE,
            u32,
            lf_checker_rt::relocated(TASK_MGR),
            target.wrapping_add(1),
            type_id,
            (&mut flag as *mut u32) as u32,
            0,
            0
        );
        if handle == 0 {
            return 0;
        }
        // Frame cell: starts as the blend input, becomes the compute call's
        // out-word, then the lerp float on that path.
        let mut cell18 = blend_in;
        let is_eq = f32::from_bits(blend) == 0.0;
        if is_eq {
            // Answer ignored: both paths converge on the query below.
            lf_checker_rt::callee_thiscall!(
                C_SETVAL,
                u32,
                handle,
                lf_checker_rt::relocated(STR_SET_KEY),
                blend_in
            );
        } else if flag & 0xff == 0 {
            let ok = lf_checker_rt::callee_thiscall!(C_CHECK, u32, this);
            if ok & 0xff != 0 {
                lf_checker_rt::callee_cdecl!(C_COMPUTE, u32, (&mut cell18 as *mut u32) as u32, this);
                let f: f32 =
                    lf_checker_rt::callee_thiscall!(C_LERP, f32, this, handle, cell18, blend);
                cell18 = f.to_bits();
            }
        }
        let slot = rd32(rd32(target).wrapping_add(VT_SLOT));
        let query: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let mut scratch: u32 = 0;
        let answer = query(target, (&mut scratch as *mut u32) as u32);
        wr32(handle.wrapping_add(HANDLE_V3), rd32(answer));
        wr32(handle.wrapping_add(HANDLE_V3 + 4), rd32(answer.wrapping_add(4)));
        wr32(handle.wrapping_add(HANDLE_V3 + 8), rd32(answer.wrapping_add(8)));
        let mat_ptr = rd32(target.wrapping_add(TGT_MATRIX));
        lf_checker_rt::callee_thiscall!(C_VEC2, u32, handle, mat_ptr);
        lf_checker_rt::callee_thiscall!(C_REG, u32, lf_checker_rt::relocated(TASK_MGR), handle, target, 0);
        if flag & 0xff != 0 {
            // First word is the zeroed-over store of the dead float.
            let mut triple = [0u32, p34, p38];
            lf_checker_rt::callee_thiscall!(C_SETVEC, u32, handle, triple.as_mut_ptr() as u32);
            lf_checker_rt::callee_thiscall!(C_POST, u32, handle);
            let tick_answer = lf_checker_rt::callee_cdecl!(C_TICK2, u32,);
            let cmp = rd32(lf_checker_rt::relocated(TICK_CMP));
            let tick = rd32(lf_checker_rt::relocated(TICK_SRC));
            let stamped = if cmp != tick_answer {
                tick
            } else {
                tick.wrapping_add(1)
            };
            wr32(handle.wrapping_add(HANDLE_STAMP), stamped);
        }
        // Linear-congruential step over the two global words.
        let s0 = rd32(lf_checker_rt::relocated(RNG_LO));
        let s1 = rd32(lf_checker_rt::relocated(RNG_HI));
        let full = (s0 as u64)
            .wrapping_mul(LCG_MUL as u64)
            .wrapping_add(s1 as u64);
        wr32(lf_checker_rt::relocated(RNG_LO), full as u32);
        wr32(lf_checker_rt::relocated(RNG_HI), (full >> 32) as u32);
        let mut rand = ((full as u32) & RAND_MASK) as f32;
        rand = mul(rand, rdf(lf_checker_rt::relocated(C_SCALE)));
        let span = sub(
            rdf(lf_checker_rt::relocated(C_HI)),
            rdf(lf_checker_rt::relocated(C_LO)),
        );
        rand = mul(rand, span);
        rand = add(rand, rdf(lf_checker_rt::relocated(C_LO)));
        let ck0 = lf_checker_rt::callee_cdecl!(C_CK1, u32,);
        let mut x7 = if ck0 & 0xff != 0 {
            rdf(lf_checker_rt::relocated(C_LO))
        } else {
            let ck1 = lf_checker_rt::callee_cdecl!(C_CK2, u32,);
            if ck1 & 0xff != 0 {
                rdf(lf_checker_rt::relocated(C_LO))
            } else {
                rand
            }
        };
        // Matrix combine. The first parameter slot holds the zero written
        // over the dead float; operand order is the original's.
        let x6 = 0.0f32;
        let x3 = f32::from_bits(p34);
        let x4 = f32::from_bits(p38);
        let m = mat_ptr;
        let t0 = mul(rdf(m), x6);
        let mut x5 = mul(rdf(m.wrapping_add(0x10)), x3);
        let mut x2 = mul(rdf(m.wrapping_add(0x14)), x3);
        x5 = add(x5, t0);
        let mut x1 = rdf(m.wrapping_add(0x18));
        let t2 = mul(rdf(m.wrapping_add(0x20)), x4);
        x7 = mul(x7, f32::from_bits(cell18));
        x5 = add(x5, t2);
        let t3 = mul(rdf(m.wrapping_add(4)), x6);
        x1 = mul(x1, x3);
        x5 = add(x5, rdf(m.wrapping_add(0x30)));
        x2 = add(x2, t3);
        let t4 = mul(rdf(m.wrapping_add(0x24)), x4);
        x2 = add(x2, t4);
        let t5 = mul(rdf(m.wrapping_add(8)), x6);
        x2 = add(x2, rdf(m.wrapping_add(0x34)));
        x1 = add(x1, t5);
        let t6 = mul(rdf(m.wrapping_add(0x28)), x4);
        x1 = add(x1, t6);
        x1 = add(x1, rdf(m.wrapping_add(0x38)));
        // Dispatch blocks. Zero words below are slots the original never
        // stores (contract stack_fill 0), including one it reads back.
        // Block A aliases the const-copy tail of block B.
        let mut blk_a = [
            rd32(lf_checker_rt::relocated(C_E8)),
            rd32(lf_checker_rt::relocated(C_EC)),
            rd32(lf_checker_rt::relocated(C_F4)),
        ];
        let mut blk_b = [
            x5.to_bits(),
            x2.to_bits(),
            x1.to_bits(),
            0u32,
            rd32(lf_checker_rt::relocated(C_E8)),
            rd32(lf_checker_rt::relocated(C_EC)),
            rd32(lf_checker_rt::relocated(C_F4)),
            0u32,
        ];
        let mut blk_c = [0u32, ONE_BITS, 0u32];
        let mut blk_d = [0u32, 0u32, ONE_BITS];
        lf_checker_rt::callee_cdecl!(
            C_BIG,
            u32,
            0,
            0,
            0x201,
            blk_d.as_mut_ptr() as u32,
            blk_c.as_mut_ptr() as u32,
            blk_b.as_mut_ptr() as u32,
            blk_a.as_mut_ptr() as u32,
            x7.to_bits(),
            0,
            rd32(lf_checker_rt::relocated(G_EXTRA)),
            rd32(lf_checker_rt::relocated(C_1A04)),
            0,
            0,
            0xffffffff,
            0,
            0
        )
    }
});
