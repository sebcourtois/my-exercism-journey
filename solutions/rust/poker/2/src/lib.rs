use std::collections::HashMap;
use std::fmt;
use RankingCategory::*;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Card {
    pub rank: u8,
    suit: char,
    display_string: String,
}

impl Card {
    pub fn from_string(input: &str) -> Self {
        let suit = input.chars().last().unwrap();
        let rank = Self::rank_from_str(input.strip_suffix(suit).unwrap());

        Card { rank, suit, display_string: input.to_string() }
    }

    pub fn new(rank: u8, suit: char) -> Self {
        let mut string = Self::string_from_rank(rank).unwrap();
        string.push(suit);

        Card { rank, suit, display_string: string }
    }

    pub fn string_from_rank(rank: u8) -> Result<String, String> {
        match rank {
            2..=10 => Ok(format!("{}", rank)),
            11 => Ok("J".to_string()),
            12 => Ok("Q".to_string()),
            13 => Ok("K".to_string()),
            1 | 14 => Ok("A".to_string()),
            _ => Err(format!("Invalid rank number: '{}'", rank))
        }
    }

    pub fn rank_from_str(input: &str) -> u8 {
        let faces = ["J", "Q", "K", "A"];
        let mut rank: u8;

        if faces.contains(&input) {
            rank = faces.iter().enumerate().find(|(_i, &f)| f == input).unwrap().0 as u8;
            rank += 1 + 10;
        } else {
            rank = input.parse().unwrap();
        }
        rank
    }
}

impl fmt::Display for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.display_string)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RankingCategory {
    HighestCard = 0,
    OnePair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    Full,
    Quads,
    StraightFlush,
}

pub struct Hand {
    display_string: String,
    sorted_cards: Vec<Card>,
}

impl Hand {
    pub fn from_string(input: &str) -> Self {
        let input_cards: Vec<Card> = input.split_whitespace().map(Card::from_string).collect();

        let mut cards_by_rank = Vec::from_iter(
            Hand::group_cards_by_rank(input_cards.iter())
        );
        cards_by_rank.sort_by_key(|(rank, cards)|
            (cards.len() as u32 * 100_u32) + rank.to_owned() as u32
        );
        let mut sorted_cards: Vec<Card> = cards_by_rank.into_iter()
            .flat_map(|(_r, cards)| cards)
            .cloned().collect();

        if Self::ace_can_start_a_straight_low(&sorted_cards) {
            let high_ace = sorted_cards.last().unwrap();
            let low_ace = Card::new(1, high_ace.suit);
            sorted_cards = [low_ace].iter().chain(sorted_cards[..4].iter()).cloned().collect();
        }

        Hand {
            display_string: input.to_string(),
            sorted_cards,
        }
        // let cards: String = sorted_cards.iter().map(|c| format!("{} ", c)).collect();
        // println!("sorted:{:?}:{}", &hand.ranking_category.unwrap(), &cards);
    }

    pub fn ace_can_start_a_straight_low(sorted_cards: &[Card]) -> bool {
        let sorted_ranks: Vec<u8> = sorted_cards.iter()
            .map(|c| c.rank)
            .collect();

        sorted_ranks == vec![2_u8, 3, 4, 5, 14]
    }

    pub fn ranking(&self) -> u32 {
        let cards_score: u32 = self.sorted_cards.iter()
            .enumerate()
            .map(|(i, card)| card.rank as u32 * 10_u32.pow(i as u32))
            .sum();

        let category_score = self.ranking_category() as u32 * 10_u32.pow(6_u32);
        cards_score + category_score
    }

    fn ranking_category(&self) -> RankingCategory {
        let a_straight = self.a_straight();
        let a_flush = self.a_flush();

        if a_straight && a_flush { return StraightFlush; }
        if a_flush { return Flush; }
        if a_straight { return Straight; }
        if self.a_quads() { return Quads; }
        if self.a_full() { return Full; }
        if self.a_trips() { return Trips; }
        if self.a_two_pair() { return TwoPair; }
        if self.a_one_pair() { return OnePair; }

        HighestCard
    }

