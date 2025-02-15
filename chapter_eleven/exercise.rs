use std::collections::HashMap;

struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn main() {
    fruit_basket();

    let mut fruit_inventory = HashMap::new();
    fruit_data(&mut fruit_inventory);
    println!("{:#?}", fruit_inventory);

    let results = "".to_string()
        + "England,France,4,2\n"
        + "France,Italy,3,1\n"
        + "Poland,Spain,2,0\n"
        + "Germany,England,2,1\n";

    println!("{:?}", results);
    build_scores_table(results);
}

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("pineapple"), 2);
    basket.insert(String::from("strawberry"), 2);
    basket.insert(String::from("peach"), 2);
    println!("{:?}", basket);
    basket
}

fn fruit_data(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        basket.entry(fruit).or_insert(1);
    }
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        let t1_score = scores.entry(team_1_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        t1_score.goals_scored += team_1_score;
        t1_score.goals_conceded += team_2_score;

        let t2_score = scores.entry(team_2_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        t2_score.goals_scored += team_2_score;
        t2_score.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't modify this function!
    fn get_fruit_data() -> HashMap<Fruit, u32> {
        let mut data = HashMap::<Fruit, u32>::new();
        data.insert(Fruit::Apple, 4);
        data.insert(Fruit::Mango, 2);
        data.insert(Fruit::Lychee, 5);

        data
    }

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut data = get_fruit_data();
        fruit_data(&mut data);
        assert_eq!(*data.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*data.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*data.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut data = get_fruit_data();
        fruit_data(&mut data);
        let count_fruit_kinds = data.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut data = get_fruit_data();
        fruit_data(&mut data);
        let count = data.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let mut data = get_fruit_data();
        fruit_data(&mut data);
        for amount in data.values() {
            assert_ne!(amount, &0);
        }
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
