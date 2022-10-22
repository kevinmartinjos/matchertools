use matchertools::*;
use std::collections::HashMap;
#[test]
fn test_simple_case() {
    let men_preferences = HashMap::from([
        //
        (0, vec![0, 1]),
        (1, vec![0, 1]),
    ]);
    let women_preferences = HashMap::from([
        //
        (0, vec![1, 0]),
        (1, vec![1, 0]),
    ]);

    let engaged_man_woman = gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get(&0), Some(&1));
    assert_eq!(engaged_man_woman.get(&1), Some(&0));
}

#[test]
fn test_simple_case_str() {
    let julius = "Julius";
    let cleopatra = "Cleopatra";
    let boudica = "Boudica";
    let vercingetorix = "Vercingetorix";

    let men_preferences = HashMap::from([
        (julius, vec![cleopatra, boudica]),
        (vercingetorix, vec![boudica, cleopatra]),
    ]);
    let women_preferences = HashMap::from([
        (cleopatra, vec![julius, vercingetorix]),
        (boudica, vec![vercingetorix, julius]),
    ]);

    let engaged_man_woman = gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get(&julius), Some(&cleopatra));
    assert_eq!(engaged_man_woman.get(&vercingetorix), Some(&boudica));
}

#[test]
fn test_simple_case_str_2() {
    let men_preferences = HashMap::from([
        ("julius", vec!["cleopatra", "boudica", "nefertiti"]),
        ("antony", vec!["cleopatra", "nefertiti", "boudica"]),
        ("vercingetorix", vec!["boudica", "nefertiti", "cleopatra"]),
    ]);
    let women_preferences = HashMap::from([
        ("cleopatra", vec!["julius", "antony", "vercingetorix"]),
        ("boudica", vec!["vercingetorix", "antony", "julius"]),
        ("nefertiti", vec!["julius", "vercingetorix", "antony"]),
    ]);

    let engaged_man_woman = gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get("julius"), Some(&"cleopatra"));
    assert_eq!(engaged_man_woman.get("antony"), Some(&"nefertiti"));
    assert_eq!(engaged_man_woman.get("vercingetorix"), Some(&"boudica"));
}
#[test]
fn test_simple_case_string_3() {
    let julius = "Julius".to_owned();
    let cleopatra = "Cleopatra".to_owned();
    let boudica = "Boudica".to_owned();
    let vercingetorix = "Vercingetorix".to_owned();

    let men_preferences = HashMap::from([
        (&julius, vec![&cleopatra, &boudica]),
        (&vercingetorix, vec![&boudica, &cleopatra]),
    ]);
    let women_preferences = HashMap::from([
        (&cleopatra, vec![&julius, &vercingetorix]),
        (&boudica, vec![&vercingetorix, &julius]),
    ]);

    let engaged_man_woman = gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get(&julius), Some(&&cleopatra));
    assert_eq!(engaged_man_woman.get(&vercingetorix), Some(&&boudica));
}
#[test]
fn test_moderate_case() {
    let men_preferences = HashMap::from([
        (0, vec![0, 1, 2, 3, 4]),
        (1, vec![4, 3, 2, 1, 0]),
        (2, vec![0, 1, 4, 2, 3]),
        (3, vec![2, 4, 3, 0, 1]),
        (4, vec![4, 0, 1, 3, 2]),
    ]);

    let women_preferences = HashMap::from([
        (0, vec![0, 1, 2, 3, 4]),
        (1, vec![1, 2, 4, 3, 0]),
        (2, vec![2, 4, 1, 0, 3]),
        (3, vec![0, 4, 3, 1, 2]),
        (4, vec![3, 0, 2, 4, 1]),
    ]);

    let engaged_man_woman = gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get(&0), Some(&0));
    assert_eq!(engaged_man_woman.get(&1), Some(&3));
    assert_eq!(engaged_man_woman.get(&2), Some(&1));
    assert_eq!(engaged_man_woman.get(&3), Some(&2));
    assert_eq!(engaged_man_woman.get(&4), Some(&4));
}
