use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// The path of the PDF document to be opened.
    #[arg(short = 'd', long = "dir")]
    pub(crate) path: Option<String>,
    /// The alias of a PDF document. When used alone, the alias will open the document bound to it.
    /// If both `--path` and `--alias` are provided, the alias will be set to the document the path leads to.
    #[arg(short, long)]
    pub(crate) alias: Option<String>,
    /// The page to jump to. Only works when either `--path` or `--alias` is provided.
    #[arg(short, long, default_value_t = 1)]
    pub(crate) page: i32,
    /// The bookmark to a specified page an aliased document. It only works when a valid alias is provided.
    /// When both `--page` and `--mark` are provided, the bookmark will be set to the page.
    #[arg(short = 'm', long = "mark", aliases = &["b", "bookmark"])]
    pub(crate) bookmark: Option<String>,
}
