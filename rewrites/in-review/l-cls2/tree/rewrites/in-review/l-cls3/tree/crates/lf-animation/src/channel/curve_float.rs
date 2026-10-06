//! A curve-float channel: polynomial segments between keys.
//!
//! Lifted from the verified rewrites of `crAnimChannelCurveFloat`. Each
//! key holds a position, a segment order and that many plus one
//! coefficients; sampling finds the bracketing segment, evaluates its
//! polynomial at the local time and scales the result. The key-list
//! management (allocators, relocators, stream serializers) is not
//! lifted: construction and destruction cover it.

use super::frame::truncate_to_i32;

/// Matches the 32-bit header: the key count is a 16-bit word.
const MAX_KEYS: usize = 0xFFFF;

/// Bytes of 32-bit header ahead of the key segments.
const BASE_SIZE: u32 = 0x18;

/// Bytes per key segment past its coefficients.
const SEGMENT_TAIL: u32 = 0x0C;

/// Adds in the original's operand order: a not-a-number payload can tell
/// the operands apart, so the order is pinned exactly like the verified
/// rewrites pin it.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Multiplies in the original's operand order (see [`fadd`]).
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Subtracts in the original's operand order (see [`fadd`]).
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// One curve key: the key position, the segment order, and the `order +
/// 1` coefficients of its polynomial in Horner order (highest power
/// first).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurveKey {
    /// Key position.
    key: u16,
    /// Segment order (polynomial degree).
    order: u8,
    /// The `order + 1` coefficients, highest power first.
    coeff: Vec<f32>,
}

impl CurveKey {
    /// A key at `key` of degree `order` with `coeff` coefficients.
    ///
    /// # Panics
    ///
    /// When `coeff` holds fewer than `order + 1` entries: every path
    /// through the segment reads that many.
    #[must_use]
    pub fn new(key: u16, order: u8, coeff: Vec<f32>) -> Self {
        assert!(
            (order as usize) + 1 <= coeff.len(),
            "segment needs order + 1 coefficients"
        );
        Self { key, order, coeff }
    }

    /// Key position.
    #[must_use]
    pub fn key(&self) -> u16 {
        self.key
    }

    /// Segment order (polynomial degree).
    #[must_use]
    pub fn order(&self) -> u8 {
        self.order
    }

    /// The coefficients, highest power first.
    #[must_use]
    pub fn coeff(&self) -> &[f32] {
        &self.coeff
    }
}

/// Polynomial segments with an output scale and bias.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurveFloat {
    /// The keys in position order.
    keys: Vec<CurveKey>,
    /// Output multiplier, applied first.
    scale: f32,
    /// Output offset, applied after the scale.
    bias: f32,
}

impl CurveFloat {
    /// A channel over `keys` with output `scale * value + bias`.
    ///
    /// # Panics
    ///
    /// When `keys` holds more than 65,535 entries: the original's count
    /// is a 16-bit word.
    #[must_use]
    pub fn new(keys: Vec<CurveKey>, scale: f32, bias: f32) -> Self {
        assert!(keys.len() <= MAX_KEYS, "key count exceeds 16 bits");
        Self { keys, scale, bias }
    }

    /// The keys in position order.
    #[must_use]
    pub fn keys(&self) -> &[CurveKey] {
        &self.keys
    }

    /// Output multiplier, applied first.
    #[must_use]
    pub const fn scale(&self) -> f32 {
        self.scale
    }

    /// Output offset, applied after the scale.
    #[must_use]
    pub const fn bias(&self) -> f32 {
        self.bias
    }

    /// Byte size of the 32-bit storage form: the header plus, per key,
    /// four bytes per order step plus the segment tail, wrapping exactly
    /// like the original.
    #[must_use]
    pub fn alloc_size(&self) -> u32 {
        let mut size = BASE_SIZE;
        for k in &self.keys {
            size = size
                .wrapping_add((k.order as u32).wrapping_mul(4))
                .wrapping_add(SEGMENT_TAIL);
        }
        size
    }

