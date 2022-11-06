use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Add,
    CatFile,
    Checkout,
    Commit,
    HashObject,
    Init,
    Log,
    LsFiles,
    LsTree,
    Merge,
    Rebase,
    RevParse,
    Rm,
    ShowRef,
    Tag
}

fn parse_sub_cmd(args: &Args) -> String {
    match args.command {
        Command::Add        => String::from("add"),
        Command::CatFile    => String::from("cat-file"),
        Command::Checkout   => String::from("checkout"),
        Command::Commit     => String::from("commit"),
        Command::HashObject => String::from("hash-object"),
        Command::Init       => String::from("init"),
        Command::Log        => String::from("log"),
        Command::LsFiles    => String::from("ls-files"),
        Command::LsTree     => String::from("ls-tree"),
        Command::Merge      => String::from("merge"),
        Command::Rebase     => String::from("rebase"),
        Command::RevParse   => String::from("rev-parse"),
        Command::Rm         => String::from("rm"),
        Command::ShowRef    => String::from("show-ref"),
        Command::Tag        => String::from("tag"),
    }
}

fn main() {
    let args = Args::parse();
    let cmd = parse_sub_cmd(&args);

    println!("Parsed cmd: {:?}", cmd)
}

