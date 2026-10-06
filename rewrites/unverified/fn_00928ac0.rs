// original: 0x00928AC0 CRenderPhaseCascadeShadows::vf7

/// Render one cascade-shadow phase: gate on a handle, a global flag and a
/// change check, then queue a setup packet, run a four-pass filter loop and
/// queue two closing packets through the render-command allocator.
///
/// `this` is the phase object. Only three parts of it are used: the vtable
/// pointer at `+0x00` (slot `+0x24` builds the first packet's payload), the
/// handle word at `+0x938` (compared for equality against -1, so signedness
/// is unobservable), and nothing else.
///
/// Behaviour:
/// - Return doing nothing when the handle is -1, when the enable flag byte
///   is clear, or when the watched float (table slot `idx*128` at
///   `FLOAT_TABLE`, where `idx` comes from one of two globals selected by
///   thread-state bits) compares EQUAL to the reference float. The original
///   uses `ucomiss`+`lahf` and exits only on the equal outcome, so less,
///   greater and unordered (NaN on either side) all proceed; a plain Rust
///   `==` has exactly these semantics (`+0.0 == -0.0`, NaN never equal).
/// - Record `this` in the in-flight global, allocate the first packet
///   (size 0x10, kind 1) and, unless null, wrap the vtable-built payload
///   into it, then hand the packet (or null) to the consumer. Run the
///   scene-setup callee with no arguments.
/// - Four passes (`esi` = 0..4, bound compared signed but never negative):
///   each pass runs two code-address-keyed lookups, publishes bit `1<<esi`
///   to a global mask, runs a four-argument keyed build, three frame-pointer
///   transforms, an allocate/bind/consume triplet, a second
///   allocate/tag/consume triplet with two global arguments, a keyed call, a
///   keyed call with a code address, a parameter-block allocate/fill/consume
///   triplet (two 1.0 floats, `0xFF000000`, three zero words, a flag byte),
///   two small-packet triplets, a keyed call, two more small-packet triplets
///   with the flag set, a keyed call, and a frame-pointer teardown.
/// - After the loop, zero the counter slot, run a final keyed lookup, run
///   the scene-finish callee, allocate the tag packet (size 8, kind 0) and,
///   unless null, stamp it (tentative tag, sequence-mixed word at `+4`
///   exactly as `w ^ ((w ^ seq) & 0x3FFF)`, sequence counter incremented,
///   final tag), hand it to the consumer, then allocate and consume the
///   default packet (size 8, kind 0).
///
/// The original builds several small structures on its own stack frame and
/// passes interior pointers to callees; this rewrite mirrors that frame
/// byte for byte (`Frame`, offsets from the balanced stack pointer) so the
/// caller's snapshots match. Slots the original never writes stay zero on
/// both sides (the contract fills uninitialized stack with zero).
///
/// No value is returned (the exit paths leave whatever was in `eax`,
/// including stack-cookie residue on the earliest gates, so the contract
/// compares no return channel).
///
/// Original: 0x00928AC0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00928ac0(this: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x938;
        const VT_BUILD_FIRST: u32 = 0x24;
        const GATE_FLAG: u32 = 0x1036AD0;
        const TLS_INDEX: u32 = 0x17ABA14;
        const TLS_STATE_OFF: u32 = 0x8D0;
        const TABLE_IDX_A: u32 = 0x1174790;
        const TABLE_IDX_B: u32 = 0x1174794;
        const TABLE_STRIDE: u32 = 128;
        const FLOAT_TABLE: u32 = 0x154CC00;
        const FLOAT_REF: u32 = 0xFE8628;
        const IN_FLIGHT: u32 = 0x12FB1B8;
        const MATRIX_BASE: u32 = 0x11A1C50;
        const MATRIX_STRIDE: u32 = 0x270;
        const LOOP_BIT: u32 = 0x11A1BD0;
        const LOOKUP_BASE: u32 = 0x1036A1C;
        const SEQ: u32 = 0x10327A0;
        const SEQ_MASK: u32 = 0x3FFF;
        const TAG_TENTATIVE: u32 = 0xE7E048;
        const TAG_FINAL: u32 = 0xE8668C;
        const CODE_A: u32 = 0x4016A0;
        const CODE_B: u32 = 0x62BF30;
        const CODE_C: u32 = 0x92DD70;
        const CODE_D: u32 = 0x92CB30;
        const CODE_E: u32 = 0x92C740;
        const GLOBAL_ARG: u32 = 0x11A1B94;
        const GLOBAL_ARG2: u32 = 0x11A1B88;
        const GLOBAL_ARG3: u32 = 0x11A1B8C;
        const ONE_BITS: u32 = 0x3F800000;
        const OPAQUE_BLACK: u32 = 0xFF000000;

        const ALLOC_FIRST: u32 = 1;
        // (vtable slot call is callee id 2, reached through the planted vtable)
        const WRAP_FIRST: u32 = 3;
        const CONSUME: u32 = 4;
        const SCENE_SETUP: u32 = 5;
        const MATRIX_LOAD: u32 = 6;
        const PAIR_BUILD: u32 = 7;
        const POST_BUILD: u32 = 8;
        const KEYED_LOOKUP: u32 = 9;
        const KEYED_LOOKUP2: u32 = 10;
        const KEYED_LOOKUP2_POST: u32 = 33;
        const KEYED_BUILD: u32 = 11;
        const XF_INIT: u32 = 12;
        const XF_APPLY: u32 = 13;
        const XF_PROJECT: u32 = 14;
        const ALLOC_BIND: u32 = 15;
        const BIND: u32 = 16;
        const ALLOC_TAG2: u32 = 17;
        const TAG2: u32 = 18;
        const KEYED_CALL: u32 = 19;
        const HANDLE_CALL: u32 = 20;
        const ALLOC_PARAMS: u32 = 21;
        const FILL_PARAMS: u32 = 22;
        const ALLOC_S0: u32 = 23;
        const SMALL_PKT: u32 = 24;
        const ALLOC_S1: u32 = 25;
        const ALLOC_S2: u32 = 26;
        const ALLOC_S3: u32 = 27;
        const ALLOC_TAG: u32 = 28;
        const XF_DONE: u32 = 29;
        const SCENE_FINISH: u32 = 30;
        const ALLOC_LAST: u32 = 31;
        const BUILD_LAST: u32 = 32;

        /// Mirror of the original's stack frame. Offsets are from the
        /// balanced stack pointer (all pushes paired); every offset below
        /// matches the original's `esp+` displacement at a balanced point.
        #[repr(C)]
        struct Frame {
            _pad0: [u8; 0x10],
            counter: u32,      // +0x10 loop counter, also the looked-up slot
            slot14: [u32; 3],  // +0x14 param-fill pointer target, then lookup slot
            s20: [u32; 8],     // +0x20 keyed-build pointer (+0x28 holds params)
            flag: u8,          // +0x40 param flag byte
            _pad1: [u8; 0x2F],
            s70: [u32; 16],    // +0x70 matrix-load object (lea has one push pending)
            _pad2: [u8; 0x20],
            s_d0: [u32; 16],   // +0xD0 pair-build second pointer, +0xE0 keyed-build pointer
            s110: [[u32; 16]; 4], // +0x110 projection inputs, one row per pass
            _pad3: [u8; 0xD0],
            s2e0: [u32; 16],   // +0x2E0 transform object
        }
        const _: () = assert!(core::mem::offset_of!(Frame, counter) == 0x10);
        const _: () = assert!(core::mem::offset_of!(Frame, slot14) == 0x14);
        const _: () = assert!(core::mem::offset_of!(Frame, s20) == 0x20);
        const _: () = assert!(core::mem::offset_of!(Frame, flag) == 0x40);
        const _: () = assert!(core::mem::offset_of!(Frame, s70) == 0x70);
        const _: () = assert!(core::mem::offset_of!(Frame, s_d0) == 0xD0);
        const _: () = assert!(core::mem::offset_of!(Frame, s110) == 0x110);
        const _: () = assert!(core::mem::offset_of!(Frame, s2e0) == 0x2E0);
        const _: () = assert!(core::mem::size_of::<Frame>() == 0x320);

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
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
        unsafe fn consume(packet: u32) {
            unsafe {
                lf_checker_rt::callee_cdecl!(CONSUME, u32, packet);
            }
        }
        #[inline(always)]
        unsafe fn vcall(this: u32, slot: u32) -> u32 {
            unsafe {
                let target: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this) + slot) as usize);
                target(this)
            }
        }

        if rd32(this.wrapping_add(HANDLE)) == 0xFFFF_FFFF {
            return 0;
        }
        if rd8(lf_checker_rt::relocated(GATE_FLAG)) == 0 {
            return 0;
        }
        let slot = rd32(lf_checker_rt::relocated(TLS_INDEX));
        let state = rd32(lf_checker_rt::tls_slot(slot as usize).wrapping_add(TLS_STATE_OFF));
        let idx = if (state >> 1) & 1 != 0 {
            rd32(lf_checker_rt::relocated(TABLE_IDX_A))
        } else if (state >> 3) & 1 != 0 {
            rd32(lf_checker_rt::relocated(TABLE_IDX_A))
        } else {
            rd32(lf_checker_rt::relocated(TABLE_IDX_B))
        };
        let watched = rdf(
            lf_checker_rt::relocated(FLOAT_TABLE)
                .wrapping_add(idx.wrapping_mul(TABLE_STRIDE)),
        );
        let reference = rdf(lf_checker_rt::relocated(FLOAT_REF));
        // ucomiss+lahf exits only on equal; less, greater and unordered
        // (NaN) proceed. f32 == matches that exactly.
        if watched == reference {
            return 0;
        }

        wr32(lf_checker_rt::relocated(IN_FLIGHT), this);

        let mut frame: Frame = core::mem::zeroed();
        let base = core::ptr::addr_of!(frame) as u32;
        let at = |off: u32| base.wrapping_add(off);

        let first: u32 = lf_checker_rt::callee_cdecl!(ALLOC_FIRST, u32, 0x10u32, 1u32);
        if first == 0 {
            consume(0);
        } else {
            let built = vcall(this, VT_BUILD_FIRST);
            let wrapped: u32 = lf_checker_rt::callee_thiscall!(WRAP_FIRST, u32, first, built);
            consume(wrapped);
        }

        let _: u32 = lf_checker_rt::callee_cdecl!(SCENE_SETUP, u32,);

        let row = rd32(lf_checker_rt::relocated(TABLE_IDX_A));
        let matrix = lf_checker_rt::relocated(MATRIX_BASE)
            .wrapping_add(row.wrapping_mul(MATRIX_STRIDE));
        let _: u32 = lf_checker_rt::callee_thiscall!(MATRIX_LOAD, u32, at(0x70), matrix);
        let _: u32 = lf_checker_rt::callee_cdecl!(PAIR_BUILD, u32, at(0x30), at(0xD0));
        let _: u32 = lf_checker_rt::callee_cdecl!(POST_BUILD, u32,);

        let mut pass: u32 = 0;
        frame.counter = 0;
        loop {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                KEYED_LOOKUP, u32,
                lf_checker_rt::relocated(CODE_A),
                at(0x10)
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(
                KEYED_LOOKUP2, u32,
                lf_checker_rt::relocated(CODE_B),
                lf_checker_rt::relocated(GLOBAL_ARG)
            );
            wr32(
                lf_checker_rt::relocated(LOOP_BIT),
                1u32 << (pass & 31),
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(
                KEYED_BUILD, u32,
                lf_checker_rt::relocated(CODE_C),
                at(0x20),
                at(0xE0),
                lf_checker_rt::relocated(LOOKUP_BASE).wrapping_add(pass.wrapping_mul(4))
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(XF_INIT, u32, at(0x2E0));
            let _: u32 = lf_checker_rt::callee_thiscall!(XF_APPLY, u32, at(0x2E0), at(0x30));
            let _: u32 = lf_checker_rt::callee_thiscall!(
                XF_PROJECT, u32,
                at(0x2E0),
                at(0x110).wrapping_add(pass.wrapping_mul(0x40))
            );

            let bound: u32 = lf_checker_rt::callee_cdecl!(ALLOC_BIND, u32, 0x410u32, 0u32);
            if bound == 0 {
                consume(0);
            } else {
                let out: u32 =
                    lf_checker_rt::callee_thiscall!(BIND, u32, bound, at(0x2E0));
                consume(out);
            }

            let tagged: u32 = lf_checker_rt::callee_cdecl!(ALLOC_TAG2, u32, 0x18u32, 0u32);
            if tagged == 0 {
                consume(0);
            } else {
                let g2 = rd32(lf_checker_rt::relocated(GLOBAL_ARG2));
                let g3 = rd32(lf_checker_rt::relocated(GLOBAL_ARG3));
                let out: u32 = lf_checker_rt::callee_thiscall!(
                    TAG2, u32, tagged, 0u32, g3, g2, 0u32
                );
                consume(out);
            }

            let _: u32 = lf_checker_rt::callee_cdecl!(
                KEYED_CALL, u32,
                lf_checker_rt::relocated(CODE_E)
            );
            let handle = rd32(this.wrapping_add(HANDLE));
            let _: u32 = lf_checker_rt::callee_cdecl!(HANDLE_CALL, u32, handle, 0xFFFF_FFFFu32, 0u32);

            frame.s20[2] = ONE_BITS;
            frame.s20[3] = ONE_BITS;
            frame.s20[4] = OPAQUE_BLACK;
            frame.s20[5] = 0;
            frame.s20[6] = 0;
            frame.s20[7] = 0;
            frame.flag = 1;
            let params: u32 = lf_checker_rt::callee_cdecl!(ALLOC_PARAMS, u32, 0x2Cu32, 0u32);
            if params == 0 {
                consume(0);
            } else {
                let out: u32 =
                    lf_checker_rt::callee_thiscall!(FILL_PARAMS, u32, params, 0u32, at(0x14));
                consume(out);
            }

            let s0: u32 = lf_checker_rt::callee_cdecl!(ALLOC_S0, u32, 0x10u32, 0u32);
            if s0 == 0 {
                consume(0);
            } else {
                let out: u32 = lf_checker_rt::callee_thiscall!(SMALL_PKT, u32, s0, 0xAu32, 0u32);
                consume(out);
            }
            let s1: u32 = lf_checker_rt::callee_cdecl!(ALLOC_S1, u32, 0x10u32, 0u32);
            if s1 == 0 {
                consume(0);
            } else {
                let out: u32 = lf_checker_rt::callee_thiscall!(SMALL_PKT, u32, s1, 6u32, 0u32);
                consume(out);
            }

            let _: u32 = lf_checker_rt::callee_cdecl!(
                KEYED_LOOKUP, u32,
                lf_checker_rt::relocated(CODE_D),
                at(0x14)
            );

            let s2: u32 = lf_checker_rt::callee_cdecl!(ALLOC_S2, u32, 0x10u32, 0u32);
            if s2 == 0 {
                consume(0);
            } else {
                let out: u32 = lf_checker_rt::callee_thiscall!(SMALL_PKT, u32, s2, 0xAu32, 1u32);
                consume(out);
            }
            let s3: u32 = lf_checker_rt::callee_cdecl!(ALLOC_S3, u32, 0x10u32, 0u32);
            if s3 == 0 {
                consume(0);
            } else {
                let out: u32 = lf_checker_rt::callee_thiscall!(SMALL_PKT, u32, s3, 6u32, 1u32);
                consume(out);
            }

            let _: u32 = lf_checker_rt::callee_cdecl!(
                KEYED_CALL, u32,
                lf_checker_rt::relocated(CODE_A)
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(XF_DONE, u32, at(0x2E0));

            pass = pass.wrapping_add(1);
            frame.counter = pass;
            if pass >= 4 {
                break;
            }
        }

        frame.counter = 0;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            KEYED_LOOKUP2_POST, u32,
            lf_checker_rt::relocated(CODE_B),
            at(0x10)
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(SCENE_FINISH, u32,);

        let tag: u32 = lf_checker_rt::callee_cdecl!(ALLOC_TAG, u32, 8u32, 0u32);
        if tag == 0 {
            consume(0);
        } else {
            let mix = rd32(tag.wrapping_add(4));
            wr32(tag, lf_checker_rt::relocated(TAG_TENTATIVE));
            let seq = rd32(lf_checker_rt::relocated(SEQ));
            let masked = (mix ^ seq) & SEQ_MASK;
            wr32(tag.wrapping_add(4), mix ^ masked);
            wr32(
                lf_checker_rt::relocated(SEQ),
                seq.wrapping_add(1),
            );
            wr32(tag, lf_checker_rt::relocated(TAG_FINAL));
            consume(tag);
        }

        let last: u32 = lf_checker_rt::callee_cdecl!(ALLOC_LAST, u32, 8u32, 0u32);
        if last == 0 {
            consume(0);
        } else {
            let out: u32 = lf_checker_rt::callee_thiscall!(BUILD_LAST, u32, last);
            consume(out);
        }
        0
    }
});
