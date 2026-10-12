//! What the lifted montage container proves, and what each proof narrows.
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

/// The eighteen named methods of the class.
pub const METHODS: &[MethodRecord] = &[
    MethodRecord {
        name: "UIMontageContainer::update_montage_clip_order",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::vf2",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::get_class_name_hash",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::navigate_montage_clip_selection",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::vf97",
        lifted: false,
        rewrite: "verified rewrite of the per-item visibility commit",
        test: "",
        narrowing: "not lifted yet: the function is 1,175 bytes with float
                    rectangle geometry, a viewport gate, a tag check and a
                    long notify chain through the global listener",
    },
    MethodRecord {
        name: "UIMontageContainer::enter_montage_editor",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::vf94",
        lifted: false,
        rewrite: "verified rewrite of the indexed pick/commit",
        test: "",
        narrowing: "not lifted yet: the function is 1,874 bytes with two
                    float rectangle blocks, a quotient-gated index rule and
                    a tag section over the global listener",
    },
    MethodRecord {
        name: "UIMontageContainer::update_clip_view_state",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::open_montage_text_entry",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::reset_montage_editor_state",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::update_controller_focus",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::vf114",
        lifted: true,
        rewrite: "verified rewrite of the state dispatcher",
        test: "lf-montage-diff: dispatch",
        narrowing: "the flag byte the regions set on each helper answer and
                    the configuration byte the toggled refresh path clears
                    are writes through collaborator objects the lift does not
                    own, so the lift does not model them and the proof does
                    not compare the collaborators' bytes; the fixed context
                    objects passed as call contexts are supplied by the
                    production world instead. Reads through collaborator
                    objects (element words, sub-object links) come from the
                    world, which the proof scripts with the same words the
                    rewrite reads from memory. All six collaborator handles
                    are assumed non-null on paths that use them (the original
                    faults on null).",
    },
    MethodRecord {
        name: "UIMontageContainer::handle_montage_editor_navigation",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::vf88",
        lifted: false,
        rewrite: "verified rewrite of the long update routine",
        test: "",
        narrowing: "not lifted yet: the function is 3,134 bytes with a list
                    synchronisation block, a panel-selection block, a float
                    measurement chain with a threshold maze, two tick-counter
                    checks and a truncation tail",
    },
    MethodRecord {
        name: "UIMontageContainer::cache_montage_timing_values",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::build_montage_timing_render_commands",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::refresh_children_then_forward",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
    MethodRecord {
        name: "UIMontageContainer::update_transition_then_notify_children",
        lifted: false,
        rewrite: "",
        test: "",
        narrowing: "not verified; not lifted",
    },
];
