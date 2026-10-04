//! Split of `lf-checker-worker` (lane k-split): setup, run both sides, compare, reset.
//! Move-only refactor of the single-file worker; behaviour unchanged.
use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{BufRead, Write};
use std::sync::mpsc;
use std::time::Duration;
use crate::protocol::*;
use crate::state::*;
use crate::image::*;
use crate::execution::*;
use crate::interception::*;


// Compare two observations per the checks object. Returns (pass, checks_json, first_mismatch).
pub(crate) fn compare(a: &Obs, b: &Obs, checks: &J) -> (bool, String, String) {
    let (pass, json, first) = compare_inner(a, b, checks);
    if pass {
        return (true, json, first);
    }
    // v4 NaN diagnostic (lane r-b91's case, NOT their fix: all-NaN-equal is
    // unsound and stays out). If the trial fails but passes with every NaN
    // canonicalized (sign/payload erased), the only difference is NaN bits
    // and the verdict says so, pointing at operand order. The trial still
    // fails; this is text only, never a pass.
    let ca = canon_nan_obs(a);
    let cb = canon_nan_obs(b);
    if compare_inner(&ca, &cb, checks).0 {
        let diag = "[nan-diagnostic: sides differ ONLY in NaN sign/payload bits; check operand order in the rewrite]";
        let first = if first.is_empty() {
            diag.to_string()
        } else {
            format!("{} {}", first, diag)
        };
        return (false, json, first);
    }
    (false, json, first)
}


pub(crate) fn is_f32_nan(v: u32) -> bool {
    (v & 0x7F800000) == 0x7F800000 && (v & 0x007FFFFF) != 0
}


pub(crate) fn canon_f32(v: u32) -> u32 {
    if is_f32_nan(v) { 0x7FC00000 } else { v }
}


// Canonicalize every float-comparable word for the NaN diagnostic recheck:
// call args, snapshots and call vector words (f32 lanes), the st0/xmm0
// return channels, pointed-to globals, and the heap/stack hashes (via the
// canonical hashes run_side maintains over the full write stream, since the
// kept write prefixes are truncated and cannot be re-hashed).
pub(crate) fn canon_nan_obs(o: &Obs) -> Obs {
    let mut c = o.clone();
    for rec in c.calls.iter_mut() {
        for w in rec.3.iter_mut() {
            *w = canon_f32(*w);
        }
        for w in rec.4.iter_mut() {
            *w = canon_f32(*w);
        }
        for w in rec.5.iter_mut() {
            *w = canon_f32(*w);
        }
        for w in rec.6.iter_mut() {
            *w = canon_f32(*w);
        }
        rec.7 = canon_f32(rec.7);
    }
    c.st0 = canon_st0_hex(&c.st0);
    c.xmm0 = canon_xmm0_hex(&c.xmm0);
    for (_, v) in c.globals_writes.iter_mut() {
        *v = canon_f32(*v);
    }
    c.heap_hash = c.heap_chash;
    c.stack_hash = c.stack_chash;
    c
}


pub(crate) fn unhex(h: &str) -> Vec<u8> {
    let mut v = Vec::new();
    let b = h.as_bytes();
    let mut i = 0;
    while i + 1 < b.len() {
        let hi = (b[i] as char).to_digit(16).unwrap_or(0);
        let lo = (b[i + 1] as char).to_digit(16).unwrap_or(0);
        v.push((hi * 16 + lo) as u8);
        i += 2;
    }
    v
}


pub(crate) fn canon_st0_hex(h: &str) -> String {
    let b = unhex(h);
    if b.len() < 10 {
        return h.to_string();
    }
    let man = u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]);
    let ex = u16::from_le_bytes([b[8], b[9]]);
    if (ex & 0x7FFF) == 0x7FFF && (man & 0x7FFFFFFFFFFFFFFF) != 0 {
        "00000000000000c0ff7f".to_string() // +qNaN, zero payload
    } else {
        h.to_string()
    }
}


pub(crate) fn canon_xmm0_hex(h: &str) -> String {
    let b = unhex(h);
    if b.len() < 16 {
        return h.to_string();
    }
    let mut b = b;
    let w0 = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    if is_f32_nan(w0) {
        b[0..4].copy_from_slice(&0x7FC00000u32.to_le_bytes());
    }
    let d0 = u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]);
    if (d0 & 0x7FF0000000000000) == 0x7FF0000000000000
        && (d0 & 0x000FFFFFFFFFFFFF) != 0
    {
        b[0..8].copy_from_slice(&0x7FF8000000000000u64.to_le_bytes());
    }
    hexbytes(&b)
}


pub(crate) fn compare_inner(a: &Obs, b: &Obs, checks: &J) -> (bool, String, String) {
    let mut parts: Vec<String> = Vec::new();
    let mut first = String::new();
    let mut ok_all = true;
    let mut chk = |name: &str, pass: bool, detail: String| {
        if !pass {
            ok_all = false;
            if first.is_empty() {
                first = format!("{}: {}", name, detail);
            }
        }
        parts.push(format!(
            "{{\"name\":\"{}\",\"passed\":{},\"detail\":\"{}\"}}",
            name,
            if pass { "true" } else { "false" },
            esc(&detail)
        ));
    };
    let want = |k: &str, d: bool| checks.get(k).map(|v| v.as_bool(d)).unwrap_or(d);
    // termination
    if want("fault", true) {
        let same = a.status == b.status || (a.status == "ok" && b.status == "ok");
        // fault codes must match too when both fault
        let pass = if a.status == "fault" && b.status == "fault" {
            a.fault_code == b.fault_code
        } else {
            same
        };
        chk(
            "termination",
            pass,
            format!("orig={} rw={} {}", a.status, b.status, b.fault),
        );
    }
    // cheat is always a failure for the rewrite side
    if b.status == "cheat" {
        chk(
            "no_cheat",
            false,
            format!("rewrite touched original code: {}", b.fault),
        );
    } else {
        chk("no_cheat", true, String::new());
    }
    if a.status != "ok" || b.status != "ok" {
        // fault-vs-fault with same code passes termination; nothing else to compare
        return (ok_all, format!("[{}]", parts.join(",")), first);
    }
    // return value
    let ret = checks.get("ret").map(|v| v.as_str()).unwrap_or("eax");
    match ret {
        "none" => chk("ret", true, String::new()),
        "al" => {
            let p = (a.regs[0] & 0xFF) == (b.regs[0] & 0xFF);
            chk(
                "ret",
                p,
                format!(
                    "al orig=0x{:x} rw=0x{:x}",
                    a.regs[0] & 0xFF,
                    b.regs[0] & 0xFF
                ),
            );
        }
        "ax" => {
            let p = (a.regs[0] & 0xFFFF) == (b.regs[0] & 0xFFFF);
            chk(
                "ret",
                p,
                format!(
                    "ax orig=0x{:x} rw=0x{:x}",
                    a.regs[0] & 0xFFFF,
                    b.regs[0] & 0xFFFF
                ),
            );
        }
        "edx_eax" => {
            let p = a.regs[0] == b.regs[0] && a.regs[2] == b.regs[2];
            chk(
                "ret",
                p,
                format!(
                    "edx:eax orig=0x{:x}:0x{:x} rw=0x{:x}:0x{:x}",
                    a.regs[2], a.regs[0], b.regs[2], b.regs[0]
                ),
            );
        }
        "st0" => {
            let (p, d, _exact) = fp_cmp(&a.st0, &b.st0, 10, checks);
            chk("ret", p, format!("st0 {}", d));
        }
        "xmm0" => {
            let (p, d, _exact) = fp_cmp(&a.xmm0, &b.xmm0, 16, checks);
            chk("ret", p, format!("xmm0 {}", d));
        }
        _ => {
            let p = a.regs[0] == b.regs[0];
            chk(
                "ret",
                p,
                format!("eax orig=0x{:x} rw=0x{:x}", a.regs[0], b.regs[0]),
            );
        }
    }
    if want("esp", true) {
        let p = a.esp_delta == b.esp_delta;
        chk(
            "esp",
            p,
            format!("delta orig={} rw={}", a.esp_delta, b.esp_delta),
        );
    }
    if want("heap", true) {
        let p = a.heap_n == b.heap_n && a.heap_hash == b.heap_hash;
        let mut d = format!("n orig={} rw={}", a.heap_n, b.heap_n);
        if !p {
            d.push_str(&format!(
                " orig_first={:?} rw_first={:?}",
                a.heap_writes.iter().take(4).collect::<Vec<_>>(),
                b.heap_writes.iter().take(4).collect::<Vec<_>>()
            ));
        }
        chk("heap", p, d);
    }
    if want("stack", true) {
        let p = a.stack_n == b.stack_n && a.stack_hash == b.stack_hash;
        chk("stack", p, format!("n orig={} rw={}", a.stack_n, b.stack_n));
    }
    if want("globals", true) {
        let p = a.globals_writes == b.globals_writes;
        chk(
            "globals",
            p,
            format!(
                "orig={:?} rw={:?}",
                lim(&a.globals_writes, 4),
                lim(&b.globals_writes, 4)
            ),
        );
    }
    if want("calls", true) {
        // Only the callee's real inputs participate: id + declared reg args
        // (cdecl/stdcall: none; thiscall: ecx; fastcall: ecx+edx; override
        // via checks.call_regs {id:[regs]}). Scratch regs at the call site
        // differ legitimately between original and rewrite codegen.
        // v4 fail-closed validations run before the key comparison: a zero
        // mask, an eax reg without its transport, or a filled call log all
        // mean calls went uncompared, so the check fails loudly.
        if let Some(z) = find_zero_mask(checks) {
            chk("calls", false, z);
        } else if let Some(e) = validate_eax_regs(checks) {
            chk("calls", false, e);
        } else if let Some(e) = validate_eax_transport(checks) {
            chk("calls", false, e);
        } else if a.log_attempted > a.log_logged || b.log_attempted > b.log_logged {
            chk(
                "calls",
                false,
                format!(
                    "call log truncated: orig attempted {} logged {}; rw attempted {} logged {} (cap {}; raise top-level log_max, max {})",
                    a.log_attempted,
                    a.log_logged,
                    b.log_attempted,
                    b.log_logged,
                    // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
                    st().log_max,
                    LOG_HARD_MAX
                ),
            );
        } else {
            let ka: Vec<String> = a.calls.iter().map(|c| callkey(c, checks)).collect();
            let kb: Vec<String> = b.calls.iter().map(|c| callkey(c, checks)).collect();
            let p = ka == kb;
            chk(
                "calls",
                p,
                format!("orig={:?} rw={:?}", lims(&ka, 3), lims(&kb, 3)),
            );
        }
    }
    if want("undeclared", true) {
        let p = a.undeclared_n == 0 && b.undeclared_n == 0;
        chk(
            "undeclared",
            p,
            format!(
                "orig_n={} rw_n={} rw_first={:?}",
                a.undeclared_n,
                b.undeclared_n,
                lim(&b.undeclared, 4)
            ),
        );
    }
    (ok_all, format!("[{}]", parts.join(",")), first)
}


