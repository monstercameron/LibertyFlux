//! The task pose volume: two vectors, a tag, a parent link and flags.
//!
//! Three verified routines share this object. In the 32-bit form `this`
//! points at two 4-float vectors (`+0x00`, `+0x10`), a tag dword (`+0x20`,
//! copied verbatim and read as a float where a radius is wanted), an
//! optional parent-link pointer (`+0x24`) and a flag byte (`+0x28`).
//! The parent link carries a matrix pointer (`+0x20`, built on demand
//! through two callees) and a fallback triple (`+0x10`); the matrix holds
//! three columns at strides of `0x10` and a translation at `+0x30`.
//!
//! [`PoseVolume::transform`] is the world-space transform,
//! [`PoseVolume::blend`] the pose blend, [`PoseVolume::contains`] the
//! point-in-volume (cone or sphere) test. Every float operation runs in
//! the original's order, pinned against reordering so results match bit
//! for bit.

#![forbid(unsafe_code)]

/// Flag byte (`+0x28`) bit 0: the volume answers at all.
pub const FLAG_ACTIVE: u8 = 0x01;
/// Flag byte bit 1: transform through the link's matrix (else copy, then
/// still add the translation tail).
pub const FLAG_TRANSFORM: u8 = 0x02;
/// Flag byte bit 2: copy path of the blend, sphere path of the volume test.
pub const FLAG_COPY: u8 = 0x04;
/// Flag byte bit 2 read as the volume test's sphere selector.
pub const FLAG_SPHERE: u8 = 0x04;
/// Triple count the volume test passes to each normaliser call.
pub const NORM_COUNT: u32 = 1;

/// A parent link: an optional matrix plus the fallback triple.
///
/// The 32-bit form keeps a matrix pointer (null until the build pair
/// installs one) and three fallback floats. The translation tail reads
/// the matrix translation when a matrix is present, the fallback triple
/// otherwise.
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    /// The link's matrix, once built.
    pub matrix: Option<Matrix>,
    /// The fallback triple (`link+0x10`).
    pub fallback: [f32; 3],
}

/// A link matrix: three columns plus the translation.
///
/// Column `i` is the 32-bit form's `mat + i * 0x10` triple; `trans` is
/// `mat + 0x30`.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    /// The three matrix columns.
    pub cols: [[f32; 3]; 3],
    /// The translation added after the column transform.
    pub trans: [f32; 3],
}

/// The task pose volume: the owning type of the three routines.
#[derive(Debug, Clone, PartialEq)]
pub struct PoseVolume {
    /// First local-space vector (`+0x00`).
    pub a: [f32; 4],
    /// Second local-space vector (`+0x10`).
    pub b: [f32; 4],
    /// Tag dword (`+0x20`), carried as bits.
    pub tag: u32,
    /// The optional parent link (`+0x24`).
    pub link: Option<Link>,
    /// The flag byte (`+0x28`).
    pub flags: u8,
}

/// The world-space transform's answer: two vectors and the tag.
#[derive(Debug, Clone, PartialEq)]
pub struct Transformed {
    /// First vector, transformed and translated.
    pub a: [f32; 4],
    /// Second vector, transformed and translated.
    pub b: [f32; 4],
    /// The tag, copied verbatim.
    pub tag: u32,
}

/// The pose blend's answer: one vector and one scalar.
#[derive(Debug, Clone, PartialEq)]
pub struct Blended {
    /// The blended (or copied) vector.
    pub out1: [f32; 4],
    /// The squared radius (copy path) or deviation sum (blend path).
    pub out2: f32,
}

/// One pose sample: what the volume's own fill slot answers.
///
/// The blend and the volume test both open by calling the volume's own
/// fill slot (thiscall on the object, two 4-float outs and one dword
/// out); the trait carries that slot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PoseSample {
    /// First filled vector.
    pub a: [f32; 4],
    /// Second filled vector.
    pub b: [f32; 4],
    /// Filled tag word, carried as bits.
    pub c: u32,
}

/// The volume's own fill slot, shared by the blend and the volume test.
pub trait PoseFill {
    /// Fills the two vectors and the tag word for `vol`.
    fn fill_pose(&mut self, vol: &PoseVolume) -> PoseSample;
}

/// The link matrix builders behind the world-space transform.
pub trait LinkMatrix {
    /// Builds the link's matrix (runs only when none is present).
    fn build_matrix(&mut self, link: &mut Link);
    /// Fetches through the link once the matrix exists (answer ignored).
    fn fetch_matrix(&mut self, link: &Link);
}

