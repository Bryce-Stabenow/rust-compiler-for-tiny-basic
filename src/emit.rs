use std::{fs::File, io, io::Write};

pub struct Emitter {
    file_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(file_path: String) -> Self {
        Emitter {
            file_path,
            header: String::from(""),
            code: String::from(""),
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code.push_str(code);
        self.code.push('\n');
    }

    pub fn header_line(&mut self, code: &str) {
        self.header.push_str(code);
        self.code.push('\n');
    }

    pub fn write_file(&self) -> io::Result<()> {
        let mut file = File::create(&self.file_path)?;

        file.write(self.header.as_bytes())?;
        file.write(self.code.as_bytes())?;
        Ok(())
    }
}
