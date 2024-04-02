use std::fs;

pub fn exec() -> Result<(), anyhow::Error> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;

    println!("Initialized git directory");

    Ok(())
}
