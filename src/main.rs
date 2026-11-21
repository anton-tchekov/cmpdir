use std::fs;
use std::env;
use std::path::PathBuf;
use std::collections::BTreeSet;

fn enumerate(set: &mut BTreeSet<String>, dir: &PathBuf, prefix: &PathBuf)
{
	match fs::read_dir(prefix.join(dir))
	{
		Ok(paths) =>
		{
			for path in paths
			{
				match path
				{
					Ok(path) =>
					{
						let name = path.path();
						let current = name.strip_prefix(prefix).unwrap().to_path_buf();
						if name.is_dir()
						{
							enumerate(set, &current, prefix);
						}

						set.insert(current.display().to_string());
					},
					Err(_) => {}
				}
			}
		},
		Err(_) =>
		{
			println!("Error opening directory \"{}\"", dir.display())
		}
	}
}

fn handle_dir(dir: &String) -> BTreeSet<String>
{
	let pb = PathBuf::new();
	let prefix = PathBuf::from(dir);
	let mut map = BTreeSet::new();
	enumerate(&mut map, &pb, &prefix);
	map
}

fn print_diff(map1: &BTreeSet<String>, map2: &BTreeSet<String>)
{
	for key in map1
	{
		if !map2.contains(key)
		{
			println!("{}", key);
		}
	}
}

fn main()
{
	let args: Vec<String> = env::args().collect();
	if args.len() != 3
	{
		println!("Usage: ./cmpdir dir1 dir2");
		return;
	}

	let dir1 = &args[1];
	let dir2 = &args[2];

	let map1 = handle_dir(dir1);
	let map2 = handle_dir(dir2);

	println!("\n>>> Missing in {}\n", dir1);
	print_diff(&map2, &map1);

	println!("\n>>> Missing in {}\n", dir2);
	print_diff(&map1, &map2);
	print!("\n");
}