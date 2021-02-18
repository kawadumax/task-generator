use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::fs;
use glob::glob;

pub fn delete_pdf() {
    for entry in glob("pdf/*.pdf").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                fs::remove_file(path);
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn init_pdf(){
    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    // let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");
}

pub fn export(){
    doc.save(&mut BufWriter::new(File::create("pdf/test_working.pdf").unwrap())).unwrap();
}
