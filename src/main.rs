use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use dirs;

#[derive(Debug)]
struct Contact {
    vcard: Option<String>,
}

fn recover_contacts() -> Result<Vec<String>> {
    let mut db_path: PathBuf = dirs::home_dir().unwrap();
    db_path.push(".local/share/evolution/addressbook/system/contacts.db");
    let conn = Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT vcard FROM folder_id WHERE vcard IS NOT NULL")?;
    let contact_iter = stmt.query_map(params![], |row| {
        Ok(Contact {
            vcard: row.get(0)?,
        })
    })?;

    let mut contacts = Vec::new();

    for contact in contact_iter {
        match contact {
            Ok(contact) => {
                if let Some(vcard) = &contact.vcard {
                    let lines: Vec<&str> = vcard.split('\n').collect();
                    for line in lines {
                        contacts.push(line.to_string());
                    }
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(contacts)
}

fn main() -> Result<()> {
    let contacts = recover_contacts()?;
    for contact in contacts {
        println!("{}", contact);
    }
    Ok(())
}
