// original: 0x00D775E0 publish_resolved_table (proposed)

/// Resolve the shared table and publish it to the global slot.
///
/// Passes the table reference to the resolver and stores the answer in
/// the global slot, returning the answer. Cdecl, no stack words.
use lf_checker_rt::{callee_cdecl, export, global, relocated};

const RESOLVE: u32 = 1;

export!(cdecl, rw_00d775e0() -> u32 {
    unsafe {
        const TABLE_REF: u32 = 0x00eec3e0;
        const SLOT: u32 = 0x01797690;
        let r = callee_cdecl!(RESOLVE, u32, relocated(TABLE_REF));
        global::<u32>(SLOT).write(r);
        r
    }
});
