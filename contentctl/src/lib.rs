use {
  serde::Deserialize,
  std::{
    collections::{
      HashMap,
      HashSet,
    },
    fs,
    path::{
      Path,
      PathBuf,
    },
  },
  thiserror::Error,
};

const PROJECT_STATUSES : &[&str] = &["active", "building", "planning", "archived",];
const POST_KINDS : &[&str] = &["blog", "cv", "note",];
const MEDIA_TYPES : &[&str] = &["photo", "video",];

#[derive(Debug, Error,)]
pub enum ContentError {
  #[error("failed to read `{path}`: {source}")]
  Read {
    path :   PathBuf,
    #[source]
    source : std::io::Error,
  },
  #[error("failed to parse TOML `{path}`: {source}")]
  Toml {
    path :   PathBuf,
    #[source]
    source : toml::de::Error,
  },
  #[error("content validation failed with {0} error(s)")]
  Validation(usize,),
  #[error("invalid content slug `{0}`")]
  InvalidSlug(String,),
  #[error("content file already exists: `{0}`")]
  AlreadyExists(PathBuf,),
  #[error("failed to write `{path}`: {source}")]
  Write {
    path :   PathBuf,
    #[source]
    source : std::io::Error,
  },
  #[error("unsupported database URL `{0}`; expected sqlite://<path> or sqlite:<path>")]
  UnsupportedDatabaseUrl(String,),
  #[error("failed to open SQLite database `{path}`: {source}")]
  DatabaseOpen {
    path :   PathBuf,
    #[source]
    source : rusqlite::Error,
  },
  #[error("failed to execute SQLite against `{path}`: {source}")]
  DatabaseExec {
    path :   PathBuf,
    #[source]
    source : rusqlite::Error,
  },
  #[error("failed to query SQLite database `{path}`: {source}")]
  DatabaseQuery {
    path :   PathBuf,
    #[source]
    source : rusqlite::Error,
  },
}

#[derive(Debug, Clone, PartialEq, Eq,)]
pub struct ValidationIssue {
  pub path :    PathBuf,
  pub message : String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq,)]
pub struct ValidationReport {
  pub errors :   Vec<ValidationIssue,>,
  pub warnings : Vec<ValidationIssue,>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub struct SyncReport {
  pub projects : i64,
  pub posts :    i64,
  pub media :    i64,
}

impl ValidationReport {
  #[must_use]
  pub const fn is_valid(&self,) -> bool { self.errors.is_empty() }

  fn error(&mut self, path : impl Into<PathBuf,>, message : impl Into<String,>,) {
    self.errors.push(ValidationIssue {
      path :    path.into(),
      message : message.into(),
    },);
  }

