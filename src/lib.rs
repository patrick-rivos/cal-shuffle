/// ABI checker. Given a GCC or LLVM command, randomly select a GCC/LLVM
/// version, linker, assembler from the provided options.
use std::{env, hash::{DefaultHasher, Hash, Hasher}, os::unix::process::CommandExt, path::{Path, PathBuf}, process::Command};

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

pub fn hash_args(args: &Vec<String>) -> u64 {
	let mut hasher = DefaultHasher::new();
	args.hash(&mut hasher);
	let hash = hasher.finish();

	let key = "SEED";
	let seed_delta = match env::var(key) {
	    Ok(val) => {
		    val.parse::<u64>().unwrap()
	    },
	    Err(_) => 0,
	};

	hash.wrapping_add(seed_delta)
}

pub fn get_cc_path() -> PathBuf {

	let key = "CC_RAND";
	match env::var(key) {
	    Ok(val) => {
		    let value: String = val.to_owned().clone();
		    let path = Path::new(&value);
		    assert!(path.exists(), "Given path for {} ({}) must exist!", key, value);
		    path.to_owned()
	    },
	    Err(e) => panic!("Error when reading env var {}: {}", key, e),
	}
}

pub fn get_cxx_path() -> PathBuf {
	let key = "CXX_RAND";
	match env::var(key) {
	    Ok(val) => {
		    let value: String = val.to_owned().clone();
		    let path = Path::new(&value);
		    assert!(path.exists(), "Given path for {} ({}) must exist!", key, value);
		    path.to_owned()
	    },
	    Err(e) => panic!("Error when reading env var {}: {}", key, e),
	}
}

const CC1_PATHS: &[&str] = &["cc1"];
const AS_PATHS: &[&str] = &["as-2-trunk","as-2-42","as-2-41"];
const LD_PATHS: &[&str] = &["ld-2-trunk","ld-2-42","ld-2-41"];

pub fn construct_b_flag(seed: u64) -> String {
	let mut seeded_rng = StdRng::seed_from_u64(seed);

	let compiler = CC1_PATHS
	    .choose(&mut seeded_rng).unwrap();
	    let assembler = AS_PATHS
	    .choose(&mut seeded_rng).unwrap();
	    let linker = LD_PATHS
	    .choose(&mut seeded_rng).unwrap();

	let exe = env::current_exe().unwrap();
	let dir = exe.parent().expect("Executable must be in some directory");

	let tool_folder = dir.join(format!("{}-{}-{}/", compiler, assembler, linker));

	assert!(tool_folder.exists(), "Tool folder {} must exist!", tool_folder.display());
	assert!(tool_folder.join("as").exists(), "as exe {} must exist!", tool_folder.display());
	assert!(tool_folder.join("ld").exists(), "ld exe {} must exist!", tool_folder.display());
	assert!(tool_folder.join("cc1").exists(), "cc1 exe {} must exist!", tool_folder.display());

	format!("-B{}", tool_folder.display())
}

pub fn exec_cmd(executable: PathBuf, args: Vec<String>) {
	Command::new(executable).args(args).exec();
}
