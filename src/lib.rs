//! A convention for libraries to bundle resource files alongside binaries.
//!
//! <p style="font-family: 'Fira Sans',sans-serif;padding:0.3em 0"><strong>
//! <a href="https://crates.io/crates/rez">📦&nbsp;&nbsp;Crates.io</a>&nbsp;&nbsp;│&nbsp;&nbsp;<a href="https://github.com/alecmocatta/rez">📑&nbsp;&nbsp;GitHub</a>&nbsp;&nbsp;│&nbsp;&nbsp;<a href="https://constellation.zulipchat.com/#narrow/stream/213236-subprojects">💬&nbsp;&nbsp;Chat</a>
//! </strong></p>
//!

use std::{
	env, fs::{self, File, OpenOptions}, io, path::{Path, PathBuf}
};

pub fn docker_images(images: &[&str]) -> Result<(), io::Error> {
	fs::write(dir().join("docker"), images.join("\n"))
}

pub fn file(mut src: &File, dest: &Path) -> Result<(), io::Error> {
	let dest = OpenOptions::new().write(true).create(true).truncate(true).open(dir().join(dest))?;
	let _ = io::copy(&mut src, &mut &dest)?;
	Ok(())
}

pub fn dir() -> PathBuf {
	let mut dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let out = dir.file_name().unwrap();
	assert_eq!(out, "out");
	let _ = dir.pop();
	let id = dir.file_name().unwrap().to_owned();
	assert!(id.to_str().unwrap().starts_with(&format!("{}-", env::var("CARGO_PKG_NAME").unwrap())));
	let _ = dir.pop();
	let build = dir.file_name().unwrap();
	assert_eq!(build, "build");
	let _ = dir.pop();
	dir.push("resources");
	fs::create_dir(&dir).or_else(|e| (e.kind() == io::ErrorKind::AlreadyExists).then(|| ()).ok_or(e)).unwrap();
	dir.push(id);
	fs::create_dir(&dir).or_else(|e| (e.kind() == io::ErrorKind::AlreadyExists).then(|| ()).ok_or(e)).unwrap();
	env_set("CARGO_RESOURCE_DIR", dir.to_str().unwrap());
	// println!("cargo:rerun-if-changed={}", dir.display());
	dir
}

fn env_set(key: &str, value: &str) {
	println!("cargo:rustc-env={}={}", key, value);
}
