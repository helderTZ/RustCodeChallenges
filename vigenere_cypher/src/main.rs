
mod vigenere {

    // from solution
    const ALPHABET: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const A: u8 = b'A';
    const Z: u8 = b'Z';
    const WRAP:u8 = 26; // ALPHABET.len() as u8

    const VIGENERE_TABLE: &'static [&'static str] = &[
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "BCDEFGHIJKLMNOPQRSTUVWXYZA",
        "CDEFGHIJKLMNOPQRSTUVWXYZAB",
        "DEFGHIJKLMNOPQRSTUVWXYZABC",
        "EFGHIJKLMNOPQRSTUVWXYZABCD",
        "FGHIJKLMNOPQRSTUVWXYZABCDE",
        "GHIJKLMNOPQRSTUVWXYZABCDEF",
        "HIJKLMNOPQRSTUVWXYZABCDEFG",
        "IJKLMNOPQRSTUVWXYZABCDEFGH",
        "JKLMNOPQRSTUVWXYZABCDEFGHI",
        "KLMNOPQRSTUVWXYZABCDEFGHIJ",
        "LMNOPQRSTUVWXYZABCDEFGHIJK",
        "MNOPQRSTUVWXYZABCDEFGHIJKL",
        "NOPQRSTUVWXYZABCDEFGHIJKLM",
        "OPQRSTUVWXYZABCDEFGHIJKLMN",
        "PQRSTUVWXYZABCDEFGHIJKLMNO",
        "QRSTUVWXYZABCDEFGHIJKLMNOP",
        "RSTUVWXYZABCDEFGHIJKLMNOPQ",
        "STUVWXYZABCDEFGHIJKLMNOPQR",
        "TUVWXYZABCDEFGHIJKLMNOPQRS",
        "UVWXYZABCDEFGHIJKLMNOPQRST",
        "VWXYZABCDEFGHIJKLMNOPQRSTU",
        "WXYZABCDEFGHIJKLMNOPQRSTUV",
        "XYZABCDEFGHIJKLMNOPQRSTUVW",
        "YZABCDEFGHIJKLMNOPQRSTUVWX",
        "ZABCDEFGHIJKLMNOPQRSTUVWXY",
    ];

    // from solution
    fn clean_input(input: &str) -> impl Iterator<Item = u8> + '_ {
        input.bytes().filter_map(|x| {
            match x {
                A ..= Z => Some(x),
                b'a' ..= b'z' => Some(x - (b'a' - A)),
                _ => None,
            }
        })
    }

    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    fn extend_str(s: &str, len: usize) -> String {
        let mut s_adjusted = s.repeat((len as f32 / s.len() as f32).ceil() as usize);
        s_adjusted.truncate(len);
        s_adjusted
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let plaintext = remove_whitespace(plaintext).to_ascii_uppercase();
        let mut ciphertext = String::with_capacity(plaintext.len());

        // extend key to have the same length as the plaintext
        let key_len_adjusted = extend_str(key, plaintext.len());

        for (i, ch) in plaintext.chars().enumerate() {
            // get vigenere_table's row from plaintext's letter
            let table_row_index = ch as usize - ('A' as usize);

            // get the vigenere_table's col from the key's letter (find it in row 0)
            let key_char = key_len_adjusted.chars().nth(i).unwrap();
            let table_row = VIGENERE_TABLE[table_row_index];
            let table_col = VIGENERE_TABLE[0].chars().position(|c| c == key_char).unwrap();

            // the collumn in the table points to the encrypted char from table_row
            ciphertext.push(table_row.chars().nth(table_col).unwrap());
        }

        ciphertext
    }

    // from solution
    pub fn decrypt_sol(ciphertext: &str, key: &str) -> String {
        let mut key_iter = key
            .bytes()
            .map(|b| {
                b - A
            })
            .cycle();

        let ciphertext = clean_input(ciphertext)
            .map(|x| {
                let letter_in_key = key_iter.next().unwrap();
                ((x + WRAP - A) - letter_in_key) % WRAP + A
            })
            .collect();

        String::from_utf8(ciphertext).unwrap()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let ciphertext = remove_whitespace(ciphertext).to_ascii_uppercase();
        let mut plaintext = String::with_capacity(ciphertext.len());

        // extend key to have the same length as the ciphertext
        let key_len_adjusted = extend_str(key, ciphertext.len());

        for (i, ch) in ciphertext.chars().enumerate() {
            // get vigenere_table's row from the key's letter
            let table_row_index = key_len_adjusted.chars().nth(i).unwrap() as usize - ('A' as usize);

            // get the vigenere_table's col from the ciphertext's letter (find it in the vigenere_table's row)
            let table_row = VIGENERE_TABLE[table_row_index];
            let table_col = table_row.chars().position(|c| c == ch).unwrap();

            // the collumn in the table points to the decrypted char from row 0
            plaintext.push(VIGENERE_TABLE[0].chars().nth(table_col).unwrap());
        }

        plaintext
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);

    println!("{}", vigenere::encrypt("TOEMPOWEREVERYONE", key));

    println!("{}", vigenere::decrypt_sol(&ciphertext, key));
}