use dioxus::prelude::*;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    println!("cargo:rerun-if-changed=src/components");
    println!("cargo:rerun-if-changed=../dioxus-kit/src");

    // Component docs live in preview, while component metadata lives with the
    // styled component crate so it can be packaged by the CLI.
    for folder in std::fs::read_dir("../dioxus-kit/src").unwrap().flatten() {
        if !folder.file_type().unwrap().is_dir() {
            continue;
        }
        let component_json = folder.path().join("component.json");
        if component_json.exists() {
            write_component_description(&component_json, &out_dir).unwrap();
        }
    }

    // Process all markdown files in each component folder.
    for folder in std::fs::read_dir("src/components").unwrap().flatten() {
        if !folder.file_type().unwrap().is_dir() {
            continue;
        }
        let folder_path = folder.path();
        walk_markdown_dir(&folder_path, &out_dir).unwrap();
    }
}

fn walk_markdown_dir(dir: &std::path::Path, out_dir: &std::path::Path) -> std::io::Result<()> {
    let folder_name = dir.file_name().unwrap();
    let folder_name = folder_name.to_string_lossy();
    let out_folder = out_dir.join(&*folder_name);
    std::fs::create_dir_all(&out_folder).unwrap();
    for file in std::fs::read_dir(dir).unwrap().flatten() {
        if file.file_type().unwrap().is_dir() {
            walk_markdown_dir(&file.path(), &out_folder)?;
            continue;
        }
        if file.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if file.path().extension() == Some(std::ffi::OsStr::new("md")) {
            let markdown = process_markdown_to_html(&file.path());
            let out_file_path = out_folder.join(file.file_name()).with_extension("html");
            std::fs::write(out_file_path, markdown).unwrap();
            continue;
        }
    }
    Ok(())
}

fn write_component_description(
    component_json_path: &std::path::Path,
    out_dir: &std::path::Path,
) -> std::io::Result<()> {
    println!("cargo:rerun-if-changed={}", component_json_path.display());
    let component_name = component_json_path
        .parent()
        .and_then(std::path::Path::file_name)
        .expect("component metadata must be in a component directory");
    let out_folder = out_dir.join(component_name);
    std::fs::create_dir_all(&out_folder)?;
    let out_file_path = out_folder.join("description.txt");
    std::fs::write(
        out_file_path,
        read_component_description(component_json_path),
    )
}

fn process_markdown_to_html(markdown_path: &std::path::Path) -> String {
    println!("cargo:rerun-if-changed={}", markdown_path.display());
    use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
    let markdown_input =
        std::fs::read_to_string(markdown_path).expect("Failed to read markdown file");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_GFM);
    let parser = Parser::new_ext(&markdown_input, options);
    let mut events = Vec::new();
    let mut code_block: Option<(CodeBlockKind<'_>, String)> = None;

    for event in parser {
        match (&mut code_block, event) {
            (None, Event::Start(Tag::CodeBlock(kind))) => {
                code_block = Some((kind, String::new()));
            }
            (Some((_, source)), Event::Text(text)) => {
                source.push_str(&text);
            }
            (Some((kind, source)), Event::End(TagEnd::CodeBlock)) => {
                events.push(Event::Html(CowStr::Boxed(
                    render_code_block_html(kind, source).into_boxed_str(),
                )));
                code_block = None;
            }
            (None, event) => events.push(event),
            (Some((_, source)), Event::Code(text)) => {
                source.push_str(&text);
            }
            (Some((_, source)), Event::Html(html) | Event::InlineHtml(html)) => {
                source.push_str(&html);
            }
            (Some(_), _) => {}
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());
    html_output
}

fn read_component_description(component_json_path: &std::path::Path) -> String {
    println!("cargo:rerun-if-changed={}", component_json_path.display());
    let input =
        std::fs::read_to_string(component_json_path).expect("Failed to read component metadata");
    let json: serde_json::Value =
        serde_json::from_str(&input).expect("Failed to parse component metadata");
    json.get("description")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("A Dioxus component.")
        .trim()
        .to_string()
}

fn render_code_block_html(kind: &pulldown_cmark::CodeBlockKind<'_>, source: &str) -> String {
    let language = match kind {
        pulldown_cmark::CodeBlockKind::Fenced(info) => {
            let slug = info.split_whitespace().next().unwrap_or_default();
            language_from_slug(slug)
        }
        pulldown_cmark::CodeBlockKind::Indented => None,
    };

    let Some(language) = language else {
        return render_plain_code_block(source);
    };

    let source = source.trim_end_matches('\n');
    let highlighted: dioxus_code::advanced::HighlightedSource =
        dioxus_code::SourceCode::new(language, source).into();

    dioxus_ssr::render_element(rsx! {
        div {
            class: "dx-preview-code-theme",
            tabindex: "0",
            dioxus_code::Code {
                src: highlighted,
                theme: dioxus_code::CodeTheme::system(
                    dioxus_code::Theme::GITHUB_LIGHT,
                    dioxus_code::Theme::GITHUB_DARK,
                ),
            }
        }
    })
}

fn render_plain_code_block(source: &str) -> String {
    let source = source.trim_end_matches('\n');

    dioxus_ssr::render_element(rsx! {
        pre {
            code { "{source}" }
        }
    })
}

fn language_from_slug(slug: &str) -> Option<dioxus_code::Language> {
    match slug {
        "" => Some(dioxus_code::Language::Rust),
        "rs" => Some(dioxus_code::Language::Rust),
        "rust" => Some(dioxus_code::Language::Rust),
        "css" => Some(dioxus_code::Language::Css),
        slug => dioxus_code::Language::from_slug(slug),
    }
}
