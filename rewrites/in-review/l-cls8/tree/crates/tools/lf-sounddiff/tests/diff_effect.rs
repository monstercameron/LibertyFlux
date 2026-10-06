//! Differential cases: the base audio effect.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value, every written byte, and every
//! collaborator call in order. Each case also runs a deliberately wrong
//! lift, which must be caught at least once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::sound::effect::Effect;
    use lf_sounddiff::rewrites::*;
    use lf_sounddiff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        Fake, Image, Rng, Stubs, U32_EDGE, VTable, assert_only_changed, check_calls, check_virtual,
        cookie,
    };

    // Object field offsets, as the verified rewrites use them.
    const VT: usize = 0x00;
    const INFO: usize = 0x04;
    const VOICE: usize = 0x08;
    const READY: usize = 0x20;
    const PARAM: usize = 0x24;
    const AUX: usize = 0x28;
    const INDEX: usize = 0x2C;
    const COUNT: usize = 0x30;
    const SLOTS: usize = 0x34;
    const NSLOT: usize = 15;
    const LIMIT: usize = 0x70;
    const ENABLED: usize = 0x71;
    const TAIL: usize = 0x72;
    const OBJ_SIZE: usize = 0x80;
    const TAG_OFF: usize = 0x0A;

    /// A test effect: the 32-bit image plus the blocks it points at.
    /// Kept alive together so addresses stay valid.
    #[allow(dead_code)]
    struct Fixture {
        obj: Image,
        info: Image,
        voice_obj: Image,
        voice_vtable: VTable,
        stubs: Stubs,
    }

    impl Fixture {
        fn build(rng: &mut Rng) -> Self {
            let obj = Image::random(OBJ_SIZE, rng);
            let info = Image::random(0x20, rng);
            let mut voice_obj = Image::random(0x10, rng);
            let mut voice_vtable = VTable::random(8, rng);
            let stubs = Stubs::new();
            voice_vtable.set(0x00, stubs.release);
            voice_vtable.set(0x08, stubs.voice_poll);
            voice_obj.w32(0, voice_vtable.addr());
            Self {
                obj,
                info,
                voice_obj,
                voice_vtable,
                stubs,
            }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }

        /// The lifted effect owning the same words the image holds.
        fn lift(&self) -> Effect {
            let mut slots = [0u32; NSLOT];
            for (i, s) in slots.iter_mut().enumerate() {
                *s = self.obj.r32(SLOTS + i * 4);
            }
            Effect {
                info: cookie(self.obj.r32(INFO)),
                voice: cookie(self.obj.r32(VOICE)),
                ready: self.obj.r32(READY),
                param: self.obj.r32(PARAM),
                aux_word: self.obj.r32(AUX),
                index: self.obj.r32(INDEX),
                count: self.obj.r32(COUNT),
                slots,
                limit: self.obj.r8(LIMIT),
                enabled: self.obj.r8(ENABLED),
                tail: [self.obj.r8(TAIL), self.obj.r8(TAIL + 1)],
            }
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use super::Effect;

        /// Builds a fresh effect with the loop bound cleared.
        pub fn fresh_no_limit() -> Effect {
            Effect {
                limit: 0,
                ..Effect::new()
            }
        }

        /// Releases unconditionally, even with no voice attached.
        pub fn reset_always<W: lf_audio::sound::EffectWorld>(v: &Effect, w: &mut W) {
            let _ = v;
            w.release_voice(v.voice);
        }

        /// Runs one iteration too many (`bl <= limit`).
        pub fn rotate_long(v: &Effect) -> (u32, [u32; 15], u8) {
            let mut c = v.clone();
            let dividend = c.count.wrapping_add(1);
            let quotient = dividend / 3;
            let dst = (dividend % 3).wrapping_mul(5);
            let src = c.count.wrapping_mul(5);
            let mut last = quotient;
            let mut bl = 0u8;
            loop {
                let limit = c.limit;
                if bl > limit {
                    break;
                }
                let s = src.wrapping_add(u32::from(bl)) as usize;
                let d = dst.wrapping_add(u32::from(bl)) as usize;
                // In-bounds only: the caller keeps the wrong run inside
                // the table plus its trailing word.
                let value = if s == 15 {
                    u32::from_le_bytes([c.limit, c.enabled, c.tail[0], c.tail[1]])
                } else {
                    c.slots[s]
                };
                if d == 15 {
                    let b = value.to_le_bytes();
                    c.limit = b[0];
                    c.enabled = b[1];
                    c.tail = [b[2], b[3]];
                } else {
                    c.slots[d] = value;
                }
                last = value;
                bl = bl.wrapping_add(1);
            }
            (last, c.slots, c.limit)
        }

        /// Resets the gains to zero instead of one.
        pub fn attach_zero_gains(v: &mut Effect) {
            v.slots = [0; 15];
        }

        /// Refreshes whenever ready, ignoring the enable flag.
        pub fn poll_no_enable(v: &Effect) -> bool {
            v.ready != 0
        }
    }

    #[test]
    fn effect_ctor_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xEFC7);
        let mut caught = 0;
        for i in 0..100u32 {
            let _ = i;
            let mut fx = Fixture::build(&mut rng);
            let before = fx.obj.buf.clone();
            rt::set_script(&[]);
            let got = unsafe { fn_008a92c0::rw_008a92c0(fx.obj.buf.as_mut_ptr()) };
            assert_eq!(got, fx.this());
            let vtable = rt::relocated(0x00E7_BC64);
            assert_eq!(fx.obj.r32(VT), vtable);
            assert_eq!(fx.obj.r32(INFO), 0);
            assert_eq!(fx.obj.r32(VOICE), 0);
            assert_eq!(fx.obj.r32(READY), 0);
            assert_eq!(fx.obj.r32(PARAM), 0);
            assert_eq!(fx.obj.r32(AUX), 0);
            assert_eq!(fx.obj.r8(LIMIT), 1);
            assert_eq!(fx.obj.r8(ENABLED), 0);
            assert_only_changed(
                &before,
                &fx.obj.buf,
                &[
                    (VT, 4),
                    (INFO, 4),
                    (VOICE, 4),
                    (READY, 4),
                    (PARAM, 4),
                    (AUX, 4),
                    (LIMIT, 2),
                ],
            );
            assert!(rt::take_numbered().is_empty());
            assert!(rt::take_virtual().is_empty());
            // The lift agrees on every byte the rewrite writes.
            let v = Effect::new();
            assert_eq!(v.info, None);
            assert_eq!(v.voice, None);
            assert_eq!(v.ready, 0);
            assert_eq!(v.param, 0);
            assert_eq!(v.aux_word, 0);
            assert_eq!(v.limit, 1);
            assert_eq!(v.enabled, 0);
            if wrong::fresh_no_limit().limit != fx.obj.r8(LIMIT) {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong ctor never caught");
    }

    #[test]
    fn effect_reset_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xE5E7);
        let mut caught = 0;
        for i in 0..100u32 {
            let mut fx = Fixture::build(&mut rng);
            // Alternate the null and attached paths.
            let attached = i % 2 == 0;
            let voice = if attached { fx.voice_obj.addr() } else { 0 };
            fx.obj.w32(VOICE, voice);
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[]);
            let _ = unsafe { fn_008a9300::rw_008a9300(fx.obj.buf.as_mut_ptr()) };
            assert_eq!(fx.obj.r32(VT), rt::relocated(0x00E7_BC64));
            assert_only_changed(&before, &fx.obj.buf, &[(VT, 4)]);
            assert!(rt::take_numbered().is_empty());
            let mut fake = Fake::new();
            v.reset(&mut fake);
            if attached {
                check_virtual(rt::take_virtual(), vec![("release", vec![voice, 1])]);
                assert_eq!(
                    fake.log,
                    vec![("fx.release".to_string(), vec![voice])],
                    "lift release call"
                );
            } else {
                assert!(rt::take_virtual().is_empty());
                assert!(fake.log.is_empty());
            }
            let mut wf = Fake::new();
            wrong::reset_always(&v, &mut wf);
            if wf.log.is_empty() == attached {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong reset never caught");
    }

    #[test]
    fn effect_rotate_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x807A);
        let mut caught = 0;
        let mut cases = 0;
        // Counts 0..=2 with every in-model bound, sweeping the bound
        // from empty through the trailing-word boundary.
        for count in 0..=2u32 {
            let dst_base = ((count + 1) % 3) * 5;
            let src_base = count * 5;
            // Largest bound keeping writes inside the table and reads
            // inside the modelled window (the rewrite reads row
            // `count`, which for count 2 ends at index 15).
            let inner = (15 - dst_base).min(16 - src_base);
            for limit in 0..=inner {
                for rep in 0..12u32 {
                    let _ = rep;
                    let mut fx = Fixture::build(&mut rng);
                    fx.obj.w32(COUNT, count);
                    fx.obj.w8(LIMIT, limit as u8);
                    let before = fx.obj.buf.clone();
                    let mut v = fx.lift();
                    rt::set_script(&[]);
                    let got = unsafe { fn_008a9370::rw_008a9370(fx.obj.buf.as_mut_ptr()) };
                    let want = v.rotate_slots();
                    assert_eq!(got, want, "count {count} limit {limit}");
                    for i in 0..NSLOT {
                        assert_eq!(fx.obj.r32(SLOTS + i * 4), v.slots[i], "slot {i}");
                    }
                    assert_eq!(fx.obj.r8(LIMIT), v.limit);
                    assert_eq!(fx.obj.r8(ENABLED), v.enabled);
                    assert_eq!(fx.obj.r8(TAIL), v.tail[0]);
                    assert_eq!(fx.obj.r8(TAIL + 1), v.tail[1]);
                    assert_only_changed(&before, &fx.obj.buf, &[(SLOTS, 60), (LIMIT, 4)]);
                    assert!(rt::take_numbered().is_empty());
                    // The wrong run stays in-model while its extra
                    // iteration avoids the trailing word entirely (a
                    // write there would re-read a wild bound).
                    if dst_base + limit <= 14 && src_base + limit <= 15 {
                        let (wlast, wslots, wlimit) = wrong::rotate_long(&fx_lift(&before, &fx));
                        if wlast != want || wslots != v.slots || wlimit != v.limit {
                            caught += 1;
                        }
                    }
                    cases += 1;
                }
            }
        }
        // Boundary: the copy lands on the trailing word itself, and the
        // re-read bound ends the loop. The arriving word's low byte is
        // pinned to zero so the loop cannot run on into wild memory.
        for (count, limit) in [(0u32, 11u8), (1u32, 6u8)] {
            for rep in 0..10u32 {
                let _ = rep;
                let mut fx = Fixture::build(&mut rng);
                fx.obj.w32(COUNT, count);
                fx.obj.w8(LIMIT, limit);
                // The word that flows onto the trailing word through
                // the overlapping copy chain starts at the source
                // row's first word: pin its low byte to zero so the
                // re-read bound ends the loop instead of running on
                // into wild memory.
                let src_first = (count * 5) as usize;
                let pinned = fx.obj.r32(SLOTS + src_first * 4) & 0xFFFF_FF00;
                fx.obj.w32(SLOTS + src_first * 4, pinned);
                let before = fx.obj.buf.clone();
                let mut v = fx.lift();
                rt::set_script(&[]);
                let got = unsafe { fn_008a9370::rw_008a9370(fx.obj.buf.as_mut_ptr()) };
                let want = v.rotate_slots();
                assert_eq!(got, want, "boundary count {count} limit {limit}");
                for i in 0..NSLOT {
                    assert_eq!(fx.obj.r32(SLOTS + i * 4), v.slots[i], "slot {i}");
                }
                assert_eq!(fx.obj.r8(LIMIT), v.limit);
                assert_eq!(v.limit, 0);
                assert_only_changed(&before, &fx.obj.buf, &[(SLOTS, 60), (LIMIT, 4)]);
                cases += 1;
            }
        }
        // Count 3 with a one-step bound reads the trailing word as its
        // source: the highest in-model source index.
        for rep in 0..10u32 {
            let _ = rep;
            let mut fx = Fixture::build(&mut rng);
            fx.obj.w32(COUNT, 3);
            fx.obj.w8(LIMIT, 1);
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[]);
            let got = unsafe { fn_008a9370::rw_008a9370(fx.obj.buf.as_mut_ptr()) };
            let want = v.rotate_slots();
            assert_eq!(got, want, "count 3 limit 1");
            for i in 0..NSLOT {
                assert_eq!(fx.obj.r32(SLOTS + i * 4), v.slots[i], "slot {i}");
            }
            assert_only_changed(&before, &fx.obj.buf, &[(SLOTS, 60), (LIMIT, 4)]);
            cases += 1;
        }
        assert!(cases > 200, "too few cases: {cases}");
        assert!(caught > 0, "wrong rotate never caught");
    }

    /// Rebuilds the lift from the pre-run image (the wrong run needs the
    /// entry state; `v` has already advanced).
    fn fx_lift(before: &[u8], fx: &Fixture) -> Effect {
        let _ = fx;
        let mut slots = [0u32; NSLOT];
        for (i, s) in slots.iter_mut().enumerate() {
            *s = u32::from_le_bytes(before[SLOTS + i * 4..SLOTS + i * 4 + 4].try_into().unwrap());
        }
        Effect {
            info: cookie(u32::from_le_bytes(before[INFO..INFO + 4].try_into().unwrap())),
            voice: cookie(u32::from_le_bytes(before[VOICE..VOICE + 4].try_into().unwrap())),
            ready: u32::from_le_bytes(before[READY..READY + 4].try_into().unwrap()),
            param: u32::from_le_bytes(before[PARAM..PARAM + 4].try_into().unwrap()),
            aux_word: u32::from_le_bytes(before[AUX..AUX + 4].try_into().unwrap()),
            index: u32::from_le_bytes(before[INDEX..INDEX + 4].try_into().unwrap()),
            count: u32::from_le_bytes(before[COUNT..COUNT + 4].try_into().unwrap()),
            slots,
            limit: before[LIMIT],
            enabled: before[ENABLED],
            tail: [before[TAIL], before[TAIL + 1]],
        }
    }

    #[test]
    fn effect_attach_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0xA77A);
        let mut caught = 0;
        let mut cases = 0;
        // Null-block path.
        for i in 0..20u32 {
            let mut fx = Fixture::build(&mut rng);
            let param = U32_EDGE[(i as usize) % U32_EDGE.len()];
            fx.obj.w32(INFO, 0x1234_5678);
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[]);
            let got = unsafe {
                fn_008a93b0::rw_008a93b0(
                    fx.obj.buf.as_mut_ptr(),
                    std::ptr::null(),
                    param,
                )
            };
            assert_eq!(got, 0);
            assert_eq!(fx.obj.r32(INFO), 0);
            assert_only_changed(&before, &fx.obj.buf, &[(INFO, 4)]);
            assert!(rt::take_numbered().is_empty());
            let mut fake = Fake::new();
            let want = v.attach(&mut fake, None, 0, param);
            assert!(!want);
            assert_eq!(v.info, None);
            let old_voice = u32::from_le_bytes(before[VOICE..VOICE + 4].try_into().unwrap());
            assert_eq!(v.voice, cookie(old_voice));
            assert_eq!(v.count, u32::from_le_bytes(before[COUNT..COUNT + 4].try_into().unwrap()));
            assert!(fake.log.is_empty());
            cases += 1;
        }
        // Attached paths: the all-ones tag skips the lookup.
        for i in 0..120u32 {
            let mut fx = Fixture::build(&mut rng);
            let info_addr = fx.info.addr();
            fx.obj.w32(INFO, 0);
            let tag = if i % 3 == 0 {
                0xFFFF_FFFF
            } else if i % 3 == 1 {
                U32_EDGE[(i as usize) % U32_EDGE.len()]
            } else {
                rng.u32()
            };
            fx.info.buf[TAG_OFF..TAG_OFF + 4].copy_from_slice(&tag.to_le_bytes());
            let param = if i % 2 == 0 {
                U32_EDGE[(i as usize) % U32_EDGE.len()]
            } else {
                rng.u32()
            };
            let handle = if tag == 0xFFFF_FFFF || i % 4 == 0 {
                0
            } else {
                fx.voice_obj.addr()
            };
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[(1, StubKind::Thiscall3, vec![handle])]);
            let mut fake = Fake::new();
            fake.answer("fx.lookup", vec![handle]);
            let got = unsafe {
                fn_008a93b0::rw_008a93b0(
                    fx.obj.buf.as_mut_ptr(),
                    fx.info.buf.as_ptr(),
                    param,
                )
            };
            // The residue shape the lift narrows away, pinned here.
            let expect_ret = if tag == 0xFFFF_FFFF {
                (info_addr & 0xFFFF_FF00) | 1
            } else {
                (handle & 0xFFFF_FF00) | 1
            };
            assert_eq!(got, expect_ret, "tag {tag:#x} handle {handle:#x}");
            assert_eq!(fx.obj.r32(INFO), info_addr);
            assert_eq!(fx.obj.r32(COUNT), 1);
            assert_eq!(fx.obj.r32(INDEX), 0);
            assert_eq!(fx.obj.r32(PARAM), param);
            assert_eq!(fx.obj.r32(VOICE), handle);
            for s in 0..NSLOT {
                assert_eq!(fx.obj.r32(SLOTS + s * 4), 0x3F80_0000, "gain {s}");
            }
            assert_only_changed(
                &before,
                &fx.obj.buf,
                &[(INFO, 4), (COUNT, 4), (INDEX, 4), (PARAM, 4), (VOICE, 4), (SLOTS, 60)],
            );
            let want = v.attach(&mut fake, cookie(info_addr), tag, param);
            assert!(want);
            assert_eq!(v.info, cookie(info_addr));
            assert_eq!(v.count, 1);
            assert_eq!(v.index, 0);
            assert_eq!(v.param, param);
            assert_eq!(v.voice, cookie(handle));
            assert_eq!(v.slots, [0x3F80_0000; NSLOT]);
            if tag == 0xFFFF_FFFF {
                assert!(rt::take_numbered().is_empty());
                assert!(fake.log.is_empty());
            } else {
                let mgr = rt::relocated(0x0115_DAD4);
                check_calls(
                    rt::take_numbered(),
                    std::mem::take(&mut fake.log),
                    vec![(
                        1,
                        vec![mgr, tag, param.wrapping_add(1)],
                        "fx.lookup",
                        vec![tag, param.wrapping_add(1)],
                    )],
                );
            }
            let mut w = v.clone();
            wrong::attach_zero_gains(&mut w);
            if w.slots != v.slots {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases > 100, "too few cases: {cases}");
        assert!(caught > 0, "wrong attach never caught");
    }

    #[test]
    fn effect_poll_matches() {
        let _lock = rt::script_lock();
        let mut rng = Rng(0x9011);
        let mut caught = 0;
        // The refresh gate crossed with the voice branch.
        for i in 0..160u32 {
            let mut fx = Fixture::build(&mut rng);
            let ready = match i % 4 {
                0 => 0,
                1 => 1,
                2 => U32_EDGE[(i as usize) % U32_EDGE.len()],
                _ => rng.u32(),
            };
            let enabled = match i % 3 {
                0 => 0,
                1 => 1,
                _ => rng.u8(),
            };
            let attached = i % 2 == 0;
            let voice = if attached { fx.voice_obj.addr() } else { 0 };
            fx.obj.w32(READY, ready);
            fx.obj.w8(ENABLED, enabled);
            fx.obj.w32(VOICE, voice);
            let index = if i % 5 == 0 {
                U32_EDGE[(i as usize) % U32_EDGE.len()]
            } else {
                rng.u32()
            };
            fx.obj.w32(INDEX, index);
            let before = fx.obj.buf.clone();
            let mut v = fx.lift();
            rt::set_script(&[(1, StubKind::Cdecl2, vec![0x5555_AAAA])]);
            rt::set_virtual(&[("voice_poll", vec![0x3333_CCCC])]);
            let mut fake = Fake::new();
            let got = unsafe { fn_008a9490::rw_008A9490(fx.this()) };
            assert_eq!(got, 0);
            v.poll(&mut fake);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            let refresh = ready != 0 && enabled != 0;
            if refresh {
                let slot = index.wrapping_mul(5).wrapping_add(13);
                let derived = fx.this().wrapping_add(slot.wrapping_mul(4));
                assert_eq!(
                    rt::take_numbered(),
                    vec![(1, vec![fx.this(), derived])],
                    "rewrite refresh call"
                );
                let mut expect_lift =
                    vec![("fx.refresh".to_string(), vec![slot])];
                if attached {
                    expect_lift.push(("fx.poll".to_string(), vec![voice]));
                }
                assert_eq!(fake.log, expect_lift, "lift calls in order");
            } else {
                assert!(rt::take_numbered().is_empty());
                if attached {
                    assert_eq!(
                        fake.log,
                        vec![("fx.poll".to_string(), vec![voice])],
                        "lift poll call"
                    );
                } else {
                    assert!(fake.log.is_empty());
                }
            }
            if attached {
                check_virtual(rt::take_virtual(), vec![("voice_poll", vec![voice])]);
            } else {
                assert!(rt::take_virtual().is_empty());
            }
            if wrong::poll_no_enable(&v) != refresh {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong poll never caught");
    }
}
