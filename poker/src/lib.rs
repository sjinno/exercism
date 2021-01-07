use std::cmp::Ordering;

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

#[derive(Debug, Eq)]
struct Card {
    number: u8,
    suit: String,
}

type Hand = Vec<Card>;

trait Rank {
    fn straight_flush(&self) -> bool;
    fn four_of_a_kind(&self) -> bool;
    fn full_house(&self) -> bool;
    fn flush(&self) -> bool;
    fn straight(&self) -> bool;
    fn three_of_a_kind(&self) -> bool;
    fn two_pair(&self) -> bool;
    fn one_pair(&self) -> bool;
    fn high_card(&self) -> bool;
}

impl Rank for Hand {
    fn straight_flush(&self) -> bool {
        let mut increment_counter = self[0].number;
        for hand in self {
            if &hand.number != &increment_counter {
                return false;
            }
            increment_counter += 1;
        }
        true
    }
    fn four_of_a_kind(&self) -> bool {
        false
    }
    fn full_house(&self) -> bool {
        false
    }
    fn flush(&self) -> bool {
        false
    }
    fn straight(&self) -> bool {
        false
    }
    fn three_of_a_kind(&self) -> bool {
        false
    }
    fn two_pair(&self) -> bool {
        false
    }
    fn one_pair(&self) -> bool {
        false
    }
    fn high_card(&self) -> bool {
        false
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let v_hands = hands.to_vec();
    let num_of_hands = v_hands.len();
    if num_of_hands == 1 {
        return Some(v_hands);
    }
    // println!("{:?}", &v_hands);
    let haands = organize_and_sort_hands(hands);
    println!("{:#?}", haands);

    for (i, hand) in haands.iter().enumerate() {
        // println!("{}", hand.straight_flush());
        if hand.straight_flush() {
            return Some(vec![v_hands[i]]);
        }
    }

    None
}

fn organize_and_sort_hands<'a>(hands: &[&'a str]) -> Vec<Vec<Card>> {
    let mut cards = Vec::<Vec<Card>>::new();
    for hand in hands {
        let hand_into_cards = hand.split(" ").collect::<Vec<_>>();
        let mut tmp = Vec::<Card>::new();
        hand_into_cards.iter().for_each(|c| {
            let card;
            let mut card_in_chars = c.chars();
            if c.len() == 3 {
                card = Card {
                    number: 10,
                    suit: card_in_chars.last().unwrap().to_string(),
                };
            } else {
                let num = card_in_chars.next().unwrap();
                let num = match num {
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 1,
                    _ => num as u8 - 48,
                };
                let suit = card_in_chars.next().unwrap().to_string();
                card = Card {
                    number: num,
                    suit: suit,
                };
            }
            tmp.push(card);
        });
        tmp.sort_by(|a, b| a.number.cmp(&b.number));
        cards.push(tmp);
    }
    cards
}
