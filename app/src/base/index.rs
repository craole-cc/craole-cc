use crate::prelude::*;

pub fn shell(options : LeptosOptions,) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />

        // ? inner_html is required — text nodes inside view! are
        // ? HTML-escaped, so plain string content never executes
        <script inner_html=THEME_INIT />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}
