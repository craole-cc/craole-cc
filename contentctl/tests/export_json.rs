#[cfg(test)]
mod tests {
  use {
    contentctl::export_static_json,
    std::{
      fs,
      path::Path,
    },
  };

  fn fixture(name : &str,) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("contentctl-json-{name}-{}", std::process::id()),);
    _ = fs::remove_dir_all(&root,);
    fs::create_dir_all(root.join("content/projects",),).unwrap();
    fs::create_dir_all(root.join("content/posts",),).unwrap();
    fs::create_dir_all(root.join("content/media",),).unwrap();
    root
  }

  fn write(path : &Path, content : &str,) { fs::write(path, content,).unwrap(); }

  fn write_valid_content(root : &Path,) {
    write(
      &root.join("content/projects/demo.toml",),
      r#"
title = "Demo Project"
slug = "demo-project"
status = "active"
description = "A useful project for static JSON export."
featured = true
published = true
sort_order = 10
tags = ["Rust", "Static"]
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
  fn exports_static_json_files() {
    let root = fixture("valid",);
    write_valid_content(&root,);
    let output_dir = root.join("dist/data",);

    let report = export_static_json(&root, &output_dir,).unwrap();

    assert_eq!(report.projects, 1);
    assert_eq!(report.posts, 1);
    assert_eq!(report.media, 0);
    let projects = fs::read_to_string(output_dir.join("projects.json",),).unwrap();
    let posts = fs::read_to_string(output_dir.join("posts.json",),).unwrap();
    let manifest = fs::read_to_string(output_dir.join("manifest.json",),).unwrap();
    assert!(projects.contains("\"slug\": \"demo-project\""), "{projects}");
    assert!(projects.contains("\"tags\": ["), "{projects}");
    assert!(posts.contains("\"body\": \"\\n# Hello"), "{posts}");
    assert!(manifest.contains("\"projects\": 1"), "{manifest}");
  }

  #[test]
  fn refuses_to_export_invalid_content() {
    let root = fixture("invalid",);
    write(
      &root.join("content/projects/demo.toml",),
      r#"
title = "Demo Project"
slug = "demo-project"
status = "wrong"
description = "Invalid project."
"#,
    );
    let output_dir = root.join("dist/data",);

    let error = export_static_json(&root, &output_dir,)
      .expect_err("invalid content should block export",)
      .to_string();

    assert!(error.contains("content validation failed"), "{error}");
    assert!(!output_dir.exists());
  }
}
