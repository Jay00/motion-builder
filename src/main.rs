use docx_rs::*;

pub fn build_styles() -> Styles {
    let style1 = Style::new("Heading1", StyleType::Paragraph)
        .name("Heading 1")
        .align(AlignmentType::Center);

    let style2 = Style::new("Heading2", StyleType::Paragraph)
        .name("Heading 2")
        .align(AlignmentType::Center);

    let c = Styles::new().add_style(style1).add_style(style2);

    c
}

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./hello.docx");
    let file = std::fs::File::create(&path).unwrap();

    let styles = build_styles();

    let p1 = Paragraph::new()
        .add_run(Run::new().add_text("Hello").style("Run1"))
        .add_run(Run::new().add_text(" World"))
        .style("Heading1")
        .page_break_before(true);

    let p2 = Paragraph::new()
        .add_run(Run::new().add_text("Hello").style("Run1"))
        .add_run(Run::new().add_text(" World"))
        .style("Heading1")
        .page_break_before(true);

    let paragraphs = [p1, p2];

    let mut doc = Docx::new().styles(styles);

    let mut children = doc.document.children.clone();

    for p in paragraphs {
        children.push(DocumentChild::Paragraph(Box::new(p)));
    }

    doc.document.children = children;

    // for p in paragraphs {
    //     let nd = doc.add_paragraph(p);
    // }

    // let doc = doc.add_paragraph(p1);

    // let doc_copy = doc.clone();

    // let children = doc_copy.document.children.clone();
    // dbg!(children);

    doc.build().pack(file)?;

    Ok(())
}
