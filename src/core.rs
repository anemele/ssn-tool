use crate::{COEFFICIENT, LAST_NUMBER};

fn get_chk_code(id_17: &str) -> Option<String> {
    if id_17.len() != 17 {
        eprintln!("Not 17 length input");
        return None;
    }

    let mut sum = 0;
    for (i, c) in id_17.chars().enumerate() {
        if !c.is_ascii_digit() {
            eprintln!("Invalid char: {c}");
            return None;
        }
        sum += COEFFICIENT[i] * (c as i32 - 48);
    }

    let tmp = LAST_NUMBER[sum as usize % 11];
    return Some(tmp.to_string());
}

pub fn check_gen2(id_18: &str) -> bool {
    if id_18.len() != 18 {
        eprintln!("Not 18 length input");
        return false;
    }

    match get_chk_code(&id_18[..17]) {
        None => false,
        Some(code) => {
            code == id_18
                .chars()
                .nth_back(0)
                .unwrap()
                .to_string()
                .to_uppercase()
        }
    }
}

pub fn cvt2to1(id_18: &str) -> Option<String> {
    if !check_gen2(&id_18) {
        eprintln!("Invalid id: {id_18}");
        return None;
    }
    let tmp = id_18[..6].to_string() + &id_18[8..17].to_string();
    Some(tmp)
}

pub fn cvt1to2(id_15: &str, year2: &str) -> Option<String> {
    if id_15.len() != 15 {
        eprintln!("Not 15 length input");
        return None;
    }
    let id_17 = id_15[..6].to_string() + year2 + &id_15[6..];
    match get_chk_code(&id_17) {
        None => None,
        Some(code) => Some(id_17 + &code),
    }
}

#[test]
fn test_check_gen2() {
    assert!(check_gen2("110101199912311230"));
    assert!(check_gen2("11010120001231129X"));
    assert!(check_gen2("11010120001231129x"));
}

#[test]
fn test_2to1() {
    assert_eq!(
        cvt2to1("110101199912311230"),
        Some("110101991231123".to_string())
    )
}

#[test]
fn test_1to2() {
    assert_eq!(
        cvt1to2("110101991231123", "19"),
        Some("110101199912311230".to_string())
    )
}
