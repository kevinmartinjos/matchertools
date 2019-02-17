use std::collections::HashMap;
use std::collections::HashSet;

pub fn gale_shapley(
    men_preferences: &HashMap<u32, Vec<u32>>,
    women_preferences: &HashMap<u32, Vec<u32>>,
) -> HashMap<u32, u32> {
    // ranks are indexed from zero
    // assert that the size of the hashmaps are the same
    // TODO: Add validations for input
    // TODO: Clone the input so that it is not consumed in the process

    let mut men_preferences = men_preferences.clone();
    let mut women_preferences = women_preferences.clone();
    let mut engaged_man_woman: HashMap<u32, u32> = HashMap::new();

    while get_unengaged_men(&men_preferences, &engaged_man_woman).len() != 0 {
        play_round(
            &mut engaged_man_woman,
            &mut men_preferences,
            &mut women_preferences,
        );
    }

    return engaged_man_woman;
}

fn play_round(
    engaged_man_woman: &mut HashMap<u32, u32>,
    mut men_preferences: &mut HashMap<u32, Vec<u32>>,
    women_preferences: &HashMap<u32, Vec<u32>>,
) {
    // 1. Find all un-engaged men
    // 2. Propose to the highest ranked woman
    // 3. For each woman, reject/engage all proposals
    // 4. update engaged_man_woman

    let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);

    let proposals = create_proposals(unengaged_men, &mut men_preferences, &women_preferences);
    accept_or_reject_proposals(proposals, engaged_man_woman, &women_preferences);
}

fn get_unengaged_men(
    men_preferences: &HashMap<u32, Vec<u32>>,
    engaged_man_woman: &HashMap<u32, u32>,
) -> HashSet<u32> {
    //TODO: Use functional programming style filters here
    let mut unengaged_men: HashSet<u32> = HashSet::new();
    for (man, _preferences) in men_preferences {
        let engaged_woman = engaged_man_woman.get(&man);
        if engaged_woman.is_none() {
            unengaged_men.insert(*man);
        }
    }

    return unengaged_men;
}

fn create_proposals(
    unengaged_men: HashSet<u32>,
    men_preferences: &mut HashMap<u32, Vec<u32>>,
    women_preferences: &HashMap<u32, Vec<u32>>,
) -> HashMap<u32, HashSet<u32>> {
    let mut proposals: HashMap<u32, HashSet<u32>> = HashMap::new();

    for woman in women_preferences.keys() {
        proposals.insert(*woman, HashSet::<u32>::new());
    }

    for unengaged_man in unengaged_men {
        let mut preferred_women = men_preferences.get_mut(&unengaged_man);
        match preferred_women {
            Some(ref mut preferred_women) => match proposals.get_mut(&preferred_women[0]) {
                Some(ref mut woman_proposal_list) => {
                    woman_proposal_list.insert(unengaged_man);
                    preferred_women.remove(0);
                }
                None => println!("Error: Woman not found in proposals hashmap"),
            },
            None => println!("Error: The unengaged_man does not have a preferred_women list"),
        }
    }

    return proposals;
}

fn accept_or_reject_proposals(
    proposals: HashMap<u32, HashSet<u32>>,
    engaged_man_woman: &mut HashMap<u32, u32>,
    women_preferences: &HashMap<u32, Vec<u32>>,
) {
    for (woman, interested_men) in proposals {
        if !interested_men.is_empty() {
            let best_interested_man =
                get_best_man_from_interested_men(woman, women_preferences, interested_men);

            if !woman_is_engaged(engaged_man_woman, woman) {
                engaged_man_woman.insert(best_interested_man.unwrap(), woman);
            } else {
                let rank_best_interested =
                    get_rank(&women_preferences, &woman, &best_interested_man.unwrap()).unwrap();
                let currently_engaged_man =
                    get_currently_engaged_man(&engaged_man_woman, &woman).unwrap();
                let rank_engaged_man =
                    get_rank(women_preferences, &woman, &currently_engaged_man).unwrap();
                if rank_best_interested < rank_engaged_man {
                    make_engagement(
                        engaged_man_woman,
                        woman,
                        best_interested_man.unwrap(),
                        currently_engaged_man,
                    );
                }
            }
        }
    }
}

fn get_best_man_from_interested_men(
    woman: u32,
    women_preferences: &HashMap<u32, Vec<u32>>,
    interested_men: HashSet<u32>,
) -> Option<u32> {
    //TODO: We do not need the entire women_preferences here. Just the preferences of 'woman' would suffice
    let men_rankings = women_preferences.get(&woman);
    match men_rankings {
        Some(men) => {
            for man in men {
                if interested_men.contains(man) {
                    return Some(*man);
                }
            }
            return None;
        }
        None => return None,
    }
}

fn woman_is_engaged(engaged_man_woman: &HashMap<u32, u32>, woman: u32) -> bool {
    for (_man, engaged_woman) in engaged_man_woman {
        if *engaged_woman == woman {
            return true;
        }
    }

    return false;
}

