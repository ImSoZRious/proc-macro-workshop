pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

pub struct CommandBuilder {
    executable: std::option::Option<String>,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: std::option::Option::None,
            args: Vec::new(),
            env: Vec::new(),
            current_dir: std::option::Option::None,
        }
    }
}
impl CommandBuilder {
    pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = std::option::Option::Some(executable);
        self
    }
    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = args;
        self
    }
    pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = std::option::Option::Some(current_dir);
        self
    }
    pub fn build(
        &mut self,
    ) -> std::result::Result<Command, std::boxed::Box<dyn std::error::Error>> {
        if false || self.executable.is_none() {
            return std::result::Result::Err(std::boxed::Box::<dyn std::error::Error>::from(
                "Unspecify field".to_owned(),
            ));
        }
        std::result::Result::Ok(Command {
            executable: self.executable.clone().unwrap(),
            args: self.args.clone(),
            env: self.env.clone(),
            current_dir: self.current_dir.clone(),
        })
    }
    pub fn arg(&mut self, value: String) -> &mut Self {
        self.args.push(value);
        self
    }
    pub fn env(&mut self, value: String) -> &mut Self {
        self.env.push(value);
        self
    }
}
