use crate::prelude::*;

#[component]
pub fn Highlights() -> impl IntoView {
  let projects = Resource::new(|| (), |()| async move { list_projects().await },);
  let art = Resource::new(|| (), |()| async move { list_media().await },);
  let posts = Resource::new(|| (), |()| async move { list_posts().await },);

  view! {
    <section class="home-highlights readable" aria-labelledby="highlights-title">
      <div class="home-highlights__header">
        <div>
          <p class="home-highlights__eyebrow">"Selected work"</p>
          <h2 id="highlights-title">"A few things worth exploring."</h2>
        </div>
        <p>"Projects, images, and recent writing from the intersection of systems and creative practice."</p>
      </div>
      <div class="home-highlights__groups">
        <Suspense fallback=|| view! { <p aria-busy="true">"Loading projects…"</p> }>
          {move || {
            view! {
              <section class="home-highlights__group" aria-labelledby="featured-projects-title">
                <div class="home-highlights__group-heading">
                  <h3 id="featured-projects-title">"Featured projects"</h3>
                  <a href="/dev">"View all →"</a>
                </div>
                <div class="home-highlights__project-grid">
                  {projects.get().and_then(Result::ok).unwrap_or_default().into_iter().filter(|project| project.featured).take(3).map(|project| view! {
                    <a class="home-highlights__project" href=format!("/dev/{}", project.slug)>
                      <span class="home-highlights__status">{project.status.clone()}</span>
                      <h4>{project.title}</h4>
                      <p>{project.description}</p>
                      <span class="home-highlights__arrow" aria-hidden="true">"↗"</span>
                    </a>
                  }).collect_view()}
                </div>
              </section>
            }
          }}
        </Suspense>
        <Suspense fallback=|| view! { <p aria-busy="true">"Loading art…"</p> }>
          {move || {
            view! {
              <section class="home-highlights__group" aria-labelledby="featured-art-title">
                <div class="home-highlights__group-heading">
                  <h3 id="featured-art-title">"Featured art"</h3>
                  <a href="/art">"View all →"</a>
                </div>
                <div class="home-highlights__art-grid">
                  {art.get().and_then(Result::ok).unwrap_or_default().into_iter().take(4).map(|item| view! {
                    <a href=format!("/art/{}", item.slug) aria-label=item.title>
                      <img src=format!("/{}", item.file_path.trim_start_matches('/')) alt=item.alt_text loading="lazy" />
                    </a>
                  }).collect_view()}
                </div>
              </section>
            }
          }}
        </Suspense>
        <Suspense fallback=|| view! { <p aria-busy="true">"Loading writing…"</p> }>
          {move || {
            view! {
              <section class="home-highlights__group" aria-labelledby="latest-writing-title">
                <div class="home-highlights__group-heading">
                  <h3 id="latest-writing-title">"Featured writing"</h3>
                  <a href="/log">"Read the log →"</a>
                </div>
                <div class="home-highlights__post-grid">
                  {posts.get().and_then(Result::ok).unwrap_or_default().into_iter().filter(|post| post.featured).take(3).map(|post| view! {
                    <a class="home-highlights__post" href=format!("/log/{}", post.slug)>
                      <span>{post.kind}</span>
                      <h4>{post.title}</h4>
                      <p>{post.excerpt.unwrap_or_else(|| "Read the latest note from the log.".into())}</p>
                    </a>
                  }).collect_view()}
                </div>
              </section>
            }
          }}
        </Suspense>
      </div>
    </section>
  }
}
