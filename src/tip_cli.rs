use std::{env, fs, io::prelude::*, path::Path};

use crate::{Command, CommandTypes, Cursor, FunctionTypes, ParseFile};

#[derive(Debug, Clone, PartialEq)]
pub struct TipCli {
    pub tip_dir: String,
    pub project_name: String,
    pub command: Command,
}

impl TipCli {
    pub fn init(cmd_args: &[String]) -> Result<TipCli, String> {
        let home_dir =
            env::var("HOME").map_err(|_| "Error: could not find the home directory: {err}")?;
        let tip_dir = format!("{home_dir}/.tip");
        if !Path::new(tip_dir.as_str()).is_dir() {
            fs::create_dir(&tip_dir).map_err(|_| "Error: tip dir couldn't be created")?;
        }
        let project_name = cmd_args[1].clone();
        if !Path::new(format!("{tip_dir}/{project_name}").as_str()).is_dir() {
            return Err("Error: project not found.".to_string());
        }
        let command = Command::new(
            cmd_args[2].as_str(),
            cmd_args[3].as_str(),
            cmd_args[4..].to_vec(),
        )?;
        Ok(TipCli {
            tip_dir,
            project_name,
            command,
        })
    }

    pub fn perform(&self) -> Cursor {
        match self.command.command_type {
            CommandTypes::Create => match self.command.command_func.func_type {
                FunctionTypes::Project => Cursor {
                    list: None,
                    ok: self.create_new_project(),
                },
                FunctionTypes::Tip => Cursor {
                    list: None,
                    ok: self.create_new_tip(),
                },
                other => Cursor {
                    list: None,
                    ok: Err(format!(
                        "Error: invalid function for this command: {}",
                        other.to_str()
                    )),
                },
            },
            _ => Cursor {
                list: None,
                ok: Err(String::from("Not implimented")),
            },
        }
    }

    pub fn get_full_path(&self) -> String {
        format!("{}/{}", self.tip_dir, self.project_name)
    }

    fn create_new_project(&self) -> Result<(), String> {
        let new_project_dir_name = self.get_full_path();
        _ = fs::create_dir(&new_project_dir_name);
        let currdir = env::current_dir()
            .map_err(|e| format!("Error: couldn't get the current directory: {e}"))?;
        let meta_data = format!(
            "@project({})\n@path({})\n@count(1)\n",
            self.project_name,
            currdir.display()
        );
        let mut meta_file = fs::File::create(format!("{}/000.tip", new_project_dir_name))
            .map_err(|e| format!("Error: couldn't create metadata file: {e}"))?;
        meta_file
            .write_all(meta_data.clone().as_bytes())
            .map_err(|e| format!("Error: couldn't write to metadata file: {e}"))?;
        Ok(())
    }

