//! `lf-test-target`: 32-bit live test for the hook engine.
//!
//! Builds its own hand-assembled functions in executable memory (so every
//! first-byte sequence is known exactly), hooks them, and proves install /
//! call-through / toggle / remove, including under concurrent calls.
//!
//! Usage: `lf-test-target [path-to-lf_proxy.dll]` (the proxy test is
//! skipped when no path is given). Exit code 0 iff every check passes.
//! Build from the repository root with `cargo build --target
//! i686-pc-windows-msvc -p lf-test-target -p lf-proxy` and pass the
//! freshly built proxy DLL; the proxy test fails closed otherwise.

// The suite builds executable test functions by hand and calls them
// through transmuted pointers; unsafe is the whole method.
#![allow(unsafe_code)]
// Test harness, not shipped code; pedantic style lints stay off here
// while correctness lints (clippy::all) apply.
#![allow(clippy::pedantic)]

use lf_hook::adapters;
use lf_hook::detour::{Detour, FollowJumps, HookError};
use lf_hook::mem;
use lf_hook::pe::live as pelive;
use lf_hook::slot::SlotHook;
use lf_registry::{HookDef, Registry, Target};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

static DET_CDECL: AtomicU32 = AtomicU32::new(0);
static TRAMP_CDECL: AtomicUsize = AtomicUsize::new(0);
static DET_STDCALL: AtomicU32 = AtomicU32::new(0);
static TRAMP_STDCALL: AtomicUsize = AtomicUsize::new(0);
static DET_TC: AtomicU32 = AtomicU32::new(0);
static CALLER_TC_ORIG: AtomicUsize = AtomicUsize::new(0);
static DET_TICK: AtomicU32 = AtomicU32::new(0);
static ORIG_TICK: AtomicUsize = AtomicUsize::new(0);
static DET_VTABLE: AtomicU32 = AtomicU32::new(0);
static ORIG_VTABLE: AtomicUsize = AtomicUsize::new(0);
static DET_CONC: AtomicU32 = AtomicU32::new(0);
static TRAMP_CONC: AtomicUsize = AtomicUsize::new(0);

extern "C" fn det_cdecl(a: u32, b: u32) -> u32 {
    DET_CDECL.fetch_add(1, Ordering::Relaxed);
    let orig: extern "C" fn(u32, u32) -> u32 =
        unsafe { std::mem::transmute(TRAMP_CDECL.load(Ordering::Relaxed)) };
    orig(a, b) + 1000
}

extern "system" fn det_stdcall(a: u32, b: u32) -> u32 {
    DET_STDCALL.fetch_add(1, Ordering::Relaxed);
    let orig: extern "system" fn(u32, u32) -> u32 =
        unsafe { std::mem::transmute(TRAMP_STDCALL.load(Ordering::Relaxed)) };
    orig(a, b) + 1000
}

extern "C" fn det_thiscall(this: *const u32, k: u32) -> u32 {
    DET_TC.fetch_add(1, Ordering::Relaxed);
    // Caller stubs are callee-pops (stdcall-shaped): they consume both
    // `this` and the stack args, like the thiscall target itself.
    let orig: extern "system" fn(*const u32, u32) -> u32 =
        unsafe { std::mem::transmute(CALLER_TC_ORIG.load(Ordering::Relaxed)) };
    orig(this, k) + 100
}

extern "system" fn det_tick() -> u32 {
    DET_TICK.fetch_add(1, Ordering::Relaxed);
    let orig: extern "system" fn() -> u32 =
        unsafe { std::mem::transmute(ORIG_TICK.load(Ordering::Relaxed)) };
    orig()
}

extern "C" fn det_vtable(x: u32) -> u32 {
    DET_VTABLE.fetch_add(1, Ordering::Relaxed);
    let orig: extern "C" fn(u32) -> u32 =
        unsafe { std::mem::transmute(ORIG_VTABLE.load(Ordering::Relaxed)) };
    orig(x) + 500
}

extern "C" fn det_conc(a: u32, b: u32) -> u32 {
    DET_CONC.fetch_add(1, Ordering::Relaxed);
    let orig: extern "C" fn(u32, u32) -> u32 =
        unsafe { std::mem::transmute(TRAMP_CONC.load(Ordering::Relaxed)) };
    orig(a, b) + 1000
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetTickCount() -> u32;
}

