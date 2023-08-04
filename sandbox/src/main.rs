use kinded::Kinded;

#[allow(dead_code)]
#[derive(Kinded)]
enum Drink {
    Mate,
    Coffee(String),
    Tea { variety: String, caffeine: bool },
}

fn main() {
    // Mate
    {
        let drink = Drink::Mate;
        assert_eq!(drink.kind(), DrinkKind::Mate);
    }

    // Coffee
    {
        let drink = Drink::Coffee("Espresso".to_owned());
        assert_eq!(drink.kind(), DrinkKind::Coffee);
    }

    // Tea
    {
        let drink = Drink::Tea {
            variety: "Green".to_owned(),
            caffeine: true,
        };
        assert_eq!(drink.kind(), DrinkKind::Tea);
    }
}
