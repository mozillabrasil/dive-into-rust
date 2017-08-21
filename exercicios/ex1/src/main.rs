fn main() { 
    /* pode deixar vazio */
}

/// Um ano bissexto ocorre a cada quatro anos,
/// exceto em anos múltiplos de 100 que não são múltiplos de 400.
/// Retorne `true` se `year` é bissexto e `false` caso contrário.
fn is_leap_year(year: i32) -> bool {
    
}


// Testes

#[test]
fn test_vanilla_leap_year() {
    assert_eq!(is_leap_year(1996), true);
}

#[test]
fn test_any_old_year() {
    assert_eq!(is_leap_year(1997), false);
}

#[test]
fn test_century() {
    assert_eq!(is_leap_year(1900), false);
}

#[test]
fn test_exceptional_centuries() {
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(2400), true);
}