    /// Evaluates one segment polynomial at `t`: orders 0 to 3 use their
    /// Horner forms (`c0`; `c0 * t + c1`; `(c0 * t + c1) * t + c2`;
    /// cubic), higher orders a generic `order`-step loop (`s = s * t +
    /// c[i]`), all in the original's operand order.
    ///
    /// # Panics
    ///
    /// When `coeffs` holds fewer than `order + 1` entries: the original
    /// reads that many words past the coefficient pointer.
    #[must_use]
    pub fn eval_segment(coeffs: &[f32], order: u32, t: f32) -> f32 {
        let c0 = coeffs[0];
        if order == 0 {
            return c0;
        }
        if order == 1 {
            return fadd(fmul(c0, t), coeffs[1]);
        }
        if order == 2 {
            let s1 = fadd(fmul(c0, t), coeffs[1]);
            return fadd(fmul(s1, t), coeffs[2]);
        }
        if order == 3 {
            let s1 = fadd(fmul(c0, t), coeffs[1]);
            let s2 = fadd(fmul(s1, t), coeffs[2]);
            return fadd(fmul(s2, t), coeffs[3]);
        }
        let mut s = c0;
        for k in 1..=order {
            s = fmul(s, t);
            s = fadd(s, coeffs[k as usize]);
        }
        s
    }

    /// Samples the channel at time `t`: negative times clamp to zero, the
    /// search target is `trunc(t) + 1`, and the first of the leading keys
    /// at or past the target decides the segment; past the last key the
    /// final segment runs with the time clamped down to it. The segment
    /// value is then scaled (`scale * value + bias`, scale first).
    ///
    /// The found segment evaluates through [`Self::eval_segment`]. The
    /// final segment's inline forms are restated separately because their
    /// linear multiply takes the time first (`t * c0`), an order the
    /// not-a-number payload can observe.
    ///
    /// # Panics
    ///
    /// When the channel holds no keys: the original reads one record
    /// before its key array there, which has no meaning here.
    #[must_use]
    pub fn sample(&self, t: f32) -> f32 {
        assert!(!self.keys.is_empty(), "sampling an empty channel");
        let mut t = t;
        if t < 0.0 {
            t = 0.0;
        }
        let count = self.keys.len() as u32;
        let isi = truncate_to_i32(t).wrapping_add(1);
        // The scan covers the leading keys only (all but the last); `ebp`
        // keeps the previous key for the local time, 0 when the first key
        // matches or nothing was scanned.
        let mut ebp: u32 = 0;
        let mut found: Option<usize> = None;
        if count > 1 {
            for (eax, k) in self.keys[..(count - 1) as usize].iter().enumerate() {
                if (k.key as i32) >= isi {
                    found = Some(eax);
                    break;
                }
                ebp = u32::from(k.key);
            }
        }
        if let Some(fi) = found {
            let seg = &self.keys[fi];
            let tl = fsub(t, ebp as f32);
            let r = Self::eval_segment(&seg.coeff, u32::from(seg.order), tl);
            return fadd(fmul(self.scale, r), self.bias);
        }
        let last = &self.keys[(count - 1) as usize];
        let lk = f32::from(last.key);
        if !(lk > t) {
            t = lk;
        }
        let tl = fsub(t, ebp as f32);
        let c = &last.coeff;
        let r = match u32::from(last.order) {
            0 => c[0],
            1 => fadd(fmul(tl, c[0]), c[1]),
            2 => {
                let s1 = fadd(fmul(tl, c[0]), c[1]);
                fadd(fmul(s1, tl), c[2])
            }
            3 => {
                let s1 = fadd(fmul(tl, c[0]), c[1]);
                let s2 = fadd(fmul(s1, tl), c[2]);
                fadd(fmul(s2, tl), c[3])
            }
            typ => {
                let mut s = c[0];
                for k in 1..=typ {
                    s = fmul(s, tl);
                    s = fadd(s, c[k as usize]);
                }
                s
            }
        };
        fadd(fmul(self.scale, r), self.bias)
    }
}
