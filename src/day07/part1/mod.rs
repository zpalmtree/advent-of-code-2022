use tree::Tree;

#[derive(Debug)]
enum BashCommand {
    // ls
    List,

    // cd <dir>
    ChangeDirectory(String),

    // cd ..
    MoveUpDirectory,
}

#[derive(Debug)]
struct ElfFile {
    name: String,

    size: usize,
}

#[derive(Debug)]
struct FileOrDir {
    name: String,

    size: Option<usize>,
}

#[derive(Debug)]
struct DirectoryNode {
    name: String,

    files: Vec<ElfFile>,
}

fn parse_bash_command(command: &String) -> BashCommand {
    if command == "ls" {
        return BashCommand::List;
    }

    let mut split = command.split(" ");

    let _ = split.next().unwrap();
    let directory = split.next().unwrap();

    if directory == ".." {
        return BashCommand::MoveUpDirectory;
    }

    return BashCommand::ChangeDirectory(directory.to_string());
}

fn parse_ls_output(output_line: &String) -> FileOrDir {
    if output_line.starts_with("dir") {
        return FileOrDir {
            name: output_line[4..].to_string(),
            size: None,
        }
    }

    let mut split = output_line.split(" ");

    let size = split.next().unwrap();
    let name = split.next().unwrap();

    return FileOrDir {
        name: name.to_string(),
        size: Some(size.parse().unwrap()),
    }
}

fn parse_bash_logs(logs: &[String]) -> Tree<DirectoryNode> {
    let mut file_system = Tree::new();

    let root = file_system.add(DirectoryNode{
        name: "/".to_string(),
        files: Vec::new(),
    });

    let mut current_directory = root;

    for log in logs.iter() {
        let message: Vec<char> = log.chars().collect();

        if message[0] == '$' {
            let command = parse_bash_command(&log[2..].to_string());

            match &command {
                // following data are files or directories
                BashCommand::List => (),

                BashCommand::ChangeDirectory(directory) => {
                    let directory = file_system.add_child(current_directory, DirectoryNode{
                        name: directory.to_string(),
                        files: Vec::new(),
                    });

                    current_directory = directory;
                },

                BashCommand::MoveUpDirectory => {
                    let directory_node = file_system.get(current_directory).unwrap();

                    if directory_node.parent.is_some() {
                        current_directory = directory_node.parent.unwrap();
                    }
                },
            }
        } else {
            let output = parse_ls_output(log);

            match output.size {
                Some(file_size) => {
                    let directory_node = file_system.get_mut(current_directory).unwrap();

                    directory_node.data.files.push(ElfFile{
                        name: output.name,
                        size: file_size,
                    });
                }
                None => (),
            }
        }
    }

    return file_system;
}

#[allow(dead_code)]
fn print_file_system(file_system: &Tree<DirectoryNode>, current_directory: usize) {
    let node = file_system.get(current_directory).unwrap();

    println!("/{}", node.data.name);

    for file in &node.data.files {
        println!("\t{}", file.name);
    }

    for child in &node.children {
        print_file_system(file_system, *child);
    }
}

fn calculate_directory_sizes(file_system: &Tree<DirectoryNode>, current_directory: usize) -> (Vec<(String, usize)>, usize) {
    let node = file_system.get(current_directory).unwrap();

    let mut directories = Vec::new();
    let mut size = 0;

    for file in &node.data.files {
        size += file.size;
    }

    for child in &node.children {
        let (mut child_directories, directory_size) = calculate_directory_sizes(file_system, *child);

        size += directory_size;

        directories.append(&mut child_directories);
    }

    directories.push((node.data.name.clone(), size));

    return (directories, size);
}

pub fn solution(data: Vec<String>) -> String {
    println!("Successfully loaded {} lines of data.", data.len());

    let file_system = parse_bash_logs(&data[1..]);

    let (directory_sizes, _) = calculate_directory_sizes(&file_system, 0);

    let mut sum = 0;

    for (_, directory_size) in directory_sizes {
        if directory_size < 100_000 {
            sum += directory_size;
        }
    }

    return sum.to_string();
}