pub fn get_rank(preferences: &HashMap<u32, Vec<u32>>, key: &u32, value: &u32) -> Option<u32> {
    //TODO: Rename 'key' and 'value' to something better
    let rankings = preferences.get(key);
    match rankings {
        Some(rankings) => {
            for (rank, value_at_rank) in rankings.iter().enumerate() {
                if value_at_rank == value {
                    return Some(rank as u32);
                }
            }

            return None;
        }
        None => return None,
    }
}

fn get_currently_engaged_man(engaged_man_woman: &HashMap<u32, u32>, woman: &u32) -> Option<u32> {
    for (man, engaged_woman) in engaged_man_woman {
        if *engaged_woman == *woman {
            return Some(*man);
        }
    }

    //TODO: Panic! The control should reach this method only if the woman is engaged
    return None;
}

fn make_engagement(
    engaged_man_woman: &mut HashMap<u32, u32>,
    woman: u32,
    man: u32,
    currently_engaged_man: u32,
) {
    engaged_man_woman.insert(man, woman);
    engaged_man_woman.remove(&currently_engaged_man);
}

#[cfg(test)]

mod tests {
    use super::*;
    // const sample_size = 5;

    fn get_preferences_config_1() -> HashMap<u32, Vec<u32>> {
        let mut preferences = HashMap::new();
        preferences.insert(0, vec![0, 1, 2, 3, 4]);
        preferences.insert(1, vec![4, 3, 2, 1, 0]);
        preferences.insert(2, vec![0, 1, 4, 2, 3]);
        preferences.insert(3, vec![2, 4, 3, 0, 1]);
        preferences.insert(4, vec![4, 0, 1, 3, 2]);

        return preferences;
    }

    fn get_preferences_config_2() -> HashMap<u32, Vec<u32>> {
        let mut preferences = HashMap::new();
        preferences.insert(0, vec![0, 1, 2, 3, 4]);
        preferences.insert(1, vec![1, 2, 4, 3, 0]);
        preferences.insert(2, vec![2, 4, 1, 0, 3]);
        preferences.insert(3, vec![0, 4, 3, 1, 2]);
        preferences.insert(4, vec![3, 0, 2, 4, 1]);

        return preferences;
    }

    #[test]
    fn test_get_unengaged_men() {
        let men_preferences = get_preferences_config_1();
        let mut engaged_man_woman = HashMap::new();
        engaged_man_woman.insert(0, 0);
        engaged_man_woman.insert(1, 1);
        engaged_man_woman.insert(2, 2);

        let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);
        assert_eq!(unengaged_men, vec![3, 4].into_iter().collect());

