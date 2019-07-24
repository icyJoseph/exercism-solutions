use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
    let n_factors = factors(&n);
    let result: String = raindrop_speak(&n, n_factors);

    return result;
}

fn factors(n: &u32) -> Vec<u32> {
    let mut n_factors: Vec<u32> = vec![]; // when we find a factor push it here
    let mut comp_factors: Vec<u32> = vec![]; // and its complement here, if its different

    for number in 1..=*n {
        if number * number > *n {
            break;
        }
        if *n % number == 0 {
            n_factors.push(number); // we could just end the problem here
            if number != *n / number {
                comp_factors.push(*n / number) // avoid duplicate factors
            }
        }
    }

    comp_factors.reverse();

    n_factors.append(&mut comp_factors);

    return n_factors;
}

fn raindrop_speak(source: &u32, factors: Vec<u32>) -> String {
    let mut rain_speak_map = HashMap::new();
    let empty_string = String::from("");

    rain_speak_map.insert(3, String::from("Pling"));
    rain_speak_map.insert(5, String::from("Plang"));
    rain_speak_map.insert(7, String::from("Plong"));

    let result: String = factors.iter().fold(String::new(), |mut prev, val| {
        let speak = match rain_speak_map.get(&val) {
            Some(w) => w,
            None => &empty_string,
        };
        prev.push_str(&speak);
        return prev;
    });
    // if nothing matched in rain_speak_map
    // return the original number
    if result.len() == 0 {
        // We could use the last element, but that would depend in having sorted factors
        // Which the factors fn does!
        // let last = *factors.last().unwrap();
        // return last.to_string();
        return source.to_string();
    };
    return result;
}
