use std::fs;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use structs::Spreadsheet;
use structs::SharedStringTable;
use structs::Stylesheet;
use super::driver;

mod chart;
mod content_types;
mod doc_props_app;
mod doc_props_core;
mod workbook;
mod worksheet;
mod rels;
mod workbook_rels;
mod worksheet_rels;
mod theme;
mod shared_strings;
mod styles;
mod drawing;
mod drawing_rels;
mod vba_project_bin;
mod comment;
mod vml_drawing;
mod media;

#[derive(Debug)]
pub enum XlsxError {
    Io(io::Error),
    Xml(quick_xml::Error),
    Zip(zip::result::ZipError),
    Uft8(FromUtf8Error),
}

impl From<io::Error> for XlsxError {
    fn from(err: io::Error) -> XlsxError {
        XlsxError::Io(err)
    }
}

impl From<quick_xml::Error> for XlsxError {
    fn from(err: quick_xml::Error) -> XlsxError {
        XlsxError::Xml(err)
    }
}

impl From<zip::result::ZipError> for XlsxError {
    fn from(err: zip::result::ZipError) -> XlsxError {
        XlsxError::Zip(err)
    }
}

impl From<FromUtf8Error> for XlsxError {
    fn from(err: FromUtf8Error) -> XlsxError {
        XlsxError::Uft8(err)
    }
}

/// write spreadsheet file.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `writer` - writer.
/// # Return value
/// * `Result` - OK is void. Err is error message. 
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let mut b: Vec::<u8> = Vec::new();
/// let _ = umya_spreadsheet::writer::xlsx::write(&book, std::io::Cursor::new(&mut b));
/// ```
pub fn write<W: io::Seek + io::Write>(spreadsheet: &Spreadsheet, writer: W) -> Result<(), XlsxError> {
    let mut arv = zip::ZipWriter::new(writer);

    // Add Content_Types
    let _= content_types::write(spreadsheet, &mut arv, "[Content_Types].xml");

    // Add docProps App
    let _= doc_props_app::write(spreadsheet, &mut arv, "docProps", "app.xml");

    // Add docProps Core
    let _= doc_props_core::write(spreadsheet, &mut arv, "docProps", "core.xml");

    // Add vbaProject.bin
    let _= vba_project_bin::write(spreadsheet, &mut arv, "xl", "vbaProject.bin");

    // Add relationships
    let _ = rels::write(spreadsheet, &mut arv, "_rels", ".rels");
    let _ = workbook_rels::write(spreadsheet, &mut arv, "xl/_rels", "workbook.xml.rels");

    // Add theme
    let _ = theme::write(spreadsheet.get_theme(), &mut arv, "xl/theme", "theme1.xml");

    // Add workbook
    let _ = workbook::write(spreadsheet, &mut arv, "xl", "workbook.xml");

    // Add worksheets and relationships (drawings, ...)
    let mut chart_id = 1;
    let mut drawing_id = 1;
    let mut comment_id = 1;
    let mut shared_string_table = SharedStringTable::default();
    shared_string_table.init_setup();
    let mut stylesheet = Stylesheet::default();
    stylesheet.init_setup();
    for i in 0..spreadsheet.get_sheet_count() {
        let p_worksheet_id:&str = &(i+1).to_string();
        let _ = worksheet::write(&spreadsheet, &i, &mut shared_string_table, &mut stylesheet, &mut arv);
        let worksheet = &spreadsheet.get_sheet_collection()[i];
        let _ = worksheet_rels::write(worksheet, p_worksheet_id, &drawing_id, &comment_id,  &mut arv);
        let _ = drawing::write(worksheet, &drawing_id, &mut arv);
        let _ = drawing_rels::write(worksheet, &drawing_id, &chart_id, &mut arv);
        let _ = comment::write(worksheet, &comment_id,  &mut arv);
        let _ = vml_drawing::write(worksheet, &comment_id,  &mut arv);

        if worksheet.has_drawing_object() {
            drawing_id += 1;
        }

        if worksheet.has_comments() {
            comment_id += 1;
        }

        for graphic_frame in worksheet.get_worksheet_drawing().get_graphic_frame_collection(){
            let chart_space = graphic_frame.get_graphic().get_graphic_data().get_chart_space();
            let _ = chart::write(chart_space, &chart_id, &mut arv);
            chart_id += 1;
        }

        for picture in worksheet.get_worksheet_drawing().get_picture_collection(){
            let _ = media::write(picture, &mut arv, "xl/media");
        }
    }

    // Add SharedStrings
    let _ = shared_strings::write(&shared_string_table, &mut arv).unwrap();

    // Add Styles
    let _ = styles::write(&stylesheet, &mut arv).unwrap();

    arv.finish()?;
    Ok(())
}

/// write spreadsheet file.
/// # Arguments
/// * `spreadsheet` - Spreadsheet structs object.
/// * `path` - file path to save.
/// # Return value
/// * `Result` - OK is void. Err is error message. 
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// let path = std::path::Path::new("./tests/result_files/zzz.xlsx");
/// let _ = umya_spreadsheet::writer::xlsx::write_to_file(&book, path);
/// ```
pub fn write_to_file(spreadsheet: &Spreadsheet, path: &Path) -> Result<(), XlsxError> {
    write(spreadsheet, &mut io::BufWriter::new(fs::File::create(path)?))
}
