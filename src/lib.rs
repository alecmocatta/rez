//! A convention for libraries to bundle resource files alongside binaries.
//!
//! <p style="font-family: 'Fira Sans',sans-serif;padding:0.3em 0"><strong>
//! <a href="https://crates.io/crates/rez">📦&nbsp;&nbsp;Crates.io</a>&nbsp;&nbsp;│&nbsp;&nbsp;<a href="https://github.com/alecmocatta/rez">📑&nbsp;&nbsp;GitHub</a>&nbsp;&nbsp;│&nbsp;&nbsp;<a href="https://constellation.zulipchat.com/#narrow/stream/213236-subprojects">💬&nbsp;&nbsp;Chat</a>
//! </strong></p>
//!

use std::{
	env, fs::{self, File, OpenOptions}, io, path::{Path, PathBuf}
};

#[derive(Debug)]
pub struct Build {
	dir: PathBuf,
}

impl Build {
	/// Call this in your build script (build.rs)
	pub fn new() -> Result<Self, io::Error> {
		let mut dir = PathBuf::from(env::var("OUT_DIR").expect("must be called from a build script (build.rs)"));
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
		dir.push(&id);
		fs::create_dir(&dir).or_else(|e| (e.kind() == io::ErrorKind::AlreadyExists).then(|| ()).ok_or(e)).unwrap();
		env_set("CARGO_RESOURCE_DIR", Path::new("resources").join(id).to_str().unwrap());
		// println!("cargo:rerun-if-changed={}", dir.display());
		Ok(Self { dir })
	}

	/// Bundle other binaries with the binary
	pub fn binaries(&self, images: &[&str]) -> Result<(), io::Error> {
		fs::write(self.dir.join("binary"), images.join("\n"))
	}

	/// Bundle docker images with the binary
	pub fn docker_images(&self, images: &[&str]) -> Result<(), io::Error> {
		fs::write(self.dir.join("docker"), images.join("\n"))
	}

	/// Bundle a file with the binary
	pub fn path(&self, src: &Path, dest: &Path) -> Result<(), io::Error> {
		fs::copy(src, dest).map(drop)
	}

	/// Bundle a file with the binary
	pub fn file(&self, mut src: &File, dest: &Path) -> Result<(), io::Error> {
		let dest = OpenOptions::new().write(true).create(true).truncate(true).open(self.dir.join(dest))?;
		io::copy(&mut src, &mut &dest).map(drop)
	}

	// Delete any bundled docker images and files
	pub fn clean(&self) -> Result<(), io::Error> {
		fs::remove_dir_all(&self.dir)?;
		fs::create_dir(&self.dir)
	}

	pub fn dir(&self) -> &Path {
		&self.dir
	}
}

/// Call this in anything that extracts binaries/artifacts from the `target` directory
pub fn dir_from_out_dir(out_dir: &Path) -> PathBuf {
	let mut dir = out_dir.to_owned();
	let out = dir.file_name().unwrap();
	assert_eq!(out, "out");
	let _ = dir.pop();
	let package = dir.file_name().unwrap().to_owned();
	let _ = dir.pop();
	let build = dir.file_name().unwrap();
	assert_eq!(build, "build");
	let _ = dir.pop();
	dir.push("resources");
	dir.push(package);
	dir
}

#[derive(Debug)]
pub struct Resources {
	exe_dir: PathBuf,
	res_dir: PathBuf,
}

impl Resources {
	/// Call this in your library
	pub fn new(resource_dir: &str) -> Result<Self, io::Error> {
		// let resource_dir = option_env!("CARGO_RESOURCE_DIR").expect("must have called Build::new in your build script (build.rs)");
		let res_dir = PathBuf::from(resource_dir);
		let exe_dir = env::current_exe()?.parent().unwrap().to_owned();
		let res_dir = exe_dir.join(res_dir);
		if res_dir.exists() {
			Ok(Self { exe_dir, res_dir })
		} else {
			Err(io::Error::new(io::ErrorKind::NotFound, format!("resource_dir \"{}\" not found", res_dir.display())))
		}
	}

	pub fn binary(&self, binary: &str) -> Result<PathBuf, io::Error> {
		let binary_ = self.exe_dir.join(binary);
		if binary_.exists() {
			Ok(binary_)
		} else {
			return Err(io::Error::new(io::ErrorKind::NotFound, format!("binary \"{}\" not found", binary)));
		}
	}

	pub fn dir(&self) -> &Path {
		&self.res_dir
	}
}

fn env_set(key: &str, value: &str) {
	println!("cargo:rustc-env={}={}", key, value);
}
