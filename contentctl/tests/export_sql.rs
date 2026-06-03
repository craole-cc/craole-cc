#[cfg(test)]
mod tests {
  use {
    contentctl::export_seed_sql,
    std::{
      fs,
      path::Path,
    },
  };

  fn fixture(name : &str,) -> std::path::PathBuf {
    let root =
      std::env::temp_dir().join(format!("contentctl-export-{name}-{}", std::process::id()),);
    _ = fs::remove_dir_all(&root,);
    fs::create_dir_all(root.join("content/projects",),).unwrap();
    fs::create_dir_all(root.join("content/posts",),).unwrap();
    fs::create_dir_all(root.join("content/media",),).unwrap();
    root
  }

  fn write(path : &Path, content : &str,) { fs::write(path, content,).unwrap(); }

  #[test]
  fn exports_valid_content_as_sqlite_seed_script() {
    let root = fixture("sql-script",);
    write(
      &root.join("content/projects/demo.toml",),
      r#"
title = "Demo Project"
slug = "demo-project"
status = "active"
description = "A useful project for testing SQL export."
repo_url = "https://github.com/craole-cc/demo-project"
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

Body with 'quotes'.
"#,
    );

    let sql = export_seed_sql(&root,).unwrap();

    assert!(sql.contains("BEGIN;"));
    assert!(sql.contains("INSERT INTO projects"));
    assert!(sql.contains("'demo-project'"));
    assert!(sql.contains("INSERT INTO project_tags"));
    assert!(sql.contains("INSERT INTO posts"));
    assert!(sql.contains("Body with ''quotes''."));
    assert!(sql.contains("COMMIT;"));
  }
}
