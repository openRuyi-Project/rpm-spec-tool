//! Shared data model, EVR comparison, and SQLite index access for repositories.
//!
//! Network access, metadata parsing, and cache publication live in
//! `rpm-spec-repo-metadata`; dependency lookup lives in
//! `rpm-spec-repo-resolver`.
//!
//! The user-facing types are:
//!
//! - [`RepoConfig`] / [`BuildrootConfig`] — TOML schema for
//!   the per-profile `[profiles.X.repos.*]` and `[profiles.X.buildroot]`
//!   blocks. Deserialised by `rpm-spec-profile` and embedded into the
//!   resolved [`rpm_spec_profile::Profile`].
//! - [`package::Package`] / [`package::NEVRA`] / [`package::Capability`] —
//!   one parsed RPM package plus the bits the resolver indexes by.
//! - [`index::RepoIndex`] / [`index::RepoUniverse`] — a single repo
//!   snapshot, and the assembled multi-repo universe per profile.
//! - [`evr::EVR`] — Epoch-Version-Release with the rpm vercmp algorithm.
//! - [`error::RepoError`] — fail variants partitioned by phase.
//!
//! This crate does not use Linux-specific APIs. Unix-specific cache publication
//! stays in `rpm-spec-repo-metadata`.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
// Documentation backlog tracked alongside the existing profile crate;
// most public items here are field-only structs whose docs go on the
// fields themselves. Tighten before the first stable release.
#![expect(
    missing_docs,
    reason = "pre-1.0: doc backlog tracked separately; switch fires loudly when backlog reaches zero"
)]

pub mod db;
pub mod error;
pub mod evr;
pub mod index;
pub mod package;

pub use error::{ErrorLocation, HttpError, RepoError, SolveError};
pub use evr::EVR;
pub use index::{
    Advisory, AdvisorySeverity, ProviderRef, RepoId, RepoIndex, RepoRevision, RepoUniverse,
};
pub use package::{
    CapVersion, Capability, ChecksumParseError, Dependency, NEVRA, Package, PkgChecksum,
};

// Re-exports of config-layer types defined in `rpm-spec-profile`.
// Repo backends and the resolver consume these without importing
// `rpm-spec-profile` directly. `RepoKind` lives in `rpm-spec-profile`
// so its config-layer types stay reachable from the crates.io-published
// profile crate without that crate depending on the internal `repo-*`
// crates.
pub use rpm_spec_profile::repos::{BuildrootConfig, RepoConfig, RepoKind, RepoRole, RepoSet};