    pub fn a_straight(&self) -> bool {
        let sorted_ranks: Vec<u8> = self.sorted_cards.iter()
            .map(|c| c.rank)
            .collect();

        let highest_rank = *sorted_ranks.last().unwrap();
        if highest_rank < 5 {
            return false;
        };
        let straight_ranks = Vec::from_iter((highest_rank + 1 - 5)..highest_rank + 1);

        sorted_ranks == straight_ranks
    }

    pub fn a_flush(&self) -> bool {
        for suit in "SHDC".chars() {
            if self.sorted_cards.iter().all(|c| c.suit == suit) {
                return true;
            }
        }
        false
    }

    pub fn a_full(&self) -> bool {
        let mut count_by_rank = Vec::from_iter(
            self.count_cards_by_rank().into_values()
        );
        count_by_rank.sort_unstable();

        count_by_rank == vec![2_u8, 3]
    }

    pub fn a_quads(&self) -> bool {
        let mut count_by_rank = Vec::from_iter(
            self.count_cards_by_rank().into_values()
        );
        count_by_rank.sort_unstable();

        count_by_rank == vec![1_u8, 4]
    }

    pub fn a_trips(&self) -> bool {
        let mut count_by_rank = Vec::from_iter(
            self.count_cards_by_rank().into_values()
        );
        count_by_rank.sort_unstable();

        count_by_rank == vec![1_u8, 1, 3]
    }

    pub fn a_two_pair(&self) -> bool {
        let mut count_by_rank = Vec::from_iter(
            self.count_cards_by_rank().into_values()
        );
        count_by_rank.sort_unstable();

        count_by_rank == vec![1_u8, 2, 2]
    }

    pub fn a_one_pair(&self) -> bool {
        let mut count_by_rank = Vec::from_iter(
            self.count_cards_by_rank().into_values()
        );
        count_by_rank.sort_unstable();

        count_by_rank == vec![1_u8, 1, 1, 2]
    }

    pub fn group_cards_by_rank<'a, I>(cards: I) -> HashMap<u8, Vec<&'a Card>>
        where I: IntoIterator<Item=&'a Card>
    {
        let mut cards_by_rank: HashMap<u8, Vec<&Card>> = HashMap::new();
        for card in cards {
            cards_by_rank
                .entry(card.rank)
                .or_default()
                .push(card);
        }
        cards_by_rank
    }

    pub fn count_cards_by_rank(&self) -> HashMap<u8, u8> {
        let mut count_by_rank: HashMap<u8, u8> = HashMap::new();

        for card in self.sorted_cards.iter() {
            count_by_rank.entry(card.rank)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        count_by_rank
    }

    pub fn highest_card(&self) -> &Card {
        self.sorted_cards.last().unwrap()
    }

    pub fn some_card_of_rank(&self, rank: u8) -> Option<&Card> {
        self.sorted_cards.iter().find(|&c| c.rank == rank)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let cards: String = self.sorted_cards.iter().map(|c| format!("{} ", c)).collect();
        write!(formatter, "{}", cards.trim())
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let ranked_hands: Vec<Hand> = hands.iter()
        .map(|hand_str| Hand::from_string(hand_str))
        .collect();

    let mut hands_by_ranking: HashMap<u32, Vec<&Hand>> = HashMap::new();
    for hand in ranked_hands.iter() {
        let ranking = hand.ranking();
        hands_by_ranking.entry(ranking)
            .or_default()
            .push(hand);
        println!("{0} : {1:?} = {2}", hand, hand.ranking_category(), ranking);
    }

    let best_hands = hands_by_ranking.iter()
        .max_by_key(|(ranking, _)| *ranking).unwrap().1;

    let best_hands: Vec<&str> = best_hands.iter()
        .map(|h| h.display_string.as_str())
        .collect();

    let best_hands: Vec<&'a str> = hands.iter()
        .cloned()
        .filter(|h| best_hands.contains(h))
        .collect();

    best_hands
}