use std::path::PathBuf;
use std::process::Command;

use gear_depot::{ArtifactManifest, ArtifactType, PackageType};

const GEAR_DEPOT: &str = env!("CARGO_BIN_EXE_gear-depot");

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn wrench_evidence_report_becomes_inspection_report_manifest() {
    let manifest = ArtifactManifest::from_wrench_evidence_report_file(&fixture(
        "wrench-portal-usage.valid.json",
    ))
    .expect("Wrench evidence report manifest is built");

    assert_eq!(
        manifest.artifact.artifact_type,
        ArtifactType::InspectionReport
    );
    assert_eq!(manifest.artifact.producer, "wrench-inspect");
    assert_eq!(manifest.package_type, PackageType::JsonBundle);
    assert_eq!(manifest.checksums.len(), 1);
    assert!(manifest.artifact.hash.starts_with("sha256:"));
    assert_eq!(manifest.artifact.manifest_ref, manifest.manifest_id);
    manifest.validate().expect("manifest validates");
}

#[test]
fn cli_emits_manifest_for_wrench_evidence_report() {
    let output = Command::new(GEAR_DEPOT)
        .args([
            "manifest",
            "evidence-report",
            fixture("wrench-portal-usage.valid.json").to_str().unwrap(),
            "--json",
        ])
        .output()
        .expect("spawn gear-depot manifest evidence-report");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("\"artifact_type\": \"inspection_report\""));
    assert!(stdout.contains("\"producer\": \"wrench-inspect\""));
    assert!(stdout.contains("\"package_type\": \"json_bundle\""));
}
