use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;

// We need discrete strength levels to compare hands, which will be a combination of:
// - overall strength (5, 4, Full, 3, 2 pair, 1 pair, high card)
// - card strength when overall strength is equated (A, K, Q ...)
pub fn solve1(input: Vec<String>) -> i32 {
    let mut hands: Vec<(String, i32, i32)> = Vec::new();
    let mut ret: i32 = 0;

    let card_strengths: HashMap<char, i32> = HashMap::from([
        ('A', 12), ('K', 11), ('Q', 10), ('J', 9), ('T', 8),
        ('9', 7), ('8', 6), ('7', 5), ('6', 4), ('5', 3),
        ('4', 2), ('3', 1), ('2', 0),
    ]);

    // Calculate overall hand strength
    for line in input {
        let hand = line.split_whitespace().nth(0).unwrap().to_string();
        let bid = line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
        
        let cards: Vec<char> = hand.chars().collect();
        let set: HashSet<_> = cards.clone().into_iter().collect();
        let mut strength: i32 = 0;

        // 5 of a kind
        if set.len() == 1 {
            strength = 0;
        }
        // 4 of a kind or Full house
        else if set.len() == 2 {
            let count = cards.clone().iter().filter(|&&c| c == cards[0]).count();
            // 4 of a kind
            if count == 1 || count == 4 {
                strength = 1;
            }
            // Full House
            else {
                strength = 2;
            }
        }
        // 3 of a kind or 2 pair
        else if set.len() == 3 {
            for c in &cards {
                let count = cards.clone().iter().filter(|x| x == &c).count();
                if count == 3 {
                    strength = 3;
                    break;
                }
                if count == 2 {
                    strength = 4;
                    break;
                }
            }
        }
        // 1 pair
        else if set.len() == 4 {
            strength = 5;
        }
        // High card
        else {
            strength = 6;
        }
        hands.push((hand, bid, strength))
    }

    // Sort each hand based on relative card strength
    hands.sort_by(|a, b| {
        let (hand_a, _, str_a) = a;
        let (hand_b, _, str_b) = b;

        if str_a < str_b {
            return Ordering::Greater;
        } else if str_a > str_b {
            return Ordering::Less;
        }

        // Iterate each char and compare the strengths
        for (char_a, char_b) in hand_a.chars().zip(hand_b.chars()) {
            match (card_strengths.get(&char_a), card_strengths.get(&char_b)) {
                (Some(strength_a), Some(strength_b)) => {
                    if strength_a > strength_b {
                        return Ordering::Greater;
                    } else if strength_a < strength_b {
                        return Ordering::Less;
                    }
                }
                _ => {}
            }
        }
        Ordering::Equal
    });

    // Add all bids multiplied by rank
    for (idx, hand) in hands.iter().enumerate() {
        let (_, bid, _) = hand;
        ret += (idx+1) as i32 * bid;
    }
    ret
}

// J is treated as the highest valuing for the overall strength, but lowest card for comparison purpose.
// Assumptions: Would a J always go to the highest count card (or highest current card if no counts > 1)? JJJJ2 -> 22222, JJJ2A -> AAA2A, A234J -> A234
pub fn solve2(input: Vec<String>) -> i32 {
    let mut hands: Vec<(String, i32, i32)> = Vec::new();
    let mut ret: i32 = 0;

    let card_strengths: HashMap<char, i32> = HashMap::from([
        ('A', 12), ('K', 11), ('Q', 10), ('J', -1), ('T', 8),
        ('9', 7), ('8', 6), ('7', 5), ('6', 4), ('5', 3),
        ('4', 2), ('3', 1), ('2', 0)
    ]);

    // Calculate overall hand strength
    for line in input {
        let hand = line.split_whitespace().nth(0).unwrap().to_string();
        let bid = line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
        
        let cards: Vec<char> = hand.chars().collect();
        let set: HashSet<_> = cards.clone().into_iter().collect();
        let mut strength: i32 = 0;

        // 5 of a kind
        if set.len() == 1 {
            strength = 0;
        }
        // 4 of a kind or Full house
        else if set.len() == 2 {
            let count = cards.clone().iter().filter(|&&c| c == cards[0]).count();
            // 4 of a kind
            if count == 1 || count == 4 {
                strength = 1;
            }
            // Full House
            else {
                strength = 2;
            }
            // If we have any amount of J's (1 or 4), then we can make 5OAK
            if cards.clone().contains(&'J') {
                strength = 0;
            }
        }
        // 3 of a kind or 2 pair
        else if set.len() == 3 {
            for c in &cards{
                let count = cards.clone().iter().filter(|x| x == &c).count();
                // If we have AAAJK -> 4OAK, JJJA2 -> 4OAK (4OAK > Fullhouse), AA22J -> Fullhouse, JJAA2 -> 4OAK
                if count == 3 {
                    strength = 3;
                    // If we have any J's, we can make a 4OAK
                    if cards.clone().contains(&'J') { strength = 1;}
                    break;
                }
                if count == 2 {
                    strength = 4;
                    // If we have 1 J, we can make fullhouse, if we have 2 J's, we can make 4OAK
                    let j_count = cards.clone().iter().filter(|&x| x == &'J').count();
                    if j_count == 1 { strength = 2;}
                    if j_count == 2 { strength = 1;}
                    break;
                }
            }
        }
        // 1 pair
        else if set.len() == 4 {
            strength = 5;
            // If we have any J's, we can make a 3OAK
            if cards.clone().contains(&'J') { strength = 3;}
        }
        // High card
        else {
            strength = 6;
            // If we have any J's, we can make a 2OAK
            if cards.clone().contains(&'J') { strength = 5;}
        }
        hands.push((hand, bid, strength))
    }

    // Sort each hand based on relative card strength
    hands.sort_by(|a, b| {
        let (hand_a, _, str_a) = a;
        let (hand_b, _, str_b) = b;

        if str_a < str_b {
            return Ordering::Greater;
        } else if str_a > str_b {
            return Ordering::Less;
        }

        // Iterate each char and compare the strengths
        for (char_a, char_b) in hand_a.chars().zip(hand_b.chars()) {
            match (card_strengths.get(&char_a), card_strengths.get(&char_b)) {
                (Some(strength_a), Some(strength_b)) => {
                    if strength_a > strength_b {
                        return Ordering::Greater;
                    } else if strength_a < strength_b {
                        return Ordering::Less;
                    }
                }
                _ => {}
            }
        }
        Ordering::Equal
    });

    // Add all bids multiplied by rank
    for (idx, hand) in hands.iter().enumerate() {
        let (_, bid, _) = hand;
        ret += (idx+1) as i32 * bid;
    }
    ret
}









