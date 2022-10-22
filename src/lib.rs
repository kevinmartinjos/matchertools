use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/**
    Returns a HashMap indicating who is engaged to whom using the Gale-Shapley algorithm

    I use the terms 'man' and 'women' here because it helps me relate to the words used in the
    original stable marriage problem.

    # Remarks:
    The number of men and women should be equal. In other words, `input_men_preferences` and `input_women_preferences` should have the
    same number of keys. Also, each 'man' in `input_men_preferences` should indicate the 'ranking' of each woman in the associated vec.
    Same holds true for women - man: each woman in `input_women_preferences` should indicate her preference in the associated vec.

    # Arguments:
    * input_men_preferences - HashMap of each men to a vec of women, ordered by preference. The most preferred woman comes first in the vec
    * input_women_preferences - HashMap of each woman to a vec of men, ordered by preference. The most preferred man comes first in the vec

    # Returns:
    A Hashmap<T, T> which maps each man to a woman. This mapping will be stable.

    # Examples
    ```
    use std::collections::{HashMap};
    let mut men_preferences= HashMap::new();
    let mut women_preferences = HashMap::new();

    men_preferences.insert("julius", vec!["cleopatra", "boudica"]);
    men_preferences.insert("vercingetorix", vec!["boudica", "cleopatra"]);

    women_preferences.insert("cleopatra", vec!["julius", "vercingetorix"]);
    women_preferences.insert("boudica", vec!["vercingetorix", "julius"]);

    let engaged_man_woman =
        matchertools::gale_shapley(&men_preferences, &women_preferences);

    assert_eq!(engaged_man_woman.get("julius"), Some(&"cleopatra"));
    assert_eq!(engaged_man_woman.get("vercingetorix"), Some(&"boudica"));
    ```
*/
pub fn gale_shapley<T>(
    input_men_preferences: &HashMap<T, Vec<T>>,
    input_women_preferences: &HashMap<T, Vec<T>>,
) -> HashMap<T, T>
where
    T: Eq + Hash + Clone,
{
    // TODO: Add validations for the input
    let mut men_preferences = HashMap::new();
    let mut women_preferences = HashMap::new();
    let mut engagements = HashMap::new();

    // I initially implemented the algorithm over usize. So I'm now trying to convert HashMap<T, Vec<T>> to HashMap<usize, Vec<usize>>.
    // TODO: Get rid of this step. Rewrite the implementation to directly work on generic types
    let men_reference_to_usize: HashMap<_, _> = input_men_preferences
        .keys()
        .enumerate()
        .map(|(idx, man)| (man.to_owned(), idx))
        .collect();

    let women_reference_to_usize: HashMap<_, _> = input_women_preferences
        .keys()
        .enumerate()
        .map(|(idx, woman)| (woman.to_owned(), idx))
        .collect();

    for (man, women) in input_men_preferences.iter() {
        let women_as_usize = women
            .iter()
            .map(|woman| *women_reference_to_usize.get(woman).unwrap())
            .collect();
        men_preferences.insert(*men_reference_to_usize.get(man).unwrap(), women_as_usize);
    }

    for (woman, men) in input_women_preferences.iter() {
        let men_as_usize = men
            .iter()
            .map(|man| *men_reference_to_usize.get(man).unwrap())
            .collect();
        women_preferences.insert(*women_reference_to_usize.get(woman).unwrap(), men_as_usize);
    }

    // men_preferences and women_preferences is HashMap<usize, Vec<usize>>
    let engagements_usize = gale_shapley_internal(&men_preferences, &women_preferences);

    // convert the resulting HashMap<usize, usize> to HashMap<T, Vec<T>>
    for (man_usize, woman_usize) in engagements_usize {
        let man = get_reference_from_usize(&men_reference_to_usize, man_usize).unwrap();
        let woman = get_reference_from_usize(&women_reference_to_usize, woman_usize).unwrap();
        engagements.insert(man, woman);
    }

    engagements
}

fn get_reference_from_usize<T>(references: &HashMap<T, usize>, value: usize) -> Option<T>
where
    T: Eq + Hash + Clone,
{
    references
        .iter()
        .find(|(_, &v)| v == value)
        .map(|(k, _)| k.to_owned())
}
/// You better go read the algorithm on wikipedia: https://en.wikipedia.org/wiki/Stable_marriage_problem
/// ranks are indexed from zero
/// TODO: Add validations for input
fn gale_shapley_internal(
    men_preferences: &HashMap<usize, Vec<usize>>,
    women_preferences: &HashMap<usize, Vec<usize>>,
) -> HashMap<usize, usize> {
    let mut men_preferences = men_preferences.clone();
    let mut engaged_man_woman: HashMap<usize, usize> = HashMap::new();

    while !get_unengaged_men(&men_preferences, &engaged_man_woman).is_empty() {
        play_round(
            &mut men_preferences,
            women_preferences,
            &mut engaged_man_woman,
        );
    }

    engaged_man_woman
}
/// 1. Find all un-engaged men
/// 2. Propose to the highest ranked woman
/// 3. For each woman, reject/engage all proposals
/// 4. update engaged_man_woman
fn play_round(
    men_preferences: &mut HashMap<usize, Vec<usize>>,
    women_preferences: &HashMap<usize, Vec<usize>>,
    engaged_man_woman: &mut HashMap<usize, usize>,
) {
    let unengaged_men = get_unengaged_men(men_preferences, engaged_man_woman);

    let proposals = create_proposals(men_preferences, unengaged_men);
    accept_or_reject_proposals(
        men_preferences,
        women_preferences,
        engaged_man_woman,
        proposals,
    );
}

