pub mod add;
pub mod mark;
pub mod default;
pub mod edit;
pub mod page;

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
    /// Add a task to  list
    Add(add::Arguments),
    /// Mark an task in the list
    Mark(mark::Arguments),
    /// Mark an task in the list
    Edit(edit::Arguments),
    /// Open a page
    Page(page::Arguments)
}