//! Differential cases, part 3: the voice tracker.
//!
//! Each case plants the tracker object (and its linked record), runs the
//! rewrite and the lift on the same inputs with the same scripted world,
//! and compares the return, every written byte and every collaborator
//! call in order. Effect hooks let the install-and-bind cases script
//! install-like callee writes on both sides. Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_voice::tracker::{
        INVALID_HANDLE, LinkedSlot, PARK_CACHED, PARK_TAG, PoolHandle, TrackerWorld, VoiceHandle,
        VoiceTracker,
    };
    use lf_audiovoicediff::rewrites::*;
    use lf_audiovoicediff::rt::{self, StubKind};
    use lf_core::Handle32;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{POOL_VA, Rng, addr, lock};

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(base: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(base as *mut u8, len) }
    }

    /// A planted tracker object with its optional linked record.
    struct TFixture {
        obj: Box<[u8]>,
        rec: Option<Box<[u8]>>,
    }

    impl TFixture {
        fn this(&self) -> u32 {
            addr(&self.obj[0])
        }
        fn link(&self) -> u32 {
            self.rec.as_ref().map_or(0, |r| addr(&r[0]))
        }
    }

    /// Plants a tracker: voice word, link word, count word over random fill.
    fn plant(rng: &mut Rng, voice: u32, with_link: bool, count: u32) -> TFixture {
        let rec = if with_link {
            let mut v = vec![0u8; 0x4C];
            rng.bytes(&mut v);
            Some(v.into_boxed_slice())
        } else {
            None
        };
        let mut v = vec![0u8; 0x3C];
        rng.bytes(&mut v);
        v[0x30..0x34].copy_from_slice(&voice.to_le_bytes());
        v[0x38..0x3C].copy_from_slice(&count.to_le_bytes());
        let fx = TFixture {
            obj: v.into_boxed_slice(),
            rec,
        };
        let link = fx.link();
        unsafe {
            image(fx.this(), 0x3C)[0x34..0x38].copy_from_slice(&link.to_le_bytes());
        }
        fx
    }

    /// Decodes the lifted tracker from a planted fixture.
    fn decode(fx: &TFixture) -> VoiceTracker {
        let img = unsafe { image(fx.this(), 0x3C) };
        let voice = u32::from_le_bytes(img[0x30..0x34].try_into().unwrap());
        let count = u32::from_le_bytes(img[0x38..0x3C].try_into().unwrap());
        let link = fx.rec.as_ref().map(|r| {
            let ri = unsafe { image(addr(&r[0]), 0x4C) };
            LinkedSlot {
                tag: ri[0x40],
                cached: u32::from_le_bytes(ri[0x48..0x4C].try_into().unwrap()),
            }
        });
        VoiceTracker::new(Handle32::new(voice), link, count)
    }

    /// The scripted world: records translated calls, scripts answers, and
    /// optionally applies install-like effects to the lifted tracker.
    #[derive(Debug, PartialEq, Eq)]
    enum WCall {
        Release(u32),
        InstallPair(u32, u32),
        Bind(u32),
        Resolve(u32, u32),
        Attach(u32),
        Refresh(u32, u32),
    }

    struct Fake {
        calls: Vec<WCall>,
        refresh_ans: u32,
        bind_ans: u32,
        resolve_ans: u32,
        install_like: bool,
    }

    impl Fake {
        fn new() -> Self {
            Self {
                calls: Vec::new(),
                refresh_ans: 0,
                bind_ans: 0,
                resolve_ans: 0,
                install_like: false,
            }
        }
    }

    impl TrackerWorld for Fake {
        fn release(&mut self, voice: VoiceHandle) {
            self.calls.push(WCall::Release(voice.get()));
        }
        fn install_pair(&mut self, tracker: &mut VoiceTracker, voice: VoiceHandle, count: u32) {
            self.calls.push(WCall::InstallPair(voice.get(), count));
            if self.install_like {
                tracker.voice = Some(voice);
                tracker.count = count;
            }
        }
        fn bind(&mut self, owner: Option<VoiceHandle>) -> u32 {
            self.calls.push(WCall::Bind(Handle32::raw_or_zero(owner)));
            self.bind_ans
        }
        fn resolve(&mut self, voice: VoiceHandle, arg: u32) -> u32 {
            self.calls.push(WCall::Resolve(voice.get(), arg));
            self.resolve_ans
        }
        fn attach(&mut self, handle: u32) {
            self.calls.push(WCall::Attach(handle));
        }
        fn refresh(&mut self, pool: Option<PoolHandle>, voice: Option<VoiceHandle>) -> u32 {
            self.calls.push(WCall::Refresh(
                Handle32::raw_or_zero(pool),
                Handle32::raw_or_zero(voice),
            ));
            self.refresh_ans
        }
    }

    /// Install-like callee effect for install-and-bind trials: stores the
    /// voice/count pair over the tracker's words, as the install routine
    /// would. Args are `[this, voice, count]`.
    fn install_hook(args: Vec<u32>) {
        let (this, voice, count) = (args[0], args[1], args[2]);
        unsafe {
            std::ptr::write_unaligned(this.wrapping_add(0x30) as *mut u32, voice);
            std::ptr::write_unaligned(this.wrapping_add(0x38) as *mut u32, count);
        }
    }

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_audio::audio_voice::tracker::{TrackerWorld, VoiceHandle, VoiceTracker};

        /// Stamps the count's second byte instead of its low byte.
        pub fn install_hi_tag(
            t: &mut VoiceTracker,
            voice: Option<VoiceHandle>,
            count: u32,
            pool: Option<super::PoolHandle>,
            world: &mut impl TrackerWorld,
        ) -> u32 {
            t.voice = voice;
            t.count = count;
            let Some(slot) = t.link.as_mut() else {
                return count;
            };
            slot.tag = (count >> 8) as u8;
            let fresh = world.refresh(pool, voice);
            slot.cached = fresh;
            fresh
        }

        /// Answers the count even when a record is linked.
        pub fn install_returns_count(
            t: &mut VoiceTracker,
            voice: Option<VoiceHandle>,
            count: u32,
            pool: Option<super::PoolHandle>,
            world: &mut impl TrackerWorld,
        ) -> u32 {
            t.voice = voice;
            t.count = count;
            let Some(slot) = t.link.as_mut() else {
                return count;
            };
            slot.tag = count as u8;
            let fresh = world.refresh(pool, voice);
            slot.cached = fresh;
            count
        }

        /// Releases on any nonzero count, missing the signed comparison.
        pub fn release_nonzero(t: &mut VoiceTracker, world: &mut impl TrackerWorld) -> bool {
            if let Some(voice) = t.voice {
                if t.count != 0 {
                    world.release(voice);
                    t.count = 0;
                }
                t.voice = None;
            }
            let Some(slot) = t.link.as_mut() else {
                return false;
            };
            slot.tag = super::PARK_TAG;
            slot.cached = super::PARK_CACHED;
            true
        }

        /// Parks the tag to 0xFE instead of 0xFF.
        pub fn release_tag_fe(t: &mut VoiceTracker, world: &mut impl TrackerWorld) -> bool {
            if let Some(voice) = t.voice {
                if t.count.cast_signed() > 0 {
                    world.release(voice);
                    t.count = 0;
                }
                t.voice = None;
            }
            let Some(slot) = t.link.as_mut() else {
                return false;
            };
            slot.tag = 0xFE;
            slot.cached = super::PARK_CACHED;
            true
        }

        /// Binds the count word instead of the owner.
        pub fn bind_count(
            t: &mut VoiceTracker,
            voice: Option<VoiceHandle>,
            count: u32,
            world: &mut impl TrackerWorld,
        ) -> u32 {
            let Some(voice) = voice else { return 0 };
            world.install_pair(t, voice, count);
            world.bind(super::Handle32::new(count))
        }

        /// Treats handle 0 as invalid instead of `INVALID_HANDLE`.
        pub fn resolve_zero_invalid(
            t: &mut VoiceTracker,
            arg: u32,
            world: &mut impl TrackerWorld,
        ) -> bool {
            let Some(voice) = t.voice else {
                return false;
            };
            let handle = world.resolve(voice, arg);
            if handle == 0 {
                return false;
            }
            world.attach(handle);
            true
        }
    }

    /// Asserts the 32-bit images equal the lifted tracker.
    fn assert_images(
        trial: u32,
        what: &str,
        fx: &TFixture,
        lift: &VoiceTracker,
        initial_obj: &[u8],
        initial_rec: Option<&[u8]>,
    ) {
        let img = unsafe { image(fx.this(), 0x3C) };
        // Untouched prefix and suffix bytes survive any method.
        assert_eq!(
            &img[..0x30],
            &initial_obj[..0x30],
            "trial {trial} {what}: prefix survives"
        );
        assert_eq!(
            &img[0x34..0x38],
            &initial_obj[0x34..0x38],
            "trial {trial} {what}: link word survives"
        );
        let voice = u32::from_le_bytes(img[0x30..0x34].try_into().unwrap());
        let count = u32::from_le_bytes(img[0x38..0x3C].try_into().unwrap());
        assert_eq!(
            voice,
            Handle32::raw_or_zero(lift.voice),
            "trial {trial} {what}: voice word"
        );
        assert_eq!(count, lift.count, "trial {trial} {what}: count word");
        if let Some(rec) = fx.rec.as_ref() {
            let ri = unsafe { image(addr(&rec[0]), 0x4C) };
            let init = initial_rec.unwrap();
            assert_eq!(
                &ri[..0x40],
                &init[..0x40],
                "trial {trial} {what}: record head survives"
            );
            assert_eq!(
                &ri[0x41..0x48],
                &init[0x41..0x48],
                "trial {trial} {what}: record middle survives"
            );
            let slot = lift.link.expect("linked lift has a slot");
            assert_eq!(ri[0x40], slot.tag, "trial {trial} {what}: tag byte");
            assert_eq!(
                u32::from_le_bytes(ri[0x48..0x4C].try_into().unwrap()),
                slot.cached,
                "trial {trial} {what}: cached word"
            );
        }
    }

    #[test]
    fn install_matches() {
        let _guard = lock();
        let mut rng = Rng(0x15A0);
        let mut caught_tag = 0;
        let mut caught_ret = 0;
        for trial in 0..60u32 {
            rt::clear_hooks();
            let with_link = trial % 2 == 0;
            let voice_raw = if trial % 3 == 0 { 0 } else { rng.u32() | 1 };
            let count = if trial == 0 { 0x1234 } else { rng.u32() };
            let pool_raw = if trial % 5 == 0 { 0 } else { rng.u32() | 1 };
            let refresh_ans = if trial == 0 { 0x77 } else { rng.u32() };
            let planted_voice = rng.u32();
            let planted_count = rng.u32();
            let fx = plant(&mut rng, planted_voice, with_link, planted_count);
            let initial_obj = unsafe { image(fx.this(), 0x3C) }.to_vec();
            let initial_rec = fx
                .rec
                .as_ref()
                .map(|r| unsafe { image(addr(&r[0]), 0x4C) }.to_vec());
            rt::set_global(POOL_VA, pool_raw);
            rt::set_script(&[(1, StubKind::Thiscall2, vec![refresh_ans])]);
            let this = fx.this();
            let mut lift = decode(&fx);
            let pre = lift.clone();
            let ret = unsafe { fn_009E15A0::rw_009e15a0(this as *mut u8, voice_raw, count) };
            let calls = rt::take_calls();
            let expect_calls = if with_link {
                vec![(1, vec![pool_raw, voice_raw])]
            } else {
                vec![]
            };
            assert_eq!(
                calls, expect_calls,
                "trial {trial}: refresh runs iff linked"
            );
            assert_eq!(
                ret,
                if with_link { refresh_ans } else { count },
                "trial {trial}: answer is the refresh or the count"
            );
            assert_eq!(
                rt::get_global(POOL_VA),
                pool_raw,
                "trial {trial}: pool global survives"
            );
            let mut fake = Fake::new();
            fake.refresh_ans = refresh_ans;
            let voice = Handle32::new(voice_raw);
            let pool = Handle32::new(pool_raw);
            let out = lift.install(voice, count, pool, &mut fake);
            assert_eq!(out, ret, "trial {trial}: lifted answer");
            let expect_fake = if with_link {
                vec![WCall::Refresh(pool_raw, voice_raw)]
            } else {
                vec![]
            };
            assert_eq!(fake.calls, expect_fake, "trial {trial}: lifted calls");
            assert_images(
                trial,
                "install",
                &fx,
                &lift,
                &initial_obj,
                initial_rec.as_deref(),
            );
            if with_link {
                let mut w = pre.clone();
                let mut wf = Fake::new();
                wf.refresh_ans = refresh_ans;
                wrong::install_hi_tag(&mut w, voice, count, pool, &mut wf);
                let touched = unsafe { image(fx.link().wrapping_add(0x40), 1)[0] };
                if w.link.unwrap().tag != touched {
                    caught_tag += 1;
                }
                let mut w = pre.clone();
                let mut wf = Fake::new();
                wf.refresh_ans = refresh_ans;
                let wret = wrong::install_returns_count(&mut w, voice, count, pool, &mut wf);
                if wret != ret {
                    caught_ret += 1;
                }
            }
        }
        assert!(caught_tag > 0, "hi-tag mutant was never caught");
        assert!(caught_ret > 0, "returns-count mutant was never caught");
    }

    #[test]
    fn release_and_park_matches() {
        let _guard = lock();
        let mut rng = Rng(0x1320);
        // Counts pinning the signed comparison: zero, positive, negative.
        const COUNTS: [u32; 6] = [0, 1, 5, 0xFFFF_FFFF, 0x8000_0000, 0x7FFF_FFFF];
        let mut caught_cmp = 0;
        let mut caught_tag = 0;
        for trial in 0..60u32 {
            rt::clear_hooks();
            let with_link = trial % 2 == 0;
            let with_voice = trial % 3 != 0;
            let count = COUNTS[trial as usize % COUNTS.len()];
            let planted_voice = if with_voice { rng.u32() | 1 } else { 0 };
            let fx = plant(&mut rng, planted_voice, with_link, count);
            let voice_raw = u32::from_le_bytes(
                unsafe { image(fx.this(), 0x3C) }[0x30..0x34]
                    .try_into()
                    .unwrap(),
            );
            let initial_obj = unsafe { image(fx.this(), 0x3C) }.to_vec();
            let initial_rec = fx
                .rec
                .as_ref()
                .map(|r| unsafe { image(addr(&r[0]), 0x4C) }.to_vec());
            rt::set_script(&[(1, StubKind::Thiscall2, vec![0])]);
            let this = fx.this();
            let mut lift = decode(&fx);
            let pre = lift.clone();
            let ret = unsafe { fn_009E1320::rw_009e1320(this as *mut u8) };
            let calls = rt::take_calls();
            let must_call = with_voice && count.cast_signed() > 0;
            let expect_calls = if must_call {
                vec![(1, vec![voice_raw, this])]
            } else {
                vec![]
            };
            assert_eq!(
                calls, expect_calls,
                "trial {trial}: release runs iff held and count positive"
            );
            assert_eq!(ret, fx.link(), "trial {trial}: answer is the link address");
            let mut fake = Fake::new();
            let out = lift.release_and_park(&mut fake);
            assert_eq!(out, with_link, "trial {trial}: lifted presence");
            assert_eq!(
                u32::from(out) * fx.link(),
                ret,
                "trial {trial}: address rebuilds"
            );
            let expect_fake = if must_call {
                vec![WCall::Release(voice_raw)]
            } else {
                vec![]
            };
            assert_eq!(fake.calls, expect_fake, "trial {trial}: lifted calls");
            assert_images(
                trial,
                "release",
                &fx,
                &lift,
                &initial_obj,
                initial_rec.as_deref(),
            );
            let mut w = pre.clone();
            let mut wf = Fake::new();
            let wout = wrong::release_nonzero(&mut w, &mut wf);
            let w_must_call = with_voice && count != 0;
            if wout != out || wf.calls.len() != usize::from(must_call) || w_must_call != must_call {
                caught_cmp += 1;
            }
            if with_link {
                let mut w = pre.clone();
                let mut wf = Fake::new();
                wrong::release_tag_fe(&mut w, &mut wf);
                let touched = unsafe { image(fx.link().wrapping_add(0x40), 1)[0] };
                if w.link.unwrap().tag != touched {
                    caught_tag += 1;
                }
            }
        }
        assert!(caught_cmp > 0, "nonzero-compare mutant was never caught");
        assert!(caught_tag > 0, "tag-fe mutant was never caught");
    }

    #[test]
    fn install_and_bind_matches() {
        let _guard = lock();
        let mut rng = Rng(0x1540);
        let mut caught = 0;
        for trial in 0..60u32 {
            let install_like = trial % 2 == 0;
            if install_like {
                rt::set_hooks(&[(1, install_hook)]);
            } else {
                rt::clear_hooks();
            }
            let with_voice = trial % 3 != 0;
            let voice_raw = if with_voice { rng.u32() | 1 } else { 0 };
            let count = rng.u32();
            let v0 = if trial == 0 { 0x1111 } else { rng.u32() };
            let c0 = rng.u32();
            let bind_ans = rng.u32();
            if trial == 0 {
                // Pin distinct words so the wrong owner is caught.
            }
            let fx = plant(&mut rng, v0, trial % 4 == 0, c0);
            let initial_obj = unsafe { image(fx.this(), 0x3C) }.to_vec();
            let initial_rec = fx
                .rec
                .as_ref()
                .map(|r| unsafe { image(addr(&r[0]), 0x4C) }.to_vec());
            rt::set_script(&[
                (1, StubKind::Thiscall3, vec![0]),
                (2, StubKind::Thiscall2, vec![bind_ans]),
            ]);
            let this = fx.this();
            let mut lift = decode(&fx);
            let pre = lift.clone();
            let ret = unsafe { fn_009E1540::rw_009e1540(this as *mut u8, voice_raw, count) };
            let calls = rt::take_calls();
            if !with_voice {
                assert_eq!(calls, vec![], "trial {trial}: no voice, no calls");
                assert_eq!(ret, 0, "trial {trial}: no voice, zero answer");
            } else {
                let owner = if install_like { voice_raw } else { v0 };
                assert_eq!(
                    calls,
                    vec![(1, vec![this, voice_raw, count]), (2, vec![owner, this])],
                    "trial {trial}: install then bind of the read-back owner"
                );
                assert_eq!(ret, bind_ans, "trial {trial}: answer is the bind");
            }
            let mut fake = Fake::new();
            fake.bind_ans = bind_ans;
            fake.install_like = install_like;
            let voice = Handle32::new(voice_raw);
            let out = lift.install_and_bind(voice, count, &mut fake);
            assert_eq!(out, ret, "trial {trial}: lifted answer");
            let expect_fake = if !with_voice {
                vec![]
            } else {
                let owner = if install_like { voice_raw } else { v0 };
                vec![WCall::InstallPair(voice_raw, count), WCall::Bind(owner)]
            };
            assert_eq!(fake.calls, expect_fake, "trial {trial}: lifted calls");
            assert_images(
                trial,
                "bind",
                &fx,
                &lift,
                &initial_obj,
                initial_rec.as_deref(),
            );
            if with_voice {
                let mut w = pre.clone();
                let mut wf = Fake::new();
                wf.bind_ans = bind_ans;
                wf.install_like = install_like;
                wrong::bind_count(&mut w, voice, count, &mut wf);
                let owner = if install_like { voice_raw } else { v0 };
                if wf.calls != vec![WCall::InstallPair(voice_raw, count), WCall::Bind(owner)] {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "bind-count mutant was never caught");
    }

    #[test]
    fn handle_resolve_matches() {
        let _guard = lock();
        let mut rng = Rng(0x1570);
        let mut caught = 0;
        for trial in 0..60u32 {
            rt::clear_hooks();
            let with_voice = trial % 4 != 0;
            let h = match trial % 3 {
                0 => INVALID_HANDLE,
                1 => 0,
                _ => {
                    let h = rng.u32();
                    if h == INVALID_HANDLE { 1 } else { h }
                }
            };
            let arg = rng.u32();
            let planted_voice = if with_voice { rng.u32() | 1 } else { 0 };
            let planted_count = rng.u32();
            let fx = plant(&mut rng, planted_voice, trial % 5 == 0, planted_count);
            let voice_raw = u32::from_le_bytes(
                unsafe { image(fx.this(), 0x3C) }[0x30..0x34]
                    .try_into()
                    .unwrap(),
            );
            let initial_obj = unsafe { image(fx.this(), 0x3C) }.to_vec();
            let initial_rec = fx
                .rec
                .as_ref()
                .map(|r| unsafe { image(addr(&r[0]), 0x4C) }.to_vec());
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![h]),
                (2, StubKind::Thiscall2, vec![0]),
            ]);
            let this = fx.this();
            let mut lift = decode(&fx);
            let pre = lift.clone();
            let ret = unsafe { fn_009E1570::rw_009e1570(this as *mut u8, arg) };
            let calls = rt::take_calls();
            let ok = with_voice && h != INVALID_HANDLE;
            let expect_calls = if !with_voice {
                vec![]
            } else if h == INVALID_HANDLE {
                vec![(1, vec![voice_raw, arg])]
            } else {
                vec![(1, vec![voice_raw, arg]), (2, vec![this, h])]
            };
            assert_eq!(
                calls, expect_calls,
                "trial {trial}: resolve then maybe attach"
            );
            assert_eq!(ret, u32::from(ok), "trial {trial}: 1 iff attached");
            let mut fake = Fake::new();
            fake.resolve_ans = h;
            let out = lift.handle_resolve(arg, &mut fake);
            assert_eq!(out, ok, "trial {trial}: lifted answer");
            let expect_fake = if !with_voice {
                vec![]
            } else if h == INVALID_HANDLE {
                vec![WCall::Resolve(voice_raw, arg)]
            } else {
                vec![WCall::Resolve(voice_raw, arg), WCall::Attach(h)]
            };
            assert_eq!(fake.calls, expect_fake, "trial {trial}: lifted calls");
            assert_images(
                trial,
                "resolve",
                &fx,
                &lift,
                &initial_obj,
                initial_rec.as_deref(),
            );
            if with_voice {
                let mut w = pre.clone();
                let mut wf = Fake::new();
                wf.resolve_ans = h;
                let wout = wrong::resolve_zero_invalid(&mut w, arg, &mut wf);
                if wout != out || wf.calls != expect_fake {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "zero-invalid mutant was never caught");
    }
}
