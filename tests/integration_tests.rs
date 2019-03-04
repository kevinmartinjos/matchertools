use std::collections::HashMap;
use std::str::FromStr;
use matchertools;

mod common;

#[test]
fn test_simple_case() {
    let mut men_preferences = HashMap::new();
    men_preferences.insert(&0, vec![&0, &1]);
    men_preferences.insert(&1, vec![&0, &1]);

    let mut women_preferences = HashMap::new();
    women_preferences.insert(&0, vec![&1, &0]);
    women_preferences.insert(&1, vec![&1, &0]);

    let engaged_man_woman =
        matchertools::gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get(&&0), Some(&&1));
    assert_eq!(engaged_man_woman.get(&&1), Some(&&0));

//    common::assert_stable_engagement(&men_preferences, &women_preferences, &engaged_man_woman);
}

#[test]
fn test_simple_case_string() {
    let mut men_preferences: HashMap<&String, Vec<&String>> = HashMap::new();
    let mut women_preferences: HashMap<&String, Vec<&String>> = HashMap::new();
    let julius = String::from_str("Julius").unwrap();
    let cleopatra = String::from_str("Cleopatra").unwrap();
    let boudica = String::from_str("Boudica").unwrap();
    let vercingetorix = String::from_str("Vercingetorix").unwrap();

    men_preferences.insert(&julius, vec![&cleopatra, &boudica]);
    men_preferences.insert(&vercingetorix, vec![&boudica, &cleopatra]);

    women_preferences.insert(&cleopatra, vec![&julius, &vercingetorix]);
    women_preferences.insert(&boudica, vec![&vercingetorix, &julius]);

    // TODO: Remove the mutable reference
    let engaged_man_woman =
        matchertools::gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get(&&julius), Some(&&cleopatra));
    assert_eq!(engaged_man_woman.get(&&vercingetorix), Some(&&boudica));
}

#[test]
fn test_moderate_case() {
    let mut men_preferences = HashMap::new();
    men_preferences.insert(&0, vec![&0, &1, &2, &3, &4]);
    men_preferences.insert(&1, vec![&4, &3, &2, &1, &0]);
    men_preferences.insert(&2, vec![&0, &1, &4, &2, &3]);
    men_preferences.insert(&3, vec![&2, &4, &3, &0, &1]);
    men_preferences.insert(&4, vec![&4, &0, &1, &3, &2]);

    let mut women_preferences = HashMap::new();
    women_preferences.insert(&0, vec![&0, &1, &2, &3, &4]);
    women_preferences.insert(&1, vec![&1, &2, &4, &3, &0]);
    women_preferences.insert(&2, vec![&2, &4, &1, &0, &3]);
    women_preferences.insert(&3, vec![&0, &4, &3, &1, &2]);
    women_preferences.insert(&4, vec![&3, &0, &2, &4, &1]);

    let engaged_man_woman =
        matchertools::gale_shapley(&mut men_preferences, &mut women_preferences);

    assert_eq!(engaged_man_woman.get(&&0), Some(&&0));
    assert_eq!(engaged_man_woman.get(&&1), Some(&&3));
    assert_eq!(engaged_man_woman.get(&&2), Some(&&1));
    assert_eq!(engaged_man_woman.get(&&3), Some(&&2));
    assert_eq!(engaged_man_woman.get(&&4), Some(&&4));

//    common::assert_stable_engagement(&men_preferences, &women_preferences, &engaged_man_woman);
}
