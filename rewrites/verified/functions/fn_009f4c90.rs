// original: 0x009F4C90 net_stats_register_all (proposed)

/// Register a fixed set of eight network statistics, publish their default
/// values, then run the achievement-blocked check for slot 0x16.
///
/// Reads the manager handle from `MGR_HANDLE`, looks each id in `STAT_IDS`
/// up through callee 2, wraps the handle through callee 3 (thiscall, no
/// stack arguments), and registers the wrapped object with callee 4 together
/// with the manager handle and the constant `USER_SLOT`. After a one-shot
/// notification (callee 5), each id is given its default from `DEFAULTS`
/// through callee 7 (thiscall, five stack words, object at base + `OBJ_OFF`
/// where base is callee 6's answer). The eight ids are then registered a
/// second time through callee 8 with the manager handle. Finally the stamp
/// word at `STAMP_SRC` is copied to `STAMP_DST` and callee 9 runs with
/// (`ACH_SLOT`, last wrapped object, manager handle); its answer is returned.
///
/// The body is straight-line: 67 outgoing calls in a fixed order, no
/// branches, no floating point. Original: 0x009F4C90 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_009f4c90() -> u32 {
    unsafe {
        const MGR_HANDLE: u32 = 0x012B_4138;
        const STAMP_SRC: u32 = 0x0117_35B4;
        const STAMP_DST: u32 = 0x012B_6228;
        const OBJ_OFF: u32 = 0x2B0;
        const USER_SLOT: u32 = 2;
        const ACH_SLOT: u32 = 0x16;
        const STAT_IDS: [u32; 8] = [3, 0x24, 0x1D, 0x1E, 0x20, 0x22, 0x23, 0x15];
        const DEFAULTS: [u32; 8] = [1, 0x0A, 0x78, 0x32, 0x96, 0x64, 0xC8, 0x14];
        const PREPARE: u32 = 1;
        const LOOKUP: u32 = 2;
        const WRAP: u32 = 3;
        const REGISTER_A: u32 = 4;
        const NOTIFY: u32 = 5;
        const GET_BASE: u32 = 6;
        const SET_DEFAULT: u32 = 7;
        const REGISTER_B: u32 = 8;
        const ACH_CHECK: u32 = 9;

        let mgr = (lf_checker_rt::global::<u32>(MGR_HANDLE) as *const u32).read();
        lf_checker_rt::callee_cdecl!(PREPARE, u32,);
        for &id in STAT_IDS.iter() {
            let handle = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
            let obj = lf_checker_rt::callee_thiscall!(WRAP, u32, handle);
            lf_checker_rt::callee_cdecl!(REGISTER_A, u32, obj, mgr, USER_SLOT);
        }
        lf_checker_rt::callee_cdecl!(NOTIFY, u32, 0);
        for (&id, &def) in STAT_IDS.iter().zip(DEFAULTS.iter()) {
            let base = lf_checker_rt::callee_cdecl!(GET_BASE, u32, 0);
            lf_checker_rt::callee_thiscall!(SET_DEFAULT, u32, base.wrapping_add(OBJ_OFF), id, def, 0, 0, 0);
        }
        let mut last_obj = 0u32;
        for &id in STAT_IDS.iter() {
            let handle = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
            let obj = lf_checker_rt::callee_thiscall!(WRAP, u32, handle);
            lf_checker_rt::callee_cdecl!(REGISTER_B, u32, obj, mgr);
            last_obj = obj;
        }
        let stamp = (lf_checker_rt::global::<u32>(STAMP_SRC) as *const u32).read();
        (lf_checker_rt::global::<u32>(STAMP_DST) as *mut u32).write(stamp);
        lf_checker_rt::callee_cdecl!(ACH_CHECK, u32, ACH_SLOT, last_obj, mgr)
    }
});
