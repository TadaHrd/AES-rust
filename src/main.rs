use aes_rust::anyway;
use cli_select::Select;

mod indexed_str;
use indexed_str::IndexedStr;
use std::io::Write;

/// Used in main function checking
macro_rules! ES_match {
    // Matches the encosure scheme
    ($v:expr; $($n:expr, $encode_fn:path, $decode_fn:path) | +; $buf:expr) => {
        $(
            if let ($n, true) = $v {
                println!("\nEncoded data:\n{}", $encode_fn($buf, ES_match!(sep)));
                return;
            }
            if let ($n, false) = $v {
                println!(
                    "\nDecoded text:\n{}",
                    $decode_fn(&$buf).unwrap_or_else(|err| format!("{:?}", err.1))
                );
                return;
            }
        )+
    };
    // Computes the separator
    (sep) => {{
        print!("\nEnter separator ('\\n' for newline): ");
        let _ = std::io::stdout().flush();

        let mut separator = String::new();
        let _ = std::io::stdin().read_line(&mut separator);

        separator.pop();
        if separator.ends_with('\r') {
            separator.pop();
        }

        separator.replace("\\n", "\n")
    }}
}

/// # The main function.
///
/// 1. Prompts the user about which encosure scheme they want to use.
/// 2. Prompts the user about whether they want to encode or decode.
/// 3. Tells the user to write the text to encode or data to encode
/// and write `.done` on an empty line after that.
/// 4. Tells the user to write the separator on a single line.
/// 5. Prints the encoded data/decoded text.
fn main() {
    let stdout = std::io::stdout();

    // Make select
    let (selected_item, encode) = {
        const ITEMS: &[IndexedStr] =
            &IndexedStr::make_array(&["Anyway (AES)", "Escaped Enyway (EAES)"]);

        let selected_item = *Select::new(ITEMS, &std::io::stdout())
            .underline_selected_item()
            .start();

        let encode = *Select::new(&IndexedStr::make_array(&["Encode", "Decode"]), &stdout)
            .underline_selected_item()
            .start();

        (selected_item, encode.idx == 0)
    };

    // Print message
    if encode {
        println!("\nType .done on an empty line to encode.\nEnter text to encode:\n");
    } else {
        println!("\nThree .done on an empty line to decode.\nEnter data to decode:\n");
    }

    // Make a buffer for storing inputs
    let mut buf = String::new();

    // Get input
    for line in std::io::stdin().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        if line == ".done" {
            buf.pop();
            break;
        }

        buf += &line;
        buf += "\n";
    }

    // Match the encosure scheme and compute separator
    ES_match!(
        (selected_item.idx, encode);
        // AES
        0, anyway::encode, anyway::decode_to_string |
        // EAES
        1, anyway::encode_escaped, anyway::decode_to_string

        ; buf
    );
}
