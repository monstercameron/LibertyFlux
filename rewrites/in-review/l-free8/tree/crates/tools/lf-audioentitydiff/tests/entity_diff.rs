//! Differential cases: the lifted audio-entity group against its rewrites.
//!
//! Each case builds the 32-bit object, runs the rewrite and the lifted
//! method on the same inputs, and compares the answer and every written
//! byte. Each method has a deliberately wrong lift that must be caught.
//! 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_audio::audio_entity::events::{EventRecord, EventTable};
    use lf_audio::audio_entity::lifecycle::{AudioEntity, BaseInit, LatchSlot, MemberInit};
    use lf_audio::audio_entity::record::{EntityRecord, PlaybackState};
    use lf_audioentitydiff::rewrites::fn_0088AC00::rw_0088ac00;
    use lf_audioentitydiff::rewrites::fn_009A3600::rw_009A3600;
    use lf_audioentitydiff::rewrites::fn_009A38A0::rw_009a38a0;
    use lf_audioentitydiff::rewrites::fn_00D8C510::rw_00d8c510;
    use lf_audioentitydiff::rewrites::fn_00D8C570::rw_00d8c570;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        ALT_VAS, CUR_VA, LATCH_CFG_VA, MODE_VA, TABLE_INNER_VA, TABLE_MAIN_VA, Rng, addr, get_u32,
        lock, put_u16, put_u32,
    };

    /// Record offsets the gate rewrites read: id word, mute bytes, flag.
    const ID_OFF: usize = 0x2E;
    const MUTE_OFF: usize = 0x218;
    const FLAG_OFF: usize = 0x1304;
    /// Entity image: flag dword plus one word of margin.
    const ENT_LEN: usize = 0x1308;
    /// Event row stride and count offset, as the append rewrite holds them.
    const ROW_STRIDE: usize = 0x6F40;
    const COUNT_OFF: usize = 0x6F0C;
    /// Record slot field offsets within a row (slot = count * 40).
    const PARAM_OFF: usize = 0x6CE0;
    const FLAG0_OFF: usize = 0x6CE4;
    const FLAG1_OFF: usize = 0x6CE5;
    const SMALL_OFF: usize = 0x6CC0;
    const BLOB_OFF: usize = 0x6CC8;
    const SLOT_LEN: usize = 40;

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_audio::audio_entity::events::{EventTable, MAX_RECORDS};
        use lf_audio::audio_entity::lifecycle::LatchSlot;
        use lf_audio::audio_entity::record::{EntityRecord, PlaybackState};

        /// Audibility gate that only accepts mode 1, forgetting mode 4.
        pub fn audible_mode1_only(record: &EntityRecord, playback: &PlaybackState) -> bool {
            if playback.mode != 1 {
                return false;
            }
            if playback.current != -1 {
                let id = i32::from(record.id);
                if id != playback.current
                    && id != playback.alternates[0]
                    && id != playback.alternates[1]
                    && id != playback.alternates[2]
                {
                    return false;
                }
            }
            record.mute[0] == 0 && record.mute[1] == 0
        }

        /// Activity gate that forgets the -2 match-anything override.
        pub fn active_no_skip(record: &EntityRecord, playback: &PlaybackState) -> bool {
            if playback.mode != 1 && playback.mode != 4 {
                return false;
            }
            let id = i32::from(record.id);
            if id == playback.current
                || id == playback.alternates[0]
                || id == playback.alternates[1]
                || id == playback.alternates[2]
            {
                return true;
            }
            playback.current == -3 && record.flag == 1
        }

        /// Append that caps the row at eleven records instead of twelve.
        pub fn append_cap11(
            table: &mut EventTable,
            key: u32,
            record: super::EventRecord,
        ) -> bool {
            let row = table.rows.get_mut(key as usize).expect("owned row");
            if row.records.len() + 1 >= MAX_RECORDS {
                return false;
            }
            row.records.push(record);
            true
        }

        /// Latch that overwrites the slot on every call.
        pub fn latch_always(slot: &mut LatchSlot, config: u32) {
            slot.slot = config;
            slot.latched = true;
        }
    }

    /// Plants the five playback globals from a lift state.
    fn plant_playback(pb: &PlaybackState) {
        unsafe {
            lf_audioentitydiff::global::<u32>(MODE_VA).write(pb.mode);
            lf_audioentitydiff::global::<i32>(CUR_VA).write(pb.current);
            for (i, va) in ALT_VAS.iter().enumerate() {
                lf_audioentitydiff::global::<i32>(*va).write(pb.alternates[i]);
            }
        }
    }

    /// Random playback state with edge-seeking values mixed in.
    ///
    /// Mode and current come from the RNG, not from the trial index: two
    /// index-driven cycles can correlate and never cover a combination
    /// the wrong lift needs (first run: no mode-4 pass ever appeared).
    fn gen_playback(rng: &mut Rng, trial: u32) -> PlaybackState {
        // Trial 0 pins a passing mode-4 case for the wrong lift.
        if trial == 0 {
            return PlaybackState {
                mode: 4,
                current: -1,
                alternates: [10, 20, 30],
            };
        }
        // Trial 1 pins a -2 override case for the wrong lift.
        if trial == 1 {
            return PlaybackState {
                mode: 1,
                current: -2,
                alternates: [10, 20, 30],
            };
        }
        let modes = [0, 1, 2, 4, 5, 0xFFFF_FFFF];
        let currents = [-3, -2, -1, 0, 1, 7, 200, 0x7FFF_FFFF, i32::MIN];
        PlaybackState {
            mode: modes[rng.below(modes.len() as u32) as usize],
            current: currents[rng.below(currents.len() as u32) as usize],
            alternates: [
                currents[rng.below(currents.len() as u32) as usize],
                rng.u32() as i32,
                currents[rng.below(currents.len() as u32) as usize],
            ],
        }
    }

    /// Random entity record with edge-seeking values mixed in.
    fn gen_record(rng: &mut Rng, trial: u32) -> EntityRecord {
        // Trial 0 pins clear mutes (with the pinned mode-4 playback the
        // wrong audible lift must diverge here); trial 1 pins a
        // non-matching id (with the pinned -2 playback the wrong active
        // lift must diverge here).
        if trial == 0 {
            return EntityRecord {
                id: 7,
                mute: [0, 0],
                flag: 0,
            };
        }
        if trial == 1 {
            return EntityRecord {
                id: 999,
                mute: [0, 0],
                flag: 0,
            };
        }
        let ids = [0, 1, 7, 200, -1, -5, i16::MIN, i16::MAX];
        let mutes = [0, 0, 1, 0xFF];
        let flags = [0, 1, 2, 0xFFFF_FFFF];
        EntityRecord {
            id: if trial % 3 == 0 {
                rng.u32() as i16
            } else {
                ids[trial as usize % ids.len()]
            },
            mute: [
                mutes[rng.below(mutes.len() as u32) as usize],
                mutes[rng.below(mutes.len() as u32) as usize],
            ],
            flag: flags[trial as usize % flags.len()],
        }
    }

    #[test]
    fn audible_gate_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA11CE);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..300 {
            let pb = gen_playback(&mut rng, trial);
            let rec = gen_record(&mut rng, trial);
            plant_playback(&pb);
            let mut image = vec![0u8; ENT_LEN];
            put_u16(&mut image, ID_OFF, rec.id as u16);
            image[MUTE_OFF] = rec.mute[0];
            image[MUTE_OFF + 1] = rec.mute[1];
            put_u32(&mut image, FLAG_OFF, rec.flag);
            let before = image.clone();
            let ent = addr(&image[0]) as *const u8;
            let got = unsafe { rw_00d8c510(ent) };
            std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
            let want = u32::from(rec.is_audible(&pb));
            assert_eq!(got, want, "trial {trial}: record {rec:?} playback {pb:?}");
            assert_eq!(image, before, "trial {trial}: gate must not write");
            if u32::from(wrong::audible_mode1_only(&rec, &pb)) != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong lift never diverged");
        assert_eq!(cases, 300);
    }

    #[test]
    fn active_gate_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAC71E);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..300 {
            let pb = gen_playback(&mut rng, trial);
            let rec = gen_record(&mut rng, trial);
            plant_playback(&pb);
            let mut image = vec![0u8; ENT_LEN];
            put_u16(&mut image, ID_OFF, rec.id as u16);
            image[MUTE_OFF] = rec.mute[0];
            image[MUTE_OFF + 1] = rec.mute[1];
            put_u32(&mut image, FLAG_OFF, rec.flag);
            let before = image.clone();
            let ent = addr(&image[0]) as *const u8;
            let got = unsafe { rw_00d8c570(ent) };
            std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
            let want = u32::from(rec.is_active(&pb));
            assert_eq!(got, want, "trial {trial}: record {rec:?} playback {pb:?}");
            assert_eq!(image, before, "trial {trial}: gate must not write");
            if u32::from(wrong::active_no_skip(&rec, &pb)) != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong lift never diverged");
        assert_eq!(cases, 300);
    }

    /// Random event record.
    fn gen_event(rng: &mut Rng) -> EventRecord {
        let mut small = [0u8; 6];
        let mut blob = [0u8; 24];
        rng.bytes(&mut small);
        rng.bytes(&mut blob);
        EventRecord {
            param: rng.u32(),
            flag0: rng.u32() as u8,
            flag1: rng.u32() as u8,
            small,
            blob,
        }
    }

    #[test]
    fn event_append_matches() {
        let _guard = lock();
        const ROWS: u32 = 3;
        let mut rng = Rng(0xE7AB1E);
        let mut cases = 0;
        let mut caught = 0;
        // Count seeds: edges around the cap plus random.
        let counts = [0u8, 1, 2, 11, 12, 13, 255];
        for trial in 0..120 {
            let key = rng.below(ROWS);
            let count = if trial < counts.len() as u32 {
                counts[trial as usize]
            } else {
                rng.u32() as u8
            };
            let record = gen_event(&mut rng);
            // Lift table: full rows wherever the count is at the cap.
            let mut table = EventTable::with_rows(ROWS as usize);
            let fill = usize::from(count.min(12));
            for _ in 0..fill {
                table.rows[key as usize].records.push(gen_event(&mut rng));
            }
            // 32-bit table image with the count byte planted.
            let mut image = vec![0u8; ROWS as usize * ROW_STRIDE];
            let row_base = key as usize * ROW_STRIDE;
            image[row_base + COUNT_OFF] = count;
            let before = image.clone();
            let mut this = vec![0u8; 0x100];
            put_u32(&mut this, 0xE8, addr(&image[0]));
            let mut src6 = [0u8; 6];
            let mut src24 = [0u8; 24];
            src6.copy_from_slice(&record.small);
            src24.copy_from_slice(&record.blob);
            let got = unsafe {
                rw_0088ac00(
                    addr(&this[0]) as *const u8,
                    key,
                    u32::from(record.flag0),
                    record.param,
                    addr(&src6[0]) as *const u8,
                    u32::from(record.flag1),
                    addr(&src24[0]) as *const u8,
                )
            };
            std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
            assert_eq!(got, 0, "trial {trial}: append always answers 0");
            let appended = table.append(key, record.clone());
            assert_eq!(
                appended,
                count < 12,
                "trial {trial}: lift disagrees on the cap"
            );
            // Count byte: bumped on append, untouched when full. Over-full
            // rows (count above the cap, which the lift represents as a
            // full row of twelve) keep their count; both sides refuse.
            let new_count = image[row_base + COUNT_OFF];
            let lift_len = table.rows[key as usize].records.len() as u8;
            if count < 12 {
                assert_eq!(new_count, count + 1, "trial {trial}: bumped");
                assert_eq!(lift_len, count + 1, "trial {trial}: lift length");
            } else {
                assert_eq!(new_count, count, "trial {trial}: full row kept");
                assert_eq!(lift_len, 12, "trial {trial}: lift full");
            }
            if appended {
                // Slot bytes decode to the appended record.
                let slot = usize::from(count) * SLOT_LEN;
                assert_eq!(
                    get_u32(&image, row_base + slot + PARAM_OFF),
                    record.param,
                    "trial {trial}: param"
                );
                assert_eq!(
                    image[row_base + slot + FLAG0_OFF],
                    record.flag0,
                    "trial {trial}: flag0"
                );
                assert_eq!(
                    image[row_base + slot + FLAG1_OFF],
                    record.flag1,
                    "trial {trial}: flag1"
                );
                assert_eq!(
                    &image[row_base + slot + SMALL_OFF..row_base + slot + SMALL_OFF + 6],
                    &record.small,
                    "trial {trial}: small"
                );
                assert_eq!(
                    &image[row_base + slot + BLOB_OFF..row_base + slot + BLOB_OFF + 24],
                    &record.blob,
                    "trial {trial}: blob"
                );
            }
            // Everything else is untouched.
            for (i, (&after, &was)) in image.iter().zip(before.iter()).enumerate() {
                let in_slot = appended
                    && i >= row_base + usize::from(count) * SLOT_LEN + SMALL_OFF
                    && i < row_base + usize::from(count) * SLOT_LEN + FLAG1_OFF + 1;
                let is_count = i == row_base + COUNT_OFF;
                if !in_slot && !is_count {
                    assert_eq!(after, was, "trial {trial}: byte {i:#x} changed");
                }
            }
            // Wrong lift: capped at eleven.
            let mut table_wrong = EventTable::with_rows(ROWS as usize);
            for _ in 0..fill {
                table_wrong.rows[key as usize].records.push(gen_event(&mut rng));
            }
            if wrong::append_cap11(&mut table_wrong, key, record) != appended {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong lift never diverged");
        assert_eq!(cases, 120);
    }

    #[test]
    fn latch_matches() {
        let _guard = lock();
        let mut rng = Rng(0x1A7C4);
        let mut cases = 0;
        let mut caught = 0;
        let flag_seeds = [0u8, 0, 1, 2, 0xFF];
        for trial in 0..120 {
            let flag = if (trial as usize) < flag_seeds.len() {
                flag_seeds[trial as usize]
            } else if trial % 2 == 0 {
                0
            } else {
                rng.u32() as u8 | 1
            };
            let initial = rng.u32();
            let config = rng.u32();
            let config_box = Box::new(config);
            lf_audioentitydiff::set_relocated(LATCH_CFG_VA, addr(&*config_box));
            let mut image = vec![0u8; 0x90];
            put_u32(&mut image, 0x18, initial);
            image[0x89] = flag;
            let this = addr(&image[0]);
            let got = unsafe { rw_009a38a0(this) };
            std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
            assert_eq!(got, 0, "trial {trial}: latch answers 0");
            let mut slot = LatchSlot {
                slot: initial,
                latched: flag != 0,
            };
            slot.latch(config);
            assert_eq!(
                get_u32(&image, 0x18),
                slot.slot,
                "trial {trial}: slot word"
            );
            assert_eq!(image[0x89], 1, "trial {trial}: flag set");
            assert!(slot.latched, "trial {trial}: lift latched");
            let mut bad = LatchSlot {
                slot: initial,
                latched: flag != 0,
            };
            wrong::latch_always(&mut bad, config);
            if bad.slot != slot.slot {
                caught += 1;
            }
            cases += 1;
            drop(config_box);
        }
        assert!(caught > 0, "wrong lift never diverged");
        assert_eq!(cases, 120);
    }

    /// Call logs for the constructor's two callee stubs.
    static BASE_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static MEMBER_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());

    extern "thiscall" fn base_stub(this: u32) -> u32 {
        BASE_LOG.lock().unwrap().push(this);
        0
    }

    extern "thiscall" fn member_stub(member: u32) -> u32 {
        MEMBER_LOG.lock().unwrap().push(member);
        0
    }

    /// Recording lift fakes sharing one order log.
    struct Order<'a> {
        log: &'a Mutex<Vec<&'static str>>,
        next: u32,
    }

    impl BaseInit for Order<'_> {
        fn init_base(&mut self) {
            self.log.lock().unwrap().push("base");
        }
    }

    impl MemberInit<u32> for Order<'_> {
        fn init_member(&mut self) -> u32 {
            let tag = if self.next == 0 { "a" } else { "b" };
            self.log.lock().unwrap().push(tag);
            let value = self.next;
            self.next += 1;
            value
        }
    }

    /// Wrong lift constructor: builds member B from the first call and
    /// member A from the second, swapping the two members.
    fn construct_swapped(
        base: &mut Order<'_>,
        members: &mut Order<'_>,
    ) -> AudioEntity<u32> {
        base.init_base();
        let member_b = members.init_member();
        let member_a = members.init_member();
        AudioEntity {
            state: 0,
            member_a,
            member_b,
        }
    }

    #[test]
    fn construct_matches() {
        let _guard = lock();
        // Distinct marker table words, planted per case.
        const MAIN_MARK: u32 = 0x1111_1111;
        const INNER_MARK: u32 = 0x2222_2222;
        lf_audioentitydiff::set_relocated(TABLE_MAIN_VA, MAIN_MARK);
        lf_audioentitydiff::set_relocated(TABLE_INNER_VA, INNER_MARK);
        lf_audioentitydiff::set_callee(1, base_stub as usize as u32);
        lf_audioentitydiff::set_callee(2, member_stub as usize as u32);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..60 {
            BASE_LOG.lock().unwrap().clear();
            MEMBER_LOG.lock().unwrap().clear();
            // Vary the untouched fill so stray writes show.
            let fill = (trial as u8).wrapping_mul(0x1F);
            let mut image = vec![fill; 0x70];
            let this = addr(&image[0]);
            let got = unsafe { rw_009A3600(this) };
            std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
            assert_eq!(got, this, "trial {trial}: construct answers this");
            // Table words and cleared state.
            assert_eq!(get_u32(&image, 0), MAIN_MARK, "trial {trial}: main table");
            assert_eq!(get_u32(&image, 8), INNER_MARK, "trial {trial}: inner table");
            assert_eq!(get_u32(&image, 0x0C), 0, "trial {trial}: state cleared");
            // Callee logs: base once with this, members at +0x30/+0x4C.
            assert_eq!(
                *BASE_LOG.lock().unwrap(),
                [this],
                "trial {trial}: base calls"
            );
            assert_eq!(
                *MEMBER_LOG.lock().unwrap(),
                [this.wrapping_add(0x30), this.wrapping_add(0x4C)],
                "trial {trial}: member calls"
            );
            // Everything else untouched.
            for (i, &b) in image.iter().enumerate() {
                let touched = (0..4).contains(&i) || (8..16).contains(&i);
                if !touched {
                    assert_eq!(b, fill, "trial {trial}: byte {i:#x} changed");
                }
            }
            // Lift side: same call order, rebuilt addresses compared.
            let log = Mutex::new(Vec::new());
            let mut base = Order { log: &log, next: 0 };
            let mut members = Order { log: &log, next: 0 };
            let entity = AudioEntity::new(&mut base, &mut members);
            assert_eq!(entity.state, 0);
            assert_eq!((entity.member_a, entity.member_b), (0, 1));
            let order = log.lock().unwrap().clone();
            assert_eq!(order, ["base", "a", "b"], "trial {trial}: lift order");
            // Rebuild the member addresses from the lift order and compare.
            let member_calls = MEMBER_LOG.lock().unwrap().clone();
            for (slot, call) in member_calls.iter().enumerate() {
                let want = this.wrapping_add([0x30, 0x4C][slot]);
                assert_eq!(*call, want, "trial {trial}: member slot {slot}");
            }
            // Wrong lift: swapped members diverge in order and values.
            let log_bad = Mutex::new(Vec::new());
            let mut base_bad = Order {
                log: &log_bad,
                next: 0,
            };
            let mut members_bad = Order {
                log: &log_bad,
                next: 0,
            };
            let bad = construct_swapped(&mut base_bad, &mut members_bad);
            if (bad.member_a, bad.member_b) != (entity.member_a, entity.member_b) {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong lift never diverged");
        assert_eq!(cases, 60);
    }
}
