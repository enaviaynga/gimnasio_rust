pub fn texto_con_4_tipos(texto: &str) -> bool {
    let mut may = false;
    let mut min = false;
    let mut num = false;
    let mut sim = false;

    for i in texto.chars() {
        if i.is_uppercase() {
            may = true;
        }

        if i.is_lowercase() {
            min = true;
        }

        if i.is_numeric() {
            num = true;
        }

        if !i.is_alphanumeric() {
            sim = true;
        }

        if may && min && num && sim {
            return true;
        }
    }

    false
}
