use crate::command::COMMAND_NAMES;

pub fn find_command_match(prefix: &str) -> Option<&str> {
  let cmd = COMMAND_NAMES.iter().find(|name| name.starts_with(prefix))?;
  Some(&cmd[prefix.len()..])
}

pub fn find_command_autocompletion(prefix: &str) -> Option<&str> {
  let cmd = COMMAND_NAMES.iter().find(|name| name.starts_with(prefix))?;
  cmd.split_whitespace().next()
}
