use std::collections::HashMap;
use matchertools;

pub fn assert_stable_engagement(men_preferences: &HashMap<u32, Vec<u32>>, women_preferences: &HashMap<u32, Vec<u32>>, engaged_man_woman: &HashMap<u32, i32>) {
    // TODO: Write a unit test to make sure that this util method is working as expected :p

    // For the marriages to be stable, we need the following conditions:
    // 1. All men must be engaged (Consequently, all women must be engaged as well)
    // 2. Let Alice and Bob both be engaged, but not to each other. Upon completion of 
    // the algorithm, it is not possible for both Alice and Bob to prefer each other over their current partners.
    // In other words, there should be no man and woman with an incentive to cheat

    assert_all_men_are_engaged(&engaged_man_woman, &men_preferences);

    // Checking if there's a possibility to cheat.
    for (man, preferences) in men_preferences {
        let woman_engaged_to = engaged_man_woman.get(&man);
        let women_who_rejected_this_man = get_women_who_rejected_this_man(*man, *woman_engaged_to.unwrap(), &men_preferences);

        for woman in women_who_rejected_this_man {
            let current_man = get_currently_engaged_man_for_woman(woman, &engaged_man_woman);
            assert_woman_does_not_prefer_this_man_over_her_current_man(woman, current_man, *man, women_preferences)
        }
    }
}

fn assert_all_men_are_engaged(engaged_man_woman: &HashMap<u32, i32>, men_preferences: &HashMap<u32, Vec<u32>>) {
    for (man, _preferences) in men_preferences {
        let woman_engaged_to = engaged_man_woman.get(&man);
        assert_ne!(woman_engaged_to, Some(&-1));
    }
}

fn get_currently_engaged_man_for_woman(woman: u32, engaged_man_woman: &HashMap<u32, i32>) -> u32 {
    for (man, engaged_woman) in engaged_man_woman {
        if *engaged_woman == woman as i32{
            return *man;
        }
    }

    panic!("A woman is not engaged!");
}

fn get_women_who_rejected_this_man(man: u32, woman_engaged_to: i32, men_preferences: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let ranked_women: Vec<u32> = men_preferences.get(&man).unwrap().to_vec();
    let mut women_who_rejected: Vec<u32> = Vec::new();

    for woman in ranked_women {
        if woman as i32 == woman_engaged_to {
            return women_who_rejected;
        }
        women_who_rejected.push(woman);
    }

    panic!("What, this man is not engaged to this woman?!!");
}

fn assert_woman_does_not_prefer_this_man_over_her_current_man(woman: u32, current_man: u32, man: u32, women_preferences: &HashMap<u32, Vec<u32>>) {
    let rank_of_current_man = matchertools::get_rank(&women_preferences, &woman, &current_man);
    let rank_of_man = matchertools::get_rank(&women_preferences, &woman, &man);

    assert!(rank_of_current_man < rank_of_man);
}