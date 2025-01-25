use crate::{cli::Cli, entry::Entry};

pub fn limine(entries: Vec<Entry>, config: &Cli) -> String {
    let timeout = config.timeout;
    let entries = entries
        .into_iter()
        .map(|e| format_entry(e, config))
        .collect::<Vec<_>>();
    let entries = entries.join("");

    format!(
        "timeout: {timeout}

{entries}"
    )
}

fn format_entry(entry: Entry, config: &Cli) -> String {
    let Entry {
        title,
        initrd,
        kernel,
        ..
    } = entry;

    let initrd = initrd.display();
    let kernel = kernel.display();
    let cmdline = &config.cmdline;

    format!(
        "/{title}
    protocol: linux
    kernel_path: boot():{kernel}
    kernel_cmdline: {cmdline}
    module_path: boot():{initrd}
"
    )
}
