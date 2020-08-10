use super::*;

impl<'ast> Parser<'ast> {
    pub fn import(&mut self) -> Result<ImportNode> {
        Err(Error::NotImplementedError)
    }

    pub fn export(&mut self) -> Result<ExportNode> {
        Err(Error::NotImplementedError)
    }
}
