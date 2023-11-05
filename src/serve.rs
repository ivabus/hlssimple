use crate::{Commands, INDEX};
use clap::Parser;
use rocket::response::content::RawHtml;
use std::fs::read_dir;
use std::path::PathBuf;

#[get("/play/<page..>")]
pub fn play(page: PathBuf) -> RawHtml<String> {
	RawHtml(format!(
		"<video controls width=\"100%\">
    <source src=\"/{}\" type=\"application/x-mpegURL\">
</video>",
		page.to_str().unwrap()
	))
}

struct StreamURL {
	alias: String,
	url: String,
}

#[get("/")]
pub fn index() -> RawHtml<String> {
	let args = Commands::parse();
	match args {
		Commands::Serve {
			streams_dir,
		} => {
			let dir = read_dir(streams_dir).unwrap();
			let mut res = INDEX.to_string();
			let mut m3u8: Vec<StreamURL> = vec![];
			for i in dir {
				let path = i.unwrap().path();
				if path.is_file() && path.file_name().unwrap().to_str().unwrap() != ".DS_Store" {
					if path.extension().unwrap().to_str().unwrap() == "m3u8" {
						let filename = path.file_name().unwrap().to_str().unwrap();
						m3u8.push(StreamURL {
							url: filename.to_string(),
							alias: filename.to_string(),
						});
					}
				}
				if path.is_dir() {
					let mut dir: Vec<String> = vec![];
					for i in read_dir(&path).unwrap() {
						dir.push(i.unwrap().file_name().to_str().unwrap().to_string())
					}
					for i in &dir {
						if i.contains(".m3u8") {
							m3u8.push(StreamURL {
								alias: path.file_name().unwrap().to_str().unwrap().to_string(),
								url: format!(
									"{}/{}",
									path.file_name().unwrap().to_str().unwrap(),
									i
								),
							});
						}
					}
				}
			}
			m3u8.sort_by_key(|x| x.alias.clone());
			if m3u8.len() == 0 {
				res += "No streams available";
			}
			for i in m3u8 {
				res += &*format!(
					"<a href=\"{}\">{}</a> <a href=\"/play/{}\">Play</a> (copy link into your player)<br>",
					&i.url, &i.alias, &i.url
				);
			}
			RawHtml(res)
		}
		_ => unreachable!(),
	}
}
