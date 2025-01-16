use std::{cmp::Reverse, collections::BTreeMap, fs, io, path::PathBuf};

use crate::cli::Cli;

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub initrd: PathBuf,
    pub kernel: PathBuf,
}

#[derive(Debug)]
struct Item {
    kind: ItemKind,
    version: Option<String>,
    path: PathBuf,
}

#[derive(Debug)]
enum ItemKind {
    Initrd,
    Kernel,
}

pub fn load_entries(config: &Cli) -> io::Result<Vec<Entry>> {
    let items = fs::read_dir(&config.scan_path)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();

                path.is_file().then_some(path)
            })
        })
        .flat_map(classify)
        .collect::<Vec<_>>();

    Ok(convert_items(items))
}

fn classify(path: PathBuf) -> Option<Item> {
    const INITRD_PATTERNS: &[&str] = &["initrd", "initramfs"];
    const KERNEL_PATTERNS: &[&str] = &["vmlinuz", "vmlinux"];

    let without_extension = {
        const EXTENSIONS: &[&str] = &["img"];
        let mut path = path.clone();
        let remove_extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| EXTENSIONS.contains(&e))
            .unwrap_or(false);

        if remove_extension {
            path.set_extension("");
        }
        path
    };

    let file_name = without_extension
        .components()
        .last()?
        .as_os_str()
        .to_str()?;

    let version = {
        let splitted = file_name.split('-').collect::<Vec<_>>();
        if splitted.len() == 1 {
            None
        } else {
            splitted.last().copied().filter(|v| !v.contains("linux"))
        }
    };

    let matches_pattern = |patterns: &[&str]| patterns.iter().any(|pat| file_name.contains(pat));

    let kind = if matches_pattern(INITRD_PATTERNS) {
        ItemKind::Initrd
    } else if matches_pattern(KERNEL_PATTERNS) {
        ItemKind::Kernel
    } else {
        return None;
    };

    let path = {
        let root = PathBuf::from("/");
        root.join(path.file_name().unwrap())
    };

    Some(Item {
        kind,
        version: version.map(ToOwned::to_owned),
        path,
    })
}

fn convert_items(items: Vec<Item>) -> Vec<Entry> {
    let mut aggregated_by_version: BTreeMap<Reverse<Option<String>>, Vec<Item>> = BTreeMap::new();

    for item in items {
        aggregated_by_version
            .entry(Reverse(item.version.clone()))
            .or_default()
            .push(item);
    }

    aggregated_by_version
        .into_iter()
        .filter_map(|(version, items)| try_to_entry(items, version.0))
        .collect()
}

fn try_to_entry(mut items: Vec<Item>, version: Option<String>) -> Option<Entry> {
    if items.len() != 2 {
        return None;
    }

    let initrd_idx = items
        .iter()
        .position(|item| matches!(item.kind, ItemKind::Initrd))?;
    let initrd = items.remove(initrd_idx);
    let kernel = items.remove(0);

    if !matches!(kernel.kind, ItemKind::Kernel) {
        return None;
    }

    let title = if let Some(version) = version {
        format!("Linux - {version}")
    } else {
        "Linux".to_owned()
    };

    Some(Entry {
        title,
        initrd: initrd.path,
        kernel: kernel.path,
    })
}
