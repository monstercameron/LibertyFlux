//! The audio entity record's playback gates.
//!
//! An entity record carries an id word, two mute bytes and a flag dword.
//! Two verified routines gate it: the audibility check (mute bytes must
//! be clear, id must match) and the activity check (no mute test, with
//! extra current-id overrides). Both read the same five playback words,
//! which travel as one [`PlaybackState`] argument.

/// The playback words both gates read: the mode, the current id, and
/// the three alternate ids. In the 32-bit form these are five globals;
/// the lift takes them as an explicit argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaybackState {
    /// Playback mode; the gates pass only for 1 or 4.
    pub mode: u32,
    /// Current entity id, or an override (-1/-2 match anything in one
    /// gate each; -3 takes the flag path in the activity gate).
    pub current: i32,
    /// Alternate ids accepted beside the current one.
    pub alternates: [i32; 3],
}

/// The entity fields the gates read: the id word at record offset
/// `0x2E`, the mute bytes at `0x218`/`0x219`, and the flag dword at
/// `0x1304`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityRecord {
    /// Entity id, compared sign-extended against the playback ids.
    pub id: i16,
    /// Mute bytes; both must be clear for the audibility gate.
    pub mute: [u8; 2],
    /// Flag dword; the activity gate accepts 1 under override -3.
    pub flag: u32,
}

impl EntityRecord {
    /// Audibility gate: true when the entity may sound.
    ///
    /// Requires playback mode 1 or 4. Unless the current-id override
    /// is -1 (match anything), the id must equal the current id or one
    /// of the three alternates. Both mute bytes must be clear.
    #[must_use]
    pub fn is_audible(&self, playback: &PlaybackState) -> bool {
        if playback.mode != 1 && playback.mode != 4 {
            return false;
        }
        if playback.current != -1 {
            let id = i32::from(self.id);
            if id != playback.current
                && id != playback.alternates[0]
                && id != playback.alternates[1]
                && id != playback.alternates[2]
            {
                return false;
            }
        }
        if self.mute[0] != 0 {
            return false;
        }
        if self.mute[1] != 0 {
            return false;
        }
        true
    }

    /// Activity gate: true when the entity counts as active.
    ///
    /// Requires playback mode 1 or 4. A current-id override of -2
    /// matches anything. Otherwise the id must equal the current id or
    /// one of the three alternates; failing that, an override of -3
    /// accepts entities whose flag dword equals 1.
    #[must_use]
    pub fn is_active(&self, playback: &PlaybackState) -> bool {
        if playback.mode != 1 && playback.mode != 4 {
            return false;
        }
        if playback.current == -2 {
            return true;
        }
        let id = i32::from(self.id);
        if id == playback.current
            || id == playback.alternates[0]
            || id == playback.alternates[1]
            || id == playback.alternates[2]
        {
            return true;
        }
        if playback.current == -3 && self.flag == 1 {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn playback(mode: u32, current: i32) -> PlaybackState {
        PlaybackState {
            mode,
            current,
            alternates: [100, 200, 300],
        }
    }

    fn record(id: i16) -> EntityRecord {
        EntityRecord {
            id,
            mute: [0, 0],
            flag: 0,
        }
    }

    #[test]
    fn audible_needs_mode_one_or_four() {
        let rec = record(7);
        for mode in [0, 1, 2, 3, 4, 5, 0xFFFF_FFFF] {
            let pb = PlaybackState {
                mode,
                current: 7,
                alternates: [0, 0, 0],
            };
            assert_eq!(
                rec.is_audible(&pb),
                mode == 1 || mode == 4,
                "mode {mode}"
            );
        }
    }

    #[test]
    fn audible_override_minus_one_matches_anything() {
        let pb = playback(1, -1);
        assert!(record(0).is_audible(&pb));
        assert!(record(i16::MIN).is_audible(&pb));
        assert!(record(i16::MAX).is_audible(&pb));
    }

    #[test]
    fn audible_id_matches_current_and_each_alternate() {
        let rec = record(200);
        assert!(rec.is_audible(&playback(4, 200)));
        assert!(rec.is_audible(&playback(4, 9)));
        assert!(!record(201).is_audible(&playback(4, 9)));
        // Third alternate position.
        assert!(record(300).is_audible(&playback(1, 9)));
        assert!(!record(301).is_audible(&playback(1, 9)));
    }

    #[test]
    fn audible_id_sign_extends() {
        // A negative id word matches only the same negative id.
        let pb = playback(1, -5);
        assert!(record(-5).is_audible(&pb));
        assert!(!record(5).is_audible(&pb));
    }

    #[test]
    fn audible_either_mute_byte_blocks() {
        let pb = playback(1, 7);
        let mut rec = record(7);
        assert!(rec.is_audible(&pb));
        rec.mute[0] = 1;
        assert!(!rec.is_audible(&pb));
        rec.mute = [0, 0xFF];
        assert!(!rec.is_audible(&pb));
    }

    #[test]
    fn active_override_minus_two_matches_anything() {
        let pb = playback(1, -2);
        assert!(record(0).is_active(&pb));
        // Mute bytes are not consulted here.
        let loud = EntityRecord {
            id: 5,
            mute: [1, 1],
            flag: 0,
        };
        assert!(loud.is_active(&pb));
        // Mode still gates.
        assert!(!record(0).is_active(&playback(0, -2)));
    }

    #[test]
    fn active_flag_path_needs_override_minus_three_and_flag_one() {
        let mut rec = record(999);
        assert!(!rec.is_active(&playback(1, -3)));
        rec.flag = 1;
        assert!(rec.is_active(&playback(1, -3)));
        rec.flag = 2;
        assert!(!rec.is_active(&playback(1, -3)));
        // Flag 1 without the override does not help.
        rec.flag = 1;
        assert!(!rec.is_active(&playback(1, 5)));
    }

    #[test]
    fn active_id_match_positions() {
        assert!(record(100).is_active(&playback(1, 5)));
        assert!(record(300).is_active(&playback(1, 5)));
        assert!(!record(301).is_active(&playback(1, 5)));
        assert!(record(i16::MIN).is_active(&playback(4, i32::from(i16::MIN))));
    }
}
