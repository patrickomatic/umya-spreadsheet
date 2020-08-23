use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use super::super::structs::spreadsheet::Spreadsheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write(spreadsheet: &Spreadsheet, dir: &TempDir, sub_dir: &str, file_name: &str) -> Result<(), XlsxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // cp:coreProperties
    write_start_tag(&mut writer, "cp:coreProperties", vec![
        ("xmlns:cp", "http://schemas.openxmlformats.org/package/2006/metadata/core-properties"),
        ("xmlns:dc", "http://purl.org/dc/elements/1.1/"),
        ("xmlns:dcterms", "http://purl.org/dc/terms/"),
        ("xmlns:dcmitype", "http://purl.org/dc/dcmitype/"),
        ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
    ], false);

    // dc:title
    write_start_tag(&mut writer, "dc:title", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_title());
    write_end_tag(&mut writer, "dc:title");

    // dc:subject
    write_start_tag(&mut writer, "dc:subject", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_subject());
    write_end_tag(&mut writer, "dc:subject");

    // dc:creator
    write_start_tag(&mut writer, "dc:creator", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_creator());
    write_end_tag(&mut writer, "dc:creator");

    // cp:keywords
    write_start_tag(&mut writer, "cp:keywords", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_keywords());
    write_end_tag(&mut writer, "cp:keywords");

    // dc:description
    write_start_tag(&mut writer, "dc:description", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_description());
    write_end_tag(&mut writer, "dc:description");

    // cp:lastModifiedBy
    write_start_tag(&mut writer, "cp:lastModifiedBy", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_last_modified_by());
    write_end_tag(&mut writer, "cp:lastModifiedBy");

    // cp:revision
    write_start_tag(&mut writer, "cp:revision", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_revision());
    write_end_tag(&mut writer, "cp:revision");

    // dcterms:created
    write_start_tag(&mut writer, "dcterms:created", vec![
        ("xsi:type", "dcterms:W3CDTF"),
    ], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_created());
    write_end_tag(&mut writer, "dcterms:created");

    // dcterms:modified
    write_start_tag(&mut writer, "dcterms:modified", vec![
        ("xsi:type", "dcterms:W3CDTF"),
    ], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_modified());
    write_end_tag(&mut writer, "dcterms:modified");

    // cp:category
    write_start_tag(&mut writer, "cp:category", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_category());
    write_end_tag(&mut writer, "cp:category");

    // cp:version
    write_start_tag(&mut writer, "cp:version", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_version());
    write_end_tag(&mut writer, "cp:version");

    write_end_tag(&mut writer, "cp:coreProperties");
    let _ = make_file_from_writer(format!("{}/{}",sub_dir,file_name).as_str(), dir, writer, Some(sub_dir)).unwrap();
    Ok(())
}