  fn warning(&mut self, path : impl Into<PathBuf,>, message : impl Into<String,>,) {
    self.warnings.push(ValidationIssue {
      path :    path.into(),
      message : message.into(),
    },);
  }
}

#[derive(Debug, Deserialize,)]
struct ProjectContent {
  title :       Option<String,>,
  slug :        Option<String,>,
  status :      Option<String,>,
  description : Option<String,>,
  repo_url :    Option<String,>,
  live_url :    Option<String,>,
  featured :    Option<bool,>,
  published :   Option<bool,>,
  sort_order :  Option<i64,>,
  tags :        Option<Vec<String,>,>,
  screenshots : Option<Vec<String,>,>,
}

#[derive(Debug, Deserialize,)]
struct MediaContent {
  title :      Option<String,>,
  slug :       Option<String,>,
  caption :    Option<String,>,
  media_type : Option<String,>,
  file_path :  Option<String,>,
  alt_text :   Option<String,>,
  published :  Option<bool,>,
  sort_order : Option<i64,>,
  taken_at :   Option<String,>,
  width :      Option<i64,>,
  height :     Option<i64,>,
  tags :       Option<Vec<String,>,>,
}

#[derive(Debug, Default,)]
struct PostFrontmatter {
  title :        Option<String,>,
  slug :         Option<String,>,
  kind :         Option<String,>,
  published :    Option<bool,>,
  featured :     Option<bool,>,
  published_at : Option<String,>,
  tags :         Vec<String,>,
  excerpt :      Option<String,>,
  cover_url :    Option<String,>,
}

struct PostContent {
  frontmatter : PostFrontmatter,
  body :        String,
}

/// Validate the repository content tree.
///
/// # Errors
///
/// Returns [`ContentError`] when content files cannot be read or TOML content cannot be parsed.
pub fn validate_content_root(root : &Path,) -> Result<ValidationReport, ContentError,> {
  let mut report = ValidationReport::default();

  validate_projects(root, &mut report,)?;
  validate_posts(root, &mut report,)?;
  validate_media(root, &mut report,)?;

  Ok(report,)
}

fn validate_projects(root : &Path, report : &mut ValidationReport,) -> Result<(), ContentError,> {
  let mut seen = HashMap::<String, PathBuf,>::new();
  for path in files_with_extension(&root.join("content/projects",), "toml",)? {
    let content = read_to_string(&path,)?;
    let project : ProjectContent =
      toml::from_str(&content,).map_err(|source| ContentError::Toml {
        path : path.clone(),
        source,
      },)?;

    validate_required(project.title.as_deref(), &path, "title", report,);
    let slug = validate_slug_field(project.slug.as_deref(), &path, "project", report,);
    validate_required(project.description.as_deref(), &path, "description", report,);

    match project.status.as_deref() {
      | Some(status,) if PROJECT_STATUSES.contains(&status,) => {}
      | Some(status,) => report.error(&path, format!("invalid project status `{status}`"),),
      | None => report.error(&path, "missing required field `status`",),
    }

    if let Some(slug,) = slug
      && let Some(first_path,) = seen.insert(slug.clone(), path.clone(),)
    {
      report.error(
        &path,
        format!(
          "duplicate project slug `{slug}` also used by `{}`",
          first_path.display()
        ),
      );
    }

    validate_optional_url(project.repo_url.as_deref(), &path, "repo_url", report,);
    validate_optional_url(project.live_url.as_deref(), &path, "live_url", report,);
    validate_sort_order(project.sort_order, &path, report,);

    if project.featured.unwrap_or(false,) && !project.published.unwrap_or(false,) {
      report.error(&path, "featured project must also be published",);
    }

    validate_tags(project.tags.as_deref(), &path, "project", report,);

    for screenshot in project.screenshots.unwrap_or_default() {
      validate_public_asset(root, &path, &screenshot, report,);
    }
  }
  Ok((),)
}

fn validate_posts(root : &Path, report : &mut ValidationReport,) -> Result<(), ContentError,> {
  let mut seen = HashMap::<String, PathBuf,>::new();
  for path in files_with_extension(&root.join("content/posts",), "md",)? {
    let content = read_to_string(&path,)?;
    let Some((frontmatter, body,),) = split_frontmatter(&content,) else {
      report.error(&path, "post is missing frontmatter",);
      continue;
    };

    let post = parse_post_frontmatter(frontmatter,);
    validate_required(post.title.as_deref(), &path, "title", report,);
    let slug = validate_slug_field(post.slug.as_deref(), &path, "post", report,);

    match post.kind.as_deref() {
      | Some(kind,) if POST_KINDS.contains(&kind,) => {}
      | Some(kind,) => report.error(&path, format!("invalid post kind `{kind}`"),),
      | None => report.error(&path, "missing required field `kind`",),
    }

    if let Some(slug,) = slug
      && let Some(first_path,) = seen.insert(slug.clone(), path.clone(),)
    {
      report.error(
        &path,
        format!(
          "duplicate post slug `{slug}` also used by `{}`",
          first_path.display()
        ),
      );
    }

    if post.published.unwrap_or(false,) && body.trim().is_empty() {
      report.error(&path, "published post body must not be empty",);
    }

    if let Some(date,) = post.published_at.as_deref() {
      validate_iso_date(date, &path, "published_at", report,);
    }

    if post.featured.unwrap_or(false,) && !post.published.unwrap_or(false,) {
      report.error(&path, "featured post must also be published",);
    }

    if post.tags.is_empty() {
      report.warning(&path, "post has no tags",);
    }
    if post
      .excerpt
      .as_deref()
      .is_some_and(|excerpt| excerpt.trim().is_empty(),)
    {
      report.warning(&path, "post excerpt is empty",);
    }
  }
  Ok((),)
}

fn validate_media(root : &Path, report : &mut ValidationReport,) -> Result<(), ContentError,> {
  let mut seen = HashMap::<String, PathBuf,>::new();
  for path in files_with_extension(&root.join("content/media",), "toml",)? {
    let content = read_to_string(&path,)?;
    let media : MediaContent = toml::from_str(&content,).map_err(|source| ContentError::Toml {
      path : path.clone(),
      source,
    },)?;

    validate_required(media.title.as_deref(), &path, "title", report,);
    let slug = validate_slug_field(media.slug.as_deref(), &path, "media", report,);

    match media.media_type.as_deref() {
      | Some(media_type,) if MEDIA_TYPES.contains(&media_type,) => {}
      | Some(media_type,) => report.error(&path, format!("invalid media type `{media_type}`"),),
      | None => report.error(&path, "missing required field `media_type`",),
    }

    if let Some(slug,) = slug
      && let Some(first_path,) = seen.insert(slug.clone(), path.clone(),)
    {
      report.error(
        &path,
        format!(
          "duplicate media slug `{slug}` also used by `{}`",
          first_path.display()
        ),
      );
    }

    match media.file_path.as_deref() {
      | Some(file_path,) => validate_public_asset(root, &path, file_path, report,),
      | None => report.error(&path, "missing required field `file_path`",),
    }

    if media.published.unwrap_or(false,)
      && media
        .alt_text
        .as_deref()
        .is_none_or(|value| value.trim().is_empty(),)
    {
      report.error(&path, "published media requires non-empty alt_text",);
    }

    validate_sort_order(media.sort_order, &path, report,);
    validate_positive(media.width, &path, "width", report,);
    validate_positive(media.height, &path, "height", report,);
    if let Some(date,) = media.taken_at.as_deref() {
      validate_iso_date(date, &path, "taken_at", report,);
    }
    validate_tags(media.tags.as_deref(), &path, "media", report,);
  }
  Ok((),)
}

fn files_with_extension(dir : &Path, extension : &str,) -> Result<Vec<PathBuf,>, ContentError,> {
  if !dir.exists() {
    return Ok(Vec::new(),);
  }

  let mut files = Vec::new();
  collect_files_with_extension(dir, extension, &mut files,)?;
  files.sort();
  Ok(files,)
}

fn collect_files_with_extension(
  dir : &Path,
  extension : &str,
  files : &mut Vec<PathBuf,>,
) -> Result<(), ContentError,> {
  let entries = fs::read_dir(dir,).map_err(|source| ContentError::Read {
    path : dir.to_path_buf(),
    source,
  },)?;

  for entry in entries {
    let entry = entry.map_err(|source| ContentError::Read {
      path : dir.to_path_buf(),
      source,
    },)?;
    let path = entry.path();
    if path.is_dir() {
      collect_files_with_extension(&path, extension, files,)?;
    } else if path.extension().is_some_and(|ext| ext == extension,) {
      files.push(path,);
    }
  }

  Ok((),)
}

fn read_to_string(path : &Path,) -> Result<String, ContentError,> {
  fs::read_to_string(path,).map_err(|source| ContentError::Read {
    path : path.to_path_buf(),
    source,
  },)
}

fn validate_required(
  value : Option<&str,>,
  path : &Path,
  field : &str,
  report : &mut ValidationReport,
) {
  if value.is_none_or(|value| value.trim().is_empty(),) {
    report.error(path, format!("missing required field `{field}`"),);
  }
}

fn validate_slug_field(
  value : Option<&str,>,
  path : &Path,
  content_type : &str,
  report : &mut ValidationReport,
) -> Option<String,> {
  let Some(slug,) = value else {
    report.error(path, "missing required field `slug`",);
    return None;
  };

  if slug.trim().is_empty() {
    report.error(path, "missing required field `slug`",);
    return None;
  }

  if !is_valid_slug(slug,) {
    report.error(path, format!("invalid {content_type} slug `{slug}`"),);
  }

  Some(slug.to_string(),)
}

fn is_valid_slug(slug : &str,) -> bool {
  let bytes = slug.as_bytes();
  !bytes.is_empty()
    && !bytes.starts_with(b"-",)
    && !bytes.ends_with(b"-",)
    && bytes
      .iter()
      .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-',)
}

fn validate_optional_url(
  value : Option<&str,>,
  path : &Path,
  field : &str,
  report : &mut ValidationReport,
) {
  let Some(url,) = value else {
    return;
  };

  if !(url.starts_with("https://",) || url.starts_with("http://",)) || url.contains(' ',) {
    report.error(path, format!("invalid URL in `{field}`: `{url}`"),);
  }
}

fn validate_sort_order(value : Option<i64,>, path : &Path, report : &mut ValidationReport,) {
  if value.is_some_and(|value| value < 0,) {
    report.error(path, "sort_order must be non-negative",);
  }
}

fn validate_positive(
  value : Option<i64,>,
  path : &Path,
  field : &str,
  report : &mut ValidationReport,
) {
  if value.is_some_and(|value| value <= 0,) {
    report.error(path, format!("{field} must be positive"),);
  }
}

fn validate_tags(
  tags : Option<&[String],>,
  path : &Path,
  content_type : &str,
  report : &mut ValidationReport,
) {
  let Some(tags,) = tags else {
    report.warning(path, format!("{content_type} has no tags"),);
    return;
  };

  if tags.is_empty() {
    report.warning(path, format!("{content_type} has no tags"),);
  }

  let mut seen = HashSet::new();
  for tag in tags {
    if tag.trim().is_empty() {
      report.error(path, "tags must not contain empty values",);
    }
    if !seen.insert(tag.to_lowercase(),) {
      report.warning(path, format!("duplicate tag `{tag}`"),);
    }
  }
}

fn validate_public_asset(
  root : &Path, path : &Path, asset : &str, report : &mut ValidationReport,
) {
  let relative = asset.trim_start_matches('/',);
  if relative.starts_with("..",) || relative.contains("/../",) {
    report.error(
      path,
      format!("asset path escapes public directory: `{asset}`"),
    );
    return;
  }

  let full_path = root.join("public",).join(relative,);
  if !full_path.is_file() {
    report.error(
      path,
      format!("asset does not exist: `{}`", full_path.display()),
    );
  }
}

fn validate_iso_date(value : &str, path : &Path, field : &str, report : &mut ValidationReport,) {
  let valid = value.len() == 10
    && value.as_bytes()[4] == b'-'
    && value.as_bytes()[7] == b'-'
    && value
      .as_bytes()
      .iter()
      .enumerate()
      .all(|(idx, byte,)| idx == 4 || idx == 7 || byte.is_ascii_digit(),);

  if !valid {
    report.error(path, format!("invalid date in `{field}`: `{value}`"),);
  }
}

fn split_frontmatter(content : &str,) -> Option<(&str, &str,),> {
  let delimiter = if content.starts_with("---\n",) {
    "---"
  } else if content.starts_with("+++\n",) {
    "+++"
  } else {
    return None;
  };

  let rest = &content[4 ..];
  let end_marker = format!("\n{delimiter}\n");
  let end = rest.find(&end_marker,)?;
  let frontmatter = &rest[.. end];
  let body = &rest[end + end_marker.len() ..];
  Some((frontmatter, body,),)
}

fn parse_post_frontmatter(frontmatter : &str,) -> PostFrontmatter {
  let mut post = PostFrontmatter::default();

  for line in frontmatter.lines() {
    let line = line.split('#',).next().unwrap_or_default().trim();
    if line.is_empty() {
      continue;
    }

    let Some((key, value,),) = line.split_once([':', '=',],) else {
      continue;
    };
    let key = key.trim();
    let value = value.trim();

    match key {
      | "title" => post.title = Some(parse_scalar(value,),),
      | "slug" => post.slug = Some(parse_scalar(value,),),
      | "kind" => post.kind = Some(parse_scalar(value,),),
      | "published" => post.published = parse_bool(value,),
      | "featured" => post.featured = parse_bool(value,),
      | "published_at" => post.published_at = Some(parse_scalar(value,),),
      | "tags" => post.tags = parse_array(value,),
      | "excerpt" => post.excerpt = Some(parse_scalar(value,),),
      | "cover_url" => post.cover_url = Some(parse_scalar(value,),),
      | _ => {}
    }
  }

  post
}

fn parse_scalar(value : &str,) -> String {
  value
    .trim()
    .trim_matches('"',)
    .trim_matches('\'',)
    .to_string()
}

fn parse_bool(value : &str,) -> Option<bool,> {
  match parse_scalar(value,).as_str() {
    | "true" => Some(true,),
    | "false" => Some(false,),
    | _ => None,
  }
}

fn parse_array(value : &str,) -> Vec<String,> {
  let value = value.trim();
  let Some(inner,) = value
    .strip_prefix('[',)
    .and_then(|value| value.strip_suffix(']',),)
  else {
    return Vec::new();
  };

  inner
    .split(',',)
    .map(parse_scalar,)
    .filter(|value| !value.is_empty(),)
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub enum ContentTemplateKind {
  Project,
  Post,
  Media,
}

impl ContentTemplateKind {
  #[must_use]
  pub const fn directory(self,) -> &'static str {
    match self {
      | Self::Project => "content/projects",
      | Self::Post => "content/posts",
      | Self::Media => "content/media",
    }
  }

  #[must_use]
  pub const fn extension(self,) -> &'static str {
    match self {
      | Self::Project | Self::Media => "toml",
      | Self::Post => "md",
    }
  }

