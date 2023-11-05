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

mod create;
mod edit;
mod serve;

#[macro_use]
extern crate rocket;

use clap::{Parser, Subcommand};
use rocket::fs::FileServer;
use std::path::PathBuf;

const INDEX: &'static str = include_str!("../index.html");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Commands {
	/// Run server instance
	#[command(visible_alias("s"))]
	Serve {
		streams_dir: PathBuf,
	},
	/// Edit ended HLS streams
	#[command(subcommand, visible_alias("e"))]
	Edit(EditCommands),

	/// Split file into stream
	#[command(visible_alias("c"))]
	Create {
		freq: f64,
		file: PathBuf,
	},
}

#[derive(Subcommand, Debug)]
enum EditCommands {
	/// Join hls stream into one file
	#[command(visible_alias("c"))]
	Concat {
		m3u8: PathBuf,
	},
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	let args = Commands::parse();
	println!("{:#?}", args);
	match args {
		Commands::Serve {
			streams_dir,
		} => {
			let _rocket = rocket::build()
				.mount("/", routes![serve::play, serve::index])
				.mount("/", FileServer::from(streams_dir))
				.launch()
				.await?;
		}
		Commands::Edit(command) => match command {
			EditCommands::Concat {
				m3u8,
			} => {
				edit::concat(m3u8);
			}
		},
		Commands::Create {
			file: _,
			freq: _,
		} => {
			todo!()
		}
	};

	Ok(())
}
