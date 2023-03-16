use crate::{data::ListData, CommandOutput};

pub fn handle(list: &ListData) -> CommandOutput {
    

    CommandOutput {
        data: list,
        page_num: None,
    }
}

