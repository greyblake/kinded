use kinded::Kinded;

#[derive(Kinded)]
enum Drink {
    Mate,
    Coffee(String),
    Tea { variety: String, caffeine: bool },
}

fn main() {
    // Create enum variants with associated data
    let espresso = Drink::Coffee("Espresso".to_owned());
    let green_tea = Drink::Tea {
        variety: "Sencha".to_owned(),
        caffeine: true,
    };
    let mate = Drink::Mate;

    // Use .kind() to get the kind without associated data
    assert_eq!(espresso.kind(), DrinkKind::Coffee);
    assert_eq!(green_tea.kind(), DrinkKind::Tea);
    assert_eq!(mate.kind(), DrinkKind::Mate);

    // Get all kind variants
    assert_eq!(DrinkKind::all(), [DrinkKind::Mate, DrinkKind::Coffee, DrinkKind::Tea]);

    // Pattern match on kind and access original data
    let description = match &espresso {
        Drink::Mate => "Just mate".to_owned(),
        Drink::Coffee(name) => format!("Coffee: {name}"),
        Drink::Tea { variety, caffeine } => {
            format!("Tea: {variety}, caffeine: {caffeine}")
        }
    };
    assert_eq!(description, "Coffee: Espresso");

    // Display trait
    assert_eq!(DrinkKind::Coffee.to_string(), "Coffee");
    assert_eq!(DrinkKind::Tea.to_string(), "Tea");
    assert_eq!(DrinkKind::Mate.to_string(), "Mate");

    // FromStr trait
    assert_eq!("Tea".parse::<DrinkKind>().unwrap(), DrinkKind::Tea);
    assert_eq!("Coffee".parse::<DrinkKind>().unwrap(), DrinkKind::Coffee);
    assert_eq!("Mate".parse::<DrinkKind>().unwrap(), DrinkKind::Mate);

    // FromStr is case-insensitive
    assert_eq!("tea".parse::<DrinkKind>().unwrap(), DrinkKind::Tea);
    assert_eq!("TEA".parse::<DrinkKind>().unwrap(), DrinkKind::Tea);

    // From trait - convert from Drink to DrinkKind
    assert_eq!(DrinkKind::from(&espresso), DrinkKind::Coffee);
    assert_eq!(DrinkKind::from(&green_tea), DrinkKind::Tea);
    assert_eq!(DrinkKind::from(&mate), DrinkKind::Mate);

    // Kind implements Copy, Clone, PartialEq, Eq, Debug
    let kind = DrinkKind::Coffee;
    let kind_copy = kind; // Copy
    let kind_clone = kind.clone(); // Clone
    assert_eq!(kind, kind_copy); // PartialEq
    assert_eq!(kind, kind_clone);
    assert_eq!(format!("{:?}", kind), "Coffee"); // Debug
}