// v2 pointer normalization: an address inside one of the worker's fixed
// windows compares as a region-relative offset, so a pointer to the
// function's own frame, the trial heap or the image matches when it refers
// to the same place on both sides even though raw addresses differ.
pub(crate) fn norm_ptr(v: u32) -> String {
    // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
    let s = st();
    let h = s.h as u32;
    if h <= v && v < h.wrapping_add(HEAP_USE as u32) {
        return format!("H+{:x}", v - h);
    }
    let sb = s.s as u32;
    if sb <= v && v < sb.wrapping_add(SSIZE as u32) {
        if v >= s.esp0 {
            return format!("S+{:x}", v - s.esp0);
        } else {
            return format!("S-{:x}", s.esp0 - v);
        }
    }
    let img = s.img as u32;
    if img <= v && v < img.wrapping_add(s.image_size as u32) {
        return format!("I+{:x}", v - img);
    }
    format!("0x{:x}", v)
}


// v4: a zero mask compares nothing (every value masks to 0), so it is
// rejected instead of comparing. Returns the failure detail, if any.
pub(crate) fn find_zero_mask(checks: &J) -> Option<String> {
    let m = checks.get("call_mask")?;
    let obj = match m {
        J::Obj(o) => o,
        _ => return None,
    };
    for (id, per) in obj {
        let per = match per {
            J::Obj(o) => o,
            _ => continue,
        };
        for (idx, mask) in per {
            if mask.as_u32() == 0 {
                return Some(format!(
                    "call_mask {}.{} is zero (rejected: a zero mask compares nothing)",
                    id, idx
                ));
            }
        }
    }
    None
}


// v4: "eax" in checks.call_regs is only meaningful with the callee's
// eax_from_stack transport (a rewrite cannot set eax any other way, and
// stubs without it never log eax). Without it the reg would compare
// stale zeros on both sides and pass vacuously: reject instead.
pub(crate) fn validate_eax_regs(checks: &J) -> Option<String> {
    let m = checks.get("call_regs")?;
    let obj = match m {
        J::Obj(o) => o,
        _ => return None,
    };
    for (id, regs) in obj {
        let wants_eax = regs.as_arr().iter().any(|r| r.as_str() == "eax");
        if !wants_eax {
            continue;
        }
        let cid = id.parse::<u32>().unwrap_or(u32::MAX);
        // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
        let ok = st()
            .callees
            .get(&cid)
            .map(|c| c.eax_from_stack.is_some())
            .unwrap_or(false);
        if !ok {
            return Some(format!(
                "call_regs {} selects eax without eax_from_stack transport (rejected: eax would be uncompared)",
                id
            ));
        }
    }
    None
}


// v4 (coordinator review): the reverse must hold as well. A callee with the
// eax transport has its stack arguments left out of the call key, so unless
// call_regs selects "eax" for it the transported argument would be compared
// nowhere. Reject instead of passing with it uncompared.
pub(crate) fn validate_eax_transport(checks: &J) -> Option<String> {
    // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
    let mut ids: Vec<u32> = st()
        .callees
        .iter()
        .filter(|(_, c)| c.eax_from_stack.is_some())
        .map(|(id, _)| *id)
        .collect();
    ids.sort();
    for id in ids {
        let selected = checks
            .get("call_regs")
            .and_then(|m| m.get(id.to_string().as_str()))
            .map(|regs| regs.as_arr().iter().any(|r| r.as_str() == "eax"))
            .unwrap_or(false);
        if !selected {
            return Some(format!(
                "callee {} has eax_from_stack but call_regs does not select eax (rejected: the transported argument would be uncompared)",
                id
            ));
        }
    }
    None
}


