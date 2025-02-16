//! Process each post (and folder) found within the site

use std::{error::Error, fs, io::Write, path::Path};

use comrak::{
    format_html_with_plugins,
    nodes::{NodeHtmlBlock, NodeMath, NodeValue},
    parse_document,
    plugins::syntect::SyntectAdapter,
    Arena, ExtensionOptions, Options, ParseOptions, Plugins, RenderOptions,
};

use crate::{
    file_system::{Node, SiteData},
    Config,
};

/// Create the output for the root site data struct
pub fn process_site_data(site: SiteData, config: &Config) -> Result<(), Box<dyn Error>> {
    node(&site.included_data, config)?;

    Ok(())
}

fn node(node: &Node, config: &Config) -> Result<(), Box<dyn Error>> {
    match node {
        Node::Folder {
            name,
            path,
            children,
        } => folder(name, path, children, config),
        Node::File { name, path, data } => file(name, path, data, config),
    }
}

fn folder(
    name: &str,
    path: &Path,
    children: &[Node],
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let path = path.strip_prefix(&config.input_dir)?;
    let out_path = config.output_dir.join(path);

    fs::create_dir_all(out_path)?;

    for child in children {
        node(child, config)?;
    }

    Ok(())
}

fn file(name: &str, path: &Path, data: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let path = path.strip_prefix(&config.input_dir)?;
    let mut out_path = config.output_dir.join(path);
    out_path.set_extension("html");

    // convert the post into markdown
    let options = markdown_options();

    let syntect = SyntectAdapter::new(None);
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&syntect);

    let arena = Arena::new();
    let root = parse_document(&arena, data, &options);

    for node in root.descendants() {
        let mut node = node.data.borrow_mut();
        let (display_math, text) = if let NodeValue::Math(NodeMath {
            display_math,
            ref mut literal,
            ..
        }) = node.value
        {
            (display_math, std::mem::take(literal))
        } else {
            continue;
        };

        let math = text;

        if display_math {
            node.value = NodeValue::HtmlBlock(NodeHtmlBlock {
                block_type: 0,
                literal: math,
            });
        } else {
            node.value = NodeValue::HtmlInline(math);
        }
    }

    let mut output = vec![];
    format_html_with_plugins(root, &options, &mut output, &plugins)?;

    let mut file = fs::File::create(out_path)?;
    file.write_all(&output)?;

    Ok(())
}

fn markdown_options() -> Options<'static> {
    Options {
        extension: ExtensionOptions {
            strikethrough: true,
            tagfilter: false,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: Some("header-".to_string()),
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: Some("---".to_string()),
            multiline_block_quotes: true,
            alerts: true,
            math_dollars: true,
            math_code: true,
            shortcodes: true,
            wikilinks_title_after_pipe: false,
            wikilinks_title_before_pipe: true,
            underline: true,
            subscript: true,
            spoiler: true,
            greentext: true,
            image_url_rewriter: None,
            link_url_rewriter: None,
        },
        parse: ParseOptions {
            smart: true,
            default_info_string: None,
            relaxed_tasklist_matching: true,
            relaxed_autolinks: false,
            broken_link_callback: None,
        },
        render: RenderOptions {
            hardbreaks: false,
            github_pre_lang: true,
            full_info_string: true,
            width: 0,
            unsafe_: true,
            escape: false,
            list_style: comrak::ListStyleType::Dash,
            sourcepos: false,
            experimental_inline_sourcepos: false,
            escaped_char_spans: true,
            ignore_setext: false,
            ignore_empty_links: true,
            gfm_quirks: true,
            prefer_fenced: true,
            figure_with_caption: true,
            tasklist_classes: true,
            ol_width: 0,
        },
    }
}
