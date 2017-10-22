// cargo-deps: maplit = "*", curl="*"
#[macro_use]
extern crate maplit;
extern crate curl;

use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::fs;
use curl::easy::Easy;

fn main()
{
    let downloads = hashmap!{
        // JQuery
        "https://code.jquery.com/jquery-3.2.1.min.js"
            => "web/assets/jquery/js/jquery.min.js",

        // Popper
        "https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.11.0/umd/popper.min.js"
            => "web/assets/popper/js/popper.min.js",

        // Bootstrap 4
        "https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/css/bootstrap.min.css"
            => "web/assets/bootstrap/css/bootstrap.min.css",
        "https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/css/bootstrap.min.css.map"
            => "web/assets/bootstrap/css/bootstrap.min.css.map",
        "https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/js/bootstrap.min.js"
            => "web/assets/bootstrap/js/bootstrap.min.js",

        // Font Awesome
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/css/font-awesome.min.css"
            => "web/assets/font-awesome/css/font-awesome.min.css",
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/fonts/FontAwesome.otf"
            => "web/assets/font-awesome/fonts/FontAwesome.otf",
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/fonts/fontawesome-webfont.eot"
            => "web/assets/font-awesome/fonts/fontawesome-webfont.eot",
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/fonts/fontawesome-webfont.svg"
            => "web/assets/font-awesome/fonts/fontawesome-webfont.svg",
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/fonts/fontawesome-webfont.ttf"
            => "web/assets/font-awesome/fonts/fontawesome-webfont.ttf",
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/fonts/fontawesome-webfont.woff"
            => "web/assets/font-awesome/fonts/fontawesome-webfont.woff",
        "https://raw.githubusercontent.com/FortAwesome/Font-Awesome/master/fonts/fontawesome-webfont.woff2"
            => "web/assets/font-awesome/fonts/fontawesome-webfont.woff2",
    };

    for (url, path) in &downloads {
        println!("{} => {}", url, path);
        download(url, path).unwrap();
    }
}

fn download(url: &'static str, path: &'static str) -> Result<(), Box<Error>>
{
    let path = Path::new(path);
    let parent = path.parent().ok_or("parent not exists")?;

    fs::create_dir_all(parent)?;
    fs::File::create(path)?;

    let mut easy = Easy::new();
    easy.url(url)?;
    easy.write_function(move |data| {
        let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
        file.write_all(data).unwrap();
        Ok(data.len())
    })?;
    easy.perform()?;
    Ok(())
}