enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Self { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        use Card::*;

        let mut final_value = 0;
        let mut aces_seen = 0;

        for card in self.cards.iter() {
            match card {
                Two     => { final_value += 2 },
                Three   => { final_value += 3 },
                Four    => { final_value += 4 },
                Five    => { final_value += 5 },
                Six     => { final_value += 6 },
                Seven   => { final_value += 7 },
                Eight   => { final_value += 8 },
                Nine    => { final_value += 9 },
                Ten | Jack | Queen | King => { final_value += 10 },
                Ace => { aces_seen += 1 },
            };
        }

        for i in 1..aces_seen {
            if final_value + 11 < 21 {
                final_value += 11;
            } else {
                final_value += 1;
            }
        }

        final_value
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_ace() {
        let hand = Hand { cards: vec![Card::Two, Card::Queen] };
        assert_eq!(hand.value(), 12);
    }

    #[test]
    fn with_ace_lower_21() {
        let hand = Hand { cards: vec![Card::Ace, Card::Five] };
        assert_eq!(hand.value(), 16);
    }

    #[test]
    fn with_ace_higher_21() {
        let hand = Hand { cards: vec![Card::Ace, Card::Queen, Card::Ten, Card::King] };
        assert_eq!(hand.value(), 31);
    }
}
