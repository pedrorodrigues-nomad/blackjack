#[cfg(test)]
mod tests {
    use crate::{buy_card, card_value, create_deck, player_score, score_table};

    #[test]
    fn should_create_deck() {
        let result = create_deck(4);
        assert_eq!(result.len(), 208)
    }

    #[test]
    fn should_buy_card() {
        let mut deck = Vec::new();
        let mut hand = Vec::new();
        let result = buy_card(&mut deck, &mut hand);

        assert_eq!(result, ())
    }

    #[test]
    fn should_card_value() {
        let result = card_value(10);
        assert_eq!(result, 10)
    }

    #[test]
    fn should_player_score() {
        let hand = Vec::new();
        let result = player_score(hand);
        assert_eq!(result, 0)
    }

    #[test]
    fn should_score_table() {
        let hand = Vec::new();
        let result = score_table(hand);
        assert_eq!(result, 0)
    }
}
