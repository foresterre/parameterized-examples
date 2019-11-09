#![allow(unused)]

use parameterized::parameterized;

enum Fruit {
    Apple,
    Banana,
    Bramble(BrambleFruit),
    Citron,
    Melon(Box<dyn Seedless>),
    Pear,
}

trait NameOf {
    fn name_of(&self) -> &str;
}

impl NameOf for Fruit {
    fn name_of(&self) -> &str {
        match self {
            Fruit::Apple => "apple",
            Fruit::Banana => "banana",
            Fruit::Citron => "citron",
            Fruit::Bramble(fruit) => fruit.name_of(),
            Fruit::Melon(melon) => melon.name_of(),
            Fruit::Pear => "pear",
        }
    }
}

enum BrambleFruit {
    Blackberry,
}

impl NameOf for BrambleFruit {
    fn name_of(&self) -> &str {
        match self {
            BrambleFruit::Blackberry => "blackberry",
        }
    }
}

trait Seedless: NameOf {
    fn seedless(&self) -> bool;
}

struct FancyWaterMelon;

impl Seedless for FancyWaterMelon {
    fn seedless(&self) -> bool {
        true
    }
}

impl NameOf for FancyWaterMelon {
    fn name_of(&self) -> &str {
        "watermelon(seedless)"
    }
}

struct HomeGrownWaterMelon;

impl Seedless for HomeGrownWaterMelon {
    fn seedless(&self) -> bool {
        false
    }
}

impl NameOf for HomeGrownWaterMelon {
    fn name_of(&self) -> &str {
        "watermelon(with-seeds)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CITRON: Fruit = Fruit::Citron;

    #[parameterized(fruit = {
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Bramble(BrambleFruit::Blackberry),
        CITRON,
        Fruit::Melon(Box::new(FancyWaterMelon{})),
        Fruit::Melon(Box::new(HomeGrownWaterMelon{})),
        Fruit::Pear
    }, name = {
        "apple",
        "banana",
        "blackberry",
        "citron",
        "watermelon(seedless)",
        "watermelon(with-seeds)",
        "pear"
    })]
    fn a_fruity_test(fruit: Fruit, name: &str) {
        assert_eq!(fruit.name_of(), name)
    }
}
