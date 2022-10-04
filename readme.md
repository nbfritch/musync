# musync

musync is a small rust program that makes two directories contain the same files using a smaller number of copies and deletes than overwriting each file.

## Usage
`musync FROM_DIR TO_DIR` or `cargo run -- FROM_DIR TO_DIR`
Where `FROM_DIR` has the files you want to sync and `TO_DIR` is the directory you want to sync the files to.

## Why
I wrote this tool after trying to sync my music collection (stored on an smb share) to my phone (over mtp) and an sd card (mounted locally).
`rsync` seemed like a good fit, but unfortunately, I am big dum and could not figure out the right subset of its many options would give me what I wanted.

## Notes
I wrote this tool as a learning exercise. Don't point it at a directory that contains any files you value.

musync only compares name, path and length of files to determine equivalence. This is to prevent reading the entire file into memory to compute hashes or checksums.
Therefore if you update a file in the source directory and it retains the same name and length it will *not be moved*.

## Applications
I created this tool after I exceeded the write endurance of an SD card from repeatedly copying my music libary to it after getting new music.
