use {
  contentctl::{
    ContentTemplateKind,
    ValidationReport,
    create_content_template,
    export_static_json,
    sync_content_database,
    validate_content_root,
  },
  std::{
    env,
    path::PathBuf,
    process::ExitCode,
  },
};

fn main() -> ExitCode {
  let mut args = env::args().skip(1,);
  let command = args.next().unwrap_or_else(|| "validate".to_string(),);

  match command.as_str() {
    | "validate" => {
      let root = args
        .next()
        .map_or_else(|| PathBuf::from(".",), PathBuf::from,);
      match validate_content_root(&root,) {
        | Ok(report,) => print_report(&report,),
        | Err(error,) => {
          eprintln!("content validation failed: {error}");
          ExitCode::FAILURE
        }
      }
    }
    | "export-sql" => {
      let root = args
        .next()
        .map_or_else(|| PathBuf::from(".",), PathBuf::from,);
      match contentctl::export_seed_sql(&root,) {
        | Ok(sql,) => {
          print!("{sql}");
          ExitCode::SUCCESS
        }
        | Err(error,) => {
          eprintln!("content SQL export failed: {error}");
          ExitCode::FAILURE
        }
      }
    }
    | "new" => create_new_content(args,),
    | "sync-db" => sync_database(args,),
    | "export-json" => export_json(args,),
    | "help" | "--help" | "-h" => {
      print_usage();
      ExitCode::SUCCESS
    }
    | other => {
      eprintln!("unknown command `{other}`");
      print_usage();
      ExitCode::FAILURE
    }
  }
}

fn create_new_content(mut args : impl Iterator<Item = String,>,) -> ExitCode {
  let Some(kind,) = args.next().and_then(|kind| ContentTemplateKind::parse(&kind,),) else {
    eprintln!("missing or invalid content type for `new`");
    print_usage();
    return ExitCode::FAILURE;
  };

  let Some(slug,) = args.next() else {
    eprintln!("missing slug for `new`");
    print_usage();
    return ExitCode::FAILURE;
  };

  let root = args
    .next()
    .map_or_else(|| PathBuf::from(".",), PathBuf::from,);

  match create_content_template(&root, kind, &slug,) {
    | Ok(path,) => {
      println!("created {}", path.display());
      ExitCode::SUCCESS
    }
    | Err(error,) => {
      eprintln!("failed to create content template: {error}");
      ExitCode::FAILURE
    }
  }
}

fn sync_database(mut args : impl Iterator<Item = String,>,) -> ExitCode {
  let root = args
    .next()
    .map_or_else(|| PathBuf::from("."), PathBuf::from,);
  let database_url = args.next().or_else(|| env::var("DATABASE_URL").ok(),).unwrap_or_else(|| {
    "sqlite://database/data/portfolio.db".to_string()
  },);

  match sync_content_database(&root, &database_url,) {
    | Ok(report,) => {
      println!(
        "content database synced: projects={} posts={} media={}",
        report.projects, report.posts, report.media
      );
      ExitCode::SUCCESS
    }
    | Err(error,) => {
      eprintln!("content database sync failed: {error}");
      ExitCode::FAILURE
    }
  }
}

fn export_json(mut args : impl Iterator<Item = String,>,) -> ExitCode {
  let root = args
    .next()
    .map_or_else(|| PathBuf::from("."), PathBuf::from,);
  let output_dir = args
    .next()
    .map_or_else(|| PathBuf::from("dist/data"), PathBuf::from,);

  match export_static_json(&root, &output_dir,) {
    | Ok(report,) => {
      println!(
        "static JSON exported to {}: projects={} posts={} media={}",
        output_dir.display(), report.projects, report.posts, report.media
      );
      ExitCode::SUCCESS
    }
    | Err(error,) => {
      eprintln!("static JSON export failed: {error}");
      ExitCode::FAILURE
    }
  }
}

fn print_report(report : &ValidationReport,) -> ExitCode {
  for warning in &report.warnings {
    eprintln!("{}\n  warning: {}", warning.path.display(), warning.message);
  }

  for error in &report.errors {
    eprintln!("{}\n  error: {}", error.path.display(), error.message);
  }

  if report.is_valid() {
    println!("content validation passed");
    ExitCode::SUCCESS
  } else {
    eprintln!(
      "content validation failed: {} error(s)",
      report.errors.len()
    );
    ExitCode::FAILURE
  }
}

fn print_usage() {
  eprintln!("Usage: contentctl <command> [args]");
  eprintln!("Commands:");
  eprintln!("  validate [repo-root]              Validate content files");
  eprintln!("  export-sql [repo-root]            Print a SQLite seed script generated from content files");
  eprintln!("  export-json [repo-root] [output-dir]");
  eprintln!("                                  Export static JSON data files");
  eprintln!("  new <project|post|media> <slug> [repo-root]");
  eprintln!("                                  Create a draft content template");
  eprintln!("  sync-db [repo-root] [database-url]");
  eprintln!("                                  Apply migrations and sync content into SQLite");
}
