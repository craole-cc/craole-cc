use {
  contentctl::{
    ValidationReport,
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
  let root = args
    .next()
    .map_or_else(|| PathBuf::from(".",), PathBuf::from,);

  match command.as_str() {
    | "validate" => match validate_content_root(&root,) {
      | Ok(report,) => print_report(&report,),
      | Err(error,) => {
        eprintln!("content validation failed: {error}");
        ExitCode::FAILURE
      }
    },
    | "export-sql" => match contentctl::export_seed_sql(&root,) {
      | Ok(sql,) => {
        print!("{sql}");
        ExitCode::SUCCESS
      }
      | Err(error,) => {
        eprintln!("content SQL export failed: {error}");
        ExitCode::FAILURE
      }
    },
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
  eprintln!("Usage: contentctl <command> [repo-root]");
  eprintln!("Commands:");
  eprintln!("  validate    Validate content files");
  eprintln!("  export-sql  Print a SQLite seed script generated from content files");
}
