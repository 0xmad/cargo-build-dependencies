use super::*;

use std::collections::HashMap;

fn get_map<'a>(name: &'a str, version: &'a str) -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();
    map.insert("name", name);
    map.insert("version", version);

    map
}

#[quickcheck]
fn test_get_string_field(expected: String) -> bool {
    let field: Toml = expected.to_string().into();
    let actual = get_string_field(Some(&field));

    expected == get_string_field(Some(&actual.into()))
}

#[test]
fn test_get_string_empty() {
    let actual = get_string_field(None);
    assert!(actual.is_empty());
}

#[test]
fn test_get_packages_empty_dependencies() {
    let packages: Vec<Toml> = vec![];
    let dependencies = get_packages(&packages, &vec![]);
    assert!(dependencies.is_empty());

    let map = get_map("name", "0.0.1");
    let packages: Vec<Toml> = vec![map.into()];
    let dependencies = get_packages(&packages, &vec![]);
    assert!(dependencies.is_empty());
}

#[quickcheck]
fn test_get_packages(name: String, version: String) -> bool {
    let packages: Vec<Toml> = vec![get_map(&name, &version).into()];
    let dependencies = get_packages(&packages, &vec![name.to_string()]);

    !dependencies.is_empty() && dependencies[0] == format!("{}:{}", name, version)
}

#[quickcheck]
fn test_get_packages_only_with_dependencies(name: String, version: String) -> bool {
    let packages: Vec<Toml> = (0..10)
        .map(|i| get_map(&format!("{}_{}", name, i), &version).into())
        .collect();
    let toml_packages = vec![format!("{}_{}", name, 0)];
    let dependencies = get_packages(&packages, &toml_packages);

    assert!(!dependencies.is_empty());
    dependencies[0] == format!("{}_0:{}", name, version)
}

#[test]
fn test_get_toml() {
    let toml = get_toml("Cargo.toml");
    assert!(toml.is_ok());
    assert!(toml.map(|i| i.is_table()).unwrap_or_default());
}

#[test]
#[should_panic = "Failed to parse toml"]
fn test_get_toml_parse_failed() {
    assert!(get_toml("not-found").is_err());
    get_toml("README.md").unwrap();
}

#[test]
fn test_get_dependencies() {
    let toml = get_toml("Cargo.toml").unwrap();
    let lock = get_toml("Cargo.lock").unwrap();
    let result = get_dependencies(&toml, &lock);

    assert_eq!(result.len(), 2);
    assert!(result.contains(&"clap:2.33.1".to_string()));
    assert!(result.contains(&"toml:0.5.6".to_string()));
}

#[test]
fn test_get_toml_dependencies() {
    let toml = get_toml("Cargo.toml").unwrap();
    let result = get_toml_dependencies(&toml);
    assert!(!result.is_empty());
}

#[test]
fn test_get_toml_dependencies_without_dependencies() {
    let mut map = HashMap::new();
    map.insert("test", "test");

    let toml: Toml = map.into();
    let result = get_toml_dependencies(&toml);
    assert!(result.is_empty());
}

#[test]
fn test_get_toml_dependencies_with_string_dependencies() {
    let mut map = HashMap::new();
    map.insert("dependencies", "test");

    let toml: Toml = map.into();
    let result = get_toml_dependencies(&toml);
    assert!(result.is_empty());
}

#[test]
fn test_get_lock_dependencies() {
    let toml = get_toml("Cargo.toml").unwrap();
    let lock = get_toml("Cargo.lock").unwrap();
    let dependencies = get_toml_dependencies(&toml);
    let result = get_lock_dependencies(&lock, &dependencies);
    assert!(!result.is_empty());
}

#[test]
fn test_get_lock_dependencies_without_package() {
    let mut map = HashMap::new();
    map.insert("test", "test");

    let toml = get_toml("Cargo.toml").unwrap();
    let lock: Toml = map.into();
    let dependencies = get_toml_dependencies(&toml);
    let result = get_lock_dependencies(&lock, &dependencies);
    assert!(result.is_empty());
}

#[test]
fn test_get_lock_dependencies_with_string_package() {
    let mut map = HashMap::new();
    map.insert("package", "test");

    let toml = get_toml("Cargo.toml").unwrap();
    let lock: Toml = map.into();
    let dependencies = get_toml_dependencies(&toml);
    let result = get_lock_dependencies(&lock, &dependencies);
    assert!(result.is_empty());
}

#[test]
fn test_get_lock_dependencies_without_dependencies() {
    let lock = get_toml("Cargo.lock").unwrap();
    let dependencies = vec![];
    let result = get_lock_dependencies(&lock, &dependencies);
    assert!(result.is_empty());
}
