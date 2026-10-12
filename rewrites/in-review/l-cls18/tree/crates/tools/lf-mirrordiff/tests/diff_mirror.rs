//! Differential tests: the lifted mirror-reflection render phase against its
//! verified 32-bit rewrites, on the same generated cases.
//!
//! Runs on the 32-bit target only (the rewrites take real addresses). Each
//! test runs a fixed number of seeded cases of one method and fails at the
//! first case where the lift and the rewrite differ.

#![cfg(target_arch = "x86")]
// Test code: the cases read and write the memory the rewrites use, through
// addresses the harness computed inside its own allocation.
#![allow(unsafe_code)]

use lf_mirrordiff::harness::{Method, run_method, run_mutant};

const CASES: usize = 4000;

fn run(method: Method, seed: u64) {
    match run_method(method, CASES, seed) {
        Ok(coverage) => {
            eprintln!("{method:?}: {} cases agree; calls seen: {:?}", coverage.cases, {
                let mut calls: Vec<_> = coverage.calls.iter().collect();
                calls.sort();
                calls
            });
        }
        Err(message) => panic!("{message}"),
    }
}

#[test]
fn deleting_destructor_matches_rewrite() {
    run(Method::DeletingDestructor, 0x0D76_9100);
}

#[test]
fn refresh_matches_rewrite() {
    run(Method::Refresh, 0x0D77_4300);
}

#[test]
fn reflect_matches_rewrite() {
    run(Method::Reflect, 0x0D77_6000);
}

#[test]
fn render_matches_rewrite() {
    run(Method::Render, 0x0D76_9500);
}

/// Each deliberately wrong copy of a method must be caught by the same cases.
fn caught(method: Method, seed: u64, name: &str) {
    let caught = run_mutant(method, 400, seed);
    eprintln!("{name}: {caught} of 400 cases caught");
    assert!(caught > 0, "{name}: the cases cannot see the changed line");
}

#[test]
fn deleting_destructor_wrong_copy_is_caught() {
    caught(Method::DeletingDestructor, 0x0D76_9100, "m1 free bit");
}

#[test]
fn refresh_wrong_copy_is_caught() {
    caught(Method::Refresh, 0x0D77_4300, "m2 remembered bit");
}

#[test]
fn reflect_wrong_copy_is_caught() {
    caught(Method::Reflect, 0x0D77_6000, "m3 status bit");
}

#[test]
fn render_wrong_copy_is_caught() {
    caught(Method::Render, 0x0D76_9500, "m4 selector flags");
}

