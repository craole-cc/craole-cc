mod db;

use {
  axum::{
    Router,
    extract::FromRef,
    serve,
  },
  core::{
    App,
    shell,
  },
  leptos::{
    logging::log,
    prelude::*,
  },
  leptos_axum::{
    LeptosRoutes,
    file_and_error_handler,
    generate_route_list,
  },
  sqlx::SqlitePool,
  std::env,
  tokio::net::TcpListener,
};

// ── App state ─────────────────────────────────────────────────────────────────
// FromRef lets Axum extract individual fields from AppState by type.
// Leptos uses this to pull LeptosOptions from AppState automatically.

#[derive(Clone,)]
struct AppState {
  leptos_options : LeptosOptions,
  pool :           SqlitePool,
}

impl FromRef<AppState,> for LeptosOptions {
  fn from_ref(state : &AppState,) -> Self { state.leptos_options.clone() }
}

impl FromRef<AppState,> for SqlitePool {
  fn from_ref(state : &AppState,) -> Self { state.pool.clone() }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<(),> {
  // Load .env first so DATABASE_URL is available for everything below.
  dotenvy::dotenv().ok();

  simple_logger::init_with_env().unwrap_or_else(|_| {
    simple_logger::init_with_level(log::Level::Info,).unwrap();
  },);

  let database_url =
    env::var("DATABASE_URL",).unwrap_or_else(|_| "sqlite:./portfolio.db".to_string(),);

  log!("Connecting to database: {}", database_url);
  let pool = db::init(&database_url,).await?;
  log!("Database ready — migrations applied");

  let conf = get_configuration(None,).unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;
  let routes = generate_route_list(App,);

  let app_state = AppState {
    leptos_options : leptos_options.clone(),
    pool :           pool.clone(),
  };

  let app = Router::new()
    // leptos_routes_with_context injects values into the Leptos reactive scope
    // for every server function call. This is how server functions reach the
    // pool via expect_context::<SqlitePool>().
    .leptos_routes_with_context(
      &app_state,
      routes,
      // additional_context — called once per request before the server fn runs
      {
        let pool = pool.clone();
        move || provide_context(pool.clone(),)
      },
      // app shell fn
      {
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone(),)
      },
    )
    // ── TODO: Admin auth middleware ────────────────────────────────────────
    // Add before the fallback when ready:
    //
    //   .nest("/admin", admin_router())
    //
    // where admin_router() wraps handlers with a RequireAuth middleware layer:
    //
    //   Router::new()
    //     .route("/", get(admin_handler))
    //     .layer(middleware::from_fn(require_auth))
    //
    // require_auth checks a signed session cookie set on POST /admin/login.
    // See core/src/pages/home/mod.rs for any admin component stubs.
    // ──────────────────────────────────────────────────────────────────────

  .fallback(file_and_error_handler::<AppState, _>(shell,),)
  .with_state::<()>(app_state,);

  log!("Listening on http://{}", addr);
  let listener = TcpListener::bind(&addr,).await?;
  serve(listener, app,).await?;

  Ok((),)
}
