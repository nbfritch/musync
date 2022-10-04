use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Debug)]
struct FileListing {
    size: u64,
    name: String,
    relative_path: String,
}

impl FileListing {
    fn equivalent_to(&self, other: &Self) -> bool {
        self.size == other.size
            && self.name == other.name
            && self.relative_path == other.relative_path
    }
}

fn crawl_dir(base_path: &Path, dir: &Path) -> io::Result<Vec<FileListing>> {
    let mut entries: Vec<FileListing> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let full_path = entry.path();
            if full_path.is_dir() {
                let sub_entries = crawl_dir(&base_path, &full_path)?;
                for sentry in sub_entries {
                    entries.push(sentry);
                }
            } else {
                let meta = entry.metadata().unwrap();
                let full_path = entry.path();
                let rel_path = full_path
                    .strip_prefix(base_path)
                    .expect("Could not strip prefix of file");
                let l = FileListing {
                    size: meta.len(),
                    relative_path: String::from(rel_path.to_str().unwrap()),
                    name: entry.file_name().into_string().unwrap(),
                };
                entries.push(l);
            }
        }
    }

    Ok(entries)
}

fn main() -> io::Result<()> {
    let cli_args = env::args();
    assert_eq!(
        cli_args.len(),
        3,
        "Incorrect number of arguments expected `2`"
    );

    let args = cli_args.collect::<Vec<_>>();

    let source_path_arg = args[1].clone();
    let dest_path_arg = args[2].clone();

    let source_path = Path::new(&source_path_arg);
    assert!(source_path.exists(), "First arg directory does not exist");
    assert!(source_path.is_dir(), "First arg is not a directory");
    let src_entries = crawl_dir(&source_path, &source_path)?;

    let dest_path = Path::new(&dest_path_arg);
    assert!(dest_path.exists(), "Second arg directory does not exist");
    assert!(dest_path.is_dir(), "Second arg is not a directory");
    let dst_entries = crawl_dir(&dest_path, &dest_path)?;

    let files_in_src_not_in_dest: Vec<&FileListing> = src_entries
        .iter()
        .filter(|s| !dst_entries.iter().any(|d| d.equivalent_to(s)))
        .collect::<Vec<_>>();
    let files_in_dest_not_in_src: Vec<&FileListing> = dst_entries
        .iter()
        .filter(|d| !src_entries.iter().any(|s| s.equivalent_to(d)))
        .collect::<Vec<_>>();

    println!("Want to delete {} files", files_in_dest_not_in_src.len());
    println!("Want to copy {} files", files_in_src_not_in_dest.len());
    println!("Enter to proceed, Control+C to exit");
    let mut dummy = String::from("?");
    io::stdin()
        .read_line(&mut dummy)
        .expect("Error reading input");

    files_in_dest_not_in_src.iter().for_each(|f| {
        let delete_file = dest_path.join(Path::new(&f.relative_path));
        println!("Deleting file {:?}", delete_file);

        let mut line = String::from("?");
        io::stdin().read_line(&mut line).expect("Bad input string");
        fs::remove_file(delete_file).expect("oopes");
    });

    files_in_src_not_in_dest.iter().for_each(|f| {
        let copy_from_path = source_path.join(Path::new(&f.relative_path));
        let copy_to_path = dest_path.join(Path::new(&f.relative_path));
        println!("cp {:?} {:?}", copy_from_path, copy_to_path);
        let to_parent_dir = copy_to_path.parent().unwrap();
        fs::create_dir_all(to_parent_dir).expect(&format!("Could not create {:?}", to_parent_dir));
        std::process::Command::new("cp")
            .args([
                format!(
                    "{}",
                    copy_from_path
                        .to_str()
                        .expect("Error stringifying from path")
                ),
                format!(
                    "{}",
                    copy_to_path.to_str().expect("Error stringifying to path")
                ),
            ])
            .status()
            .expect("Error copying file");
    });

    Ok(())
}
