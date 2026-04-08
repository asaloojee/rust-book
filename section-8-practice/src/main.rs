fn main() {
    // let mut v = vec![4, 2, 1, 5, 14, 67, 33];
    // v.sort();
    // println!("{:?}", v);
    // let v_len = v.len();
    // println!("Length of vector: {v_len}");
    // let v_median = v[v_len / 2];
    //
    // println!("The median of vector v is: {v_median}");

    fn is_vowel(c: char) -> bool {
        let vowels: [char; 10] = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
        vowels.contains(&c)
    }

    fn convert_string(string: &str) {
        let mut string_to_chars = string.chars();
        let first_char = string_to_chars.next().unwrap();

        let first_letter_vowel = is_vowel(first_char);
        println!("first letter of {string} is a vowel: {first_letter_vowel}");
    }

    convert_string("OONGABOONGA");
}
