use crate::prelude::*;

mod hero;
use hero::*;
mod tryout;
use tryout::*;
// mod blog;
// use blog::*;

#[component]
pub fn Temp() -> impl IntoView {
  view! {
    <Hero />
    // <BlogPost1 />
    <TryOut />
  }
}