fn get_unengaged_men(
    men_preferences: &HashMap<usize, Vec<usize>>,
    engaged_man_woman: &HashMap<usize, usize>,
) -> HashSet<usize> {
    men_preferences
        .keys()
        .filter(|man| engaged_man_woman.get(man).is_none())
        .cloned()
        .collect()
}

fn create_proposals(
    men_preferences: &HashMap<usize, Vec<usize>>,
    unengaged_men: HashSet<usize>,
) -> HashMap<usize, HashSet<usize>> {
    let mut proposals: HashMap<usize, HashSet<usize>> = HashMap::new();

    for man in unengaged_men {
        let preferred_woman = men_preferences.get(&man).unwrap()[0];
        proposals
            .entry(preferred_woman)
            .and_modify(|proposals| {
                proposals.insert(man);
            })
            .or_insert_with(|| [man].into_iter().collect());
    }

    proposals
}
/// Tentatively accepts proposals. The rejections are permanent. The `engaged_man_woman` HashMap represents an unstable engagement. It suddenly
/// becomes 'stable' (go read about gale-shapley to understand what stable means) in the final round, when everyone is engaged to someone
fn accept_or_reject_proposals(
    men_preferences: &mut HashMap<usize, Vec<usize>>,
    women_preferences: &HashMap<usize, Vec<usize>>,
    engaged_man_woman: &mut HashMap<usize, usize>,
    proposals: HashMap<usize, HashSet<usize>>,
) {
    for (woman, interested_men) in proposals {
        let best_interested_man =
            get_best_man_from_men_interested_in_a_woman(woman, women_preferences, &interested_men)
                .unwrap();
        if let Some(man_currently_engaged_to) = get_currently_engaged_man(engaged_man_woman, &woman)
        {
            let rank_best_interested_man =
                get_rank(women_preferences, &woman, &best_interested_man).unwrap();
            let rank_currently_engaged_man =
                get_rank(women_preferences, &woman, &man_currently_engaged_to).unwrap();
            if rank_best_interested_man < rank_currently_engaged_man {
                break_engagement(engaged_man_woman, man_currently_engaged_to);
                make_engagement(engaged_man_woman, best_interested_man, woman);
            }
        } else {
            make_engagement(engaged_man_woman, best_interested_man, woman);
        }

        for man in interested_men.iter() {
            men_preferences.get_mut(man).unwrap().remove(0);
        }
    }
}
/// TODO: We do not need the entire women_preferences here. Just the preferences of 'woman' would suffice
///
/// return None if Could not find best man from a set of interested men!
fn get_best_man_from_men_interested_in_a_woman(
    woman: usize,
    women_preferences: &HashMap<usize, Vec<usize>>,
    interested_men: &HashSet<usize>,
) -> Option<usize> {
    women_preferences
        .get(&woman)
        .unwrap()
        .iter()
        .find(|man| interested_men.contains(man))
        .copied()
}

fn get_rank(
    preferences: &HashMap<usize, Vec<usize>>,
    preferences_of: &usize,
    item: &usize,
) -> Option<usize> {
    preferences
        .get(preferences_of)
        .as_ref()?
        .iter()
        .position(|&v| v == *item)
        .map(|rank| rank as usize)
}

/// Returns the man a woman is currently engaged to
fn get_currently_engaged_man(
    engaged_man_woman: &HashMap<usize, usize>,
    woman: &usize,
) -> Option<usize> {
    engaged_man_woman
        .iter()
        .find(|(_, engaged_woman)| engaged_woman == &woman)
        .map(|(man, _)| man.to_owned())
}

fn make_engagement(engaged_man_woman: &mut HashMap<usize, usize>, man: usize, woman: usize) {
    engaged_man_woman.insert(man, woman);
}

fn break_engagement(engaged_man_woman: &mut HashMap<usize, usize>, man: usize) {
    engaged_man_woman.remove(&man);
}

#[cfg(test)]

mod tests {
    use super::*;
    // const sample_size = 5;

