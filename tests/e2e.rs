//! End-to-end test suite for Lièvre
//!
//! Each test is sandboxed: creates own data, cleans up, runs independently.
//! Requires running server: docker compose up -d
//! Run all: cargo test --test e2e -- --ignored
//! Run domain: cargo test --test e2e auth -- --ignored

mod activities;
mod auth;
mod common;
mod federation;
mod feed;
mod health;
mod imports;
mod social;
