use kinded::Kinded;

#[allow(dead_code)]
#[derive(Kinded)]
enum Beverage {
    Mate,
    Coffee(String),
    Tea { variety: String, caffeine: bool },
}

fn main() {
    // Mate
    {
        let drink = Beverage::Mate;
        assert_eq!(drink.kind(), BeverageKind::Mate);
    }

    // Coffee
    {
        let drink = Beverage::Coffee("Espresso".to_owned());
        assert_eq!(drink.kind(), BeverageKind::Coffee);
    }

    // Tea
    {
        let drink = Beverage::Tea {
            variety: "Green".to_owned(),
            caffeine: true,
        };
        assert_eq!(drink.kind(), BeverageKind::Tea);
    }
}
