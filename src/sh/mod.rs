use std::process::Command;

pub fn git_commit(message: &str) {
  let running_command = format!("git commit -m \"{}\"", message.trim());
    println!("{}", running_command);
    Command::new("sh")
      .arg("-c")
      .arg(running_command)
      .spawn()
      .expect("failed to commit");
}