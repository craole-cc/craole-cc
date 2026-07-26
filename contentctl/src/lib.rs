use {
  serde::{
    Deserialize,
    Serialize,
  },
  std::{
    collections::{
      HashMap,
      HashSet,
    },
    fmt::Write as _,
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
  #[error("failed to serialize JSON `{path}`: {source}")]
  Json {
    path :   PathBuf,
    #[source]
    source : serde_json::Error,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize,)]
pub struct ExportJsonReport {
  pub projects : usize,
  pub posts :    usize,
  pub media :    usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize,)]
pub struct StaticSiteReport {
  pub projects : usize,
  pub posts :    usize,
  pub media :    usize,
  pub pages :    usize,
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

#[derive(Debug, Deserialize, Serialize,)]
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

#[derive(Debug, Deserialize, Serialize,)]
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

#[derive(Debug, Default, Serialize,)]
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

#[derive(Debug, Serialize,)]
struct PostContent {
  #[serde(flatten)]
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
  if asset.starts_with("https://",) {
    if asset.len() <= "https://".len()
      || asset.chars().any(char::is_whitespace,)
      || !asset["https://".len() ..].contains('.',)
    {
      report.error(path, format!("invalid remote media URL: `{asset}`"),);
    }
    return;
  }
  if asset.starts_with("http://",) {
    report.error(path, format!("remote media URL must use HTTPS: `{asset}`"),);
    return;
  }

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
      "title = \"{title}\"\nslug = \"{slug}\"\nstatus = \"planning\"\ndescription = \"TODO: \
       Short, portfolio-ready project summary.\"\nfeatured = false\npublished = false\nsort_order \
       = 0\ntags = []\nscreenshots = []\n",
    ),
    | ContentTemplateKind::Post => format!(
      "---\ntitle: \"{title}\"\nslug: \"{slug}\"\nkind: \"note\"\npublished: false\nfeatured: \
       false\ntags: []\n---\n\n# {title}\n\nTODO: Write the post body.\n",
    ),
    | ContentTemplateKind::Media => format!(
      "title = \"{title}\"\nslug = \"{slug}\"\nmedia_type = \"photo\"\nfile_path = \
       \"/media/art/{slug}.webp\"\nalt_text = \"\"\npublished = false\nsort_order = 0\ntaken_at = \
       \"\"\nwidth = 0\nheight = 0\ntags = []\n",
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
    .collect::<Vec<_,>>()
    .join(" ",)
}

/// Export valid content as static-friendly JSON files.
///
/// # Errors
///
/// Returns [`ContentError`] when validation fails, content cannot be read, JSON cannot be
/// serialized, or files cannot be written.
pub fn export_static_json(
  root : &Path,
  output_dir : &Path,
) -> Result<ExportJsonReport, ContentError,> {
  let report = validate_content_root(root,)?;
  if !report.is_valid() {
    return Err(ContentError::Validation(report.errors.len(),),);
  }

  let projects = load_projects(root,)?;
  let posts = load_posts(root,)?;
  let media = load_media(root,)?;
  let export_report = ExportJsonReport {
    projects : projects.len(),
    posts :    posts.len(),
    media :    media.len(),
  };

  fs::create_dir_all(output_dir,).map_err(|source| ContentError::Write {
    path : output_dir.to_path_buf(),
    source,
  },)?;

  write_json_file(&output_dir.join("projects.json",), &projects,)?;
  write_json_file(&output_dir.join("posts.json",), &posts,)?;
  write_json_file(&output_dir.join("media.json",), &media,)?;
  write_json_file(&output_dir.join("manifest.json",), &export_report,)?;

  Ok(export_report,)
}

fn write_json_file<T : Serialize,>(path : &Path, value : &T,) -> Result<(), ContentError,> {
  let json = serde_json::to_string_pretty(value,).map_err(|source| ContentError::Json {
    path : path.to_path_buf(),
    source,
  },)?;
  fs::write(path, format!("{json}\n"),).map_err(|source| ContentError::Write {
    path : path.to_path_buf(),
    source,
  },)
}

/// Export a minimal static fallback site backed by generated JSON data.
///
/// # Errors
///
/// Returns [`ContentError`] when validation fails, content cannot be read, JSON cannot be
/// serialized, assets cannot be copied, or HTML files cannot be written.
pub fn export_static_site(
  root : &Path,
  output_dir : &Path,
) -> Result<StaticSiteReport, ContentError,> {
  let report = validate_content_root(root,)?;
  if !report.is_valid() {
    return Err(ContentError::Validation(report.errors.len(),),);
  }

  let mut projects = load_projects(root,)?;
  let mut posts = load_posts(root,)?;
  let media = load_media(root,)?;
  projects.sort_by_key(|project| project.sort_order.unwrap_or(0,),);
  posts.sort_by(|left, right| {
    right
      .frontmatter
      .published_at
      .cmp(&left.frontmatter.published_at,)
      .then_with(|| left.frontmatter.slug.cmp(&right.frontmatter.slug,),)
  },);

  fs::create_dir_all(output_dir,).map_err(|source| ContentError::Write {
    path : output_dir.to_path_buf(),
    source,
  },)?;
  copy_public_assets(root, output_dir,)?;
  copy_extra_static_assets(root, output_dir,)?;
  let json_report = export_static_json(root, &output_dir.join("data",),)?;

  let published_projects = projects
    .iter()
    .filter(|project| project.published.unwrap_or(false,),)
    .collect::<Vec<_,>>();

  let published_posts = posts
    .iter()
    .filter(|post| post.frontmatter.published.unwrap_or(false,),)
    .collect::<Vec<_,>>();
  let published_media = media
    .iter()
    .filter(|item| item.published.unwrap_or(false,),)
    .collect::<Vec<_,>>();

  let mut pages = 0;
  write_html_file(
    &output_dir.join("index.html",),
    &render_home_page(&published_projects, &published_posts, &published_media,),
  )?;
  pages += 1;

  write_html_file(
    &output_dir.join("dev/index.html",),
    &render_projects_index(&published_projects,),
  )?;
  pages += 1;

  for project in &published_projects {
    if let Some(slug,) = project.slug.as_deref() {
      write_html_file(
        &output_dir.join("dev",).join(slug,).join("index.html",),
        &render_project_page(project,),
      )?;
      pages += 1;
    }
  }

  write_html_file(
    &output_dir.join("log/index.html",),
    &render_posts_index(&published_posts,),
  )?;
  pages += 1;

  for post in &published_posts {
    if let Some(slug,) = post.frontmatter.slug.as_deref() {
      write_html_file(
        &output_dir.join("log",).join(slug,).join("index.html",),
        &render_post_page(post,),
      )?;
      pages += 1;
    }
  }

  write_html_file(
    &output_dir.join("art/index.html",),
    &render_media_index(&published_media,),
  )?;
  pages += 1;

  write_html_file(&output_dir.join("404.html",), &render_404_page(),)?;
  pages += 1;

  write_html_file(
    &output_dir.join("sitemap.xml",),
    &render_sitemap(&published_projects, &published_posts,),
  )?;

  Ok(StaticSiteReport {
    projects : json_report.projects,
    posts : json_report.posts,
    media : json_report.media,
    pages,
  },)
}

fn write_html_file(path : &Path, content : &str,) -> Result<(), ContentError,> {
  if let Some(parent,) = path.parent() {
    fs::create_dir_all(parent,).map_err(|source| ContentError::Write {
      path : parent.to_path_buf(),
      source,
    },)?;
  }
  fs::write(path, content,).map_err(|source| ContentError::Write {
    path : path.to_path_buf(),
    source,
  },)
}

fn copy_public_assets(root : &Path, output_dir : &Path,) -> Result<(), ContentError,> {
  let public_dir = root.join("public",);
  if !public_dir.exists() {
    return Ok((),);
  }
  copy_dir_contents(&public_dir, output_dir,)
}

fn copy_extra_static_assets(root : &Path, output_dir : &Path,) -> Result<(), ContentError,> {
  let assets_dir = root.join("assets",);
  if !assets_dir.exists() {
    return Ok((),);
  }
  let destination = output_dir.join("assets",);
  fs::create_dir_all(&destination,).map_err(|source| ContentError::Write {
    path : destination.clone(),
    source,
  },)?;
  copy_dir_contents(&assets_dir, &destination,)
}

fn copy_dir_contents(source : &Path, destination : &Path,) -> Result<(), ContentError,> {
  for entry in fs::read_dir(source,).map_err(|source_error| ContentError::Read {
    path :   source.to_path_buf(),
    source : source_error,
  },)?
  {
    let entry = entry.map_err(|source_error| ContentError::Read {
      path :   source.to_path_buf(),
      source : source_error,
    },)?;
    let source_path = entry.path();
    let destination_path = destination.join(entry.file_name(),);
    if source_path.is_dir() {
      fs::create_dir_all(&destination_path,).map_err(|source_error| ContentError::Write {
        path :   destination_path.clone(),
        source : source_error,
      },)?;
      copy_dir_contents(&source_path, &destination_path,)?;
    } else if source_path.is_file() {
      if let Some(parent,) = destination_path.parent() {
        fs::create_dir_all(parent,).map_err(|source_error| ContentError::Write {
          path :   parent.to_path_buf(),
          source : source_error,
        },)?;
      }
      fs::copy(&source_path, &destination_path,).map_err(|source_error| ContentError::Write {
        path :   destination_path,
        source : source_error,
      },)?;
    }
  }
  Ok((),)
}

fn page_shell(title : &str, active : &str, body : &str,) -> String {
  let escaped_title = escape_html(title,);
  let home = site_path("",);
  let dev = site_path("dev/",);

  let log = site_path("log/",);
  let art = site_path("art/",);
  format!(
    r#"<!doctype html>
<html lang="en" data-theme="dark">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{escaped_title}</title>
  <meta name="description" content="Craole.CC — creative engineering, Rust systems, and visual narrative.">
  <style>{}</style>
</head>
<body data-section="{}">
  <header class="site-header">
    <a class="brand" href="{home}" aria-label="Craole.CC home"><span class="brand__mark">CC</span><span>Craole.CC</span></a>
    <nav aria-label="Primary navigation">
      <a href="{dev}">Dev</a>

      <a href="{log}">Log</a>
      <a href="{art}">Art</a>
    </nav>
  </header>
  <main>{body}</main>
  <footer><strong>Craole.CC</strong><span>Creative engineering & visual narrative.</span></footer>
</body>
</html>
"#,
    static_css(),
    escape_html(active,),
  )
}

fn site_path(path : &str,) -> String {
  let base = std::env::var("CRAOLE_STATIC_BASE_PATH",).unwrap_or_else(|_| "/".to_string(),);
  let mut base = if base.trim().is_empty() {
    "/".to_string()
  } else {
    base
  };
  if !base.starts_with('/',) {
    base.insert(0, '/',);
  }
  if !base.ends_with('/',) {
    base.push('/',);
  }
  format!("{}{}", base, path.trim_start_matches('/'))
}

#[allow(clippy::format_collect)]
fn render_home_page(
  projects : &[&ProjectContent],
  posts : &[&PostContent],
  media : &[&MediaContent],
) -> String {
  let dev = site_path("dev/",);
  let log = site_path("log/",);
  let art = site_path("art/",);
  let icons = [
    ("Rust", site_path("icons/logos/rust.svg",),),
    ("Leptos", site_path("icons/logos/leptos.svg",),),
    ("Nix", site_path("icons/logos/nixos.svg",),),
    ("Python", site_path("icons/logos/python.svg",),),
    ("GitHub", site_path("icons/logos/github.svg",),),
    ("Linux", site_path("icons/logos/linux.svg",),),
  ];
  let icon_html = icons
    .iter()
    .map(|(name, src,)| {
      format!(
        "<li><img src=\"{}\" alt=\"{}\"><span>{}</span></li>",
        escape_attr(src,),
        escape_attr(name,),
        escape_html(name,),
      )
    },)
    .collect::<String>();
  page_shell(
    "Craole.CC",
    "home",
    &format!(
      "<section class=\"hero\"><figure class=\"hero__backdrop\" aria-hidden=\"true\"><span \
       class=\"hero__slide hero__slide--one\"></span><span \
       class=\"hero__scrim\"></span></figure><article class=\"hero__content\"><p \
       class=\"eyebrow\">Creative engineering & visual narrative</p><h1><span>Craig \
       </span><em>Craole</em><span> Cole</span></h1><p class=\"hero__sub\">Raised on rhythm, \
       building with Rust. Code is another instrument of expression through structure.</p><div \
       class=\"hero__actions\"><a class=\"button button--primary\" href=\"{dev}\">See the \
       work</a><a class=\"button\" href=\"{log}\">Read the \
       log</a></div></article></section><section class=\"vision\"><div \
       class=\"vision__label\">The Vision</div><p>From music production to teaching to systems \
       programming, the through-line is expression: structure with soul, precision with \
       personality.</p></section><section class=\"stack\"><p class=\"eyebrow\">Tools & \
       language</p><ul>{icon_html}</ul></section><section \
       class=\"section-grid\"><article><h2>Featured projects</h2>{}</article><article><h2>Latest \
       writing</h2>{}</article><article><h2>Art & media</h2><p>{} published media item(s). <a \
       href=\"{art}\">Explore the visual side</a>.</p></article></section>",
      render_project_cards(projects,),
      render_post_list(posts,),
      media.len(),
    ),
  )
}

fn render_projects_index(projects : &[&ProjectContent],) -> String {
  page_shell(
    "Projects | Craole.CC",
    "dev",
    &format!(
      "<section><h1>Projects</h1>{}</section>",
      render_project_cards(projects,)
    ),
  )
}

fn render_project_page(project : &ProjectContent,) -> String {
  let title = project.title.as_deref().unwrap_or("Untitled project",);
  let tags = render_tags(project.tags.as_deref().unwrap_or(&[],),);
  let repo = project
    .repo_url
    .as_deref()
    .map(|url| format!("<a href=\"{}\">Repository</a>", escape_attr(url)),)
    .unwrap_or_default();
  let live = project
    .live_url
    .as_deref()
    .map(|url| format!("<a href=\"{}\">Live site</a>", escape_attr(url)),)
    .unwrap_or_default();
  page_shell(
    title,
    "dev",
    &format!(
      "<article><p><a href=\"{}\">← Projects</a></p><h1>{}</h1><p class=\"lede\">{}</p><p>Status: \
       <strong>{}</strong></p><div class=\"tags\">{tags}</div><p class=\"links\">{} \
       {}</p></article>",
      site_path("dev/",),
      escape_html(title,),
      escape_html(project.description.as_deref().unwrap_or_default(),),
      escape_html(project.status.as_deref().unwrap_or("planning",),),
      repo,
      live,
    ),
  )
}

fn render_posts_index(posts : &[&PostContent],) -> String {
  page_shell(
    "Log | Craole.CC",
    "log",
    &format!(
      "<section><h1>Log</h1>{}</section>",
      render_post_list(posts,)
    ),
  )
}

fn render_post_page(post : &PostContent,) -> String {
  let title = post
    .frontmatter
    .title
    .as_deref()
    .unwrap_or("Untitled post",);
  let date = post.frontmatter.published_at.as_deref().unwrap_or("Draft",);
  page_shell(
    title,
    "log",
    &format!(
      "<article><p><a href=\"{}\">← Log</a></p><h1>{}</h1><p class=\"eyebrow\">{}</p><div \
       class=\"tags\">{}</div><div class=\"prose\">{}</div></article>",
      site_path("log/",),
      escape_html(title,),
      escape_html(date,),
      render_tags(&post.frontmatter.tags,),
      markdown_to_html(&post.body,),
    ),
  )
}

#[allow(clippy::format_collect)]
fn render_media_index(media : &[&MediaContent],) -> String {
  let items = if media.is_empty() {
    format!(
      "<p>No published media yet. See <a href=\"{}\">media.json</a>.</p>",
      site_path("data/media.json",)
    )
  } else {
    media
      .iter()
      .map(|item| {
        format!(
          "<article class=\"card\"><h2>{}</h2><p>{}</p><p><code>{}</code></p></article>",
          escape_html(item.title.as_deref().unwrap_or("Untitled media",),),
          escape_html(item.alt_text.as_deref().unwrap_or_default(),),
          escape_html(item.file_path.as_deref().unwrap_or_default(),),
        )
      },)
      .collect::<String>()
  };
  page_shell(
    "Art | Craole.CC",
    "art",
    &format!("<section><h1>Art</h1>{items}</section>"),
  )
}

fn render_404_page() -> String {
  page_shell(
    "Not found | Craole.CC",
    "404",
    &format!(
      "<section><h1>Not found</h1><p>This static fallback does not include that route.</p><p><a \
       href=\"{}\">Return home</a></p></section>",
      site_path("",)
    ),
  )
}

#[allow(clippy::format_collect)]
fn render_sitemap(projects : &[&ProjectContent], posts : &[&PostContent],) -> String {
  let mut urls = vec![
    "/".to_string(),
    "/dev/".to_string(),
    "/log/".to_string(),
    "/art/".to_string(),
  ];
  urls.extend(
    projects
      .iter()
      .filter_map(|project| project.slug.as_ref().map(|slug| format!("/dev/{slug}/"),),),
  );
  urls.extend(posts.iter().filter_map(|post| {
    post
      .frontmatter
      .slug
      .as_ref()
      .map(|slug| format!("/log/{slug}/"),)
  },),);
  let body = urls
    .into_iter()
    .map(|url| format!("  <url><loc>{}</loc></url>\n", escape_html(&url)),)
    .collect::<String>();
  format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{body}</urlset>\n")
}

fn render_project_cards(projects : &[&ProjectContent],) -> String {
  if projects.is_empty() {
    return "<p>No published projects yet.</p>".to_string();
  }

  projects.iter().fold(String::new(), |mut html, project| {
    let title = project.title.as_deref().unwrap_or("Untitled project",);
    let slug = project.slug.as_deref().unwrap_or_default();
    write!(
      html,
      "<article class=\"card\"><h2><a href=\"{}\">{}</a></h2><p>{}</p><div \
       class=\"tags\">{}</div></article>",
      site_path(&format!("dev/{}/", escape_attr(slug))),
      escape_html(title,),
      escape_html(project.description.as_deref().unwrap_or_default(),),
      render_tags(project.tags.as_deref().unwrap_or(&[],),),
    )
    .expect("writing to String should not fail",);
    html
  },)
}

#[allow(clippy::format_collect)]
fn render_post_list(posts : &[&PostContent],) -> String {
  if posts.is_empty() {
    return "<p>No published posts yet.</p>".to_string();
  }
  posts
    .iter()
    .map(|post| {
      let title = post
        .frontmatter
        .title
        .as_deref()
        .unwrap_or("Untitled post",);
      let slug = post.frontmatter.slug.as_deref().unwrap_or_default();
      format!(
        "<article class=\"card\"><h2><a href=\"{}\">{}</a></h2><p>{}</p><p \
         class=\"eyebrow\">{}</p></article>",
        site_path(&format!("log/{}/", escape_attr(slug))),
        escape_html(title,),
        escape_html(post.frontmatter.excerpt.as_deref().unwrap_or_default(),),
        escape_html(
          post
            .frontmatter
            .published_at
            .as_deref()
            .unwrap_or("undated",),
        ),
      )
    },)
    .collect::<String>()
}

#[allow(clippy::format_collect)]
fn render_tags(tags : &[String],) -> String {
  tags
    .iter()
    .map(|tag| format!("<span>{}</span>", escape_html(tag)),)
    .collect::<String>()
}

#[allow(clippy::format_push_string)]
fn markdown_to_html(markdown : &str,) -> String {
  let mut html = String::new();
  let mut paragraph = Vec::new();
  for line in markdown.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      flush_paragraph(&mut html, &mut paragraph,);
    } else if let Some(heading,) = trimmed.strip_prefix("# ",) {
      flush_paragraph(&mut html, &mut paragraph,);
      html.push_str(&format!("<h1>{}</h1>", escape_html(heading)),);
    } else if let Some(heading,) = trimmed.strip_prefix("## ",) {
      flush_paragraph(&mut html, &mut paragraph,);
      html.push_str(&format!("<h2>{}</h2>", escape_html(heading)),);
    } else if let Some(heading,) = trimmed.strip_prefix("### ",) {
      flush_paragraph(&mut html, &mut paragraph,);
      html.push_str(&format!("<h3>{}</h3>", escape_html(heading)),);
    } else {
      paragraph.push(trimmed.to_string(),);
    }
  }
  flush_paragraph(&mut html, &mut paragraph,);
  html
}

