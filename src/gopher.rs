pub const HOST: &str = "localhost";
pub const PORT: &str = "70";

// Gopher menu item types
pub const ITEM_FILE: char = '0';
pub const ITEM_DIRECTORY: char = '1';
pub const ITEM_ERROR: char = '3';
pub const ITEM_INFO: char = 'i';
pub const ITEM_IMAGE: char = 'I';

pub fn create_menu_line(
    item_type: char,
    display_name: &str,
    selector: &str,
) -> String {
    format!(
        "{}{}\t/{}\t{}\t{}\r\n",
        item_type,
        display_name,
        selector,
        HOST,
        PORT
    )
}