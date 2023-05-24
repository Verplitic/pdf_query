use crate::script::{convert_path, run_script};
use clap::Parser;

mod cli;
mod parser;
mod script;

fn main() {
    #[allow(deprecated)]
    let hm_dir = std::env::home_dir().unwrap().join(".pqconfig");
    let mut profile = match parser::Profile::from(hm_dir) {
        Ok(profile) => profile,
        Err(e) => {
            eprintln!("Err reading user profile:\n{e}");
            return;
        }
    };

    let cli = cli::Cli::parse();
    let mut g_path = String::new();
    let mut g_num = cli.page;
    let (mut alias_exist, mut bookmark_exist) = (false, false);

    if cli.alias.is_some() {
        alias_exist = true;
    }

    if cli.bookmark.is_some() {
        bookmark_exist = true;
    }

    if let Some(path) = &cli.path {
        let c_path = convert_path(path.clone());

        if alias_exist {
            if let Err(e) = profile.modify_alias(&cli.alias.clone().unwrap(), &c_path) {
                eprintln!("Err modifying alias:\n{e}");
                return;
            }
        }

        g_path = c_path;
    } else {
        if !alias_exist {
            eprintln!("Neither document path nor alias is provided.");
            return;
        }

        if let Some(al) = profile.find_alias(&cli.alias.clone().unwrap()) {
            g_path = al.clone();
        } else {
            eprintln!("Undefined alias `{}`", cli.alias.unwrap());
            return;
        }
    }


    if bookmark_exist {
        if !alias_exist {
            eprintln!("Cannot use bookmark because no alias is provided.");
            return;
        } else if g_num != 1 {
            if let Err(e) = profile.modify_bookmark(
                &cli.alias.clone().unwrap(),
                &cli.bookmark.clone().unwrap(),
                g_num,
            ) {
                eprintln!("Err modifying bookmark:\n{e}");
                return;
            }
        }

        if let Some(b) = profile.find_bookmark(&cli.alias.clone().unwrap(), &cli.bookmark.clone().unwrap()) {
            g_num = b.clone();
        }
    }

    if let Err(e) = run_script(g_path, g_num) {
        eprintln!("Err running command:\n{e}");
    }
}
