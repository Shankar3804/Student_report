use std::fs::File;
use std::io::{self, BufWriter, Write};
use printpdf::*;

fn calculate_average(total_marks: f64, subjects: u32) -> f64 {
    total_marks / subjects as f64
}

fn assign_grade(avg: f64) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn generate_pdf_report(name: &str, total: f64, subjects: u32, avg: f64, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let lines = vec![
        "Student Report Card".to_string(),
        "----------------------".to_string(),
        format!("Name: {}", name),
        format!("Total Marks: {}", total),
        format!("Number of Subjects: {}", subjects),
        format!("Average Marks: {:.2}", avg),
        format!("Grade: {}", grade),
    ];

    for (i, line) in lines.iter().enumerate() {
        current_layer.use_text(
            line,
            14.0,                           // font size as f64
            Mm(20.0),
            Mm(270.0 - (i as f64 * 10.0)),  // vertical spacing
            &font,
        );
    }

    let file = File::create("report_card.pdf").unwrap();
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).unwrap();

    println!("✅ Report card saved as 'report_card.pdf'");
}

fn main() {
    let mut name = String::new();
    let mut total_str = String::new();
    let mut subjects_str = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_str).unwrap();
    let total_marks: f64 = total_str.trim().parse().expect("Please enter valid marks");

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subjects_str).unwrap();
    let num_subjects: u32 = subjects_str.trim().parse().expect("Please enter valid number");

    let avg = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(avg);

    generate_pdf_report(name, total_marks, num_subjects, avg, grade);
}
