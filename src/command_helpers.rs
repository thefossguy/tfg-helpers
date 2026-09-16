use std::io;
use std::process::{Command, ExitStatus, Output};

pub trait CommandOutputStatus {
    fn was_process_successful(&self) -> bool;
}

trait HasExitStatus {
    fn exit_status(&self) -> &ExitStatus;
}

impl HasExitStatus for Output {
    fn exit_status(&self) -> &ExitStatus {
        &self.status
    }
}

impl HasExitStatus for ExitStatus {
    fn exit_status(&self) -> &Self {
        self
    }
}

impl<T: HasExitStatus> CommandOutputStatus for Result<T, io::Error> {
    fn was_process_successful(&self) -> bool {
        match self {
            Err(_) => false,
            Ok(inner) => matches!(inner.exit_status().code(), Some(0)),
        }
    }
}

pub fn get_command_argv(command: &Command) -> Vec<String> {
    let mut argv: Vec<String> = command
        .get_args()
        .map(|arg| arg.to_string_lossy().to_string())
        .collect();
    argv.insert(0, command.get_program().to_string_lossy().to_string());
    argv
}

#[macro_export]
macro_rules! log_then_output {
    ($command:expr) => {{
        $crate::log_info!(
            "Running: {:?}",
            $crate::command_helpers::get_command_argv(&$command)
        );
        $command.output()
    }};
}

#[macro_export]
macro_rules! log_then_status {
    ($command:expr, $formatter:path) => {{
        $formatter!(
            "Running: {:?}",
            $crate::command_helpers::get_command_argv(&$command)
        );
        $command.status()
    }};

    ($command:expr) => {{
        eprintln!(
            "Running: {:?}",
            $crate::command_helpers::get_command_argv(&$command)
        );
        $command.status()
    }};
}

#[macro_export]
macro_rules! get_process_stdout {
    ($process_output:expr) => {
        String::from_utf8_lossy(&$process_output.stdout)
            .trim()
            .to_string()
    };
}

#[macro_export]
macro_rules! get_process_stderr {
    ($process_output:expr) => {
        String::from_utf8_lossy(&$process_output.stderr)
            .trim()
            .to_string()
    };
}

#[macro_export]
macro_rules! get_formatted_process_stderr {
    ($process_output:expr) => {
        $crate::make_formatted_error!($crate::get_process_stderr!($process_output))
    };
}

#[macro_export]
macro_rules! make_formatted_error {
    ($passed_error:expr) => {
        format!("\n```\n{}\n```", $passed_error)
    };
}

#[macro_export]
macro_rules! return_stderr_as_err {
    ($base_error_message:expr, $process_output_result:expr) => {{
        // Re-assign in case the `$base_error_message` is not an identifier.
        let base_error_message = $base_error_message;
        match $process_output_result {
            Ok(process_output) => Err(format!(
                "{base_error_message}{}",
                $crate::get_formatted_process_stderr!(process_output)
            )
            .into()),
            Err(e) => {
                Err(format!("{base_error_message}{}", $crate::make_formatted_error!(e)).into())
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;
    use std::process::Command;

    #[test]
    fn was_process_successful_output_err() {
        let mut cmd = Command::new("command-will-never-exist");
        let cmd_output = cmd.output();
        assert_eq!(false, (&cmd_output).was_process_successful());
        assert_matches!(cmd_output, Err(e) if e.kind() == std::io::ErrorKind::NotFound);
    }

    #[test]
    fn was_process_successful_status_err() {
        let mut cmd = Command::new("command-will-never-exist");
        let cmd_status = cmd.status();
        assert_eq!(false, (&cmd_status).was_process_successful());
        assert_matches!(cmd_status, Err(e) if e.kind() == std::io::ErrorKind::NotFound);
    }

    #[test]
    fn was_process_successful_output_ok_no_exit_status() {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", "kill -9 $$"]);
        let cmd_output = cmd.output();
        assert_eq!(false, (&cmd_output).was_process_successful());
        assert_matches!(cmd_output.ok(), Some(ref output_result_inner) if output_result_inner.status.code().is_none());
    }

    #[test]
    fn was_process_successful_status_ok_no_exit_status() {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", "kill -9 $$"]);
        let cmd_status = cmd.status();
        assert_eq!(false, (&cmd_status).was_process_successful());
        assert_matches!(cmd_status.ok(), Some(ref status_result_inner) if status_result_inner.code().is_none());
    }

    #[test]
    fn was_process_successful_output_ok_bad_exit_status() {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", "exit 1"]);
        let cmd_output = cmd.output();
        assert_eq!(false, (&cmd_output).was_process_successful());
        assert_matches!(cmd_output.ok(), Some(ref output_result_inner) if output_result_inner.status.code().is_some());
    }

    #[test]
    fn was_process_successful_status_ok_bad_exit_status() {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", "exit 1"]);
        let cmd_status = cmd.status();
        assert_eq!(false, (&cmd_status).was_process_successful());
        assert_matches!(cmd_status.ok(), Some(ref status_result_inner) if status_result_inner.code().is_some());
    }

    #[test]
    fn was_process_successful_output_ok_good_exit_status() {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", "exit 0"]);
        let cmd_output = cmd.output();
        assert_eq!(true, (&cmd_output).was_process_successful());
        assert_matches!(cmd_output.ok(), Some(ref output_result_inner) if output_result_inner.status.code().is_some());
    }

    #[test]
    fn was_process_successful_status_ok_good_exit_status() {
        let mut cmd = Command::new("bash");
        cmd.args(["-c", "exit 0"]);
        let cmd_status = cmd.status();
        assert_eq!(true, (&cmd_status).was_process_successful());
        assert_matches!(cmd_status.ok(), Some(ref status_result_inner) if status_result_inner.code().is_some());
    }

    #[test]
    fn get_command_argv_test() {
        let command = "mock-command".to_string();
        let args = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()];
        let mut cmd = Command::new(&command);
        cmd.args(&args);
        let mut original_cmd_argv = args;
        original_cmd_argv.insert(0, command);
        let constructed_cmd_argv = get_command_argv(&cmd);
        assert_eq!(original_cmd_argv, constructed_cmd_argv);
    }
}
