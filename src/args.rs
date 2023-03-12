pub mod add;
pub mod mark;
pub mod default;

use clap::{ Parser, Subcommand };

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct ClerkArgs{
    #[clap(subcommand)]
    /// Test
    pub entity_type: Option<EntityType>,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Add a todo item to todo list
    Add(add::Arguments),
    // #[clap(override_usage("clerk.exe <INDEX_OF_MAINTASK> [INDEX_OF_SUBTASK] [OPTION]\n"))]
    /// Mark an item in the todo list
    Mark(mark::Arguments)
}