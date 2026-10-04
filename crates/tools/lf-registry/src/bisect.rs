//! Bisection planner: find the one replacement behind a regression.
//!
//! The live switch is `bisect <a> <b>` in `switches.lf` (or
//! [`Registry::bisect`](crate::Registry::bisect)): it enables the
//! name-sorted hooks with ranks in `[a, b)` and disables the rest. This
//! module decides which ranges to try, so a hunt over `n` hooks takes
//! `ceil(log2(n))` runs of the game instead of guesswork.
//!
//! Assumptions, stated because the answer is only as good as they are:
//! - With every suspect on the regression shows; with every hook off it
//!   does not (check both before starting).
//! - Exactly one hook causes it, alone. When two hooks only fail together,
//!   or the symptom is intermittent, halving can point at an innocent
//!   hook: confirm the answer by running with only that hook on
//!   ([`BisectPlan::confirm_line`]) and with only that hook off.
//!
//! Each step: write [`BisectPlan::control_line`] to `switches.lf`, run the
//! scenario, then call [`BisectPlan::record`] with whether the regression
//! appeared. The plan is pure data (two indices), so a tool can persist it
//! between game runs with [`BisectPlan::encode`] / [`BisectPlan::decode`].

/// A regression hunt over `total` name-sorted hooks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BisectPlan {
    total: usize,
    /// First suspect rank (inclusive).
    lo: usize,
    /// Last suspect rank (exclusive).
    hi: usize,
    /// Runs recorded so far.
    steps: usize,
}

impl BisectPlan {
    /// Every one of `total` hooks is a suspect.
    #[must_use]
    pub fn new(total: usize) -> Self {
        Self::within(total, 0, total)
    }

    /// Only ranks `[lo, hi)` are suspects (clamped to `total`), for a hunt
    /// narrowed by an earlier one or by a subsystem toggle.
    #[must_use]
    pub fn within(total: usize, lo: usize, hi: usize) -> Self {
        let hi = hi.min(total);
        BisectPlan {
            total,
            lo: lo.min(hi),
            hi,
            steps: 0,
        }
    }

    /// Current suspect ranks `[lo, hi)`.
    #[must_use]
    pub fn suspects(&self) -> (usize, usize) {
        (self.lo, self.hi)
    }

    /// Runs recorded so far.
    #[must_use]
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Runs still needed in the worst case: `ceil(log2(suspects))`.
    #[must_use]
    pub fn remaining_steps(&self) -> usize {
        let n = self.hi - self.lo;
        if n <= 1 {
            0
        } else {
            (usize::BITS - (n - 1).leading_zeros()) as usize
        }
    }

    /// The single remaining suspect, once found.
    #[must_use]
    pub fn culprit(&self) -> Option<usize> {
        (self.hi - self.lo == 1).then_some(self.lo)
    }

    /// True when no suspect is left (an empty plan, or a hunt whose
    /// answers contradicted the assumptions).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hi == self.lo
    }

    /// Ranks to enable on the next run: the lower half of the suspects,
    /// `[lo, mid)`. `None` once a culprit is found or nothing is left.
    #[must_use]
    pub fn probe(&self) -> Option<(usize, usize)> {
        if self.hi - self.lo < 2 {
            return None;
        }
        Some((self.lo, self.lo + (self.hi - self.lo) / 2))
    }

    /// The control-file line for the next run (`bisect <a> <b>`).
    #[must_use]
    pub fn control_line(&self) -> Option<String> {
        self.probe().map(|(a, b)| format!("bisect {a} {b}"))
    }

    /// The control-file line that runs the culprit alone, to confirm it.
    #[must_use]
    pub fn confirm_line(&self) -> Option<String> {
        self.culprit().map(|c| format!("bisect {c} {}", c + 1))
    }

    /// Record the run of [`probe`](Self::probe): `regressed` is true when
    /// the regression appeared with only the probed half on. The culprit
    /// is then in that half; otherwise it is in the other one. A call when
    /// no probe is pending changes nothing.
    pub fn record(&mut self, regressed: bool) {
        let Some((a, b)) = self.probe() else {
            return;
        };
        if regressed {
            self.lo = a;
            self.hi = b;
        } else {
            self.lo = b;
        }
        self.steps += 1;
    }

    /// One-line text form: `total lo hi steps`.
    #[must_use]
    pub fn encode(&self) -> String {
        format!("{} {} {} {}", self.total, self.lo, self.hi, self.steps)
    }

    /// Parse [`encode`](Self::encode) output; `None` for anything else.
    #[must_use]
    pub fn decode(text: &str) -> Option<Self> {
        let v: Vec<usize> = text
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .ok()?;
        match v.as_slice() {
            [total, lo, hi, steps] if lo <= hi && hi <= total => Some(BisectPlan {
                total: *total,
                lo: *lo,
                hi: *hi,
                steps: *steps,
            }),
            _ => None,
        }
    }
}