/// Which of the four normaliser calls is running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormSlot {
    /// First call: the normalised axis pair.
    First,
    /// Second call: the point offset against that axis.
    Second,
    /// Third call: the centre-to-rim direction.
    Third,
    /// Fourth call: the point offset again.
    Fourth,
}

/// The volume test's solver callees: angle, trigonometry, normalisers.
pub trait ConeSolvers {
    /// Base angle from the two centres' planar words.
    fn base_angle(&mut self, a0: f32, a1: f32, b0: f32, b1: f32) -> f32;
    /// Cosine-like factor of the wrapped angle.
    fn cos_factor(&mut self, ang: f32) -> f32;
    /// Sine-like factor of the wrapped angle.
    fn sin_factor(&mut self, ang: f32) -> f32;
    /// Normalises one source triple into the destination triple.
    fn normalise(&mut self, slot: NormSlot, dst: &mut [u32; 3], src: &[u32; 3], count: u32);
}

/// The read-only tuning words the volume test and the blend read.
///
/// Each field is one relocated float or mask word; the caller (or the
/// subsystem state, once it lifts) supplies them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngleTuning {
    /// The one-half factor.
    pub half: f32,
    /// The one factor of the normalisation guards.
    pub one: f32,
    /// The half-pi shift added to the base angle.
    pub half_pi: f32,
    /// The full-turn modulus of the wrap loops.
    pub tau: f32,
    /// Sign-flip mask xored into the rim-point lane.
    pub neg_mask: u32,
    /// Absolute-value mask of the second dot product.
    pub abs_mask: u32,
}

/// Pinned-order float multiply.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Pinned-order float add.
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Pinned-order float subtract.
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// Pinned-order float divide.
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

/// The sphere path: the tag-as-float squared must strictly exceed the
/// squared distance from the centre to the point.
fn sphere_contains(fa: [f32; 3], fc: f32, pt: [f32; 3]) -> bool {
    let d4 = sub(fa[1], pt[1]);
    let d0 = sub(fa[0], pt[0]);
    let d8 = sub(fa[2], pt[2]);
    let dd = add(add(mul(d0, d0), mul(d4, d4)), mul(d8, d8));
    let cc = mul(fc, fc);
    cc > dd
}

/// The cone path's normalised dot-product gates and height gates.
#[allow(
    clippy::neg_cmp_op_on_partial_ord,
    reason = "negated comparisons reproduce the original's unordered-fails shape exactly"
)]
fn cone_gates<S: ConeSolvers>(
    solvers: &mut S,
    tuning: &AngleTuning,
    fa: [f32; 3],
    fb: [f32; 3],
    px: f32,
    py: f32,
    pt: [f32; 3],
) -> bool {
    let (cx, cy) = (fa[0], fa[1]);
    let dx1 = sub(fb[0], cx);
    let dy1 = sub(fb[1], cy);
    let ex = sub(cx, px);
    let ey = sub(cy, py);
    let len1sq = add(mul(dy1, dy1), mul(dx1, dx1));
    let len2sq = add(mul(ey, ey), mul(ex, ex));
    let len1 = len1sq.sqrt();
    let len2 = len2sq.sqrt();
    let qx = sub(pt[0], cx);
    let qy = sub(pt[1], cy);
    let s1 = if len1sq == 0.0 {
        0.0
    } else {
        div(tuning.one, len1sq.sqrt())
    };
    let nx1 = mul(dx1, s1);
    let ny1 = mul(dy1, s1);
    let mut d1 = [0u32; 3];
    let mut d2 = [0u32; 3];
    let src1 = [nx1.to_bits(), ny1.to_bits(), fa[0].to_bits()];
    solvers.normalise(NormSlot::First, &mut d1, &src1, NORM_COUNT);
    let src2 = [qx.to_bits(), qy.to_bits(), nx1.to_bits()];
    solvers.normalise(NormSlot::Second, &mut d2, &src2, NORM_COUNT);
    let fd1 = [
        f32::from_bits(d1[0]),
        f32::from_bits(d1[1]),
        f32::from_bits(d1[2]),
    ];
    let fd2 = [
        f32::from_bits(d2[0]),
        f32::from_bits(d2[1]),
        f32::from_bits(d2[2]),
    ];
    let dot1 = add(
        add(mul(fd2[0], fd1[0]), mul(fd2[1], fd1[1])),
        mul(fd2[2], fd1[2]),
    );
    if !(dot1 >= 0.0) {
        return false;
    }
    if !(len1 >= dot1) {
        return false;
    }
    let len3sq = add(mul(ey, ey), mul(ex, ex));
    let s3 = if len3sq == 0.0 {
        0.0
    } else {
        div(tuning.one, len3sq.sqrt())
    };
    let mx = mul(ex, s3);
    let my = mul(ey, s3);
    let src3 = [mx.to_bits(), my.to_bits(), qx.to_bits()];
    solvers.normalise(NormSlot::Third, &mut d2, &src3, NORM_COUNT);
    let src4 = [qx.to_bits(), qy.to_bits(), nx1.to_bits()];
    solvers.normalise(NormSlot::Fourth, &mut d1, &src4, NORM_COUNT);
    let fd1 = [
        f32::from_bits(d1[0]),
        f32::from_bits(d1[1]),
        f32::from_bits(d1[2]),
    ];
    let fd2 = [
        f32::from_bits(d2[0]),
        f32::from_bits(d2[1]),
        f32::from_bits(d2[2]),
    ];
    let dot2 = add(
        add(mul(fd1[0], fd2[0]), mul(fd1[1], fd2[1])),
        mul(fd1[2], fd2[2]),
    );
    let absdot = f32::from_bits(dot2.to_bits() & tuning.abs_mask);
    if !(len2 >= absdot) {
        return false;
    }
    if !(pt[2] >= fa[2]) {
        return false;
    }
    if !(fb[2] >= pt[2]) {
        return false;
    }
    true
}