    fn get_preferences_config_1() -> HashMap<usize, Vec<usize>> {
        let mut preferences = HashMap::new();
        preferences.insert(0, vec![0, 1, 2, 3, 4]);
        preferences.insert(1, vec![4, 3, 2, 1, 0]);
        preferences.insert(2, vec![0, 1, 4, 2, 3]);
        preferences.insert(3, vec![2, 4, 3, 0, 1]);
        preferences.insert(4, vec![4, 0, 1, 3, 2]);

        preferences
    }

    fn get_preferences_config_2() -> HashMap<usize, Vec<usize>> {
        let mut preferences = HashMap::new();
        preferences.insert(0, vec![0, 1, 2, 3, 4]);
        preferences.insert(1, vec![1, 2, 4, 3, 0]);
        preferences.insert(2, vec![2, 4, 1, 0, 3]);
        preferences.insert(3, vec![0, 4, 3, 1, 2]);
        preferences.insert(4, vec![3, 0, 2, 4, 1]);

        preferences
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
        let unengaged_men: HashSet<usize> = [0, 1, 2, 3, 4].into_iter().collect();

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

        let proposals = create_proposals(&men_preferences, unengaged_men);

        assert_eq!(
            proposals.get(&0),
            Some(&vec![0, 1, 2, 3, 4].into_iter().collect())
        );
    }

    #[test]
    fn test_create_proposals_base_case_with_better_preferences() {
        // Testing the base case, i.e create proposals in the first round when no man is engaged to a woman
        let unengaged_men: HashSet<usize> = [0, 1, 2, 3, 4].iter().cloned().collect();
        let men_preferences = get_preferences_config_1();
        let proposals = create_proposals(&men_preferences, unengaged_men);

        assert_eq!(proposals.get(&0), Some(&vec![0, 2].into_iter().collect()));
        assert_eq!(proposals.get(&2), Some(&vec![3].into_iter().collect()));
        assert_eq!(proposals.get(&4), Some(&vec![1, 4].into_iter().collect()));
    }

    #[test]
    fn test_get_best_man_from_interested_men() {
        let women_preferences = get_preferences_config_2();
        let woman: usize = 1; // The second woman
        let interested_men: HashSet<usize> = [0, 1, 2, 3, 4].into_iter().collect();

        let best_man =
            get_best_man_from_men_interested_in_a_woman(woman, &women_preferences, &interested_men);
        assert_eq!(best_man, Some(1));

        let woman: usize = 2;
        let interested_men: HashSet<usize> = [0, 1, 2, 3, 4].into_iter().collect();

        let best_man =
            get_best_man_from_men_interested_in_a_woman(woman, &women_preferences, &interested_men);
        assert_eq!(best_man, Some(2));
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
        let mut engaged_man_woman: HashMap<usize, usize> = HashMap::new();

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
        let mut engaged_man_woman: HashMap<usize, usize> = HashMap::new();
        engaged_man_woman.insert(0, 0);
        engaged_man_woman.insert(1, 1);

        let woman = 0;
        let the_better_man = 2;
        make_engagement(&mut engaged_man_woman, the_better_man, woman);

        assert_eq!(engaged_man_woman.get(&the_better_man), Some(&0));
        assert_eq!(engaged_man_woman.get(&1), Some(&1));
    }

    #[test]
    fn test_accept_or_reject_proposals() {
        let mut men_preferences = get_preferences_config_1();
        let women_preferences = get_preferences_config_2();

        let mut engaged_man_woman: HashMap<usize, usize> = HashMap::new();
        let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);
        let proposals = create_proposals(&men_preferences, unengaged_men);
        println!("proposals: {:?}", proposals);
        // Proposals would be:
        // 0 - {0, 2}
        // 2 - {3}
        // 4 - {1, 4}
        accept_or_reject_proposals(
            &mut men_preferences,
            &women_preferences,
            &mut engaged_man_woman,
            proposals,
        );
        assert_eq!(engaged_man_woman.get(&0), Some(&0));
        assert_eq!(engaged_man_woman.get(&1), None);
        assert_eq!(engaged_man_woman.get(&2), None);
        assert_eq!(engaged_man_woman.get(&3), Some(&2));
        assert_eq!(engaged_man_woman.get(&4), Some(&4));
        println!("engaged: {:?}", engaged_man_woman);

        // Another round. Men 1 & 2
        let unengaged_men = get_unengaged_men(&men_preferences, &engaged_man_woman);
        let proposals = create_proposals(&men_preferences, unengaged_men);
        println!("proposals: {:?}", proposals);
        // proposals would be
        // 1 - {2}
        // 3 - {1}
        accept_or_reject_proposals(
            &mut men_preferences,
            &women_preferences,
            &mut engaged_man_woman,
            proposals,
        );
        println!("engaged: {:?}", engaged_man_woman);
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