struct Suite {
    pass: u32,
    fail: u32,
}

impl Suite {
    fn check(&mut self, name: &str, cond: bool) {
        if cond {
            self.pass += 1;
            println!("TEST {name} PASS");
        } else {
            self.fail += 1;
            println!("TEST {name} FAIL");
        }
    }
}

fn make_exec(bytes: &[u8]) -> *mut u8 {
    let p = mem::alloc_exec(bytes.len()).expect("alloc_exec");
    // SAFETY: `p` is a fresh allocation of exactly `bytes.len()` bytes.
    unsafe { mem::write_exec(p, bytes) };
    p
}

fn call_cdecl2(addr: usize, a: u32, b: u32) -> u32 {
    let f: extern "C" fn(u32, u32) -> u32 = unsafe { std::mem::transmute(addr) };
    f(a, b)
}

fn call_stdcall2(addr: usize, a: u32, b: u32) -> u32 {
    let f: extern "system" fn(u32, u32) -> u32 = unsafe { std::mem::transmute(addr) };
    f(a, b)
}

fn call_cdecl1(addr: usize, a: u32) -> u32 {
    let f: extern "C" fn(u32) -> u32 = unsafe { std::mem::transmute(addr) };
    f(a)
}

fn main() {
    let mut s = Suite { pass: 0, fail: 0 };

    // ---- 1. cdecl hook lifecycle ----
    // mov eax,[esp+4]; add eax,[esp+8]; ret
    let t_cdecl = make_exec(&[0x8B, 0x44, 0x24, 0x04, 0x03, 0x44, 0x24, 0x08, 0xC3]);
    let t_cdecl_a = t_cdecl as usize;
    s.check("cdecl-orig", call_cdecl2(t_cdecl_a, 40, 2) == 42);
    let mut h =
        Detour::create(t_cdecl_a, det_cdecl as *const () as usize, FollowJumps::Off).unwrap();
    TRAMP_CDECL.store(h.trampoline(), Ordering::Relaxed);
    s.check("cdecl-trampoline", call_cdecl2(h.trampoline(), 40, 2) == 42);
    s.check("cdecl-verify-off", h.verify());
    h.enable().unwrap();
    s.check(
        "cdecl-hooked",
        call_cdecl2(t_cdecl_a, 40, 2) == 1042 && DET_CDECL.load(Ordering::Relaxed) == 1,
    );
    s.check("cdecl-verify-on", h.verify());
    h.disable().unwrap();
    s.check("cdecl-disabled", call_cdecl2(t_cdecl_a, 40, 2) == 42);
    h.enable().unwrap();
    s.check("cdecl-reenabled", call_cdecl2(t_cdecl_a, 1, 1) == 1002);
    h.disable().unwrap();
    // Integrity self-check: corrupt, verify fails, restore repairs.
    unsafe { (t_cdecl_a as *mut u8).write(0xCC) };
    s.check("cdecl-verify-corrupt", !h.verify());
    h.restore().unwrap();
    s.check(
        "cdecl-restored",
        h.verify() && call_cdecl2(t_cdecl_a, 40, 2) == 42,
    );

    // ---- 2. stdcall hook ----
    // mov eax,[esp+4]; mov edx,[esp+8]; imul eax,edx; ret 8
    let t_std = make_exec(&[
        0x8B, 0x44, 0x24, 0x04, 0x8B, 0x54, 0x24, 0x08, 0x0F, 0xAF, 0xC2, 0xC2, 0x08, 0x00,
    ]);
    let t_std_a = t_std as usize;
    s.check("stdcall-orig", call_stdcall2(t_std_a, 6, 7) == 42);
    let mut hs =
        Detour::create(t_std_a, det_stdcall as *const () as usize, FollowJumps::Off).unwrap();
    TRAMP_STDCALL.store(hs.trampoline(), Ordering::Relaxed);
    // Trampoline must preserve callee-pop: call through a stack probe.
    s.check(
        "stdcall-trampoline",
        call_stdcall2(hs.trampoline(), 6, 7) == 42,
    );
    hs.enable().unwrap();
    s.check("stdcall-hooked", call_stdcall2(t_std_a, 6, 7) == 1042);
    // Stack discipline: caller with pushed args still balanced (no crash +
    // subsequent call correct).
    s.check(
        "stdcall-balanced",
        call_stdcall2(t_std_a, 1, 2) == 1002 && call_cdecl2(t_cdecl_a, 1, 1) == 2,
    );
    hs.disable().unwrap();
    s.check("stdcall-disabled", call_stdcall2(t_std_a, 6, 7) == 42);

    // ---- 3. thiscall via generated stubs ----
    // mov eax,[ecx]; add eax,[esp+4]; ret 4
    let t_tc = make_exec(&[0x8B, 0x01, 0x03, 0x44, 0x24, 0x04, 0xC2, 0x04, 0x00]);
    let t_tc_a = t_tc as usize;
    let obj = Box::new(40u32);
    let obj_p = &*obj as *const u32;
    let caller: extern "system" fn(*const u32, u32) -> u32 =
        unsafe { std::mem::transmute(adapters::make_thiscall_caller_stub(t_tc_a).unwrap()) };
    s.check("thiscall-orig", caller(obj_p, 2) == 42);
    let dstub = adapters::make_thiscall_detour_stub(det_thiscall as *const () as usize, 1).unwrap();
    let mut htc = Detour::create(t_tc_a, dstub as usize, FollowJumps::Off).unwrap();
    let caller_orig = adapters::make_thiscall_caller_stub(htc.trampoline()).unwrap();
    CALLER_TC_ORIG.store(caller_orig as usize, Ordering::Relaxed);
    // Trampoline-as-thiscall works before enabling.
    let pre: extern "system" fn(*const u32, u32) -> u32 =
        unsafe { std::mem::transmute(caller_orig as usize) };
    s.check("thiscall-trampoline", pre(obj_p, 2) == 42);
    htc.enable().unwrap();
    s.check(
        "thiscall-hooked",
        caller(obj_p, 2) == 142 && DET_TC.load(Ordering::Relaxed) == 1,
    );
    s.check(
        "thiscall-regs-preserved",
        regs_preserved(caller as usize, obj_p, 2),
    );
    htc.disable().unwrap();
    s.check("thiscall-disabled", caller(obj_p, 2) == 42);

    // ---- 4. short first instructions (1-byte nops + mov) ----
    let t_short = make_exec(&[0x90, 0x90, 0x90, 0xB8, 0x34, 0x12, 0x00, 0x00, 0xC3]);
    let t_short_a = t_short as usize;
    s.check("short-orig", call_cdecl1(t_short_a, 0) == 0x1234);
    let mut hsh =
        Detour::create(t_short_a, det_short as *const () as usize, FollowJumps::Off).unwrap();
    s.check("short-moved-8", hsh.moved_len() == 8);
    TRAMP_SHORT.store(hsh.trampoline(), Ordering::Relaxed);
    s.check(
        "short-trampoline",
        call_cdecl1(hsh.trampoline(), 0) == 0x1234,
    );
    hsh.enable().unwrap();
    s.check("short-hooked", call_cdecl1(t_short_a, 0) == 0x1234 + 7);
    hsh.disable().unwrap();
    s.check("short-disabled", call_cdecl1(t_short_a, 0) == 0x1234);

    // ---- 5. function starting with a relative jump ----
    // E9 +5; 5x nop padding; mov eax,0x7777; ret
    let t_jmp = make_exec(&[
        0xE9, 0x05, 0x00, 0x00, 0x00, 0x90, 0x90, 0x90, 0x90, 0x90, 0xB8, 0x77, 0x77, 0x00, 0x00,
        0xC3,
    ]);
    let t_jmp_a = t_jmp as usize;
    s.check("jumpstart-orig", call_cdecl1(t_jmp_a, 0) == 0x7777);
    // Follow-on: hook lands on the real body.
    let mut hj = Detour::create(t_jmp_a, det_short as *const () as usize, FollowJumps::On).unwrap();
    s.check("jumpstart-followed", hj.target() == t_jmp_a + 10);
    TRAMP_SHORT.store(hj.trampoline(), Ordering::Relaxed);
    hj.enable().unwrap();
    s.check("jumpstart-hooked", call_cdecl1(t_jmp_a, 0) == 0x7777 + 7);
    hj.disable().unwrap();
    // Follow-off: the jump itself is relocated into the trampoline.
    let mut hj2 =
        Detour::create(t_jmp_a, det_short as *const () as usize, FollowJumps::Off).unwrap();
    TRAMP_SHORT.store(hj2.trampoline(), Ordering::Relaxed);
    s.check(
        "jumpstart-trampoline-nofollow",
        call_cdecl1(hj2.trampoline(), 0) == 0x7777,
    );
    hj2.enable().unwrap();
    s.check(
        "jumpstart-hooked-nofollow",
        call_cdecl1(t_jmp_a, 0) == 0x7777 + 7,
    );
    hj2.disable().unwrap();

    // ---- 6. short conditional jump inside the moved range (widening) ----
    // cmp dword[esp+4],0; jz +6; mov eax,1; ret; mov eax,2; ret
    let t_jcc = make_exec(&[
        0x83, 0x7C, 0x24, 0x04, 0x00, 0x74, 0x06, 0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3, 0xB8, 0x02,
        0x00, 0x00, 0x00, 0xC3,
    ]);
    let t_jcc_a = t_jcc as usize;
    s.check(
        "jcc-orig",
        call_cdecl1(t_jcc_a, 0) == 2 && call_cdecl1(t_jcc_a, 5) == 1,
    );
    let mut hjcc =
        Detour::create(t_jcc_a, det_short as *const () as usize, FollowJumps::Off).unwrap();
    TRAMP_SHORT.store(hjcc.trampoline(), Ordering::Relaxed);
    s.check(
        "jcc-trampoline",
        call_cdecl1(hjcc.trampoline(), 0) == 2 && call_cdecl1(hjcc.trampoline(), 9) == 1,
    );
    hjcc.enable().unwrap();
    s.check("jcc-hooked", call_cdecl1(t_jcc_a, 0) == 2 + 7);
    hjcc.disable().unwrap();

    // ---- 7. hook creation failures are clean errors ----
    let tiny = make_exec(&[0x33, 0xC0, 0xC3]); // xor eax,eax; ret (3 bytes)
    s.check(
        "fail-too-short",
        matches!(
            Detour::create(
                tiny as usize,
                det_short as *const () as usize,
                FollowJumps::Off
            ),
            Err(HookError::TooShort)
        ),
    );
    let bad = make_exec(&[0x0F, 0x04, 0x90, 0x90, 0x90, 0x90]);
    s.check(
        "fail-decode",
        matches!(
            Detour::create(
                bad as usize,
                det_short as *const () as usize,
                FollowJumps::Off
            ),
            Err(HookError::DecodeFailed)
        ),
    );
    s.check(
        "fail-unreadable",
        matches!(
            Detour::create(0x1, det_short as *const () as usize, FollowJumps::Off),
            Err(HookError::UnreadableTarget)
        ),
    );
    s.check(
        "fail-range",
        matches!(
            Detour::create(t_cdecl_a, 0xFFFF_0000, FollowJumps::Off),
            Err(HookError::DetourOutOfRange)
        ),
    );

    // ---- 8. vtable slot on a read-only page ----
    let f1 = make_exec(&[0x8B, 0x44, 0x24, 0x04, 0x83, 0xC0, 0x01, 0xC3]); // x+1
    let f2 = make_exec(&[0x8B, 0x44, 0x24, 0x04, 0x83, 0xC0, 0x0A, 0xC3]); // x+10
    let vtab = mem::alloc_exec(8).unwrap();
    unsafe {
        (vtab as *mut u32).write(f1 as u32);
        (vtab.add(4) as *mut u32).write(f2 as u32);
    }
    unsafe {
        let mut _old = 0;
        mem::VirtualProtect(vtab as *mut _, 8, mem::PAGE_READONLY, &mut _old);
    }
    let call_slot = |slot: usize| -> u32 {
        let p = unsafe { ((vtab as usize + slot * 4) as *const u32).read() } as usize;
        call_cdecl1(p, 100)
    };
    s.check("vtable-orig", call_slot(0) == 101 && call_slot(1) == 110);
    let mut hv = SlotHook::create(
        (vtab as usize + 4) as *mut usize,
        det_vtable as *const () as usize,
    )
    .unwrap();
    ORIG_VTABLE.store(hv.original(), Ordering::Relaxed);
    s.check("vtable-orig-read", hv.original() == f2 as usize);
    hv.enable().unwrap();
    s.check("vtable-hooked", call_slot(1) == 610 && call_slot(0) == 101);
    hv.disable().unwrap();
    s.check("vtable-disabled", call_slot(1) == 110);

    // ---- 9. import slot (kernel32 GetTickCount via our own IAT entry) ----
    let exe_base = unsafe { mem::GetModuleHandleW(std::ptr::null()) } as usize;
    let before = unsafe { GetTickCount() };
    s.check("iat-orig", before != 0);
    match pelive::find_import_slot(exe_base, "kernel32.dll", "GetTickCount") {
        Some(slot) => {
            let mut hi =
                SlotHook::create(slot as *mut usize, det_tick as *const () as usize).unwrap();
            ORIG_TICK.store(hi.original(), Ordering::Relaxed);
            hi.enable().unwrap();
            let v = unsafe { GetTickCount() };
            s.check(
                "iat-hooked",
                DET_TICK.load(Ordering::Relaxed) == 1 && v != 0,
            );
            hi.disable().unwrap();
            let n = DET_TICK.load(Ordering::Relaxed);
            let _ = unsafe { GetTickCount() };
            s.check("iat-disabled", DET_TICK.load(Ordering::Relaxed) == n);
        }
        None => s.check("iat-slot-found", false),
    }

    // ---- 10. concurrency: toggle under load from 4 threads ----
    // mov eax,[esp+4]; add eax,[esp+8]; ret
    let t_conc = make_exec(&[0x8B, 0x44, 0x24, 0x04, 0x03, 0x44, 0x24, 0x08, 0xC3]);
    let t_conc_a = t_conc as usize;
    let mut hc =
        Detour::create(t_conc_a, det_conc as *const () as usize, FollowJumps::Off).unwrap();
    TRAMP_CONC.store(hc.trampoline(), Ordering::Relaxed);
    hc.enable().unwrap();
    let bad_results = AtomicU32::new(0);
    let done = AtomicU32::new(0);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for i in 0..5000u32 {
                    let r = call_cdecl2(t_conc_a, i, 1);
                    if r != i + 1 && r != i + 1 + 1000 {
                        bad_results.fetch_add(1, Ordering::Relaxed);
                    }
                }
                done.fetch_add(1, Ordering::Relaxed);
            });
        }
        let mut min_frozen = usize::MAX;
        for _ in 0..100 {
            hc.disable().unwrap();
            std::thread::sleep(std::time::Duration::from_micros(100));
            let f = hc.enable().unwrap();
            min_frozen = min_frozen.min(f);
        }
        // Workers still running; leave enabled until they finish.
        while done.load(Ordering::Relaxed) < 4 {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        s.check("conc-no-crash", true);
        s.check("conc-freeze-worked", min_frozen > 0);
    });
    hc.disable().unwrap();
    s.check("conc-results", bad_results.load(Ordering::Relaxed) == 0);
    s.check(
        "conc-final",
        call_cdecl2(t_conc_a, 10, 20) == 30 && hc.verify(),
    );

    // ---- 11. registry: control file + bisect + self-check, live ----
    let r1 = make_exec(&[0x8B, 0x44, 0x24, 0x04, 0x83, 0xC0, 0x01, 0xC3]); // x+1
    let r2 = make_exec(&[0x8B, 0x44, 0x24, 0x04, 0x83, 0xC0, 0x02, 0xC3]); // x+2
    let r1_a = r1 as usize;
    let r2_a = r2 as usize;
    TRAMP_SHORT.store(0, Ordering::Relaxed);
    let mut reg = Registry::new(exe_base);
    // det_short adds 7 via TRAMP_SHORT; point it per call below.
    reg.add(HookDef {
        name: "test.add1".to_string(),
        subsystem: "test".to_string(),
        target: Target::Absolute {
            addr: r1_a,
            expected: vec![0x8B, 0x44, 0x24],
            follow_jumps: false,
        },
        detour: det_short as *const () as usize,
    });
    reg.add(HookDef {
        name: "test.add2".to_string(),
        subsystem: "test".to_string(),
        target: Target::Absolute {
            addr: r2_a,
            expected: vec![0x8B, 0x44, 0x24],
            follow_jumps: false,
        },
        detour: det_short as *const () as usize,
    });
    reg.resolve_all();
    let states: Vec<String> = reg
        .status()
        .iter()
        .map(|(n, _, st)| format!("{n}={st:?}"))
        .collect();
    s.check(
        "registry-resolved",
        states == vec!["test.add1=Off", "test.add2=Off"],
    );
    // Control file drive.
    let dir = std::env::temp_dir().join("lf-hook-test");
    let _ = std::fs::create_dir_all(&dir);
    let ctl = dir.join(format!("switches-{}.lf", std::process::id()));
    std::fs::write(&ctl, "test.add1 on\ntest.add2 off\n").unwrap();
    let text = std::fs::read_to_string(&ctl).unwrap();
    let cmds: Vec<_> = lf_registry::parse_control(&text)
        .into_iter()
        .collect::<Result<_, _>>()
        .unwrap();
    // det_short needs the right trampoline per target; enable add1 only, so
    // point TRAMP_SHORT at add1's trampoline. Reach it via a probe hook.
    let probe = Detour::create(r1_a, det_short as *const () as usize, FollowJumps::Off).unwrap();
    TRAMP_SHORT.store(probe.trampoline(), Ordering::Relaxed);
    std::mem::forget(probe); // keep trampoline alive for the registry hook below
    for line in lf_registry::apply_commands(&mut reg, &cmds) {
        println!("  control: {line}");
    }
    s.check(
        "registry-control",
        reg.active_names() == vec!["test.add1".to_string()],
    );
    s.check(
        "registry-behavior",
        call_cdecl1(r1_a, 10) == 11 + 7 && call_cdecl1(r2_a, 10) == 12,
    );
    // Bisect: name-sorted [add1, add2]; [1,2) enables add2 only. Re-point
    // the shared detour trampoline at add2 first.
    let probe2 = Detour::create(r2_a, det_short as *const () as usize, FollowJumps::Off).unwrap();
    TRAMP_SHORT.store(probe2.trampoline(), Ordering::Relaxed);
    std::mem::forget(probe2);
    reg.bisect(1, 2);
    s.check(
        "registry-bisect",
        reg.active_names() == vec!["test.add2".to_string()],
    );
    s.check(
        "registry-bisect-behavior",
        call_cdecl1(r2_a, 10) == 12 + 7 && call_cdecl1(r1_a, 10) == 11,
    );
    // Self-check repairs corruption.
    unsafe { (r2_a as *mut u8).write(0xCC) };
    let repaired = reg.verify_and_restore();
    s.check("registry-repair", repaired == vec!["test.add2".to_string()]);
    let _ = reg.set_all(false);
    let _ = std::fs::remove_file(&ctl);

    // ---- 12. proxy DLL load + forward (needs argv[1]) ----
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // SAFETY: single-threaded test setup; no other thread reads env here.
        unsafe { std::env::set_var("LF_INIT_TIMEOUT_MS", "2000") };
        let dll = args[1].clone();
        let w = mem::to_wide(&dll);
        let hmod = unsafe { mem::LoadLibraryExW(w.as_ptr(), std::ptr::null_mut(), 0) };
        s.check("proxy-loads", !hmod.is_null());
        if !hmod.is_null() {
            let tgt = unsafe { mem::GetProcAddress(hmod, c"timeGetTime".as_ptr().cast()) };
            s.check("proxy-has-timeGetTime", !tgt.is_null());
            if !tgt.is_null() {
                let f: extern "system" fn() -> u32 = unsafe { std::mem::transmute(tgt) };
                let v = f();
                // Direct system value for comparison.
                let sys = mem::to_wide(r"C:\Windows\System32\winmm.dll");
                // 32-bit process: System32 resolves to the 32-bit DLL.
                let hsys = unsafe { mem::LoadLibraryExW(sys.as_ptr(), std::ptr::null_mut(), 0) };
                let direct = unsafe { mem::GetProcAddress(hsys, c"timeGetTime".as_ptr().cast()) };
                let g: extern "system" fn() -> u32 = unsafe { std::mem::transmute(direct) };
                let v2 = g();
                s.check("proxy-forwards", v != 0 && v.abs_diff(v2) < 60_000);
            }
            let asi = unsafe { mem::GetProcAddress(hmod, c"InitializeASI".as_ptr().cast()) };
            s.check("proxy-has-InitializeASI", !asi.is_null());
            // Give the init thread a moment, then confirm it failed closed
            // (test exe has no .tbm section) and logged.
            std::thread::sleep(std::time::Duration::from_millis(2600));
            let logpath = std::path::Path::new(&dll)
                .parent()
                .unwrap()
                .join("libertyflux.log");
            let logged = std::fs::read_to_string(&logpath)
                .map(|t| t.contains("proxy init") && t.contains("staying disabled"))
                .unwrap_or(false);
            s.check("proxy-fail-closed-logged", logged);
        }
    } else {
        println!("TEST proxy-loads SKIP (no dll path given)");
    }

    println!("RESULT pass={} fail={}", s.pass, s.fail);
    if s.fail > 0 {
        std::process::exit(1);
    }
}

