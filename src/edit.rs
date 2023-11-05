use rocket::form::validate::Contains;
use std::ffi::OsString;
use std::fs::DirBuilder;
use std::path::PathBuf;

pub fn concat(m3u8: PathBuf) {
	todo!(); // OBS can remux files into .mp4 so I don't need to implement this at
	     // the moment
	     /*
		 if !m3u8.exists() {
			 eprintln!(
				 "{}: {}: No such file or directory",
				 std::env::args().next().unwrap(),
				 m3u8.to_str().unwrap()
			 );
			 std::process::exit(1);
		 }

		 let _dir = std::fs::read_dir(m3u8.parent().unwrap()).unwrap();
		 let mut tsFiles: Vec<PathBuf> = vec![];
		 for i in _dir {
			 let file = i.unwrap();
			 if &file.file_name().to_str().unwrap() == &".DS_Store" {
				 continue;
			 }
			 if file.file_name().to_str().unwrap().as_bytes().ends_with(b".ts") {
				 tsFiles.push(file.path());
			 }
		 }
		 tsFiles.sort();
		 println!("{:#?}", tsFiles);
		  */
}

pub fn split(file: PathBuf) {
	todo!()
}
