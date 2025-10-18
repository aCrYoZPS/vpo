use std::{io, u8};

pub fn generate_html_file(rows: u8, columns: usize) -> String {
    let html_template = "<!DOCTYPE html>\n\
                     <head>\n\
                       <title>Table demo</title>\n\
                       <style>\n\
                         html, body {\n\
                           margin: 0;\n\
                           padding: 0;\n\
                           width: 100%;\n\
                           height: 100%;\n\
                           overflow: hidden;\n\
                         }\n\
                         table {\n\
                           width: 100%;\n\
                           height: 100vh;\n\
                           border-collapse: collapse;\n\
                           background: linear-gradient(to right, black, white);\n
                           table-layout: fixed;\n\
                         }\n\
                         th, td {\n\
                           border:none;\n\
                         }\n\
                       </style>\n\
                     </head>\n";

    return html_template.to_string()
        + &format!("<body>{}</body>", generate_html_table(rows, columns));
}

pub fn generate_html_table(rows: u8, columns: usize) -> String {
    let mut rows_string = String::new();
    for i in 0..rows {
        rows_string.push_str(&generate_html_table_row(columns, generate_gray(i, rows)));
    }
    return format!("<table><tbody>{}</tbody></table>", rows_string);
}

pub fn generate_html_table_row(columns: usize, gray: u8) -> String {
    let cell = format!(
        "<td style='background-color: rgb({}, {}, {});'></td>",
        gray, gray, gray
    );
    return format!("<tr>{}</tr>", cell.repeat(columns));
}

pub fn generate_gray(index: u8, total: u8) -> u8 {
    let step = u8::MAX as f64 / (total - 1) as f64;
    let gray_value: usize = step.round() as usize * (total - index) as usize;
    if gray_value > u8::MAX as usize {
        return u8::MAX;
    } else {
        return gray_value as u8;
    }
}

pub fn input_u8() -> Result<u8, String> {
    let mut input_buffer = String::new();
    match io::stdin().read_line(&mut input_buffer) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: {}", err));
        }
    };

    return match input_buffer.trim().parse::<u8>() {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("Error: {}", err)),
    };
}
