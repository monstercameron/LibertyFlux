//! IMA 4-bit ADPCM decoding shared by banks and streamed files.
//!
//! Two samples per byte, low nibble first, with the standard IMA step table
//! and index adjustments. [`decode_mono`] decodes one sound from a fresh
//! state; [`ImaState`] carries the predictor across streamed blocks during
//! linear playback.

use crate::{Error, ErrorKind};

/// Step sizes indexed by the decoder state (0-88).
const STEP: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2500, 2750, 3025, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];

/// Index adjustment per nibble value.
const INDEX_ADJ: [i32; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];

/// Highest decoder state index into [`STEP`].
const MAX_INDEX: i32 = 88;

/// A stateful IMA ADPCM decoder: the 16-bit predictor and the step index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImaState {
    predictor: i32,
    index: i32,
}

impl ImaState {
    /// A fresh decoder: silent predictor, smallest step.
    #[must_use]
    pub fn new() -> ImaState {
        ImaState {
            predictor: 0,
            index: 0,
        }
    }

    /// Decode one 4-bit nibble to a 16-bit sample.
    fn nibble(&mut self, n: u8) -> i16 {
        let step = STEP[usize::try_from(self.index).unwrap_or(0)];
        let mut diff = step >> 3;
        if n & 4 != 0 {
            diff += step;
        }
        if n & 2 != 0 {
            diff += step >> 1;
        }
        if n & 1 != 0 {
            diff += step >> 2;
        }
        if n & 8 != 0 {
            self.predictor -= diff;
        } else {
            self.predictor += diff;
        }
        self.predictor = self
            .predictor
            .clamp(i32::from(i16::MIN), i32::from(i16::MAX));
        self.index = (self.index + INDEX_ADJ[usize::from(n)]).clamp(0, MAX_INDEX);
        // The clamp above keeps the predictor inside the i16 range, so this
        // conversion cannot fail; the fallback is unreachable.
        i16::try_from(self.predictor).unwrap_or(0)
    }

    /// Decode `want` samples from `data` (low nibble first), appending to
    /// `out`. The state advances, so repeated calls chain seamlessly.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::Truncated`] when `data` holds fewer than `want`
    /// nibbles; nothing is appended then.
    ///
    /// # Panics
    ///
    /// Never panics: the length check up front keeps every byte read in range.
    pub fn decode_bytes(
        &mut self,
        data: &[u8],
        want: usize,
        out: &mut Vec<i16>,
    ) -> Result<(), Error> {
        if want > data.len().saturating_mul(2) {
            return Err(Error::new(
                0,
                ErrorKind::Truncated,
                format!(
                    "need {want} nibbles, input holds {}",
                    data.len().saturating_mul(2)
                ),
            ));
        }
        out.reserve(want);
        // Decode low nibble first; `want` was checked against the capacity.
        let mut done = 0usize;
        for byte in data {
            for shift in [0, 4] {
                if done >= want {
                    return Ok(());
                }
                done += 1;
                out.push(self.nibble((byte >> shift) & 0x0F));
            }
        }
        Ok(())
    }
}

impl Default for ImaState {
    fn default() -> ImaState {
        ImaState::new()
    }
}

/// Decode `want` mono samples from `data` starting from a fresh state.
///
/// # Errors
///
/// Returns [`ErrorKind::Truncated`] when `data` holds fewer than `want`
/// nibbles.
///
/// # Panics
///
/// Never panics: the length check keeps every byte read in range.
pub fn decode_mono(data: &[u8], want: usize) -> Result<Vec<i16>, Error> {
    let mut out = Vec::new();
    ImaState::new().decode_bytes(data, want, &mut out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_state_is_silent() {
        let state = ImaState::new();
        assert_eq!(state, ImaState::default());
        assert_eq!(decode_mono(&[0x00], 2).unwrap(), vec![0, 0]);
    }

    #[test]
    fn low_nibble_comes_first() {
        // High nibble 0, low nibble 7: first sample climbs, second rests.
        let pcm = decode_mono(&[0x07], 2).unwrap();
        assert_eq!(pcm[0], 11);
        assert_eq!(pcm[1], 11 + (16 >> 3));
    }

    #[test]
    fn short_input_fails_without_appending() {
        let mut out = vec![1i16];
        let mut state = ImaState::new();
        assert_eq!(
            state.decode_bytes(&[0x77], 3, &mut out).unwrap_err().kind(),
            ErrorKind::Truncated
        );
        assert_eq!(out, vec![1i16]);
    }

    #[test]
    fn state_chains_across_calls() {
        let mut a = Vec::new();
        let mut s = ImaState::new();
        s.decode_bytes(&[0x77], 2, &mut a).unwrap();
        s.decode_bytes(&[0x77], 2, &mut a).unwrap();
        assert_eq!(a, decode_mono(&[0x77, 0x77], 4).unwrap());
    }
}
