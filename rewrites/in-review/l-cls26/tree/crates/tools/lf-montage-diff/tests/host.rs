//! Host-side edge cases for the portable montage container method.

use std::collections::HashMap;

use lf_core::Handle32;
use lf_input_frontend::montage_container::{
    HelperContextTag, LookupContextTag, MontageContainer, MontageGlobals, StateTag,
};
use lf_montage_diff::fake::{Ctx, FakeWorld, Script};

fn handle<T>(value: u32) -> Handle32<T> {
    Handle32::new(value).expect("host fixtures use nonzero opaque handles")
}

#[test]
fn zero_state_flag_clears_only_cursor_and_pending_without_collaborator_calls() {
    let mut container = MontageContainer {
        own: handle(10),
        list: handle(20),
        aux: handle(30),
        cfg: handle(40),
        out: handle(50),
        view: handle(60),
        src: handle(70),
        want: 80,
        dropped: 90,
        pending: 100,
        cursor: 110,
        mode: 7,
        phase: 8,
    };
    let mut world = FakeWorld::new(Ctx::default(), Script::new(), HashMap::new());
    let globals = MontageGlobals {
        state_flag: 0,
        state: handle::<StateTag>(120),
        helper_context: handle::<HelperContextTag>(130),
        lookup_context: handle::<LookupContextTag>(140),
    };

    assert_eq!(container.dispatch_state(&mut world, &globals), 0);
    assert_eq!(container.cursor, 0);
    assert_eq!(container.pending, 0);
    assert_eq!(container.want, 80);
    assert_eq!(container.dropped, 90);
    assert_eq!(container.mode, 7);
    assert_eq!(container.phase, 8);
    assert!(world.events.is_empty());
    assert!(world.effects.is_empty());
}