  #[must_use]
  pub fn parse(value : &str,) -> Option<Self,> {
    match value {
      | "project" | "projects" => Some(Self::Project,),
      | "post" | "posts" => Some(Self::Post,),
      | "media" => Some(Self::Media,),
      | _ => None,
    }
  }
}

/// Create a draft content template for a project, post, or media item.
///
/// # Errors
///
/// Returns [`ContentError`] when the slug is invalid, the target file already exists, or the file
/// cannot be written.
pub fn create_content_template(
  root : &Path,
  kind : ContentTemplateKind,
  slug : &str,
) -> Result<PathBuf, ContentError,> {
  if !is_valid_slug(slug,) {
    return Err(ContentError::InvalidSlug(slug.to_string(),),);
  }

  let dir = root.join(kind.directory(),);
  let path = dir.join(format!("{}.{}", slug, kind.extension()),);
  if path.exists() {
    return Err(ContentError::AlreadyExists(path,),);
  }

  fs::create_dir_all(&dir,).map_err(|source| ContentError::Write {
    path : dir.clone(),
    source,
  },)?;

  let content = render_template(kind, slug,);
  fs::write(&path, content,).map_err(|source| ContentError::Write {
    path : path.clone(),
    source,
  },)?;

  Ok(path,)
}

fn render_template(kind : ContentTemplateKind, slug : &str,) -> String {
  let title = title_from_slug(slug,);
  match kind {
    | ContentTemplateKind::Project => format!(
      "title = \"{title}\"\nslug = \"{slug}\"\nstatus = \"planning\"\ndescription = \"TODO: Short, portfolio-ready project summary.\"\nfeatured = false\npublished = false\nsort_order = 0\ntags = []\nscreenshots = []\n",
    ),
    | ContentTemplateKind::Post => format!(
      "---\ntitle: \"{title}\"\nslug: \"{slug}\"\nkind: \"note\"\npublished: false\nfeatured: false\ntags: []\n---\n\n# {title}\n\nTODO: Write the post body.\n",
    ),
    | ContentTemplateKind::Media => format!(
      "title = \"{title}\"\nslug = \"{slug}\"\nmedia_type = \"photo\"\nfile_path = \"/media/art/{slug}.webp\"\nalt_text = \"\"\npublished = false\nsort_order = 0\ntaken_at = \"\"\nwidth = 0\nheight = 0\ntags = []\n",
    ),
  }
}

