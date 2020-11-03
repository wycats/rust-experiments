use glob::glob;
use handlebars::Handlebars;
use serde_json::json;
use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string("tests/ui/fail/template.hbs")?;
    let handlebars = Handlebars::new();

    let template = Template {
        handlebars,
        template,
    };

    let out: PathBuf = env::var("OUT_DIR").unwrap().into();
    let out = out.join("test/ui");
    println!("cargo:rustc-env=CRATE_UI_TESTS={}", out.display());

    let fail = OutDir::new(out.join("fail")).mkdir()?;
    let pass = OutDir::new(out.join("pass")).mkdir()?;

    for entry in glob("tests/ui/fail/*.src").unwrap() {
        let input = InputFile::consume(entry?);
        template.render_fail(&input, &fail)?;
    }

    let pass_file = PassFile::load("tests/ui/pass.src")?;
    pass_file.emit(&pass)?;

    // for entry in glob("tests/ui/fail/*.stderr").unwrap() {
    //     let entry = InputFile::consume(entry?);
    //     fail.copy_shallow(&entry)?;
    // }

    Ok(())
}

struct InputFile {
    path: PathBuf,
}

impl InputFile {
    fn consume(path: impl Into<PathBuf>) -> InputFile {
        let path = path.into();
        println!("cargo:rustc-rerun-if-changed={}", path.display());
        InputFile { path }
    }

    fn file_stem(&self) -> &OsStr {
        self.path
            .file_stem()
            .expect("An input file must have a file stem")
    }

    #[allow(unused)]
    fn file_name(&self) -> &OsStr {
        self.path
            .file_name()
            .expect("An input file must have a file name")
    }

    fn read(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(&self.path)?)
    }
}

struct OutFile {
    path: PathBuf,
}

impl OutFile {
    fn new(path: impl Into<PathBuf>) -> OutFile {
        OutFile { path: path.into() }
    }

    fn with_extension(&self, ext: impl AsRef<OsStr>) -> OutFile {
        OutFile::new(self.path.with_extension(ext))
    }

    fn write(&self, output: impl AsRef<[u8]>) -> Result<(), Box<dyn Error>> {
        fs::write(&self.path, output)?;

        Ok(())
    }
}

struct OutDir {
    path: PathBuf,
}

impl OutDir {
    fn new(path: impl Into<PathBuf>) -> OutDir {
        Self { path: path.into() }
    }

    fn file(&self, path: impl AsRef<Path>) -> OutFile {
        OutFile::new(self.path.join(path.as_ref()))
    }

    fn mkdir(self) -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all(&self.path)?;

        Ok(self)
    }

    #[allow(unused)]
    fn copy_shallow(&self, src: &InputFile) -> Result<(), Box<dyn std::error::Error>> {
        let dest = self.path.join(src.path.file_name().unwrap());
        fs::copy(&src.path, dest)?;

        Ok(())
    }
}

struct PassFile {
    template: Template,
    cases: Vec<String>,
}

impl PassFile {
    fn load(path: impl AsRef<Path>) -> Result<PassFile, Box<dyn Error>> {
        let body = fs::read_to_string(path.as_ref())?;

        let mut parts = body.split("\n---\n");

        let template = parts.next().expect("Expected pass file to have a template");
        let cases = parts.map(|p| p.to_string()).collect();

        Ok(PassFile {
            template: Template {
                handlebars: Handlebars::new(),
                template: template.to_string(),
            },
            cases,
        })
    }

    fn emit(&self, out: &OutDir) -> Result<(), Box<dyn Error>> {
        for (i, case) in self.cases.iter().enumerate() {
            self.template.render(&format!("{}-pass", i), case, out)?
        }

        Ok(())
    }
}

struct Template {
    handlebars: Handlebars<'static>,
    template: String,
}

impl Template {
    fn render_fail(
        &self,
        input: &InputFile,
        out: &OutDir,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("cargo:rustc-rerun-if-changed={}", input.path.display());

        let source = input.read()?;
        let mut source = source.split("\n---\n");
        let code = source
            .next()
            .unwrap_or_else(|| panic!("Expected code in {}", input.path.display()));

        let stderr = source
            .next()
            .unwrap_or_else(|| panic!("Expected stderr after `---` in {}", input.path.display()));

        self.render(input.file_stem(), code, out)?;

        let stderr_out = out.file(input.file_stem()).with_extension("stderr");

        stderr_out.write(stderr)?;

        Ok(())
    }

    fn render(
        &self,
        filename: impl AsRef<OsStr>,
        code: &str,
        out: &OutDir,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rendered = self
            .handlebars
            .render_template(&self.template, &json!({ "code": code }))?;

        let rs_out = out.file(filename.as_ref()).with_extension("rs");

        rs_out.write(rendered)?;

        Ok(())
    }
}
