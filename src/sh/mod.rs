use std::process::Command;

pub fn git_commit(message: &str) {
  let running_command = format!("git commit -m \"{}\"", message.trim());
    println!("{}", running_command);
    Command::new("sh")
      .arg("-c")
      .arg(running_command)
      .output()
      .expect("failed to commit");
}

// pub fn copy_to_clip(s: &str) {
//     Command::new("sh")
//       .arg("-c")
//       .arg(format!("echo {} | pbcopy", s))
//       .output()
//       .expect("failed to commit");
// }