use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {

    
    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    // let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");
    
    doc.save(&mut BufWriter::new(File::create("pdf/test_working.pdf").unwrap())).unwrap();
}