fn title_from_slug(slug : &str,) -> String {
  slug
    .split('-',)
    .filter(|part| !part.is_empty(),)
    .map(|part| {
      let mut chars = part.chars();
      let Some(first,) = chars.next() else {
        return String::new();
      };
      format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
    },)
    .collect::<Vec<_>>()
    .join(" ",)
}

/// Export valid content files as a `SQLite` seed script.
///
/// # Errors
///
/// Returns [`ContentError`] when validation fails, content cannot be read, or TOML content cannot
/// be parsed.
#[allow(clippy::format_push_string)]
pub fn export_seed_sql(root : &Path,) -> Result<String, ContentError,> {
  let report = validate_content_root(root,)?;
  if !report.is_valid() {
    return Err(ContentError::Validation(report.errors.len(),),);
  }

  let projects = load_projects(root,)?;
  let posts = load_posts(root,)?;
  let media = load_media(root,)?;

  let mut sql = String::from(
    "PRAGMA foreign_keys = ON;\nBEGIN;\nDELETE FROM project_tags;\nDELETE FROM post_tags;\nDELETE \
     FROM media_tags;\nDELETE FROM projects;\nDELETE FROM posts;\nDELETE FROM media;\n",
  );

  for project in projects {
    sql.push_str(&format!(
      "INSERT INTO projects (title, slug, description, status, repo_url, live_url, screenshots, \
       featured, published, sort_order) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {});\n",
      sql_string(project.title.as_deref().unwrap_or_default(),),
      sql_string(project.slug.as_deref().unwrap_or_default(),),
      sql_string(project.description.as_deref().unwrap_or_default(),),
      sql_string(project.status.as_deref().unwrap_or("planning",),),
      sql_optional(project.repo_url.as_deref(),),
      sql_optional(project.live_url.as_deref(),),
      sql_optional(
        project
          .screenshots
          .as_ref()
          .map(|screenshots| screenshots.join(",",))
          .as_deref(),
      ),
      bool_sql(project.featured.unwrap_or(false,),),
      bool_sql(project.published.unwrap_or(false,),),
      project.sort_order.unwrap_or(0,),
    ),);

    let slug = project.slug.as_deref().unwrap_or_default();
    for tag in project.tags.unwrap_or_default() {
      sql.push_str(&format!(
        "INSERT INTO project_tags (project_id, tag) SELECT id, {} FROM projects WHERE slug = {};\n",
        sql_string(&tag,),
        sql_string(slug,),
      ),);
    }
  }

  for post in posts {
    let frontmatter = post.frontmatter;
    sql.push_str(&format!(
      "INSERT INTO posts (title, slug, body, excerpt, kind, featured, cover_url, published, \
       published_at) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {});\n",
      sql_string(frontmatter.title.as_deref().unwrap_or_default(),),
      sql_string(frontmatter.slug.as_deref().unwrap_or_default(),),
      sql_string(&post.body,),
      sql_optional(frontmatter.excerpt.as_deref(),),
      sql_string(frontmatter.kind.as_deref().unwrap_or("blog",),),
      bool_sql(frontmatter.featured.unwrap_or(false,),),
      sql_optional(frontmatter.cover_url.as_deref(),),
      bool_sql(frontmatter.published.unwrap_or(false,),),
      sql_optional(frontmatter.published_at.as_deref(),),
    ),);

    let slug = frontmatter.slug.as_deref().unwrap_or_default();
    for tag in frontmatter.tags {
      sql.push_str(&format!(
        "INSERT INTO post_tags (post_id, tag) SELECT id, {} FROM posts WHERE slug = {};\n",
        sql_string(&tag,),
        sql_string(slug,),
      ),);
    }
  }

  for item in media {
    sql.push_str(&format!(
      "INSERT INTO media (title, slug, caption, media_type, file_path, alt_text, width, height, \
       published, sort_order, taken_at) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});\n",
      sql_string(item.title.as_deref().unwrap_or_default(),),
      sql_string(item.slug.as_deref().unwrap_or_default(),),
      sql_optional(item.caption.as_deref(),),
      sql_string(item.media_type.as_deref().unwrap_or("photo",),),
      sql_string(item.file_path.as_deref().unwrap_or_default(),),
      sql_string(item.alt_text.as_deref().unwrap_or_default(),),
      sql_i64_optional(item.width,),
      sql_i64_optional(item.height,),
      bool_sql(item.published.unwrap_or(false,),),
      item.sort_order.unwrap_or(0,),
      sql_optional(item.taken_at.as_deref(),),
    ),);

    let slug = item.slug.as_deref().unwrap_or_default();
    for tag in item.tags.unwrap_or_default() {
      sql.push_str(&format!(
        "INSERT INTO media_tags (media_id, tag) SELECT id, {} FROM media WHERE slug = {};\n",
        sql_string(&tag,),
        sql_string(slug,),
      ),);
    }
  }

  sql.push_str("COMMIT;\n",);
  Ok(sql,)
}

