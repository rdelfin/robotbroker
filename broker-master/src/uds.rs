use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::io;
use tempdir::TempDir;

pub struct UdsGenerator {
    temp_dir: TempDir,
}

impl UdsGenerator {
    pub fn new() -> Result<UdsGenerator, io::Error> {
        Ok(UdsGenerator {
            temp_dir: TempDir::new("robotbroker")?,
        })
    }

    pub fn generate_uds(&self) -> String {
        const FILE_LEN: usize = 20;

        let filename: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(FILE_LEN)
            .map(char::from)
            .collect();

        let mut path = self.temp_dir.path().to_owned();
        path.push(&filename);
        path.set_extension("sock");
        path.to_str().unwrap().to_string()
    }
}
