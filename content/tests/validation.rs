#[cfg(test)]
mod tests {
  use {
    content::validate_content_root,
    std::{
      fs,
      path::Path,
    },
  };

  fn fixture(name : &str,) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("content-{name}-{}", std::process::id()),);
    _ = fs::remove_dir_all(&root,);
    fs::create_dir_all(root.join("assets/projects",),).unwrap();
    fs::create_dir_all(root.join("assets/posts",),).unwrap();
    fs::create_dir_all(root.join("assets/media",),).unwrap();
    fs::create_dir_all(root.join("assets/media/images",),).unwrap();
    root
  }

  fn write(path : &Path, content : &str,) { fs::write(path, content,).unwrap(); }

  fn valid_project() -> &'static str {
    r#"
title = "Demo Project"
slug = "demo-project"
status = "building"
description = "A useful project for validating the content pipeline."
repo_url = "https://github.com/craole-cc/demo-project"
live_url = "https://example.com"
featured = true
published = true
sort_order = 10
tags = ["Rust", "Leptos"]
screenshots = ["/media/images/demo_home.webp"]
"#
  }

  #[test]
  fn accepts_valid_project_content() {
    let root = fixture("valid-project",);
    write(
      &root.join("assets/media/images/demo_home.webp",),
      "fake image",
    );
    write(&root.join("assets/projects/demo.toml",), valid_project(),);

    let report = validate_content_root(&root,).unwrap();

    assert!(report.is_valid(), "{report:#?}");
  }

  #[test]
  fn rejects_duplicate_project_slugs() {
    let root = fixture("duplicate-projects",);
    write(
      &root.join("assets/media/images/demo_home.webp",),
      "fake image",
    );
    write(&root.join("assets/projects/one.toml",), valid_project(),);
    write(&root.join("assets/projects/two.toml",), valid_project(),);

    let report = validate_content_root(&root,).unwrap();

    assert!(!report.is_valid());
    assert!(
      report.errors.iter().any(|error| error
        .message
        .contains("duplicate project slug `demo-project`")),
      "{report:#?}"
    );
  }

  #[test]
  fn rejects_invalid_project_status() {
    let root = fixture("invalid-status",);
    let project = valid_project().replace("building", "nearly-done",);
    write(&root.join("assets/projects/demo.toml",), &project,);

    let report = validate_content_root(&root,).unwrap();

    assert!(!report.is_valid());
    assert!(
      report.errors.iter().any(|error| error
        .message
        .contains("invalid project status `nearly-done`")),
      "{report:#?}"
    );
  }

  #[test]
  fn rejects_missing_project_screenshot_asset() {
    let root = fixture("missing-screenshot",);
    write(&root.join("assets/projects/demo.toml",), valid_project(),);

    let report = validate_content_root(&root,).unwrap();

    assert!(!report.is_valid());
    assert!(
      report
        .errors
        .iter()
        .any(|error| error.message.contains("asset does not exist")),
      "{report:#?}"
    );
  }

  #[test]
  fn accepts_markdown_post_with_frontmatter() {
    let root = fixture("valid-post",);
    write(
      &root.join("assets/posts/hello.md",),
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

    let report = validate_content_root(&root,).unwrap();

    assert!(report.is_valid(), "{report:#?}");
  }

  #[test]
  fn accepts_https_remote_media_url() {
    let root = fixture("remote-media",);
    write(
      &root.join("assets/media/remote.toml",),
      r#"title = "Remote Study"
slug = "remote-study"
media_type = "photo"
file_path = "https://images.unsplash.com/photo-1234567890-example"
alt_text = "A remotely hosted study."
published = true
width = 1200
height = 800
tags = ["art"]
"#,
    );

    let report = validate_content_root(&root,).unwrap();

    assert!(report.is_valid(), "{report:#?}");
  }

  #[test]
  fn rejects_http_remote_media_url() {
    let root = fixture("insecure-remote-media",);
    write(
      &root.join("assets/media/remote.toml",),
      r#"title = "Remote Study"
slug = "remote-study"
media_type = "photo"
file_path = "http://images.example.com/photo.jpg"
alt_text = "An insecure remote study."
published = true
width = 1200
height = 800
tags = ["art"]
"#,
    );

    let report = validate_content_root(&root,).unwrap();

    assert!(!report.is_valid());
    assert!(
      report
        .errors
        .iter()
        .any(|error| error.message.contains("must use HTTPS")),
      "{report:#?}"
    );
  }
}