/// Validate content, apply migrations, seed `SQLite`, and return seeded table counts.
///
/// # Errors
///
/// Returns [`ContentError`] when validation fails, migrations cannot be read, the database URL is
/// unsupported, `SQLite` execution fails, or final counts cannot be queried.
pub fn sync_content_database(
  root : &Path,
  database_url : &str,
) -> Result<SyncReport, ContentError,> {
  let report = validate_content_root(root,)?;
  if !report.is_valid() {
    return Err(ContentError::Validation(report.errors.len(),),);
  }

  let database_path = sqlite_database_path(database_url,)?;
  if let Some(parent,) = database_path.parent() {
    fs::create_dir_all(parent,).map_err(|source| ContentError::Write {
      path : parent.to_path_buf(),
      source,
    },)?;
  }

  let connection = rusqlite::Connection::open(&database_path,).map_err(|source| {
    ContentError::DatabaseOpen {
      path : database_path.clone(),
      source,
    }
  },)?;
  connection
    .execute_batch("PRAGMA foreign_keys = ON;",)
    .map_err(|source| ContentError::DatabaseExec {
      path : database_path.clone(),
      source,
    },)?;

  for migration in migration_files(root,)? {
    let sql = read_to_string(&migration,)?;
    connection
      .execute_batch(&sql,)
      .map_err(|source| ContentError::DatabaseExec {
        path : database_path.clone(),
        source,
      },)?;
  }

  let seed_sql = export_seed_sql(root,)?;
  connection
    .execute_batch(&seed_sql,)
    .map_err(|source| ContentError::DatabaseExec {
      path : database_path.clone(),
      source,
    },)?;

  Ok(SyncReport {
    projects : count_rows(&connection, &database_path, "projects",)?,
    posts :    count_rows(&connection, &database_path, "posts",)?,
    media :    count_rows(&connection, &database_path, "media",)?,
  },)
}