    fn create_new_tip(&self) -> Result<(), String> {
        let meta_file = fs::read_to_string(self.get_full_path())
            .map_err(|e| format!("Error opening metadaa file: {e}"))?;
        let mut meta_parsed = ParseFile::parse_meta_file(&meta_file)?;
        let new_tip_file_path = format!("{}/{:03}.tip", self.get_full_path(), meta_parsed.count);
        let func_args = self.command.command_func.func_args.clone();
        let critical: u8 = func_args[0].trim().parse().map_err(|_| {
            "Wrong datatype: critical(first argument) must be an unsigned integer".to_string()
        })?;
        let mut title = String::new();
        // This means we don't have to pass the title as one argument
        func_args[1..].iter().for_each(|line| {
            for ch in line.chars() {
                title.push(ch);
            }
            title.push(' ');
        });
        meta_parsed.update_meta_for_new_tip(self)?;
        let mut new_tip_file =
            fs::File::create(new_tip_file_path).map_err(|_| "Error: couldn't create tip file")?;
        let new_file_content = format!(
            "@title({})\n@critical({})\n@done({})\n",
            title.trim(),
            critical,
            false
        );
        new_tip_file
            .write_all(new_file_content.as_bytes())
            .map_err(|e| format!("Couldn't write to new tip file: {e}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Function;

    use super::*;

    #[test]
    fn tip_init_works() {
        let args = vec![
            String::from("current_project"),
            String::from("tip"),
            String::from("create"),
            String::from("@project"),
        ];
        let tip_cli = TipCli::init(&args).expect("init failed");
        let command = Command::new("create", "@project", vec![]).expect("command new failed");
        let tip_dir = "/home/prashant/.tip".to_string();
        let project_name = args[1].clone();
        let tip_want = TipCli {
            tip_dir,
            project_name,
            command,
        };
        assert_eq!(tip_cli, tip_want);
    }
    #[test]
    fn perform_function_create_works() {
        let args = vec![
            String::from("current_project"),
            String::from("tip"),
            String::from("create"),
            String::from("@project"),
        ];
        // Assuming these will not error out.
        fs::create_dir("temp").expect("temp dir not created");
        let mut tip_cli = TipCli::init(&args).expect("init failed");
        tip_cli.tip_dir = "./temp".to_string();
        let cursor_want = Cursor {
            list: None,
            ok: Ok(()),
        };
        let cursor_got = tip_cli.perform();
        assert_eq!(cursor_got, cursor_want);
        fs::remove_dir_all(tip_cli.tip_dir)
            .map_err(|e| format!("couldn't remove directory: {e}"))
            .unwrap();
    }
    fn setup_for_new_tip(dir_name: &str, project_name: &str) {
        match fs::create_dir(dir_name) {
            Ok(res) => res,
            Err(error) => match error.kind() {
                std::io::ErrorKind::AlreadyExists => {}
                _ => {
                    panic!("Error: {error:?}")
                }
            },
        }
        let cmd_setup = Command::new("create", "@project", vec![]).expect("setup: failed cmd new");
        let tip_cli_setup = TipCli {
            tip_dir: dir_name.to_string(),
            project_name: project_name.to_string(),
            command: cmd_setup,
        };
        tip_cli_setup.perform();
    }
    #[test]
    fn perform_function_tip_works() {
        let setup_dir_name = "temp2";
        let project_dir_name = "imgv";
        setup_for_new_tip(setup_dir_name, project_dir_name);
        let cmd = Command::new(
            "create",
            "@tip",
            vec!["4".to_string(), "Add file explorer".to_string()],
        )
        .expect("Error: creating new command");
        let tip_cli = TipCli {
            tip_dir: setup_dir_name.to_string(),
            project_name: project_dir_name.to_string(),
            command: cmd,
        };
        let cursor_want = Cursor {
            list: None,
            ok: Ok(()),
        };
        let cursor_got = tip_cli.perform();
        assert_eq!(cursor_got, cursor_want);
        fs::remove_dir_all(setup_dir_name).expect("couldn't remove directory");
    }
    #[test]
    fn tip_create_validation_works() {
        let setup_dir_name = "temp0";
        let project_dir_name = "imgv";
        setup_for_new_tip(setup_dir_name, project_dir_name);
        let cmd = Command::new(
            "create",
            "@tip",
            vec!["5".to_string(), "Add file explorer".to_string()],
        )
        .expect("Error: creating new command");
        let tip_cli = TipCli {
            tip_dir: setup_dir_name.to_string(),
            project_name: project_dir_name.to_string(),
            command: cmd,
        };
        let cursor_got = tip_cli.perform();
        let cursor_want = Cursor {
            list: None,
            ok: Ok(()),
        };
        assert_eq!(cursor_got, cursor_want);
        fs::remove_dir_all(setup_dir_name).expect("couldn't remove directory");
    }
    #[test]
    fn tip_create_validation_works_with_1() {
        let setup_dir_name = "temp3";
        let project_dir_name = "imgv";
        setup_for_new_tip(setup_dir_name, project_dir_name);
        let cmd = Command::new(
            "create",
            "@tip",
            vec!["1".to_string(), "Add file explorer".to_string()],
        )
        .expect("Error: creating new command");
        let tip_cli = TipCli {
            tip_dir: setup_dir_name.to_string(),
            project_name: project_dir_name.to_string(),
            command: cmd,
        };
        let cursor_got = tip_cli.perform();
        let cursor_want = Cursor {
            list: None,
            ok: Ok(()),
        };
        assert_eq!(cursor_got, cursor_want);
        fs::remove_dir_all(setup_dir_name).expect("couldn't remove directory");
    }
    #[test]
    fn tip_create_validation_works_with_10() {
        let setup_dir_name = "temp4";
        let project_dir_name = "imgv";
        setup_for_new_tip(setup_dir_name, project_dir_name);
        let cmd = Command::new(
            "create",
            "@tip",
            vec!["10".to_string(), "Add file explorer".to_string()],
        )
        .expect("Error: creating new command");
        let tip_cli = TipCli {
            tip_dir: setup_dir_name.to_string(),
            project_name: project_dir_name.to_string(),
            command: cmd,
        };
        let cursor_got = tip_cli.perform();
        let cursor_want = Cursor {
            list: None,
            ok: Ok(()),
        };
        assert_eq!(cursor_got, cursor_want);
        fs::remove_dir_all(setup_dir_name).expect("couldn't remove directory");
    }
    #[test]
    fn tip_create_validation_not_works() {
        let cmd_got = Command::new(
            "create",
            "@tip",
            vec!["100".to_string(), "Add file explorer".to_string()],
        );
        let function_want = Function {
            func_args: vec![],
            func_type: FunctionTypes::Tip,
        };
        let cmd_want = Command {
            command_type: CommandTypes::Create,
            command_func: function_want,
        };
        assert_ne!(cmd_got, Ok(cmd_want));
    }
}
