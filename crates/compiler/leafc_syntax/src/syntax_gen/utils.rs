use std::{
    fmt,
    fs,
    mem,
    path::{
        Path,
        PathBuf,
    },
};

use xshell::{
    cmd,
    Shell,
};

/// The name of the module that contains the generated code.
pub const GENERATOR: &str = "syntax_gen";

#[derive(Debug)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
}

#[derive(Clone)]
pub struct CommentBlock {
    pub id:       String,
    pub line:     usize,
    pub contents: Vec<String>,
    is_doc:       bool,
}

pub fn list_rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut res = list_files(dir);
    res.retain(|it| {
        it.file_name().unwrap_or_default().to_str().unwrap_or_default().ends_with(".rs")
    });
    res
}

pub fn list_files(dir: &Path) -> Vec<PathBuf> {
    let mut res = Vec::new();
    let mut work = vec![dir.to_path_buf()];
    while let Some(dir) = work.pop() {
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            let path = entry.path();
            let is_hidden =
                path.file_name().unwrap_or_default().to_str().unwrap_or_default().starts_with('.');
            if !is_hidden {
                if file_type.is_dir() {
                    work.push(path);
                } else if file_type.is_file() {
                    res.push(path);
                }
            }
        }
    }
    res
}

impl CommentBlock {
    pub fn extract(tag: &str, text: &str) -> Vec<CommentBlock> {
        assert!(tag.starts_with(char::is_uppercase));

        let tag = format!("{tag}:");
        // Would be nice if we had `.retain_mut` here!
        CommentBlock::extract_untagged(text)
            .into_iter()
            .filter_map(|mut block| {
                let first = block.contents.remove(0);
                first.strip_prefix(&tag).map(|id| {
                    if block.is_doc {
                        panic!("Use plain (non-doc) comments with tags like {tag}:\n    {first}");
                    }

                    block.id = id.trim().to_string();
                    block
                })
            })
            .collect()
    }

    pub fn extract_untagged(text: &str) -> Vec<CommentBlock> {
        let mut res = Vec::new();

        let lines = text.lines().map(str::trim_start);

        let dummy_block = CommentBlock {
            id:       String::new(),
            line:     0,
            contents: Vec::new(),
            is_doc:   false,
        };
        let mut block = dummy_block.clone();
        for (line_num, line) in lines.enumerate() {
            match line.strip_prefix("//") {
                Some(mut contents) => {
                    if let Some('/' | '!') = contents.chars().next() {
                        contents = &contents[1..];
                        block.is_doc = true;
                    }
                    if let Some(' ') = contents.chars().next() {
                        contents = &contents[1..];
                    }
                    block.contents.push(contents.to_string());
                }
                None => {
                    if !block.contents.is_empty() {
                        let block = mem::replace(&mut block, dummy_block.clone());
                        res.push(block);
                    }
                    block.line = line_num + 2;
                }
            }
        }
        if !block.contents.is_empty() {
            res.push(block);
        }
        res
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.file.strip_prefix(project_root()).unwrap().display().to_string();
        let path = path.replace('\\', "/");
        let name = self.file.file_name().unwrap();
        write!(
            f,
            "https://github.com/pulanski/leafc/blob/master/{}#L{}[{}]",
            path,
            self.line,
            name.to_str().unwrap()
        )
    }
}

fn ensure_rustfmt(sh: &Shell) {
    let version = cmd!(sh, "rustup run stable rustfmt --version").read().unwrap_or_default();
    if !version.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. Please run `rustup component add \
             rustfmt --toolchain stable` to install it.",
        );
    }
}

pub fn reformat(text: String) -> String {
    let sh = Shell::new().unwrap();
    ensure_rustfmt(&sh);
    let rustfmt_toml = project_root().join("rustfmt.toml");
    let mut stdout = cmd!(
        sh,
        "rustup run stable rustfmt --config-path {rustfmt_toml} --config fn_single_line=true"
    )
    .stdin(text)
    .read()
    .unwrap();
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}

/// Mutates `text` in place, adding a preamble to the top of the file.
///
/// The preamble is a comment block containing a warning not to edit the file by
/// hand, and a link to the source of the file.
///
/// The `gen_kind` parameter is used to determine the contents of the preamble.
pub fn add_preamble(mut text: String, gen_kind: GeneratorKind) -> String {
    let sources = gen_kind.relevant_sources();
    let generated_note = format!(
        "//! Generated by `{GENERATOR}`, do not edit by hand.\n//!\n//! To regenerate this file, \
         run `cargo xtask syntaxgen` or `cargo test -p leafc_syntax`.\n//!\n//! Source files \
         relevant to syntax generation include {sources}."
    );

    let preamble = match gen_kind {
        GeneratorKind::SyntaxKind => {
            format!("{}\n{}\n\n", generated_note, gen_kind.mod_doc_comment(),)
        }
        GeneratorKind::SyntaxNode => {
            format!("//! Generated by `{GENERATOR}`, do not edit by hand.\n\n")
        }
        GeneratorKind::SyntaxToken => {
            format!("//! Generated by `{GENERATOR}`, do not edit by hand.\n\n")
        }
    };

    // let preamble = format!("//! Generated by `{GENERATOR}`, do not edit by
    // hand.\n\n");
    text.insert_str(0, &preamble);
    text
}

/// The **kind** of source code to **generate**. This is used to generate
/// different code for different contexts (e.g. `SyntaxKind` vs `ast::Name`).
pub enum GeneratorKind {
    /// The various **syntax kinds** (e.g. `SyntaxKind::NAME`).
    SyntaxKind,
    /// The various **syntax nodes** (e.g. `ast::Name`).
    SyntaxNode,
    /// The various **syntax tokens** (e.g. `ast::NameRef`).
    SyntaxToken,
}

impl GeneratorKind {
    /// Returns the **source files** relevant to the given `GeneratorKind`.
    pub fn relevant_sources(&self) -> &'static str {
        let path_prefix = "crates/leafc_syntax/src/syntax_gen/";

        let relevant_files = ["ast_src.rs", "sourcegen_ast", "utils"];

        match self {
            GeneratorKind::SyntaxKind => "crates/leafc_syntax/src/syntaxgen/syntax_kind.rs",
            GeneratorKind::SyntaxNode => "foobasdfbaz",
            GeneratorKind::SyntaxToken => "fklhjbarbaz",
        }
    }

    /// Returns the **doc comment** for the given `GeneratorKind`.
    /// This is used to generate the top-level doc comment for the generated
    /// file.
    pub fn mod_doc_comment(&self) -> &'static str {
        match self {
            GeneratorKind::SyntaxKind => {
                "//! This module contains the definition of the `SyntaxKind` enum, which is used \
                 to represent the different kinds of syntax nodes in the tree. This includes \
                 tokens (i.e. leaf nodes) as well as non-terminal nodes (i.e. internal nodes)."
            }
            GeneratorKind::SyntaxNode => "/// A syntax node.",
            GeneratorKind::SyntaxToken => "/// A syntax token.",
        }
    }
}

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and then fails the test.
pub fn ensure_file_contents(file: &Path, contents: &str) {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            // File is already up to date.
            return;
        }
    }

    let display_path = file.strip_prefix(project_root()).unwrap_or(file);
    eprintln!(
        "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
        display_path.display()
    );
    if std::env::var("CI").is_ok() {
        eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
    }
    if let Some(parent) = file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(file, contents).unwrap();
    panic!("some file was not up to date and has been updated, simply re-run the tests");
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let res = PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned();
    res
}