fn sqlite_database_path(database_url : &str,) -> Result<PathBuf, ContentError,> {
  let path = if let Some(path,) = database_url.strip_prefix("sqlite://",) {
    path
  } else if let Some(path,) = database_url.strip_prefix("sqlite:",) {
    path
  } else {
    return Err(ContentError::UnsupportedDatabaseUrl(database_url.to_string(),),);
  };

  if path.trim().is_empty() || path == ":memory:" {
    return Ok(PathBuf::from(path,),);
  }

  Ok(PathBuf::from(path,),)
}

fn migration_files(root : &Path,) -> Result<Vec<PathBuf,>, ContentError,> {
  files_with_extension(&root.join("database/migrations",), "sql",)
}

fn count_rows(
  connection : &rusqlite::Connection,
  database_path : &Path,
  table : &str,
) -> Result<i64, ContentError,> {
  connection
    .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get(0,),)
    .map_err(|source| ContentError::DatabaseQuery {
      path : database_path.to_path_buf(),
      source,
    },)
}

fn load_projects(root : &Path,) -> Result<Vec<ProjectContent,>, ContentError,> {
  files_with_extension(&root.join("content/projects",), "toml",)?
    .into_iter()
    .map(|path| {
      let content = read_to_string(&path,)?;
      toml::from_str(&content,).map_err(|source| ContentError::Toml { path, source, },)
    },)
    .collect()
}

