// std
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::constants::*;


/// Writes the script file for executing locale.xml file
///
/// # Returns
/// `Result<(), std::io::Error>` - Whether the operation was successful or not
fn write_script_file() -> Result<(), std::io::Error> {
    let mut script_file = File::create(SCRIPT_PATH)?;
    script_file.write_all(b"@echo off\n")?;
    script_file.write_all(b"control.exe intl.cpl,,/f:\"locale.xml\"\n")?;
    Ok(())
}


/// Writes the XML file containing locale and language preferences
///
/// # Returns
/// `Result<(), std::io::Error>` - Whether the operation was successful or not
pub fn write_xml_file() -> Result<(), std::io::Error> {
    let mut xml_file = File::create(XML_PATH)?;
    xml_file.write_all(XML_CONTENT.as_bytes())?;
    write_script_file()?;
    Ok(())
}


/// Removes the created XML and script files
///
/// # Returns
/// `Result<(), std::io::Error>` - Whether the operation was successful or not
pub fn remove_files() -> Result<(), std::io::Error> {
    let file_paths: [&str; 2] = [XML_PATH, SCRIPT_PATH];
    for file_path in &file_paths {
        fs::remove_file(file_path)?;
    }
    Ok(())
}
