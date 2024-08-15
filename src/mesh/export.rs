use super::*;

const HEAD: &str = r"";

impl Mesh {
    pub fn export(&self, path: &str) -> Result<(), Error> {
        let mut out = HEAD.to_owned();
        
        fs::write(path, out)?;
        Ok(())
    }
}