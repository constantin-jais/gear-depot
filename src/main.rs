use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => {
            println!("{}", gear_depot::summary());
        }
        [command, subcommand, path] if command == "manifest" && subcommand == "evidence-report" => {
            let manifest =
                gear_depot::ArtifactManifest::from_wrench_evidence_report_file(Path::new(path))?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
        [command, subcommand, path, flag]
            if command == "manifest" && subcommand == "evidence-report" && flag == "--json" =>
        {
            let manifest =
                gear_depot::ArtifactManifest::from_wrench_evidence_report_file(Path::new(path))?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
        _ => {
            eprintln!("usage: gear-depot manifest evidence-report <wrench-evidence.json> [--json]");
            std::process::exit(2);
        }
    }

    Ok(())
}
