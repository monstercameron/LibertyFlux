//! What the lifted replay progress bar proves, and what each proof narrows.
//!
//! Each row names a method of the class, the verified rewrite it is proven
//! against, and the differential test that holds it. The narrowing column
//! lists what the proof does not cover; an empty narrowing means every call,
//! argument, write and result of the rewrite is compared.

/// One method of the class and the state of its lift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodRecord {
    /// The method's name as the inventory gives it.
    pub name: &'static str,
    /// Whether the method is lifted into this module.
    pub lifted: bool,
    /// The verified rewrite it is proven against, or empty when none.
    pub rewrite: &'static str,
    /// The differential test that holds it.
    pub test: &'static str,
    /// What the proof does not cover.
    pub narrowing: &'static str,
}

/// The eight named methods of the class.
pub const METHODS: &[MethodRecord] = &[
    MethodRecord {
        name: "CReplayProgressBar::initialize",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "CReplayProgressBar::CReplayProgressBar_2",
        lifted: true,
        rewrite: "verified rewrite of the teardown routine",
        test: "lf-replaybar-diff: teardown",
        narrowing: "the object's live-table word written at the start is not visible to the \
                    collaborators, so the lift does not model it; a collaborator that writes \
                    into the object's owned slots during a release is not modelled, and the \
                    second read of a slot after its first release is the one place that would \
                    see it",
    },
    MethodRecord {
        name: "CReplayProgressBar::vf0",
        lifted: true,
        rewrite: "verified rewrite of the deleting destructor",
        test: "lf-replaybar-diff: deleting destructor (its teardown call runs the lifted teardown on the lift side and the verified teardown rewrite on the rewrite side)",
        narrowing: "the storage is released through one call; the flag bit tested is bit 0 only",
    },
    MethodRecord {
        name: "CReplayProgressBar::update_timeline_layout",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "CReplayProgressBar::find_nearest_timeline_entry",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "CReplayProgressBar::count_entries_before_current_time",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "CReplayProgressBar::vf2",
        lifted: false,
        rewrite: "verified rewrite (replay progress draw)",
        test: "",
        narrowing: "deferred in this lane: the function is 4,095 bytes with about twenty calls, \
                    float compares and a draw path; the lane did not reach it",
    },
    MethodRecord {
        name: "CReplayProgressBar::vf1",
        lifted: true,
        rewrite: "verified rewrite of the per-frame update",
        test: "lf-replaybar-diff: update",
        narrowing: "the proof's rewrite stack words are not compared; the rewrite zeroes its \
                    out-words before a call, which the lift does not do: the lift takes the \
                    words the collaborator returns; the inner object's flag byte is read through \
                    the collaborator and its value is not checked against memory the lift owns",
    },
];