#[allow(clippy::format_push_string)]
fn flush_paragraph(html : &mut String, paragraph : &mut Vec<String,>,) {
  if paragraph.is_empty() {
    return;
  }
  html.push_str(&format!("<p>{}</p>", escape_html(&paragraph.join(" "))),);
  paragraph.clear();
}

fn escape_html(value : &str,) -> String {
  value
    .replace('&', "&amp;",)
    .replace('<', "&lt;",)
    .replace('>', "&gt;",)
    .replace('"', "&quot;",)
    .replace('\'', "&#39;",)
}

fn escape_attr(value : &str,) -> String { escape_html(value,) }

const fn static_css() -> &'static str {
  r":root{color-scheme:dark;--bg:#05070d;--panel:rgba(13,18,29,.78);--panel-strong:rgba(18,25,39,.92);--text:#f7efe2;--muted:#b8c0cc;--gold:#f4ce73;--gold-2:#b77b2d;--line:rgba(244,206,115,.18);--glow:rgba(244,206,115,.28)}*{box-sizing:border-box}html{scroll-behavior:smooth}body{margin:0;font-family:'Plus Jakarta Sans',Inter,ui-sans-serif,system-ui,sans-serif;background:radial-gradient(circle at 20% 0%,#263040 0,#111827 34rem,#05070d 72rem);color:var(--text);line-height:1.6;min-height:100vh}a{color:inherit;text-decoration:none}a:hover{color:var(--gold)}.site-header{position:sticky;top:0;z-index:10;display:flex;align-items:center;justify-content:space-between;gap:1.25rem;padding:1rem clamp(1rem,4vw,4.5rem);background:rgba(5,7,13,.72);border-bottom:1px solid rgba(255,255,255,.08);backdrop-filter:blur(18px)}.brand{display:flex;align-items:center;gap:.75rem;font-weight:900;letter-spacing:.02em}.brand__mark{display:grid;place-items:center;width:2.35rem;height:2.35rem;border-radius:999px;background:linear-gradient(135deg,var(--gold),var(--gold-2));color:#10141f;box-shadow:0 0 34px var(--glow)}nav{display:flex;gap:.35rem;flex-wrap:wrap}nav a{padding:.55rem .85rem;border-radius:999px;color:#d6dbe5}nav a:hover,body[data-section=dev] nav a[href$='dev/'],body[data-section=data] nav a[href$='data/'],body[data-section=log] nav a[href$='log/'],body[data-section=art] nav a[href$='art/']{background:rgba(244,206,115,.12);color:var(--gold)}main{overflow:hidden}.hero{position:relative;min-height:calc(100vh - 4.5rem);display:grid;grid-template-columns:minmax(0,1fr) minmax(16rem,24rem);gap:clamp(1.5rem,4vw,4rem);align-items:center;padding:clamp(5rem,11vw,9rem) clamp(1rem,5vw,5rem)}.hero__backdrop{position:absolute;inset:0;margin:0;z-index:-2}.hero__slide{position:absolute;inset:0;background-size:cover;background-position:center;filter:saturate(1.05) contrast(1.05)}.hero__slide--one{background-image:url('https://images.unsplash.com/photo-1433086966358-54859d0ed716?auto=format&fit=crop&w=1920&q=80')}.hero__scrim{position:absolute;inset:0;background:linear-gradient(90deg,rgba(5,7,13,.95),rgba(5,7,13,.68) 46%,rgba(5,7,13,.28)),radial-gradient(circle at 74% 54%,rgba(244,206,115,.18),transparent 25rem)}.hero__content{max-width:62rem}.eyebrow{margin:0 0 .85rem;color:var(--gold);text-transform:uppercase;letter-spacing:.18em;font-size:.76rem;font-weight:800}.hero h1{margin:0;font-size:clamp(3.8rem,11vw,9.5rem);line-height:.86;letter-spacing:-.075em;text-wrap:balance}.hero h1 em{font-style:italic;color:var(--gold);text-shadow:0 0 40px var(--glow)}.hero__sub{max-width:45rem;font-size:clamp(1.15rem,2vw,1.6rem);color:#e6dfd2;margin:1.25rem 0 0}.hero__actions{display:flex;gap:.85rem;flex-wrap:wrap;margin-top:2rem}.button{display:inline-flex;align-items:center;justify-content:center;min-height:2.85rem;padding:.8rem 1.1rem;border:1px solid rgba(255,255,255,.2);border-radius:999px;background:rgba(255,255,255,.08);font-weight:800}.button--primary{background:linear-gradient(135deg,var(--gold),var(--gold-2));border-color:transparent;color:#111827}.vision,.stack,.section-grid,section:not(.hero){max-width:1180px;margin:0 auto;padding:clamp(3rem,7vw,6rem) clamp(1rem,4vw,3rem)}.vision{display:grid;grid-template-columns:12rem 1fr;gap:clamp(1rem,4vw,4rem);border-top:1px solid var(--line);border-bottom:1px solid var(--line)}.vision__label{color:var(--gold);font-size:.8rem;letter-spacing:.18em;text-transform:uppercase;font-weight:900}.vision p{font-size:clamp(1.6rem,4vw,3rem);line-height:1.1;margin:0;letter-spacing:-.04em}.stack ul{display:grid;grid-template-columns:repeat(auto-fit,minmax(8rem,1fr));gap:.85rem;list-style:none;margin:1rem 0 0;padding:0}.stack li,.card{border:1px solid rgba(255,255,255,.1);background:var(--panel);border-radius:1.2rem;box-shadow:0 20px 60px rgba(0,0,0,.24)}.stack li{display:flex;align-items:center;gap:.75rem;padding:1rem}.stack img{width:2rem;height:2rem;object-fit:contain}.section-grid{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:1rem}.section-grid>article,section>article:first-child:not(.hero__content){border:1px solid rgba(255,255,255,.1);background:rgba(9,13,23,.64);border-radius:1.6rem;padding:clamp(1.1rem,3vw,2rem)}h1,h2,h3{line-height:1.05;letter-spacing:-.04em}h1{font-size:clamp(2.4rem,7vw,5rem)}h2{font-size:clamp(1.8rem,4vw,3rem);margin:.2rem 0 1rem}.card{padding:1.1rem;margin:1rem 0}.card h2{font-size:1.35rem}.card p{color:var(--muted)}.lede{font-size:1.25rem;color:#e7e0d2}.tags{display:flex;gap:.45rem;flex-wrap:wrap;margin:.9rem 0}.tags span{border:1px solid rgba(244,206,115,.2);border-radius:999px;padding:.22rem .65rem;color:#f6d98c;background:rgba(244,206,115,.08);font-size:.85rem}.links{display:flex;gap:1rem;flex-wrap:wrap}.prose{max-width:72ch}.prose p{color:#d5dbe6}footer{display:flex;justify-content:space-between;gap:1rem;flex-wrap:wrap;padding:2rem clamp(1rem,4vw,4rem);color:#aeb6c4;border-top:1px solid rgba(255,255,255,.08)}code{color:#c6f6d5}@media (max-width:820px){.site-header{align-items:flex-start;flex-direction:column}.hero{grid-template-columns:1fr;min-height:auto;padding-top:4rem}.vision{grid-template-columns:1fr}.section-grid{grid-template-columns:1fr}.hero h1{font-size:clamp(3.4rem,18vw,6rem)}}"
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

  let connection =
    rusqlite::Connection::open(&database_path,).map_err(|source| ContentError::DatabaseOpen {
      path : database_path.clone(),
      source,
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
    return Err(ContentError::UnsupportedDatabaseUrl(
      database_url.to_string(),
    ),);
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
    .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
      row.get(0,)
    },)
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
