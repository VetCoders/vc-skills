use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use tempfile::tempdir;
use vibecrafted_operator::polarize::{PolarizeBand, current_intents_from_home, read_intent};
use vibecrafted_operator::skills_catalog::{
    CATALOG, SkillAgent, SkillPayload, build_skill_launch_command,
};

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[test]
fn catalog_covers_existing_vibecrafted_skill_directories() {
    let skill_root = Path::new("/Users/polyversai/Libraxis/vc-runtime/vibecrafted/skills");
    let mut existing = fs::read_dir(skill_root)
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            if path.join("SKILL.md").is_file() {
                entry.file_name().to_str().map(ToOwned::to_owned)
            } else {
                None
            }
        })
        .filter(|name| name.starts_with("vc-"))
        .collect::<BTreeSet<_>>();
    existing.remove("foundations");

    let catalog = CATALOG
        .iter()
        .map(|entry| entry.slug.to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(catalog, existing);
    assert!(
        CATALOG
            .iter()
            .any(|entry| entry.slug == "vc-polarize" && entry.emphasized()),
        "vc-polarize must be an emphasized operator entrypoint"
    );
}

#[test]
fn skill_launch_command_assembles_argv_for_every_skill_and_agent() {
    let agents = [
        SkillAgent::Claude,
        SkillAgent::Codex,
        SkillAgent::Gemini,
        SkillAgent::Any,
    ];
    for entry in CATALOG {
        for agent in agents {
            let mut env = BTreeMap::<String, OsString>::new();
            env.insert("VIBECRAFTED_ROOT".to_string(), "/tmp/repo".into());
            let command = build_skill_launch_command(
                "/usr/bin/vibecrafted",
                entry.slug,
                agent,
                SkillAgent::Codex,
                &SkillPayload::Prompt("ship the skill surface".to_string()),
                env,
            );
            let args = command
                .args
                .iter()
                .map(|value| value.to_string_lossy().into_owned())
                .collect::<Vec<_>>();
            assert_eq!(command.program, PathBuf::from("/usr/bin/vibecrafted"));
            assert_eq!(args[0], entry.command_token());
            assert_eq!(args[1], agent.resolved_cli_token(SkillAgent::Codex));
            assert_eq!(args[2], "--prompt");
            assert_eq!(args[3], "ship the skill surface");
            assert_eq!(
                command.env.get("VIBECRAFTED_ROOT"),
                Some(&OsString::from("/tmp/repo"))
            );
        }
    }
}

#[test]
fn skill_launch_command_supports_file_payload_and_empty_payload() {
    let file_command = build_skill_launch_command(
        "vibecrafted",
        "vc-polarize",
        SkillAgent::Codex,
        SkillAgent::Claude,
        &SkillPayload::File("/tmp/prism-pack.md".into()),
        BTreeMap::new(),
    );
    let file_args = file_command
        .args
        .iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        file_args,
        vec!["polarize", "codex", "--file", "/tmp/prism-pack.md"]
    );

    let empty_command = build_skill_launch_command(
        "vibecrafted",
        "vc-init",
        SkillAgent::Any,
        SkillAgent::Gemini,
        &SkillPayload::None,
        BTreeMap::new(),
    );
    let empty_args = empty_command
        .args
        .iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert_eq!(empty_args, vec!["init", "gemini"]);
}

#[test]
fn polarize_intent_ingests_prism_payload_and_renders_band_action() {
    let home = tempdir().unwrap();
    let prism = home
        .path()
        .join("artifacts/VetCoders/vc-operator/2026_0508/polarize/polr-123/prism.json");
    fs::create_dir_all(prism.parent().unwrap()).unwrap();
    fs::write(
        &prism,
        r#"{"schema_version":"loctree.prism.v1","total_score":13}"#,
    )
    .unwrap();

    let intent = read_intent(&prism).unwrap();
    assert_eq!(intent.band, PolarizeBand::Doctrine);
    assert_eq!(intent.score, 13);
    assert_eq!(intent.run_id, "polr-123");
    assert!(
        intent
            .summary_line()
            .contains("canonical doctrine plus regression contract")
    );

    let intents = current_intents_from_home(home.path(), Path::new("/tmp/repo"));
    assert_eq!(intents, vec![intent]);
}

#[test]
fn polarize_intent_discovery_skips_malformed_prisms_without_hiding_valid_intents() {
    let home = tempdir().unwrap();
    let valid_prism = home
        .path()
        .join("artifacts/VetCoders/vc-operator/2026_0508/polarize/polr-valid/prism.json");
    let malformed_prism = home
        .path()
        .join("artifacts/VetCoders/vc-operator/2026_0508/polarize/polr-bad/prism.json");
    fs::create_dir_all(valid_prism.parent().unwrap()).unwrap();
    fs::create_dir_all(malformed_prism.parent().unwrap()).unwrap();
    fs::write(
        &valid_prism,
        r#"{"schema_version":"loctree.prism.v1","total_score":9,"run_id":"polr-valid"}"#,
    )
    .unwrap();
    fs::write(
        &malformed_prism,
        r#"{"schema_version":"loctree.prism.v1","total_score":"bad"}"#,
    )
    .unwrap();

    assert!(read_intent(&malformed_prism).is_err());
    let intents = current_intents_from_home(home.path(), Path::new("/tmp/repo"));

    assert_eq!(intents.len(), 1);
    assert_eq!(intents[0].run_id, "polr-valid");
    assert_eq!(intents[0].band, PolarizeBand::Pass);
}

#[cfg(unix)]
#[test]
fn polarize_intent_discovery_does_not_follow_symlinked_directories() {
    let home = tempdir().unwrap();
    let escaped = tempdir().unwrap();
    let valid_prism = home
        .path()
        .join("artifacts/VetCoders/vc-operator/2026_0508/polarize/polr-valid/prism.json");
    let escaped_prism = escaped
        .path()
        .join("VetCoders/vc-operator/2026_0508/polarize/polr-escaped/prism.json");
    fs::create_dir_all(valid_prism.parent().unwrap()).unwrap();
    fs::create_dir_all(escaped_prism.parent().unwrap()).unwrap();
    fs::write(
        &valid_prism,
        r#"{"schema_version":"loctree.prism.v1","total_score":9,"run_id":"polr-valid"}"#,
    )
    .unwrap();
    fs::write(
        &escaped_prism,
        r#"{"schema_version":"loctree.prism.v1","total_score":14,"run_id":"polr-escaped"}"#,
    )
    .unwrap();
    symlink(escaped.path(), home.path().join("artifacts/escaped-link")).unwrap();

    let intents = current_intents_from_home(home.path(), Path::new("/tmp/repo"));

    assert_eq!(intents.len(), 1);
    assert_eq!(intents[0].run_id, "polr-valid");
}
