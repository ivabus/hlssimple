/*
 * MIT License
 *
 * Copyright (c) 2023 Ivan Bushchik
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#[macro_use]
extern crate rocket;

use clap::Parser;
use rocket::fs::FileServer;
use rocket::response::content::RawHtml;
use std::fs::read_dir;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	streams_dir: String,
}

#[get("/play/<page..>")]
fn play(page: PathBuf) -> RawHtml<String> {
	RawHtml(format!(
		"<video controls>
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
fn index() -> RawHtml<String> {
	let args = Args::parse();
	let dir = read_dir(args.streams_dir).unwrap();
	let mut res = match smurf::io::read_file_to_str(&PathBuf::from("./index.html")) {
		Some(s) => s,
		None => String::new(),
	};
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
						url: format!("{}/{}", path.file_name().unwrap().to_str().unwrap(), i),
					});
				}
			}
		}
	}
	m3u8.sort_by_key(|x| x.alias.clone());
	for i in m3u8 {
		res += &*format!(
			"<a href=\"{}\">{}</a> <a href=\"/play/{}\">Play</a> (copy link into your player)<br>",
			&i.url, &i.alias, &i.url
		);
	}
	RawHtml(res)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	let args = Args::parse();
	let _rocket = rocket::build()
		.mount("/", routes![play, index])
		.mount("/", FileServer::from(args.streams_dir))
		.launch()
		.await?;

	Ok(())
}