/// One column transform `dst[i] = col_i . v`, in the original's exact
/// operation order. The `w` slot is zero: the original copied an
/// uninitialised scratch word there and the verified rewrite pins it to
/// zero, which is what this matches.
#[allow(
    clippy::many_single_char_names,
    reason = "names mirror the verified rewrite's accumulators one for one"
)]
fn xform_one(mat: &Matrix, v: [f32; 3]) -> [f32; 4] {
    let (vx, vy, vz) = (v[0], v[1], v[2]);
    let t0 = mul(mat.cols[0][0], vx);
    let mut x = mul(mat.cols[1][0], vy);
    let mut y = mul(mat.cols[1][1], vy);
    x = add(x, t0);
    let t = mul(mat.cols[2][0], vz);
    let mut z = mul(mat.cols[1][2], vy);
    x = add(x, t);
    let t = mul(mat.cols[0][1], vx);
    y = add(y, t);
    let t = mul(mat.cols[2][1], vz);
    y = add(y, t);
    let t = mul(mat.cols[0][2], vx);
    z = add(z, t);
    let t = mul(mat.cols[2][2], vz);
    let (ox, oy) = (x, y);
    z = add(z, t);
    [ox, oy, z, 0.0]
}

impl PoseVolume {
    /// Builds a volume from its words.
    #[must_use]
    pub const fn new(a: [f32; 4], b: [f32; 4], tag: u32, link: Option<Link>, flags: u8) -> Self {
        Self {
            a,
            b,
            tag,
            link,
            flags,
        }
    }

    /// Transforms both vectors into world space.
    ///
    /// `None` when the active bit is clear (the 32-bit form answers 0
    /// and writes nothing). A linkless volume copies straight through;
    /// a linked volume without the transform bit copies and still adds
    /// the translation tail; otherwise each vector runs through the
    /// link's matrix (built on demand) before the tail.
    ///
    /// # Panics
    ///
    /// When a build call leaves no matrix: the original then reads
    /// through a null matrix pointer and faults.
    pub fn transform<L: LinkMatrix>(&mut self, links: &mut L) -> Option<Transformed> {
        if self.flags & FLAG_ACTIVE == 0 {
            return None;
        }
        let Some(link) = self.link.as_mut() else {
            return Some(Transformed {
                a: self.a,
                b: self.b,
                tag: self.tag,
            });
        };
        let (mut a, mut b);
        if self.flags & FLAG_TRANSFORM == 0 {
            // Copy, then fall through to the translation tail.
            a = self.a;
            b = self.b;
        } else {
            if link.matrix.is_none() {
                links.build_matrix(link);
                links.fetch_matrix(link);
            }
            let mat = link.matrix.as_ref().expect("link build left no matrix");
            a = xform_one(mat, [self.a[0], self.a[1], self.a[2]]);
            // Second site: the matrix is re-checked and the build pair
            // re-runs only if it somehow came back null. Unreachable in
            // practice (nothing runs between the two reads, and a null
            // after the first build faults at the first vector), kept
            // as a faithful mirror of the 32-bit form.
            if link.matrix.is_none() {
                links.build_matrix(link);
                links.fetch_matrix(link);
            }
            let mat = link.matrix.as_ref().expect("link build left no matrix");
            b = xform_one(mat, [self.b[0], self.b[1], self.b[2]]);
        }
        // Translation tail: the matrix translation when a matrix is
        // present, else the link's fallback triple.
        let t = match &link.matrix {
            Some(m) => m.trans,
            None => link.fallback,
        };
        a[0] = add(t[0], a[0]);
        a[1] = add(t[1], a[1]);
        a[2] = add(t[2], a[2]);
        b[0] = add(t[0], b[0]);
        b[1] = add(t[1], b[1]);
        b[2] = add(t[2], b[2]);
        Some(Transformed {
            a,
            b,
            tag: self.tag,
        })
    }

