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
        lifted: true,
        rewrite: "verified vf97 rewrite; r-b608 v6 accepted the measured 1,122-byte function",
        test: "lf-montage-diff: commit_item_matches_its_rewrite; x86, 900 trials over nine paths",
        narrowing: "collaborator bodies are scripted; calls, arguments, order, container words, and listener bytes are compared. The accepted rewrite's stack-check narrowing remains: a dead incoming-argument slot is observed through the tail-call argument.",
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
        lifted: true,
        rewrite: "verified vf94 rewrite; r-b608 v6 accepted the 1,874-byte function",
        test: "lf-montage-diff: indexed_pick_matches_its_rewrite; x86, 1,300 trials over thirteen paths",
        narrowing: "collaborator bodies are scripted; calls, arguments, order, return, container words, and listener bytes are compared. Getter answers are fixed per property within a trial, with one targeted changed-center second-gate case. Handles are non-null where the original dereferences them.",
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
        rewrite: "verified vf114 rewrite; a-S07 v6 accepted the signed comparison",
        test: "lf-montage-diff: dispatch; x86, 2,400 trials over eight paths",
        narrowing: "collaborator bodies are scripted and are not executed; the proof compares every call and argument in order, consumed scripted answers, the container bytes, and the full helper/configuration blocks with direct writes. The shared state flag is passed as a value at the lift boundary. Handles must be non-null on paths where the original faults on null.",
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
        rewrite: "verified vf88 rewrite; a-S05 v6 accepted 1,200 trials",
        test: "",
        narrowing: "not lifted: the accepted contract fixes a never-written stack word to one nonzero fill, the rewrite says that forces a call block, but the result lists no narrowing; behavior under other fills remains unverified",
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
