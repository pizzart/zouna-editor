use walkdir::DirEntry;
use walkdir::WalkDir;

pub fn visit_dirs(dir: &str) -> Vec<DirEntry> {
    let mut entries: Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new(dir)
        // .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        entries.push(entry)
    }
    return entries;
}
