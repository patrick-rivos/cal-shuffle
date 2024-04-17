use std::env;
use log::trace;

use cal_shuffle::{construct_b_flag, get_cxx_path, hash_args, exec_cmd};

fn main() {
	let args: Vec<String> = env::args().collect();
	trace!("Args: {:?}", &args);

	let hash = hash_args(&args);
	trace!("Hash: {:?}", hash);

	let b_flag = construct_b_flag(hash);

	trace!("{}", b_flag);

	let cxx_path = get_cxx_path();

	trace!("Command: {} {} {}", cxx_path.display(), b_flag, args[1..].join(" "));

	let mut cmd_args = vec![b_flag];
	cmd_args.append(&mut args[1..].to_vec());
	exec_cmd(cxx_path, cmd_args);
    }
