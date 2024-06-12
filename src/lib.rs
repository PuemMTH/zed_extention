use zed_extension_api as zed;

struct HelloExtension;

impl zed::Extension for HelloExtension {
    fn new() -> Self {
        HelloExtension
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "echo".to_string(),
            args: vec!["Hello, world!".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(HelloExtension);
