#!/usr/bin/env rust-script

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
  env,
  ffi::OsString,
  fs,
  io::{
    self,
    Write,
  },
  path::{
    Path,
    PathBuf,
  },
  process::{
    Command,
    Stdio,
  },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
enum Action {
  Loc,
  Src,
  Cp,
}

#[derive(Debug,)]
struct Config {
  action :   Action,
  raw :      bool,
  commands : Vec<String,>,
}

fn usage() {
  eprintln!(
    r#"Usage: cmd ACTION [OPTIONS] COMMAND...

Inspect commands available on PATH.

Actions:
  loc              Print command paths
  src              Show command source with bat if available, otherwise cat
  cp               Copy command source to clipboard

Options:
  -x, --raw        Copy raw source without headers (cp only)
  -h, --help       Show this help text

Examples:
  cmd loc gcp update
  cmd src gcp
  cmd cp gcp update
  cmd cp --raw gcp"#
  );
}

fn parse_args() -> Result<Option<Config,>, i32,> {
  let mut args = env::args().skip(1,);

  let Some(action_raw,) = args.next() else {
    usage();
    return Ok(None,);
  };

  if action_raw == "-h" || action_raw == "--help" {
    usage();
    return Ok(None,);
  }

  let action = match action_raw.as_str() {
    | "loc" => Action::Loc,
    | "src" => Action::Src,
    | "cp" => Action::Cp,
    | _ => {
      eprintln!("Unknown action: {action_raw}");
      usage();
      return Err(2,);
    }
  };

  let mut raw = false;
  let mut commands = Vec::new();

  while let Some(arg,) = args.next() {
    match arg.as_str() {
      | "-x" | "--raw" => {
        raw = true;
      }
      | "-h" | "--help" => {
        usage();
        return Ok(None,);
      }
      | "--" => {
        commands.extend(args,);
        break;
      }
      | _ if arg.starts_with('-',) => {
        eprintln!("Unknown option: {arg}");
        usage();
        return Err(2,);
      }
      | _ => {
        commands.push(arg,);
        commands.extend(args,);
        break;
      }
    }
  }

  if commands.is_empty() {
    eprintln!("Missing command name.");
    usage();
    return Err(2,);
  }

  Ok(Some(Config {
    action,
    raw,
    commands,
  },),)
}

fn is_executable(path : &Path,) -> bool {
  if !path.is_file() {
    return false;
  }

  #[cfg(unix)]
  {
    match fs::metadata(path,) {
      | Ok(meta,) => meta.permissions().mode() & 0o111 != 0,
      | Err(_,) => false,
    }
  }

  #[cfg(not(unix))]
  {
    true
  }
}

fn find_cmd(name : &str,) -> Option<PathBuf,> {
  let candidate = Path::new(name,);

  if name.contains('/',) && is_executable(candidate,) {
    return Some(candidate.to_path_buf(),);
  }

  let path_var : OsString = env::var_os("PATH",)?;
  for dir in env::split_paths(&path_var,) {
    let path = dir.join(name,);

    if is_executable(&path,) {
      return Some(path,);
    }
  }

  None
}

fn resolve_command(name : &str,) -> Result<PathBuf, (),> {
  match find_cmd(name,) {
    | Some(path,) => Ok(path,),
    | None => {
      eprintln!("Command not found: {name}");
      Err((),)
    }
  }
}

fn run_viewer(path : &Path,) -> io::Result<bool,> {
  let viewer = find_cmd("bat",).or_else(|| find_cmd("cat",),);

  let Some(viewer,) = viewer else {
    eprintln!("Error: neither bat nor cat was found.");
    return Ok(false,);
  };

  let status = Command::new(viewer,).arg(path,).status()?;
  Ok(status.success(),)
}

fn clipboard_command() -> Option<(PathBuf, Vec<&'static str,>,),> {
  if let Some(path,) = find_cmd("clip",) {
    return Some((path, vec![],),);
  }

  if let Some(path,) = find_cmd("wl-copy",) {
    return Some((path, vec![],),);
  }

  if let Some(path,) = find_cmd("xclip",) {
    return Some((path, vec!["-selection", "clipboard"],),);
  }

  if let Some(path,) = find_cmd("pbcopy",) {
    return Some((path, vec![],),);
  }

  None
}

fn copy_to_clipboard(bytes : &[u8],) -> io::Result<bool,> {
  let Some((program, args,),) = clipboard_command() else {
    eprintln!("Error: no clipboard command found.");
    return Ok(false,);
  };

  let mut child = Command::new(program,)
    .args(args,)
    .stdin(Stdio::piped(),)
    .spawn()?;

  if let Some(stdin,) = child.stdin.as_mut() {
    stdin.write_all(bytes,)?;
  }

  let status = child.wait()?;
  Ok(status.success(),)
}

fn execute_loc(commands : &[String],) -> i32 {
  let mut status = 0;

  for cmd in commands {
    match resolve_command(cmd,) {
      | Ok(path,) => println!("{}", path.display()),
      | Err((),) => status = 1,
    }
  }

  status
}

fn execute_src(commands : &[String],) -> i32 {
  let mut status = 0;

  for cmd in commands {
    let Ok(path,) = resolve_command(cmd,) else {
      status = 1;
      continue;
    };

    match run_viewer(&path,) {
      | Ok(true,) => {}
      | Ok(false,) => status = 1,
      | Err(err,) => {
        eprintln!("Error showing {}: {err}", path.display());
        status = 1;
      }
    }
  }

  status
}

fn execute_cp(commands : &[String], raw : bool,) -> i32 {
  let mut status = 0;
  let mut output = Vec::new();
  let mut first = true;

  for cmd in commands {
    let Ok(path,) = resolve_command(cmd,) else {
      status = 1;
      continue;
    };

    if !first {
      output.extend_from_slice(b"\n\n",);
    }

    first = false;

    if !raw {
      let header = format!("# cmd: {} ({})\n\n", cmd, path.display());
      output.extend_from_slice(header.as_bytes(),);
    }

    match fs::read(&path,) {
      | Ok(bytes,) => output.extend_from_slice(&bytes,),
      | Err(err,) => {
        eprintln!("Error reading {}: {err}", path.display());
        status = 1;
      }
    }
  }

  if output.is_empty() {
    return status;
  }

  match copy_to_clipboard(&output,) {
    | Ok(true,) => status,
    | Ok(false,) => 1,
    | Err(err,) => {
      eprintln!("Error copying to clipboard: {err}");
      1
    }
  }
}

fn execute(config : Config,) -> i32 {
  match config.action {
    | Action::Loc => execute_loc(&config.commands,),
    | Action::Src => execute_src(&config.commands,),
    | Action::Cp => execute_cp(&config.commands, config.raw,),
  }
}

fn main() {
  let status = match parse_args() {
    | Ok(Some(config,),) => execute(config,),
    | Ok(None,) => 0,
    | Err(code,) => code,
  };

  std::process::exit(status,);
}
