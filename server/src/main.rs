use {
  app::*,
  axum::Router,
  leptos::{
    logging::log,
    prelude::*,
  },
  leptos_axum::{
    LeptosRoutes,
    file_and_error_handler,
    generate_route_list,
  },
  tokio::net::TcpListener,
};

#[tokio::main]
async fn main() {
  let conf = get_configuration(None,).unwrap();
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  let routes = generate_route_list(App,);

  let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone(),)
    },)
    .fallback(file_and_error_handler(shell,),)
    .with_state(leptos_options,);

  log!("listening on http://{}", &addr);
  let listener = TcpListener::bind(&addr,).await.unwrap();
  axum::serve(listener, app.into_make_service(),)
    .await
    .unwrap();
}
