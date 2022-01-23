use std::process::Command;

pub fn git_commit(message: &str) {
  let runningCommand = format!("git commit -m \"{}\"", message);
  println!("{}", runningCommand);
  Command::new("sh")
    .arg("-c")
    .arg(runningCommand)
    .output()
    .expect("failed to commit");
}