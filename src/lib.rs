//! supply-depot — Sovereign registry proxy/cache and supply-chain policy POC backed by Starmetal.
//!
//! This crate is intentionally a minimal skeleton. The first implementation
//! increments must keep the upstream boundary explicit and preserve the
//! sovereign constraints documented in `docs/adr/0001-scope-and-upstream-policy.md`.

/// Static project metadata used by the CLI and smoke tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectCard {
    pub name: &'static str,
    pub role: &'static str,
    pub upstream: &'static str,
    pub relationship: &'static str,
}

/// The repository's initial scope card.
pub const PROJECT: ProjectCard = ProjectCard {
    name: "supply-depot",
    role: "sovereign supply-chain depot",
    upstream: "Starmetal",
    relationship: "Infrastructure POC for registry caching and policy enforcement; not critical production path until promoted.",
};

/// Human-readable summary for CLI smoke runs.
pub fn summary() -> String {
    format!(
        "{} — {} (upstream: {})",
        PROJECT.name, PROJECT.role, PROJECT.upstream
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_card_names_the_repo_and_upstream() {
        assert_eq!(PROJECT.name, "supply-depot");
        assert_eq!(PROJECT.upstream, "Starmetal");
        assert!(summary().contains(PROJECT.role));
    }
}