fn load_media(root : &Path,) -> Result<Vec<MediaContent,>, ContentError,> {
  files_with_extension(&root.join("content/media",), "toml",)?
    .into_iter()
    .map(|path| {
      let content = read_to_string(&path,)?;
      toml::from_str(&content,).map_err(|source| ContentError::Toml { path, source, },)
    },)
    .collect()
}

fn load_posts(root : &Path,) -> Result<Vec<PostContent,>, ContentError,> {
  files_with_extension(&root.join("content/posts",), "md",)?
    .into_iter()
    .map(|path| {
      let content = read_to_string(&path,)?;
      let Some((frontmatter, body,),) = split_frontmatter(&content,) else {
        return Err(ContentError::Validation(1,),);
      };
      Ok(PostContent {
        frontmatter : parse_post_frontmatter(frontmatter,),
        body :        body.to_string(),
      },)
    },)
    .collect()
}

fn bool_sql(value : bool,) -> i32 { i32::from(value,) }

fn sql_i64_optional(value : Option<i64,>,) -> String {
  value.map_or_else(|| "NULL".to_string(), |value| value.to_string(),)
}

fn sql_optional(value : Option<&str,>,) -> String {
  value.map_or_else(|| "NULL".to_string(), sql_string,)
}

fn sql_string(value : &str,) -> String { format!("'{}'", value.replace('\'', "''")) }
