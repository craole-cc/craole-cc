#[cfg(test)]
mod tests {
  use {
    contentctl::sync_content_database,
    std::{
      fs,
      path::Path,
    },
  };

  fn fixture(name : &str,) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("contentctl-sync-{name}-{}", std::process::id()),);
    _ = fs::remove_dir_all(&root,);
    fs::create_dir_all(root.join("content/projects",),).unwrap();
    fs::create_dir_all(root.join("content/posts",),).unwrap();
    fs::create_dir_all(root.join("content/media",),).unwrap();
    fs::create_dir_all(root.join("database/migrations",),).unwrap();
    root
  }

  fn write(path : &Path, content : &str,) { fs::write(path, content,).unwrap(); }

  fn copy_schema_migration(root : &Path,) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"),);
    let schema =
      fs::read_to_string(manifest_dir.join("../database/migrations/0001_schema.sql",),).unwrap();
    write(&root.join("database/migrations/0001_schema.sql",), &schema,);
  }

  fn write_valid_content(root : &Path,) {
    write(
      &root.join("content/projects/demo.toml",),
      r#"
title = "Demo Project"
slug = "demo-project"
status = "active"
description = "A useful project for testing database sync."
featured = true
published = true
sort_order = 10
tags = ["Rust", "SQLite"]
"#,
    );
    write(
      &root.join("content/posts/hello.md",),
      r#"---
title: "Hello"
slug: "hello"
kind: "blog"
published: true
published_at: "2026-06-03"
tags: ["Rust"]
excerpt: "A short post."
---

# Hello

Body.
"#,
    );
  }

  #[test]
  fn syncs_valid_content_into_sqlite_database() {
    let root = fixture("valid",);
    copy_schema_migration(&root,);
    write_valid_content(&root,);
    let database_url = format!(
      "sqlite://{}",
      root.join("database/data/portfolio.db",).display()
    );

    let report = sync_content_database(&root, &database_url,).unwrap();

    assert_eq!(report.projects, 1);
    assert_eq!(report.posts, 1);
    assert_eq!(report.media, 0);
    assert!(root.join("database/data/portfolio.db",).is_file());
  }

  #[test]
  fn refuses_to_sync_invalid_content() {
    let root = fixture("invalid",);
    copy_schema_migration(&root,);
    write(
      &root.join("content/projects/demo.toml",),
      r#"
title = "Demo Project"
slug = "demo-project"
status = "invalid"
description = "Invalid project."
"#,
    );
    let database_url = format!(
      "sqlite://{}",
      root.join("database/data/portfolio.db",).display()
    );

    let error = sync_content_database(&root, &database_url,)
      .expect_err("invalid content should block sync",)
      .to_string();

    assert!(error.contains("content validation failed"), "{error}");
    assert!(!root.join("database/data/portfolio.db",).exists());
  }
}