static TRAMP_SHORT: AtomicUsize = AtomicUsize::new(0);

extern "C" fn det_short(x: u32) -> u32 {
    let orig: extern "C" fn(u32) -> u32 =
        unsafe { std::mem::transmute(TRAMP_SHORT.load(Ordering::Relaxed)) };
    orig(x) + 7
}

/// Hand-assembled caller: sets sentinel registers, calls `target` as
/// cdecl(this, k), verifies ebx/esi/edi survived, returns 1/0.
fn regs_preserved(target: usize, this: *const u32, k: u32) -> bool {
    let mut b: Vec<u8> = vec![
        0x53, 0x56, 0x57, // push ebx,esi,edi
        0xBB, 0x11, 0x11, 0x11, 0x11, // mov ebx, sent1
        0xBE, 0x22, 0x22, 0x22, 0x22, // mov esi, sent2
        0xBF, 0x33, 0x33, 0x33, 0x33, // mov edi, sent3
        0x68, 0, 0, 0, 0, // push k
        0x68, 0, 0, 0, 0, // push this
        0xB8, 0, 0, 0, 0, // mov eax, target
        0xFF, 0xD0, // call eax (callee pops this + k: no add esp here)
    ];
    b[19..23].copy_from_slice(&k.to_le_bytes());
    b[24..28].copy_from_slice(&(this as u32).to_le_bytes());
    b[29..33].copy_from_slice(&(target as u32).to_le_bytes());
    let chk = b.len();
    b.extend_from_slice(&[
        0x81, 0xFB, 0x11, 0x11, 0x11, 0x11, // cmp ebx,sent1
        0x75, 0x00, // jne fail (patched)
        0x81, 0xFE, 0x22, 0x22, 0x22, 0x22, // cmp esi,sent2
        0x75, 0x00, // jne fail
        0x81, 0xFF, 0x33, 0x33, 0x33, 0x33, // cmp edi,sent3
        0x75, 0x00, // jne fail
        0xB8, 0x01, 0x00, 0x00, 0x00, // mov eax,1
        0x5F, 0x5E, 0x5B, 0xC3, // pop edi,esi,ebx; ret
    ]);
    let fail = b.len();
    b.extend_from_slice(&[
        0xB8, 0x00, 0x00, 0x00, 0x00, // mov eax,0
        0x5F, 0x5E, 0x5B, 0xC3, // pop edi,esi,ebx; ret
    ]);
    // Patch the three jne rel8s.
    for (jne_at, next) in [
        (chk + 6, chk + 8),
        (chk + 14, chk + 16),
        (chk + 22, chk + 24),
    ] {
        b[jne_at + 1] = (fail as i32 - next as i32) as u8;
    }
    let p = make_exec(&b);
    call_cdecl1(p as usize, 0) == 1
}