pub(crate) fn callkey(c: &CallRec, checks: &J) -> String {
    let id = c.0.to_string();
    // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
    let cal = st().callees.get(&c.0).cloned().unwrap_or_default();
    let regs: Vec<String> = match checks.get("call_regs").and_then(|m| m.get(id.as_str())) {
        Some(v) => v.as_arr().iter().map(|r| r.as_str().to_string()).collect(),
        None => match cal.conv.as_str() {
            "thiscall" => vec!["ecx".to_string()],
            "fastcall" => vec!["ecx".to_string(), "edx".to_string()],
            _ => Vec::new(),
        },
    };
    let mut k = format!("id={}", c.0);
    for r in &regs {
        match r.as_str() {
            "ecx" => k.push_str(&format!(" ecx={}", norm_ptr(c.1))),
            "edx" => k.push_str(&format!(" edx={}", norm_ptr(c.2))),
            "eax" => k.push_str(&format!(" eax={}", norm_ptr(c.7))), // v4 (r-s111)
            _ => {}
        }
    }
    // v2: stack args are normalized too; volatile indexes can be dropped
    // with checks.call_skip {id:[idx,...]} (folded from lane q-16). Args of
    // a transport callee are skipped: the original side pushes nothing.
    let skip: Vec<usize> = match checks.get("call_skip").and_then(|m| m.get(id.as_str())) {
        Some(v) => v.as_arr().iter().map(|x| x.as_usize()).collect(),
        None => Vec::new(),
    };
    if cal.xmm0_from_stack.is_none()
        && cal.xmm1_from_stack.is_none()
        && cal.eax_from_stack.is_none()
    {
        // v4 per-argument masks (lanes r-b39, r-n117, r-n86; alias
        // checks.call_low8 from lane r-n118): checks.call_mask
        // {id:{idx:mask}} compares (value & mask) as raw hex instead of the
        // normalized pointer. Use: the original pushes a one-byte value as
        // a full word whose upper bytes are caller leftovers (not
        // behaviour); the mask keeps the meaningful bytes compared exactly,
        // unlike call_skip which drops the arg. A full (all-ones) mask is
        // the default comparison. Zero masks never reach here (rejected
        // above). The alias maps each listed index to 0xFF; an explicit
        // call_mask entry wins over the alias for the same argument.
        let mut masks: HashMap<usize, u32> = HashMap::new();
        if let Some(v) = checks.get("call_low8").and_then(|m| m.get(id.as_str())) {
            for x in v.as_arr() {
                masks.insert(x.as_usize(), 0xFF);
            }
        }
        if let Some(J::Obj(o)) = checks.get("call_mask").and_then(|m| m.get(id.as_str())) {
            for (idx, m) in o {
                if let Ok(i) = idx.parse::<usize>() {
                    masks.insert(i, m.as_u32());
                }
            }
        }
        let args: Vec<String> = c
            .3
            .iter()
            .enumerate()
            .filter(|(i, _)| !skip.contains(i))
            .map(|(i, &a)| match masks.get(&i) {
                Some(0xFFFF_FFFF) | None => norm_ptr(a),
                Some(m) => format!("0x{:x}", a & m),
            })
            .collect();
        k.push_str(&format!(" args={:?}", args));
    } else {
        k.push_str(" args=transport");
    }
    // Pointed-to snapshots compare by value (raw words: contents, not addresses).
    if !c.4.is_empty() {
        k.push_str(&format!(" snap={:?}", c.4));
    }
    if cal.logxmm {
        k.push_str(&format!(" xmm0={:?}", c.5));
    }
    if cal.logxmm1 {
        k.push_str(&format!(" xmm1={:?}", c.6));
    }
    k
}


pub(crate) fn lims(v: &[String], n: usize) -> Vec<String> {
    v.iter().take(n).cloned().collect()
}


pub(crate) fn lim(v: &[(u32, u32)], n: usize) -> Vec<String> {
    v.iter()
        .take(n)
        .map(|(a, b)| format!("0x{:x}=0x{:x}", a, b))
        .collect()
}


// FP compare: exact bits first; else f32/f64 abs+rel tolerance. Returns (pass, detail, exact).
pub(crate) fn fp_cmp(ah: &str, bh: &str, nbytes: usize, checks: &J) -> (bool, String, bool) {
    if ah == bh {
        return (true, "exact".to_string(), true);
    }
    let unhex = |h: &str| -> Vec<u8> {
        let mut v = Vec::new();
        let b = h.as_bytes();
        let mut i = 0;
        while i + 1 < b.len() {
            let hi = (b[i] as char).to_digit(16).unwrap_or(0);
            let lo = (b[i + 1] as char).to_digit(16).unwrap_or(0);
            v.push((hi * 16 + lo) as u8);
            i += 2;
        }
        v
    };
    let ab = unhex(ah);
    let bb = unhex(bh);
    let mut detail = format!("orig={} rw={} ", ah, bh);
    if ab.len() < nbytes || bb.len() < nbytes {
        return (false, detail + "short", false);
    }
    // Bitwise difference with zero tolerance is a failure, full stop.
    // (IEEE == would call -0.0 and +0.0 equal; the checker must not.)
    let tol = checks.get("fp_tol").map(|v| v.as_str()).unwrap_or("");
    let (tabs, trel) = if tol.is_empty() {
        (0.0f64, 0.0f64)
    } else {
        let mut sp = tol.split(',');
        let a = sp.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let r = sp.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        (a, r)
    };
    if tabs == 0.0 && trel == 0.0 {
        return (false, detail + "bitwise differ, zero tolerance", false);
    }
    // f32 lane (low 4 bytes little-endian) and f64 lane (low 8)
    let a32 = f32::from_le_bytes([ab[0], ab[1], ab[2], ab[3]]);
    let b32 = f32::from_le_bytes([bb[0], bb[1], bb[2], bb[3]]);
    let (abs, rel) = (tabs, trel);
    let close32 = (a32 == b32)
        || (!a32.is_nan()
            && !b32.is_nan()
            && ((a32 - b32).abs() as f64 <= abs + rel * (b32.abs() as f64)));
    if nbytes == 16
        && checks
            .get("fp64")
            .map(|v| v.as_bool(false))
            .unwrap_or(false)
    {
        let a64 = f64::from_le_bytes([ab[0], ab[1], ab[2], ab[3], ab[4], ab[5], ab[6], ab[7]]);
        let b64 = f64::from_le_bytes([bb[0], bb[1], bb[2], bb[3], bb[4], bb[5], bb[6], bb[7]]);
        let close64 = (a64 == b64)
            || (!a64.is_nan() && !b64.is_nan() && ((a64 - b64).abs() <= abs + rel * b64.abs()));
        detail.push_str(&format!("f64 {} vs {}", a64, b64));
        return (close64, detail, false);
    }
    // x87 st0: 80-bit extended; compare via f64 conversion of the 64-bit mantissa+exponent
    if nbytes == 10 {
        let cvt = |x: &[u8]| -> f64 {
            let man = u64::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]);
            let ex = u16::from_le_bytes([x[8], x[9]]);
            let sign = if ex & 0x8000 != 0 { -1.0 } else { 1.0 };
            let e = (ex & 0x7FFF) as i32;
            if e == 0 && man == 0 {
                return 0.0 * sign;
            }
            if e == 0x7FFF {
                return if man & 0x7FFFFFFFFFFFFFFF == 0 {
                    sign * f64::INFINITY
                } else {
                    f64::NAN
                };
            }
            sign * (man as f64) * 2f64.powi(e - 16383 - 63)
        };
        let a = cvt(&ab);
        let b = cvt(&bb);
        let close =
            (a == b) || (!a.is_nan() && !b.is_nan() && ((a - b).abs() <= abs + rel * b.abs()));
        detail.push_str(&format!("st0 {} vs {}", a, b));
        return (close, detail, false);
    }
    detail.push_str(&format!("f32 {} vs {}", a32, b32));
    (close32, detail, false)
}


