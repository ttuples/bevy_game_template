// Rename template files.
file::rename("templates/Cargo.toml.template", "Cargo.toml");
file::rename("templates/main.rs.template", "src/main.rs");
file::rename("templates/Cargo.toml.mobile.template", "mobile/Cargo.toml");
file::rename("templates/lib.rs.template", "mobile/src/lib.rs");

// Generate `Cargo.lock`.
system::command("cargo", ["update", "--package", variable::get("project-name")]);