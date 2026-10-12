//! Route point tables for the follow-point task.
//!
//! A route is a signed 32-bit count and an ordered list of points. The game
//! stores it as the count followed by one 16-byte entry per point (three
//! single-precision coordinates and a pad word). The lifted form keeps the
//! points in order; the count is the number of points, and entries are
//! indexed from zero.

/// One route point: three single-precision coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RoutePoint {
    /// Coordinate along the first horizontal axis.
    pub x: f32,
    /// Coordinate along the second horizontal axis.
    pub y: f32,
    /// Coordinate along the vertical axis.
    pub z: f32,
}

/// The most points a route can hold: its count is a signed 32-bit word.
pub const MAX_POINTS: usize = i32::MAX as usize;

/// An ordered route of points.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteTable {
    points: Vec<RoutePoint>,
}

impl RouteTable {
    /// Builds a route from its points, in order.
    ///
    /// # Panics
    ///
    /// When there are more points than a signed 32-bit count can hold.
    #[must_use]
    pub fn new(points: Vec<RoutePoint>) -> Self {
        assert!(
            points.len() <= MAX_POINTS,
            "route longer than its signed 32-bit count"
        );
        Self { points }
    }

    /// Number of points in the route.
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Whether the route holds no points.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// The point at `index`, counted from zero.
    ///
    /// # Panics
    ///
    /// When `index` is not below [`RouteTable::len`]. The verified methods
    /// are proven only for indices inside the route; a lift must not read
    /// past its end.
    #[must_use]
    pub fn point(&self, index: usize) -> RoutePoint {
        self.points[index]
    }

    /// Empties the route. The original zeroes the count word and leaves the
    /// entries in place; no verified method reads an entry past the count,
    /// so the entries are dropped here.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// All points, in order.
    #[must_use]
    pub fn points(&self) -> &[RoutePoint] {
        &self.points
    }
}
