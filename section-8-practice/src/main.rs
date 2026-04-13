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
        let text: String = String::from(string);
        let mut complete_string = String::new();

        for word in text.split_whitespace() {
            let mut string_to_chars = word.chars();
            let first_char = string_to_chars.next().unwrap();

            let first_letter_vowel = is_vowel(first_char);

            if first_letter_vowel {
                let result = format!("{word}-hay");

                if !complete_string.is_empty() {
                    complete_string.push(' ');
                }
                complete_string.push_str(&result);
            } else {
                let rest_of_word = string_to_chars.collect::<String>();
                let result = format!("{rest_of_word}-{first_char}ay");

                if !complete_string.is_empty() {
                    complete_string.push(' ');
                }
                complete_string.push_str(&result);
            }
        }
        println!("{complete_string}");
    }

    convert_string("BING BING BONG");
}
