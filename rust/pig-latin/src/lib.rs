use regex::Regex;

const SUFFIX: &str = "ay";

pub fn translate(input: &str) -> String {
    let words = input.split_ascii_whitespace();

    let translated: Vec<String> = words
        .map(String::from)
        .map(vowel_start)
        .map(consonant_cluster)
        .collect();

    return translated.join(" ");
}

fn vowel_start(word: String) -> String {
    let test = Regex::new(r"(^[aeiou])|(^xr)|(^yt)").unwrap();
    let mut translated_word = String::from(&word);
    if test.is_match(&word) {
        translated_word.push_str(SUFFIX);
    }

    return translated_word;
}

fn consonant_cluster(word: String) -> String {
    let test = Regex::new(r"(?P<cluster>^[b-df-hj-np-tv-z]+)(?P<rest>.*)").unwrap();
    let mut translated_word = String::new();
    println!("{}", word);
    if test.is_match(&word) {
        let cluster = test
            .captures(&word)
            .unwrap()
            .name("cluster")
            .unwrap()
            .as_str();
        let rest = test.captures(&word).unwrap().name("rest").unwrap().as_str();

        let cluster_len = cluster.len();
        let rest_len = rest.len();
        if cluster.ends_with("y") && cluster_len > 1 || rest_len == 0 {
            // when rest len is zero, everything was a consonant!
            let y_index = cluster.find("y").unwrap_or_else(|| cluster_len);
            let without_y = &cluster[0..y_index];
            let rest_without_y = &cluster[y_index + 1..];
            if y_index == cluster_len {
                translated_word.push_str(word.as_str());
                return translated_word;
            }
            println!("{}", without_y);
            translated_word.push_str("y");
            translated_word.push_str(rest_without_y);
            translated_word.push_str(without_y);
            translated_word.push_str(SUFFIX);
            return translated_word;

        // y case
        } else if cluster.ends_with("q") && rest.starts_with("u") {
            let qu_rest = &rest[1..];
            translated_word.push_str(qu_rest);
            translated_word.push_str(cluster);
            translated_word.push_str("u");
            translated_word.push_str(SUFFIX);
            return translated_word;
        //qu case
        } else if cluster.starts_with("xr") || cluster.starts_with("yt") {
            translated_word.push_str(word.as_str());
            return translated_word;
        }

        translated_word.push_str(rest);
        translated_word.push_str(cluster);
        translated_word.push_str(SUFFIX);
        return translated_word;
    }
    translated_word.push_str(word.as_str());
    return translated_word;
}
