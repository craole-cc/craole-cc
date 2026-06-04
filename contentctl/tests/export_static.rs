#[cfg(test)]
mod tests {
  use {
    contentctl::export_static_site,
    std::{
      fs,
      path::Path,
    },
  };

  fn fixture(name : &str,) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("contentctl-static-{name}-{}", std::process::id()),);
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
description = "A useful project for static HTML export."
repo_url = "https://github.com/craole-cc/demo-project"
live_url = "https://example.com"
featured = true
published = true
sort_order = 10
tags = ["Rust", "Static"]
"#,
    );
    write(
      &root.join("content/posts/hello.md",),
      r#"---
title: "Hello Static"
slug: "hello-static"
kind: "blog"
published: true
published_at: "2026-06-03"
tags: ["Rust"]
excerpt: "A short static post."
---

# Hello Static

Body for static fallback.
"#,
    );
  }

  #[test]
  fn exports_static_site_pages_and_data() {
    let root = fixture("valid",);
    write_valid_content(&root,);
    let output_dir = root.join("dist",);

    let report = export_static_site(&root, &output_dir,).unwrap();

    assert_eq!(report.projects, 1);
    assert_eq!(report.posts, 1);
    assert_eq!(report.media, 0);
    assert!(report.pages >= 5, "{report:#?}");

    let index = fs::read_to_string(output_dir.join("index.html",),).unwrap();
    let dev_index = fs::read_to_string(output_dir.join("dev/index.html",),).unwrap();
    let project = fs::read_to_string(output_dir.join("dev/demo-project/index.html",),).unwrap();
    let log_index = fs::read_to_string(output_dir.join("log/index.html",),).unwrap();
    let post = fs::read_to_string(output_dir.join("log/hello-static/index.html",),).unwrap();
    let manifest = fs::read_to_string(output_dir.join("data/manifest.json",),).unwrap();

    assert!(index.contains("Craole.CC Static Preview"), "{index}");
    assert!(index.contains("Demo Project"), "{index}");
    assert!(dev_index.contains("/dev/demo-project/"), "{dev_index}");
    assert!(project.contains("A useful project for static HTML export."), "{project}");
    assert!(log_index.contains("/log/hello-static/"), "{log_index}");
    assert!(post.contains("Body for static fallback."), "{post}");
    assert!(manifest.contains("\"projects\": 1"), "{manifest}");
  }

  #[test]
  fn refuses_to_export_static_site_for_invalid_content() {
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
    let output_dir = root.join("dist",);

    let error = export_static_site(&root, &output_dir,)
      .expect_err("invalid content should block static export",)
      .to_string();

    assert!(error.contains("content validation failed"), "{error}");
    assert!(!output_dir.exists());
  }
}