pub(crate) fn cmd_setup(s: &mut State, q: &J) -> String {
    if !s.mapped {
        let exe_path = q.get("exe").map(|v| v.as_str()).unwrap_or("");
        if exe_path.is_empty() {
            return "{\"ok\":false,\"error\":\"no exe\"}".to_string();
        }
        let bytes = std::fs::read(exe_path).unwrap_or_else(|_| Vec::new());
        if bytes.is_empty() {
            return "{\"ok\":false,\"error\":\"cannot read exe\"}".to_string();
        }
        s.before_stack = vec![0u32; STACK_SNAP_LEN / 4];
        s.exe_bytes = bytes.clone();
        if let Err(e) = map_image(s, &bytes) {
            return format!("{{\"ok\":false,\"error\":\"{}\"}}", esc(&e));
        }
    }
    // v4: per-setup call-log cap (default 256). The region always
    // reserves LOG_HARD_MAX entries, so raising needs no layout change.
    let log_max = q.get("log_max").map(|v| v.as_u32()).unwrap_or(LOG_MAX as u32);
    if log_max == 0 || log_max as usize > LOG_HARD_MAX {
        return format!(
            "{{\"ok\":false,\"error\":\"log_max {} out of range 1..{}\"}}",
            log_max, LOG_HARD_MAX
        );
    }
    s.log_max = log_max;
    // callees (stub space is reused per setup, but the first 16 bytes
    // hold the emitted TLS/ESP helpers and are never reused)
    s.callees.clear();
    s.stub_off = 16;
    if let Some(cs) = q.get("callees") {
        for c in cs.as_arr() {
            let id = c.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let conv = c
                .get("conv")
                .map(|v| v.as_str())
                .unwrap_or("cdecl")
                .to_string();
            let nargs = c.get("nargs").map(|v| v.as_usize()).unwrap_or(0);
            if nargs > LOG_MAXW {
                return format!(
                    "{{\"ok\":false,\"error\":\"callee {} nargs {} exceeds log window {}\"}}",
                    id, nargs, LOG_MAXW
                );
            }
            let ret = c
                .get("ret")
                .map(|v| v.as_str())
                .unwrap_or("u32")
                .to_string();
            // (Setup-time scripts are vestigial: per-trial answers arrive
            // with each trial request, so the setup list is not stored.)
            // v3: callee-cleaned stubs pop the full argument count. (v2 capped
            // the cleanup at 8 words like the log window, under-popping for
            // nargs > 8 and corrupting the caller's frame: lane r-b01's fix.)
            // v4 (lane r-s94): "noclean" callees pop nothing on the original
            // side; the stub pops pop_rw only on the rewrite side.
            let noclean = c.get("noclean").map(|v| v.as_bool(false)).unwrap_or(false);
            let pop_rw = (nargs as u32) * 4;
            let mut pop = match conv.as_str() {
                "cdecl" => 0,
                _ => (nargs as u32) * 4, // stdcall/thiscall/fastcall/custom pop
            };
            if noclean {
                pop = 0;
            }
            // v2: out-param writes [{arg|reg, at, n}], snapshots, xmm logging.
            let mut writes = Vec::new();
            if let Some(ws) = c.get("writes") {
                for w in ws.as_arr() {
                    let (kind, idx) = if let Some(r) = w.get("reg") {
                        match r.as_str() {
                            "edx" => (2u8, 0usize),
                            _ => (1u8, 0usize),
                        }
                    } else {
                        (0u8, w.get("arg").map(|v| v.as_usize()).unwrap_or(0))
                    };
                    let at = w.get("at").map(|v| v.as_usize()).unwrap_or(0);
                    let n = w.get("n").map(|v| v.as_usize()).unwrap_or(0);
                    if at + n > WRITEW_PER_ID {
                        return format!(
                            "{{\"ok\":false,\"error\":\"callee {} writes overflow\"}}",
                            id
                        );
                    }
                    // v4 (lane r-b109): optional destination byte offset
                    // added to the pointer (default 0 = v3 behavior exactly).
                    let dst = w.get("dst").map(|v| v.as_usize()).unwrap_or(0);
                    writes.push((kind, idx, at, n, dst));
                }
            }
            let mut snap = Vec::new();
            let mut snap_total = 0usize;
            if let Some(ss) = c.get("snap") {
                for sn in ss.as_arr() {
                    let kind = sn.get("kind").map(|v| v.as_str()).unwrap_or("arg");
                    let k = match kind {
                        "ecx" => 1u8,
                        "edx" => 2u8,
                        _ => 0u8,
                    };
                    let idx = sn.get("idx").map(|v| v.as_usize()).unwrap_or(0);
                    let n = sn.get("n").map(|v| v.as_usize()).unwrap_or(0);
                    snap_total += n;
                    snap.push((k, idx, n));
                }
            }
            if snap_total > SNAP_MAXW {
                return format!("{{\"ok\":false,\"error\":\"callee {} snap overflow\"}}", id);
            }
            let logxmm = c.get("logxmm").map(|v| v.as_bool(false)).unwrap_or(false);
            let xmm0_from_stack = c.get("xmm0_from_stack").map(|v| v.as_usize());
            let logxmm1 = c.get("logxmm1").map(|v| v.as_bool(false)).unwrap_or(false);
            let xmm1_from_stack = c.get("xmm1_from_stack").map(|v| v.as_usize());
            let preserve = c.get("preserve").map(|v| v.as_bool(false)).unwrap_or(false);
            let eax_from_stack = c.get("eax_from_stack").map(|v| v.as_usize());
            let cal = Callee {
                id,
                conv,
                nargs,
                pop,
                ret,
                stub_addr: 0,
                tail_addr: 0,
                writes,
                snap,
                logxmm,
                xmm0_from_stack,
                logxmm1,
                xmm1_from_stack,
                preserve,
                eax_from_stack,
                noclean,
                pop_rw,
            };
            let bytes = emit_stub(&cal, None, s);
            let addr = s.stub_base + s.stub_off as u32;
            if s.stub_off + bytes.len() > SB_STUB_MAX {
                return "{\"ok\":false,\"error\":\"stub overflow\"}".to_string();
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
                let proc = GetCurrentProcess();
                FlushInstructionCache(proc, addr as *const c_void, bytes.len());
                *((s.ctable + id * 4) as *mut u32) = addr;
            }
            s.stub_off += (bytes.len() + 15) & !15;
            let mut cal = cal;
            cal.stub_addr = addr;
            s.callees.insert(id, cal);
        }
    }
    // E8 patches
    let mut errors: Vec<String> = Vec::new();
    if let Some(ps) = q.get("patches") {
        for p in ps.as_arr() {
            let site_rva = p.get("site").map(|v| v.as_u32()).unwrap_or(0);
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let stub = match s.callees.get(&id) {
                Some(c) => c.stub_addr,
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            let site = s.img + site_rva as usize;
            match patch_e8(site, stub) {
                Ok(orig) => s.patches.push((site, orig, 5)),
                Err(e) => errors.push(e),
            }
        }
    }
    // v2 E9 tail-jump patches: the tail stub logs like a normal stub
    // but returns straight to the trampoline (outer_pop + the E8 slot).
    let outer_pop = q.get("outer_pop").map(|v| v.as_u32()).unwrap_or(0);
    s.last_outer_pop = outer_pop; // k-split dump-only record
    if let Some(ps) = q.get("tailpatches") {
        for p in ps.as_arr() {
            let site_rva = p.get("site").map(|v| v.as_u32()).unwrap_or(0);
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let cal = match s.callees.get(&id) {
                Some(c) => c.clone(),
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            let tail = if cal.tail_addr != 0 {
                cal.tail_addr
            } else {
                let bytes = emit_stub(&cal, Some(outer_pop), s);
                let addr = s.stub_base + s.stub_off as u32;
                if s.stub_off + bytes.len() > SB_STUB_MAX {
                    return "{\"ok\":false,\"error\":\"stub overflow\"}".to_string();
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
                    let proc = GetCurrentProcess();
                    FlushInstructionCache(proc, addr as *const c_void, bytes.len());
                }
                s.stub_off += (bytes.len() + 15) & !15;
                if let Some(c) = s.callees.get_mut(&id) {
                    c.tail_addr = addr;
                }
                addr
            };
            let site = s.img + site_rva as usize;
            match patch_e9(site, tail) {
                Ok(orig) => s.patches.push((site, orig, 5)),
                Err(e) => errors.push(e),
            }
        }
    }
    // IAT patches
    let mut iat_done: Vec<String> = Vec::new();
    if let Some(ps) = q.get("iat") {
        for p in ps.as_arr() {
            let dll = p.get("dll").map(|v| v.as_str()).unwrap_or("");
            let name = p.get("name").map(|v| v.as_str()).unwrap_or("");
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let stub = match s.callees.get(&id) {
                Some(c) => c.stub_addr,
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            match find_iat(s, dll, name) {
                Some(slot) => unsafe {
                    let mut orig = [0u8; 8];
                    std::ptr::copy_nonoverlapping(slot as *const u8, orig.as_mut_ptr(), 4);
                    *(slot as *mut u32) = stub;
                    s.patches.push((slot, orig, 4));
                    iat_done.push(format!("{}!{}", dll, name));
                },
                None => errors.push(format!("iat {}!{} not found", dll, name)),
            }
        }
    }
    // declared globals
    s.globals.clear();
    if let Some(gs) = q.get("globals") {
        for g in gs.as_arr() {
            let rva = g.get("rva").map(|v| v.as_u32()).unwrap_or(0);
            let size = g.get("size").map(|v| v.as_usize()).unwrap_or(0);
            s.globals.push((s.img + rva as usize, size));
        }
    }
    // DLL
    let mut dll_base = String::from("null");
    let mut expmap = String::new();
    if let Some(d) = q.get("dll").map(|v| v.as_str()).filter(|x| !x.is_empty()) {
        if !s.dll.is_null() {
            unsafe {
                FreeLibrary(s.dll);
            }
            s.dll = std::ptr::null_mut();
        }
        s.exports.clear();
        let w = wide(d);
        let h = unsafe { LoadLibraryW(w.as_ptr()) };
        if h.is_null() {
            errors.push(format!("LoadLibrary failed: {}", d));
        } else {
            s.dll = h;
            dll_base = format!("\"0x{:x}\"", h as usize);
            // patch CHECKER_XBASE + CHECKER_CTABLE + CHECKER_XMM data exports
            // (CHECKER_XMM is v2-only; older lane DLLs lack it and report an
            // error string here, which is harmless unless the contract uses it)
            for (sym, val) in [
                ("CHECKER_XBASE\0", s.img as u32),
                ("CHECKER_CTABLE\0", s.ctable),
                ("CHECKER_XMM\0", s.m_xmm_mirror),
                ("CHECKER_TLS\0", s.m_tls_mirror),
            ] {
                let p = unsafe { GetProcAddress(h, sym.as_ptr() as *const i8) };
                if !p.is_null() {
                    unsafe {
                        *(p as *mut u32) = val;
                    }
                } else {
                    errors.push(format!("dll export {} missing", sym.trim_end_matches('\0')));
                }
            }
            let mut parts = Vec::new();
            if let Some(es) = q.get("exports") {
                for e in es.as_arr() {
                    let name = e.as_str().to_string();
                    let sym = format!("{}\0", name);
                    let p = unsafe { GetProcAddress(h, sym.as_ptr() as *const i8) };
                    if p.is_null() {
                        errors.push(format!("export {} missing", name));
                    } else {
                        s.exports.insert(name.clone(), p as u32);
                        parts.push(format!("\"{}\":\"0x{:x}\"", name, p as usize));
                    }
                }
            }
            expmap = parts.join(",");
        }
    }
    // v2: stub addresses for driver-side planting (vtables, data slots).
    let mut stubs: Vec<(&u32, &Callee)> = s.callees.iter().collect();
    stubs.sort_by_key(|(id, _)| *id);
    let stubmap = stubs
        .iter()
        .map(|(id, c)| format!("\"{}\":\"0x{:x}\"", id, c.stub_addr))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"ok\":true,\"img_base\":{},\"delta\":{},\"relocs\":{},\"heap\":{},\"stack\":{},\"dll_base\":{},\"text_lo\":{},\"text_hi\":{},\"exports\":{{{}}},\"stub_addrs\":{{{}}},\"iat_patched\":[{}],\"errors\":[{}]}}",
        hx(s.img as u32),
        hx(s.delta),
        s.relocs,
        hx(s.h as u32),
        hx(s.s as u32),
        dll_base,
        hx(s.text_lo as u32),
        hx(s.text_hi as u32),
        expmap,
        stubmap,
        iat_done
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(","),
        errors
            .iter()
            .map(|x| format!("\"{}\"", esc(x)))
            .collect::<Vec<_>>()
            .join(",")
    )
}


// k-split debug-only byte dump (the one permitted addition): re-emits every
// stub, tail stub and patch descriptor for the current setup and returns
// their hashes plus full bytes, so a split build can be proven
// byte-identical. Not part of the checker protocol; the driver never sends
// it during trials, and it changes no worker state.
pub(crate) fn bytes_fnv(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in data {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001B3);
    }
    h
}


pub(crate) fn cmd_dump_bytes(s: &State) -> String {
    let mut ids: Vec<u32> = s.callees.keys().cloned().collect();
    ids.sort();
    let mut stub_parts = Vec::new();
    for id in &ids {
        let cal = s.callees.get(id).cloned().unwrap_or_default();
        let bytes = emit_stub(&cal, None, s);
        stub_parts.push(format!(
            "\"{}\":{{\"len\":{},\"fnv\":\"0x{:x}\",\"hex\":\"{}\",\"addr\":{}}}",
            id,
            bytes.len(),
            bytes_fnv(&bytes),
            hexbytes(&bytes),
            hx(cal.stub_addr)
        ));
        if cal.tail_addr != 0 {
            let tbytes = emit_stub(&cal, Some(s.last_outer_pop), s);
            stub_parts.push(format!(
                "\"{}t\":{{\"len\":{},\"fnv\":\"0x{:x}\",\"hex\":\"{}\",\"addr\":{}}}",
                id,
                tbytes.len(),
                bytes_fnv(&tbytes),
                hexbytes(&tbytes),
                hx(cal.tail_addr)
            ));
        }
    }
    let mut patch_parts = Vec::new();
    for (addr, orig, len) in &s.patches {
        let cur: Vec<u8> =
            unsafe { std::slice::from_raw_parts(*addr as *const u8, *len).to_vec() };
        patch_parts.push(format!(
            "{{\"site\":{},\"len\":{},\"orig\":\"{}\",\"cur\":\"{}\"}}",
            hx((*addr - s.img) as u32),
            len,
            hexbytes(&orig[..*len]),
            hexbytes(&cur)
        ));
    }
    format!(
        "{{\"ok\":true,\"log_max\":{},\"outer_pop\":{},\"stubs\":{{{}}},\"patches\":[{}]}}",
        s.log_max,
        s.last_outer_pop,
        stub_parts.join(","),
        patch_parts.join(",")
    )
}


pub(crate) fn cmd_teardown(s: &mut State) -> String {
    for (addr, orig, len) in s.patches.clone() {
        unsafe {
            std::ptr::copy_nonoverlapping(orig.as_ptr(), addr as *mut u8, len);
        }
    }
    s.patches.clear();
    unsafe {
        std::ptr::write_bytes(s.ctable as *mut u8, 0, CTABLE_SIZE);
    }
    s.callees.clear();
    s.globals.clear();
    if !s.dll.is_null() {
        unsafe {
            FreeLibrary(s.dll);
        }
        s.dll = std::ptr::null_mut();
    }
    s.exports.clear();
    restore_data();
    "{\"ok\":true}".to_string()
}


pub(crate) fn parse_trial(s: &State, q: &J) -> Result<(TrialReq, J), String> {
    let fn_rva = q.get("fn_rva").map(|v| v.as_u32()).unwrap_or(0);
    let fn_addr = (s.img + fn_rva as usize) as u32;
    let rw_addr = if let Some(e) = q.get("export") {
        *s.exports
            .get(e.as_str())
            .ok_or_else(|| format!("unknown export {}", e.as_str()))?
    } else if let Some(a) = q.get("rw_rva") {
        (s.img + a.as_u32() as usize) as u32
    } else {
        return Err("trial needs export or rw_rva".to_string());
    };
    let regs_v = q
        .get("regs")
        .map(|v| v.as_arr().to_vec())
        .unwrap_or_default();
    if regs_v.len() != 7 {
        return Err("regs must have 7 entries".to_string());
    }
    let mut regs = [0u32; 7];
    for i in 0..7 {
        regs[i] = regs_v[i].as_u32();
    }
    let stack: Vec<u32> = q
        .get("stack")
        .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
        .unwrap_or_default();
    let mut heapsegs = Vec::new();
    if let Some(hs) = q.get("heapsegs") {
        for hseg in hs.as_arr() {
            let off = hseg.get("off").map(|v| v.as_usize()).unwrap_or(0);
            let words: Vec<u32> = hseg
                .get("words")
                .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
                .unwrap_or_default();
            if off + words.len() * 4 > HEAP_USE {
                return Err("heapseg out of range".to_string());
            }
            heapsegs.push((off, words));
        }
    }
    let mut globals_fill = Vec::new();
    if let Some(gf) = q.get("globals_fill") {
        if gf.as_str() == "random" {
            // deterministic per-trial fill of declared ranges
            let trial = q.get("trial").map(|v| v.as_u32()).unwrap_or(0);
            let seed = q.get("seed").map(|v| v.as_u32()).unwrap_or(0);
            for (ri, &(lo, len)) in s.globals.iter().enumerate() {
                let mut words = Vec::new();
                for wi in 0..(len / 4) {
                    words.push(fill_word(seed, trial, ri as u32, wi as u32));
                }
                globals_fill.push((lo, words));
            }
        } else if let Some(arr) = match gf {
            J::Arr(a) => Some(a),
            _ => None,
        } {
            for g in arr {
                let rva = g.get("rva").map(|v| v.as_u32()).unwrap_or(0);
                let words: Vec<u32> = g
                    .get("words")
                    .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
                    .unwrap_or_default();
                globals_fill.push((s.img + rva as usize, words));
            }
        }
        // "pristine" or missing: no fill
    }
    let trial = q.get("trial").map(|v| v.as_u32()).unwrap_or(0);
    let seed = q.get("seed").map(|v| v.as_u32()).unwrap_or(0);
    let checks = q.get("checks").cloned().unwrap_or(J::Obj(HashMap::new()));
    let fulldata = checks
        .get("fulldata")
        .map(|v| v.as_bool(true))
        .unwrap_or(true);
    // v2: fabricated TLS slots [{slot|slot_rva, value}]
    let mut tls = Vec::new();
    if let Some(ts) = q.get("tls") {
        for te in ts.as_arr() {
            let slot = if let Some(rva) = te.get("slot_rva") {
                let addr = s.img + rva.as_u32() as usize;
                unsafe { *((addr & !3) as *const u32) } // slot index word
            } else {
                te.get("slot").map(|v| v.as_u32()).unwrap_or(0)
            };
            let val = te.get("value").map(|v| v.as_u32()).unwrap_or(0);
            if slot < 64 {
                tls.push((slot, val));
            } else {
                return Err("tls slot >= 64 needs expansion slots (unsupported)".to_string());
            }
        }
    }
    // v2: XMM entry values (sparse {reg:[4 dwords]} or dense [32])
    let mut xmm = [0u32; 32];
    if let Some(x) = q.get("xmm") {
        match x {
            J::Arr(a) => {
                for (i, v) in a.iter().take(32).enumerate() {
                    xmm[i] = v.as_u32();
                }
            }
            J::Obj(m) => {
                for (k, v) in m {
                    if let Ok(r) = k.parse::<usize>()
                        && r < 8
                    {
                        for (i, w) in v.as_arr().iter().take(4).enumerate() {
                            xmm[r * 4 + i] = w.as_u32();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    // v2: defined uninitialized-stack fill (integer or "pattern")
    let stack_fill = match q.get("stack_fill") {
        Some(J::Int(i)) => Some(*i as u32),
        Some(J::Str(st)) if st.as_str() != "pattern" => Some(parse_u32(st)),
        _ => None,
    };
    Ok((
        TrialReq {
            fn_addr,
            rw_addr,
            regs,
            stack,
            heapsegs,
            globals_fill,
            trial,
            seed,
            fulldata,
            tls,
            xmm,
            stack_fill,
        },
        checks,
    ))
}


// The trial body runs on a dedicated thread so a hang can be cut off.
pub(crate) fn trial_body(
    req: TrialReq,
    checks: J,
    script_vals: Vec<(u32, u32, u32, Vec<u32>, Vec<(u32, u32)>)>,
) -> String {
    // v2: per-callee script slots + out-param write words, filled once per
    // trial and shared by both sides. v3: per-call answer sequences; a
    // callee without "seq" gets [(script lo,hi)] with length 1, exactly v2.
    unsafe {
        // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
        std::ptr::write_bytes(st().m_script_tab as *mut u8, 0, SCRIPT_TAB_SIZE);
        // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
        std::ptr::write_bytes(st().m_writebuf as *mut u8, 0, WRITEBUF_SIZE);
        // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
        std::ptr::write_bytes(st().m_seq_tab as *mut u8, 0, SEQ_TAB_SIZE);
        // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
        std::ptr::write_bytes(st().m_seq_len as *mut u8, 0, SEQ_LEN_SIZE);
        for (id, lo, hi, w, seq) in &script_vals {
            if *id < 256 {
                // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
                *((st().m_script_tab + id * 8) as *mut u32) = *lo;
                // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
                *((st().m_script_tab + id * 8 + 4) as *mut u32) = *hi;
                for (j, word) in w.iter().take(WRITEW_PER_ID).enumerate() {
                    // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
                    *((st().m_writebuf + id * 64 + j as u32 * 4) as *mut u32) = *word;
                }
                let owned: Vec<(u32, u32)> = if seq.is_empty() {
                    vec![(*lo, *hi)]
                } else {
                    seq[..seq.len().min(SEQ_MAX)].to_vec()
                };
                for (k, (slo, shi)) in owned.iter().enumerate() {
                    // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
                    *((st().m_seq_tab + id * 128 + k as u32 * 8) as *mut u32) = *slo;
                    // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
                    *((st().m_seq_tab + id * 128 + k as u32 * 8 + 4) as *mut u32) = *shi;
                }
                // QUARANTINE: keeps the global accessor: this runs on the trial thread, which cannot receive state from main without synchronization.
                *((st().m_seq_len + id * 4) as *mut u32) = owned.len() as u32;
            }
        }
    }
    let mut f0 = 0i64;
    let mut f1 = 0i64;
    let mut fq = 0i64;
    unsafe {
        QueryPerformanceFrequency(&mut fq);
        QueryPerformanceCounter(&mut f0);
    }
    let orig = run_side(
        req.fn_addr,
        &req.regs,
        &req.stack,
        &req.heapsegs,
        &req.globals_fill,
        req.trial,
        req.seed,
        &req.tls,
        &req.xmm,
        req.stack_fill,
        false,
        req.fulldata,
    );
    let rw = run_side(
        req.rw_addr,
        &req.regs,
        &req.stack,
        &req.heapsegs,
        &req.globals_fill,
        req.trial,
        req.seed,
        &req.tls,
        &req.xmm,
        req.stack_fill,
        true,
        req.fulldata,
    );
    unsafe {
        QueryPerformanceCounter(&mut f1);
    }
    let us = if fq > 0 {
        ((f1 - f0) as f64 * 1e6 / fq as f64) as u64
    } else {
        0
    };
    let (pass, checks_json, first) = compare(&orig, &rw, &checks);
    // exact-fp flag for the driver's exact-match counts
    let ret = checks.get("ret").map(|v| v.as_str()).unwrap_or("eax");
    let exact = match ret {
        "st0" => orig.st0 == rw.st0,
        "xmm0" => orig.xmm0 == rw.xmm0,
        _ => true,
    };
    format!(
        "{{\"ok\":true,\"pass\":{},\"checks\":{},\"first_mismatch\":\"{}\",\"fp_exact\":{},\"trial_us\":{},\"orig\":{},\"rw\":{}}}",
        if pass { "true" } else { "false" },
        checks_json,
        esc(&first),
        if exact { "true" } else { "false" },
        us,
        obs_json(&orig),
        obs_json(&rw)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::*;
    use crate::state::*;
    use crate::trial::*;

    fn checks(s: &str) -> J {
        parse_json(s).expect("checks parse")
    }

    fn full_checks() -> J {
        // the shared fixture holds an eax-transport callee (id 11), which the
        // fail-closed validation requires to be selected whenever calls run
        checks(r#"{"ret":"eax","esp":true,"heap":true,"stack":true,"globals":true,"calls":true,"undeclared":true,"call_regs":{"11":["eax"]}}"#)
    }

    fn ret_only(ret: &str) -> J {
        checks(&format!(
            r#"{{"ret":"{ret}","esp":false,"heap":false,"stack":false,"globals":false,"calls":false,"undeclared":false}}"#
        ))
    }

    fn base_obs() -> Obs {
        let mut o = Obs::default();
        o.status = "ok".to_string();
        o.regs = [1, 2, 3, 4, 5, 6, 7, 0x200FC000];
        o.esp_delta = 0;
        o.heap_n = 3;
        o.heap_hash = 0xABC;
        o.heap_writes = vec![(0, 11), (4, 12)];
        o.stack_n = 1;
        o.stack_hash = 0xDEF;
        o.stack_writes = vec![(0x100, 9)];
        o.globals_writes = vec![(0x100, 9)];
        o.calls = vec![(7, 0, 0, vec![1], vec![], [0; 4], [0; 4], 0)];
        o.log_attempted = 1;
        o.log_logged = 1;
        o
    }

    // ---- pointer normalization ----

    #[test]
    fn norm_ptr_regions() {
        test_install();
        assert_eq!(norm_ptr(0x30000123), "H+123");
        assert_eq!(norm_ptr(0x200FD010), "S+10");
        assert_eq!(norm_ptr(0x200FCFF0), "S-10");
        assert_eq!(norm_ptr(0x10001234), "I+1234");
        assert_eq!(norm_ptr(0x12345678), "0x12345678");
        // window ends are exclusive
        assert_eq!(norm_ptr(0x30000000 + HEAP_USE as u32), "0x300f0000");
        assert_eq!(norm_ptr(0x30000000 + HEAP_USE as u32 - 1), "H+effff");
    }

    // ---- comparison: equal + each channel ----

    #[test]
    fn compare_equal_passes() {
        test_install();
        let (a, b) = (base_obs(), base_obs());
        let (pass, _json, first) = compare(&a, &b, &full_checks());
        assert!(pass, "{first}");
        assert!(first.is_empty());
    }

    #[test]
    fn compare_ret_eax() {
        test_install();
        let (mut a, b) = (base_obs(), base_obs());
        a.regs[0] = 99;
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("ret: "), "{first}");
    }

    #[test]
    fn compare_ret_widths() {
        test_install();
        let (mut a, b) = (base_obs(), base_obs());
        a.regs[0] = 0x1101;
        // low byte equal -> al passes, ax/eax fail
        let mut c = b.clone();
        c.regs[0] = 0x2201;
        assert!(compare(&a, &c, &ret_only("al")).0);
        assert!(!compare(&a, &c, &ret_only("ax")).0);
        assert!(!compare(&a, &c, &ret_only("eax")).0);
        // none compares nothing
        let mut d = b.clone();
        d.regs[0] = 0xDEAD;
        assert!(compare(&a, &d, &ret_only("none")).0);
        // edx_eax covers the high word
        let mut e = b.clone();
        e.regs[2] = 0xBEEF;
        assert!(!compare(&a, &e, &ret_only("edx_eax")).0);
    }

    #[test]
    fn compare_ret_fp_exact() {
        test_install();
        let (mut a, mut b) = (base_obs(), base_obs());
        a.st0 = "00000000000000803f9f".to_string();
        b.st0 = "00000000000000803f9f".to_string();
        assert!(compare(&a, &b, &ret_only("st0")).0);
        b.st0 = "01000000000000803f9f".to_string();
        let (pass, _j, first) = compare(&a, &b, &ret_only("st0"));
        assert!(!pass);
        assert!(first.starts_with("ret: "), "{first}");
        a.xmm0 = "0000803f000000000000000000000000".to_string();
        b.xmm0 = "0000803f000000000000000000000000".to_string();
        assert!(compare(&a, &b, &ret_only("xmm0")).0);
    }

    #[test]
    fn compare_esp() {
        test_install();
        let (mut a, b) = (base_obs(), base_obs());
        a.esp_delta = 4;
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("esp: "), "{first}");
    }

    #[test]
    fn compare_heap_stack_accumulators() {
        test_install();
        // count and hash both accumulate: either differing fails ...
        let (mut a, b) = (base_obs(), base_obs());
        a.heap_n = 4;
        assert!(!compare(&a, &b, &full_checks()).0);
        let (mut a, b) = (base_obs(), base_obs());
        a.heap_hash = 0xABD;
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("heap: "), "{first}");
        let (mut a, b) = (base_obs(), base_obs());
        a.stack_hash = 0xDE0;
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("stack: "), "{first}");
        // ... but the kept write prefixes are display only: same (n, hash)
        // with different prefixes still passes.
        let (mut a, mut b) = (base_obs(), base_obs());
        a.heap_writes = vec![(0, 0x1111)];
        b.heap_writes = vec![(0, 0x2222)];
        a.stack_writes = vec![(0x100, 1)];
        b.stack_writes = vec![(0x100, 2)];
        assert!(compare(&a, &b, &full_checks()).0);
    }

    #[test]
    fn compare_globals() {
        test_install();
        let (mut a, b) = (base_obs(), base_obs());
        a.globals_writes = vec![(0x100, 10)];
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("globals: "), "{first}");
    }

    #[test]
    fn compare_calls_sequence() {
        test_install();
        let (mut a, b) = (base_obs(), base_obs());
        a.calls[0].3 = vec![2];
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("calls: "), "{first}");
    }

    #[test]
    fn compare_undeclared() {
        test_install();
        let (a, mut b) = (base_obs(), base_obs());
        b.undeclared_n = 1;
        b.undeclared = vec![(0x200, 7)];
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("undeclared: "), "{first}");
    }

    #[test]
    fn compare_termination_and_cheat() {
        test_install();
        // ok vs fault fails termination, and nothing else is compared
        let (a, mut b) = (base_obs(), base_obs());
        b.status = "fault".to_string();
        b.fault_code = 0xC0000005;
        b.fault = "code=0xc0000005".to_string();
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("termination: "), "{first}");
        // fault vs fault with the same code passes termination
        let (mut a2, mut b2) = (base_obs(), base_obs());
        a2.status = "fault".to_string();
        a2.fault_code = 5;
        b2.status = "fault".to_string();
        b2.fault_code = 5;
        assert!(compare(&a2, &b2, &full_checks()).0);
        // cheat on the rewrite side always fails
        let (mut a3, mut b3) = (base_obs(), base_obs());
        a3.status = "cheat".to_string();
        b3.status = "cheat".to_string();
        b3.fault = "touched code".to_string();
        let (pass, _j, first) = compare(&a3, &b3, &full_checks());
        assert!(!pass);
        assert!(first.starts_with("no_cheat: "), "{first}");
    }

    // ---- the fail-closed rules ----

    #[test]
    fn truncated_log_fails() {
        test_install();
        let (mut a, b) = (base_obs(), base_obs());
        a.log_attempted = 300;
        a.log_logged = 256;
        let (pass, _j, first) = compare(&a, &b, &full_checks());
        assert!(!pass);
        assert!(first.contains("call log truncated"), "{first}");
        assert!(first.contains("cap 256"), "{first}");
    }

    #[test]
    fn zero_mask_fails() {
        test_install();
        assert!(find_zero_mask(&checks(r#"{"call_mask":{"7":{"0":0}}}"#)).is_some());
        assert!(find_zero_mask(&checks(r#"{"call_mask":{"7":{"0":255}}}"#)).is_none());
        assert!(find_zero_mask(&checks(r#"{}"#)).is_none());
        assert!(find_zero_mask(&checks(r#"{"call_mask":[]}"#)).is_none());
        let (a, b) = (base_obs(), base_obs());
        let (pass, _j, first) =
            compare(&a, &b, &checks(r#"{"ret":"none","call_mask":{"7":{"0":0}}}"#));
        assert!(!pass);
        assert!(first.contains("zero"), "{first}");
    }

    #[test]
    fn nan_diagnostic_points_at_operand_order() {
        test_install();
        // sides differ ONLY in NaN payload bits of a call arg
        let (mut a, mut b) = (base_obs(), base_obs());
        a.calls = vec![(7, 0, 0, vec![0x7FC00001], vec![], [0; 4], [0; 4], 0)];
        b.calls = vec![(7, 0, 0, vec![0x7FC00002], vec![], [0; 4], [0; 4], 0)];
        let ck = checks(r#"{"ret":"none","esp":false,"heap":false,"stack":false,"globals":false,"calls":true,"undeclared":false,"call_regs":{"11":["eax"]}}"#);
        let (pass, _j, first) = compare(&a, &b, &ck);
        assert!(!pass);
        assert!(first.contains("[nan-diagnostic"), "{first}");
        // a real difference is not diagnosed as NaN-only
        let (mut c, mut d) = (base_obs(), base_obs());
        c.calls = vec![(7, 0, 0, vec![1], vec![], [0; 4], [0; 4], 0)];
        d.calls = vec![(7, 0, 0, vec![2], vec![], [0; 4], [0; 4], 0)];
        let (pass, _j, first) = compare(&c, &d, &ck);
        assert!(!pass);
        assert!(!first.contains("[nan-diagnostic"), "{first}");
    }

    // ---- call keys and eax validations ----

    fn crec(id: u32, ecx: u32, edx: u32, args: Vec<u32>) -> CallRec {
        (id, ecx, edx, args, vec![], [0; 4], [0; 4], 0)
    }

    #[test]
    fn callkey_regs_and_skip() {
        test_install();
        let none = checks("{}");
        // cdecl: no regs by default
        assert_eq!(callkey(&crec(7, 1, 2, vec![1, 2]), &none), "id=7 args=[\"0x1\", \"0x2\"]");
        // thiscall: ecx included and normalized
        let k = callkey(&crec(9, 0x30000010, 0, vec![]), &none);
        assert!(k.contains("ecx=H+10"), "{k}");
        // call_skip drops arg indexes
        let sk = checks(r#"{"call_skip":{"7":[0]}}"#);
        assert_eq!(callkey(&crec(7, 0, 0, vec![1, 2]), &sk), "id=7 args=[\"0x2\"]");
    }

    #[test]
    fn callkey_masks_and_alias() {
        test_install();
        // explicit mask compares (value & mask) as raw hex
        let m = checks(r#"{"call_mask":{"7":{"1":255}}}"#);
        assert_eq!(
            callkey(&crec(7, 0, 0, vec![0x1234, 0x12AB]), &m),
            "id=7 args=[\"0x1234\", \"0xab\"]"
        );
        // call_low8 alias maps to 0xFF; explicit mask wins
        let l = checks(r#"{"call_low8":{"7":[0]}}"#);
        assert_eq!(
            callkey(&crec(7, 0, 0, vec![0x12AB, 9]), &l),
            "id=7 args=[\"0xab\", \"0x9\"]"
        );
        let both = checks(r#"{"call_low8":{"7":[0]},"call_mask":{"7":{"0":15}}}"#);
        assert_eq!(
            callkey(&crec(7, 0, 0, vec![0x12AB]), &both),
            "id=7 args=[\"0xb\"]"
        );
    }

    #[test]
    fn callkey_transport_and_vectors() {
        test_install();
        let none = checks("{}");
        // eax-transport callee: stack args replaced by the marker
        let k = callkey(&crec(11, 0, 0, vec![1, 2]), &none);
        assert!(k.contains("args=transport"), "{k}");
        // snapshots compare by value; xmm words logged when declared
        let c = (13, 0, 0, vec![], vec![5, 6], [1, 2, 3, 4], [9, 9, 9, 9], 0);
        let k = callkey(&c, &none);
        assert!(k.contains("snap=[5, 6]"), "{k}");
        assert!(k.contains("xmm0=[1, 2, 3, 4]"), "{k}");
        assert!(k.contains("xmm1=[9, 9, 9, 9]"), "{k}");
    }

    #[test]
    fn eax_validations() {
        test_install();
        // eax selected without the transport is rejected ...
        let bad = checks(r#"{"call_regs":{"7":["eax"]}}"#);
        assert!(validate_eax_regs(&bad).is_some());
        // ... but fine with it, and other regs need nothing
        let ok = checks(r#"{"call_regs":{"11":["eax"]}}"#);
        assert!(validate_eax_regs(&ok).is_none());
        assert!(validate_eax_regs(&checks(r#"{"call_regs":{"9":["ecx"]}}"#)).is_none());
        assert!(validate_eax_regs(&checks("{}")).is_none());
        // reverse: a transport callee must have eax selected
        assert!(validate_eax_transport(&checks("{}")).is_some());
        assert!(validate_eax_transport(&checks(r#"{"call_regs":{"11":["ecx"]}}"#)).is_some());
        assert!(validate_eax_transport(&ok).is_none());
        // both validations fail the calls check loudly
        let (a, b) = (base_obs(), base_obs());
        let (pass, _j, first) = compare(&a, &b, &bad);
        assert!(!pass);
        assert!(first.starts_with("calls: "), "{first}");
    }

    // ---- float helpers ----

    #[test]
    fn nan_canon_helpers() {
        assert!(is_f32_nan(0x7FC00001));
        assert!(is_f32_nan(0xFF800001));
        assert!(!is_f32_nan(0x7F800000));
        assert!(!is_f32_nan(0x3F800000));
        assert_eq!(canon_f32(0x7FC00001), 0x7FC00000);
        assert_eq!(canon_f32(5), 5);
        assert_eq!(unhex("ab00ff"), vec![0xAB, 0x00, 0xFF]);
        // x87 NaN -> +qNaN, zero payload; anything else passes through
        assert_eq!(
            canon_st0_hex("0100000000000000ff7f"),
            "00000000000000c0ff7f"
        );
        assert_eq!(
            canon_st0_hex("00000000000000803f9f"),
            "00000000000000803f9f"
        );
        assert_eq!(canon_st0_hex("short"), "short");
        // xmm0 low f32 NaN canonicalized in place
        let x = canon_xmm0_hex("0100c07f000000000000000000000000");
        assert!(x.starts_with("0000c07f"), "{x}");
        assert_eq!(canon_xmm0_hex("short"), "short");
    }

    #[test]
    fn fp_compare() {
        let no_tol = checks("{}");
        assert!(fp_cmp("ab12", "ab12", 2, &no_tol).0);
        let (pass, detail, exact) = fp_cmp("ab12", "ac12", 2, &no_tol);
        assert!(!pass && !exact);
        assert!(detail.contains("zero tolerance"), "{detail}");
        assert!(fp_cmp("ab", "a", 2, &no_tol).1.contains("short"));
        // tolerance compares f32 lanes (16-byte xmm vectors)
        let tol = checks(r#"{"fp_tol":"0.01,0"}"#);
        let h = |f: f32| {
            let mut v = vec![0u8; 16];
            v[0..4].copy_from_slice(&f.to_le_bytes());
            hexbytes(&v)
        };
        assert!(fp_cmp(&h(1.0), &h(1.005), 16, &tol).0);
        assert!(!fp_cmp(&h(1.0), &h(1.5), 16, &tol).0);
    }

    #[test]
    fn limit_helpers() {
        assert_eq!(lims(&["a".to_string(), "b".to_string()], 1), vec!["a".to_string()]);
        assert_eq!(lim(&[(1, 2), (3, 4)], 1), vec!["0x1=0x2".to_string()]);
    }
}

