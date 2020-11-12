use derive_new::new;
use std::{fmt, io};

#[derive(Debug, Default, new)]
pub struct Buf {
    #[new(default)]
    vec: Vec<u8>,
}

impl Buf {
    pub fn into_string(self) -> String {
        String::from_utf8_lossy(&self.vec).to_string()
    }

    pub fn collect_string(
        callback: impl FnOnce(&mut dyn io::Write) -> fmt::Result,
    ) -> Result<String, fmt::Error> {
        let mut buf = Buf::default();
        callback(&mut buf)?;

        Ok(buf.into_string())
    }
}

impl io::Write for Buf {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.vec.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl fmt::Write for Buf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.vec.extend(s.as_bytes());
        Ok(())
    }
}
