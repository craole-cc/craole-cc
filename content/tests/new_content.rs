#[cfg(test)]
mod tests {
  use {
    content::{
      ContentTemplateKind,
      create_content_template,
      validate_content_root,
    },
    std::fs,
  };

  fn fixture(name : &str,) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("content-new-{name}-{}", std::process::id()),);
    _ = fs::remove_dir_all(&root,);
    fs::create_dir_all(&root,).unwrap();
    root
  }

  #[test]
  fn creates_project_template_at_slug_path() {
    let root = fixture("project",);

    let created =
      create_content_template(&root, ContentTemplateKind::Project, "demo-project",).unwrap();

    assert_eq!(created, root.join("assets/projects/demo-project.toml",));
    let content = fs::read_to_string(created,).unwrap();
    assert!(content.contains("title = \"Demo Project\""));
    assert!(content.contains("slug = \"demo-project\""));
    assert!(content.contains("status = \"planning\""));
    assert!(content.contains("published = false"));
  }

  #[test]
  fn creates_post_template_at_slug_path() {
    let root = fixture("post",);

    let created =
      create_content_template(&root, ContentTemplateKind::Post, "hello-craole",).unwrap();

    assert_eq!(created, root.join("assets/posts/hello-craole.md",));
    let content = fs::read_to_string(created,).unwrap();
    assert!(content.contains("title: \"Hello Craole\""));
    assert!(content.contains("slug: \"hello-craole\""));
    assert!(content.contains("kind: \"note\""));
    assert!(content.contains("published: false"));
  }

  #[test]
  fn created_project_and_post_templates_are_valid_drafts() {
    let root = fixture("valid-drafts",);
    create_content_template(&root, ContentTemplateKind::Project, "demo-project",).unwrap();
    create_content_template(&root, ContentTemplateKind::Post, "hello-craole",).unwrap();

    let report = validate_content_root(&root,).unwrap();

    assert!(report.is_valid(), "{report:#?}");
  }

  #[test]
  fn rejects_invalid_template_slug() {
    let root = fixture("bad-slug",);

    let error = create_content_template(&root, ContentTemplateKind::Project, "Bad Slug",)
      .expect_err("invalid slug should fail",)
      .to_string();

    assert!(error.contains("invalid content slug `Bad Slug`"), "{error}");
  }

  #[test]
  fn refuses_to_overwrite_existing_content() {
    let root = fixture("existing",);
    let created = create_content_template(&root, ContentTemplateKind::Media, "demo-art",).unwrap();

    let error = create_content_template(&root, ContentTemplateKind::Media, "demo-art",)
      .expect_err("existing file should fail",)
      .to_string();

    assert!(error.contains(&format!(
      "content file already exists: `{}`",
      created.display()
    )));
  }
}
