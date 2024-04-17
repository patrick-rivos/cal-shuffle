use std::env;
use log::trace;

use cal_shuffle::{construct_b_flag, get_cc_path, hash_args, exec_cmd};

fn main() {
	let args: Vec<String> = env::args().collect();
	trace!("Args: {:?}", &args);

	let hash = hash_args(&args);
	trace!("Hash: {:?}", hash);

	let b_flag = construct_b_flag(hash);

	trace!("{}", b_flag);

	let cc_path = get_cc_path();

	trace!("Command: {} {} {}", cc_path.display(), b_flag, args[1..].join(" "));

	let mut cmd_args = vec![b_flag];
	cmd_args.append(&mut args[1..].to_vec());
	exec_cmd(cc_path, cmd_args);
}
