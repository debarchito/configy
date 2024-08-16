use crate::sync::parse::get_entries;

pub fn list_links() {
  let entries = get_entries();
  let mut i = 1;

  msg!("<b>[?] The available links are:\n");

  for entry in entries {
    for value in entry.1 {
      msg!("<b>{}. <w>{} <b>==> <w>{}", i, entry.0, value);
      i += 1;
    }
  }

  msg!("</rs>");
}