        engaged_man_woman.insert(3, 3);
        let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);
        assert_eq!(unengaged_men, vec![4].into_iter().collect());

        engaged_man_woman.insert(4, 4);
        let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);
        assert_eq!(unengaged_men, vec![].into_iter().collect());
    }

    #[test]
    fn test_create_proposals_base_case() {
        // Testing the base case, i.e create proposals in the first round when no man is engaged to a woman
        let unengaged_men: HashSet<u32> = [0, 1, 2, 3, 4].iter().cloned().collect();

        let mut men_preferences = HashMap::new();
        men_preferences.insert(0, vec![0, 1, 2, 3, 4]);
        men_preferences.insert(1, vec![0, 1, 2, 3, 4]);
        men_preferences.insert(2, vec![0, 1, 2, 3, 4]);
        men_preferences.insert(3, vec![0, 1, 2, 3, 4]);
        men_preferences.insert(4, vec![0, 1, 2, 3, 4]);

        let mut women_preferences = HashMap::new();
        women_preferences.insert(0, vec![0, 1, 2, 3, 4]);
        women_preferences.insert(1, vec![0, 1, 2, 3, 4]);
        women_preferences.insert(2, vec![0, 1, 2, 3, 4]);
        women_preferences.insert(3, vec![0, 1, 2, 3, 4]);
        women_preferences.insert(4, vec![0, 1, 2, 3, 4]);

        let proposals = create_proposals(unengaged_men, &mut men_preferences, &women_preferences);

        assert_eq!(
            proposals.get(&0),
            Some(&vec![0, 1, 2, 3, 4].into_iter().collect())
        );
        assert_eq!(proposals.get(&1), Some(&vec![].into_iter().collect()));
        assert_eq!(proposals.get(&2), Some(&vec![].into_iter().collect()));
        assert_eq!(proposals.get(&3), Some(&vec![].into_iter().collect()));
        assert_eq!(proposals.get(&4), Some(&vec![].into_iter().collect()));
    }

    #[test]
    fn test_create_proposals_base_case_with_better_preferences() {
        // Testing the base case, i.e create proposals in the first round when no man is engaged to a woman
        let unengaged_men: HashSet<u32> = [0, 1, 2, 3, 4].iter().cloned().collect();
        let mut men_preferences = get_preferences_config_1();
        let women_preferences = get_preferences_config_2();
        let proposals = create_proposals(unengaged_men, &mut men_preferences, &women_preferences);

        assert_eq!(proposals.get(&0), Some(&vec![0, 2].into_iter().collect()));
        assert_eq!(proposals.get(&1), Some(&vec![].into_iter().collect()));
        assert_eq!(proposals.get(&2), Some(&vec![3].into_iter().collect()));
        assert_eq!(proposals.get(&3), Some(&vec![].into_iter().collect()));
        assert_eq!(proposals.get(&4), Some(&vec![1, 4].into_iter().collect()));
    }

    #[test]
    fn test_get_best_man_from_interested_men() {
        let women_preferences = get_preferences_config_2();
        let woman: u32 = 1; // The second woman
        let interested_men: HashSet<u32> = vec![0, 1, 2, 3, 4].into_iter().collect();

        let best_man = get_best_man_from_interested_men(woman, &women_preferences, interested_men);
        assert_eq!(best_man, Some(1));

        let woman: u32 = 2;
        let interested_men: HashSet<u32> = vec![0, 1, 2, 3, 4].into_iter().collect();

        let best_man = get_best_man_from_interested_men(woman, &women_preferences, interested_men);
        assert_eq!(best_man, Some(2));
    }

    #[test]
    fn test_woman_is_engaged() {
        let mut engaged_man_woman: HashMap<u32, u32> = HashMap::new();

        engaged_man_woman.insert(0, 0);

        let is_engaged = woman_is_engaged(&engaged_man_woman, 0);
        assert!(is_engaged);

        let is_engaged = woman_is_engaged(&engaged_man_woman, 1);
        assert!(!is_engaged);
    }

    #[test]
    fn test_get_rank() {
        let men_preferences = get_preferences_config_2();

        let rank = get_rank(&men_preferences, &1, &1);
        assert_eq!(rank, Some(0));

        let rank = get_rank(&men_preferences, &4, &1);
        assert_eq!(rank, Some(4));

        let rank = get_rank(&men_preferences, &4, &16);
        assert_eq!(rank, None);
    }

    #[test]
    fn test_get_currently_engaged_man() {
        let mut engaged_man_woman: HashMap<u32, u32> = HashMap::new();

        engaged_man_woman.insert(0, 0);
        engaged_man_woman.insert(2, 4);

        let engaged_man = get_currently_engaged_man(&engaged_man_woman, &0);
        assert_eq!(engaged_man, Some(0));

        let engaged_man = get_currently_engaged_man(&engaged_man_woman, &1);
        assert_eq!(engaged_man, None);

        let engaged_man = get_currently_engaged_man(&engaged_man_woman, &4);
        assert_eq!(engaged_man, Some(2));
    }

    #[test]
    fn test_make_engagement() {
        let mut engaged_man_woman: HashMap<u32, u32> = HashMap::new();
        engaged_man_woman.insert(0, 0);
        engaged_man_woman.insert(1, 1);

        let woman = 0;
        let currently_engaged_man = 0;
        let the_better_man = 2;
        make_engagement(
            &mut engaged_man_woman,
            woman,
            the_better_man,
            currently_engaged_man,
        );

        assert_eq!(engaged_man_woman.get(&currently_engaged_man), None);
        assert_eq!(engaged_man_woman.get(&the_better_man), Some(&0));
        assert_eq!(engaged_man_woman.get(&1), Some(&1));
    }

    #[test]
    fn test_accept_or_reject_proposals() {
        let mut men_preferences = get_preferences_config_1();
        let women_preferences = get_preferences_config_2();

        let mut engaged_man_woman: HashMap<u32, u32> = HashMap::new();
        let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);
        let proposals = create_proposals(unengaged_men, &mut men_preferences, &women_preferences);

        // Proposals would be:
        // 0 - {0, 2}
        // 1 - {}
        // 2 - {3}
        // 3 - {}
        // 4 - {1, 4}
        accept_or_reject_proposals(proposals, &mut engaged_man_woman, &women_preferences);
        assert_eq!(engaged_man_woman.get(&0), Some(&0));
        assert_eq!(engaged_man_woman.get(&1), None);
        assert_eq!(engaged_man_woman.get(&2), None);
        assert_eq!(engaged_man_woman.get(&3), Some(&2));
        assert_eq!(engaged_man_woman.get(&4), Some(&4));

        // Another round. Men 1 & 2
        let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);
        let proposals = create_proposals(unengaged_men, &mut men_preferences, &women_preferences);

        // proposals would be
        // 0 - {}
        // 1 - {2}
        // 2 - {}
        // 3 - {1}
        // 4 - {}
        accept_or_reject_proposals(proposals, &mut engaged_man_woman, &women_preferences);
        assert_eq!(engaged_man_woman.get(&0), Some(&0));
        assert_eq!(engaged_man_woman.get(&1), Some(&3));
        assert_eq!(engaged_man_woman.get(&2), Some(&1));
        assert_eq!(engaged_man_woman.get(&3), Some(&2));
        assert_eq!(engaged_man_woman.get(&4), Some(&4));

        assert_eq!(
            get_unengaged_men(&men_preferences, &engaged_man_woman).len(),
            0
        );
    }
}