    /// Blends the two filled pose vectors into one output.
    ///
    /// With the copy bit set the first vector is copied bitwise and the
    /// scalar is the tag-as-float squared; otherwise the vector averages
    /// the two fills lane-wise (the `w` lane copies the second fill's)
    /// and the scalar sums the squared deviations plus the halved tag
    /// squared, in the original's accumulation order.
    pub fn blend<F: PoseFill>(&self, filler: &mut F, half: f32) -> Blended {
        let sample = filler.fill_pose(self);
        if self.flags & FLAG_COPY != 0 {
            let cf = f32::from_bits(sample.c);
            Blended {
                out1: sample.a,
                out2: mul(cf, cf),
            }
        } else {
            let a0 = sample.a[0];
            let a1 = sample.a[1];
            let a2 = sample.a[2];
            let b0 = sample.b[0];
            let b1 = sample.b[1];
            let b2 = sample.b[2];
            let cf = f32::from_bits(sample.c);
            let mut s3 = add(b1, a1);
            let mut s4 = add(b0, a0);
            let mut s2 = add(b2, a2);
            let mut s0 = cf;
            s3 = mul(s3, half);
            s4 = mul(s4, half);
            let mut d7 = sub(a1, s3);
            s2 = mul(s2, half);
            let mut d6 = sub(a0, s4);
            s0 = mul(s0, half);
            d7 = mul(d7, d7);
            let mut d5 = sub(a2, s2);
            d6 = mul(d6, d6);
            s0 = mul(s0, s0);
            d7 = add(d7, d6);
            d5 = mul(d5, d5);
            d7 = add(d7, d5);
            d7 = add(d7, s0);
            Blended {
                out1: [s4, s3, s2, sample.b[3]],
                out2: d7,
            }
        }
    }

    /// Tests whether a point is inside the volume.
    ///
    /// With the sphere bit set the tag-as-float squared must strictly
    /// exceed the squared distance from the first fill's centre.
    /// Otherwise a rim point is placed from the solved angle at half the
    /// tag's distance, and the point must pass two normalised dot-product
    /// gates and lie between the fills' heights. Any failed gate, or any
    /// NaN in a comparison, answers false.
    pub fn contains<P: PoseFill, S: ConeSolvers>(
        &self,
        filler: &mut P,
        solvers: &mut S,
        tuning: &AngleTuning,
        pt: [f32; 3],
    ) -> bool {
        if self.flags & FLAG_ACTIVE == 0 {
            return false;
        }
        let sample = filler.fill_pose(self);
        let fa = [sample.a[0], sample.a[1], sample.a[2]];
        let fb = [sample.b[0], sample.b[1], sample.b[2]];
        let fc = f32::from_bits(sample.c);
        if self.flags & FLAG_SPHERE != 0 {
            return sphere_contains(fa, fc, pt);
        }
        let c1 = solvers.base_angle(sample.a[0], sample.a[1], sample.b[0], sample.b[1]);
        let mut ang = add(c1, tuning.half_pi);
        while ang < 0.0 {
            ang = add(ang, tuning.tau);
        }
        while ang > tuning.tau {
            ang = sub(ang, tuning.tau);
        }
        let radius = mul(fc, tuning.half);
        let cosv = solvers.cos_factor(ang);
        let px = add(mul(cosv, radius), fa[0]);
        let sinv = solvers.sin_factor(ang);
        let py = add(
            f32::from_bits(mul(sinv, radius).to_bits() ^ tuning.neg_mask),
            fa[1],
        );
        cone_gates(solvers, tuning, fa, fb, px, py, pt)
    }
}
