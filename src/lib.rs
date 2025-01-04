const COEFFICIENT: [i32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const LAST_NUMBER: &[u8] = b"10X987654321";

fn get_chk_code(id_17: &str) -> anyhow::Result<char> {
    if id_17.len() != 17 {
        anyhow::bail!("Not 17 length input: {:}", id_17);
    }

    let mut sum = 0;
    for (i, c) in id_17.chars().take(17).enumerate() {
        if !c.is_ascii_digit() {
            anyhow::bail!("Invalid char: {:}", c);
        }
        sum += COEFFICIENT[i] * (c as i32 - 48);
    }

    let res = LAST_NUMBER[sum as usize % 11];
    Ok(res as char)
}

pub fn check_gen2(id_18: &str) -> anyhow::Result<()> {
    if id_18.len() != 18 {
        anyhow::bail!("Not 18 length input: {:}", id_18);
    }

    let code = get_chk_code(&id_18[..17])?;
    // here use `unwrap` is ok, because the input is 18 length.
    let ok = id_18.chars().nth_back(0).unwrap().to_ascii_uppercase() == code;
    if !ok {
        anyhow::bail!("Invalid id: {:}", id_18);
    }

    Ok(())
}

pub fn cvt2to1(id_18: &str) -> anyhow::Result<String> {
    check_gen2(&id_18)?;
    let res = format!("{}{}", &id_18[..6], &id_18[8..17]);
    Ok(res)
}

pub fn cvt1to2(id_15: &str, year2: &str) -> anyhow::Result<String> {
    if id_15.len() != 15 {
        anyhow::bail!("Not 15 length input: {:}", id_15);
    }
    let mut id_17 = id_15[..6].to_string() + year2 + &id_15[6..];
    let code = get_chk_code(&id_17)?;
    id_17.push(code);
    Ok(id_17)
}

#[test]
fn test_check_gen2() {
    assert!(check_gen2("110101199912311230").is_ok());
    assert!(check_gen2("11010120001231129X").is_ok());
    assert!(check_gen2("11010120001231129x").is_ok());
}

#[test]
fn test_2to1() {
    assert_eq!(cvt2to1("110101199912311230").unwrap(), "110101991231123")
}

#[test]
fn test_1to2() {
    assert_eq!(
        cvt1to2("110101991231123", "19").unwrap(),
        "110101199912311230"
    )
}