/// Which registered hooks a `bisect a b` enables: entry `i` (registration
/// order) is on when its rank among the names sorted ascending (stable for
/// equal names) lies in `[a, b)`. [`Registry::bisect`](crate::Registry::bisect)
/// applies exactly this selection.
#[must_use]
pub fn wanted_by_rank(names: &[&str], a: usize, b: usize) -> Vec<bool> {
    let mut order: Vec<usize> = (0..names.len()).collect();
    order.sort_by(|&x, &y| names[x].cmp(names[y]));
    let mut rank = vec![0usize; names.len()];
    for (r, &i) in order.iter().enumerate() {
        rank[i] = r;
    }
    rank.iter().map(|&r| r >= a && r < b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run a hunt against a simulated single culprit.
    fn hunt(total: usize, culprit: usize) -> BisectPlan {
        let mut plan = BisectPlan::new(total);
        while let Some((a, b)) = plan.probe() {
            plan.record(culprit >= a && culprit < b);
        }
        plan
    }

    #[test]
    fn finds_every_single_culprit_in_log2_steps() {
        for total in 1..=130 {
            let bound = BisectPlan::new(total).remaining_steps();
            for culprit in 0..total {
                let plan = hunt(total, culprit);
                assert_eq!(plan.culprit(), Some(culprit), "total {total}");
                assert!(plan.steps() <= bound, "total {total} culprit {culprit}");
            }
        }
    }

    #[test]
    fn step_bound_is_ceil_log2() {
        let cases = [(0, 0), (1, 0), (2, 1), (3, 2), (4, 2), (5, 3), (8, 3)];
        for (total, steps) in cases {
            assert_eq!(BisectPlan::new(total).remaining_steps(), steps, "{total}");
        }
        assert_eq!(BisectPlan::new(8115).remaining_steps(), 13);
    }

    #[test]
    fn control_lines_follow_the_halves() {
        let mut plan = BisectPlan::new(10);
        assert_eq!(plan.control_line().as_deref(), Some("bisect 0 5"));
        plan.record(false);
        assert_eq!(plan.suspects(), (5, 10));
        assert_eq!(plan.control_line().as_deref(), Some("bisect 5 7"));
        plan.record(true);
        assert_eq!(plan.suspects(), (5, 7));
        assert_eq!(plan.control_line().as_deref(), Some("bisect 5 6"));
        plan.record(false);
        assert_eq!(plan.culprit(), Some(6));
        assert_eq!(plan.control_line(), None);
        assert_eq!(plan.confirm_line().as_deref(), Some("bisect 6 7"));
        // Recording with nothing pending changes nothing.
        plan.record(true);
        assert_eq!((plan.culprit(), plan.steps()), (Some(6), 3));
    }

    #[test]
    fn narrowed_and_empty_hunts() {
        let plan = BisectPlan::within(100, 40, 60);
        assert_eq!(plan.probe(), Some((40, 50)));
        let clamped = BisectPlan::within(10, 4, 99);
        assert_eq!(clamped.suspects(), (4, 10));
        let empty = BisectPlan::new(0);
        assert!(empty.is_empty());
        assert_eq!((empty.probe(), empty.culprit()), (None, None));
        let inverted = BisectPlan::within(10, 8, 3);
        assert!(inverted.is_empty());
    }

    #[test]
    fn plan_round_trips_as_text() {
        let mut plan = BisectPlan::new(37);
        plan.record(true);
        plan.record(false);
        let text = plan.encode();
        assert_eq!(BisectPlan::decode(&text), Some(plan));
        assert_eq!(BisectPlan::decode("10 6 5 0"), None);
        assert_eq!(BisectPlan::decode("10 0 11 0"), None);
        assert_eq!(BisectPlan::decode("10 0 5"), None);
        assert_eq!(BisectPlan::decode("a b c d"), None);
    }

    #[test]
    fn ranks_follow_sorted_names() {
        let names = ["c.x", "a.x", "d.x", "b.x"];
        // Sorted: a.x(1) b.x(3) c.x(0) d.x(2).
        assert_eq!(wanted_by_rank(&names, 0, 2), vec![false, true, false, true]);
        assert_eq!(wanted_by_rank(&names, 2, 4), vec![true, false, true, false]);
        assert_eq!(wanted_by_rank(&names, 0, 0), vec![false; 4]);
        assert_eq!(wanted_by_rank(&names, 0, 99), vec![true; 4]);
        // Equal names keep registration order.
        assert_eq!(
            wanted_by_rank(&["z", "z", "a"], 1, 2),
            vec![true, false, false]
        );
    }

    #[test]
    fn a_hunt_drives_rank_selection() {
        // The planner's ranges, applied through the registry's selection,
        // isolate the hook whose name sorts to the culprit rank.
        let names = ["e", "b", "a", "d", "c", "f"];
        let bad = "d";
        let mut plan = BisectPlan::new(names.len());
        while let Some((a, b)) = plan.probe() {
            let on = wanted_by_rank(&names, a, b);
            let regressed = names.iter().zip(&on).any(|(n, &o)| o && *n == bad);
            plan.record(regressed);
        }
        let c = plan.culprit().unwrap();
        let on = wanted_by_rank(&names, c, c + 1);
        let only: Vec<&str> = names
            .iter()
            .zip(&on)
            .filter(|(_, o)| **o)
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(only, vec![bad]);
    }
}
