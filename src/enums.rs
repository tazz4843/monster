use rand::Rng;

pub enum DoorTypes {
    ClosedShortDoor,
    OpenShortDoor,
    ClosedSquareTallDoor,
    OpenSquareTallDoor,
    ClosedRoundTallDoor,
    OpenRoundTallDoor,
}

pub enum PropTypes {
    SmallChair,
    LargePinkChair,
    LargeRedChair,
    LargeBlueChair,

    SmallTable,
    LargeTable,
}

pub enum WallType {
    TallUpperLeftCornerWall,
    TallUpperRightCornerWall,
    TallLowerRightCornerWall,
    TallLowerLeftCornerWall,
    TallVerticalWall,
    TallLeftRightWall,
    TallTopBottomWall,
    TallLeftWall,
    TallRightWall,
    TallWall,

    ShortLeftWall,
    ShortRightWall,
    ShortWall,
    ShortLeftRightWall,

    Floor,
}

pub enum CarpetType {
    NoWalls,

    LeftWall,
    RightWall,
    TopWall,
    BottomWall,

    LeftTopWall,
    LeftBottomWall,

    RightTopWall,
    RightBottomWall,

    LeftRightWall,

    TopBottomWall,

    UpperRightCornerWall,
    UpperLeftCornerWall,
    BottomRightCornerWall,
    BottomLeftCornerWall,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum PersonType {
    /// Looks like a gnome
    Gnome(GnomeType),
    /// Looks like a cat/human mix
    CatKid(CatKidType),
    /// Looks like a baby chick
    Chick(ChickType),
    /// Looks like a duck/human mix
    DuckBoy(DuckType),
    /// Looks like a fox
    Fox,
    /// White person wearing tuxedo
    Person1,
    /// White person with long hair in sweater
    Person2,
    /// White person with short hair in sweater
    Person3,
    /// Brown person with short hair in t-shirt
    Person4,
    /// Black person with long hair in suit
    Person5,
    /// White person with short hair in sweater
    Person6,
    /// White person with long hair in suit
    Person7,
    /// White person with no hair in sweater
    Person8,
    /// Tan person with long hair in suit
    Person9,
    /// Tan person with short hair in sweater
    Person10,
    /// Brown person with short hair in sweater
    Person11,
    /// White person with long hair in sweater
    Person12,
    /// Brown person with long hair in overalls
    Person13,
    /// White person with long hair in sweater
    Person14,
    /// Tan person with long hair in sweater
    Person15,
}

pub fn rand_person<R: Rng + ?Sized>(rng: &mut R, colored: bool) -> PersonType {
    match rng.gen_range(1..=15) {
        1 => PersonType::Person1,
        2 => PersonType::Person2,
        3 => PersonType::Person3,
        4 => match colored {
            true => PersonType::Person4,
            false => PersonType::Person2,
        },
        5 => match colored {
            true => PersonType::Person5,
            false => PersonType::Person3,
        },
        6 => PersonType::Person6,
        7 => PersonType::Person7,
        8 => PersonType::Person8,
        9 => match colored {
            true => PersonType::Person9,
            false => PersonType::Person6,
        },
        10 => match colored {
            true => PersonType::Person10,
            false => PersonType::Person8,
        },
        11 => match colored {
            true => PersonType::Person11,
            false => PersonType::Person7,
        },
        12 => PersonType::Person12,
        13 => match colored {
            true => PersonType::Person13,
            false => PersonType::Person12,
        },
        14 => PersonType::Person14,
        15 => match colored {
            true => PersonType::Person15,
            false => PersonType::Person14,
        },
        _ => PersonType::Fox,
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum GnomeType {
    NoType,
    WalkingForward1,
    WalkingForward2,
    WalkingBackward1,
    WalkingBackward2,
    WalkingRight,
    IdleRight,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum CatKidType {
    NoType,
    WalkingForward1,
    WalkingForward2,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum ChickType {
    NoType,
    WalkingForward1,
    WalkingForward2,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum DuckType {
    NoType,
    WalkingForward1,
    WalkingForward2,
}
