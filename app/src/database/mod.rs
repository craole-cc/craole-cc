pub mod media;
pub mod posts;
pub mod projects;

pub mod _prelude {
  pub use {
    crate::prelude::{icons::from_tag, *},
    pulldown_cmark::{Options, Parser, html},
  };
}

pub mod prelude {
  pub use super::{
    media::{
      Media, get_media_by_slug, list_media, list_media_by_tag, list_media_by_tags, list_media_tags,
      list_related_media, search_media,
    },
    posts::{GetPostBySlug, Post, PostSummary, get_post_by_slug, list_posts, search_posts},
    projects::{
      Project, ProjectDetail, get_featured_projects, get_project_by_id, get_project_by_slug,
      list_project_tags, list_projects, list_projects_by_status, list_projects_by_tag,
      search_projects,
    },
  };
}
