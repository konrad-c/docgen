#[derive(Clone,Debug)]
pub enum NameType {
    First,
    Last,
    Full
}

#[derive(Clone,Debug)]
pub enum LocationType {
    Place,
    Street,
    Address
}

#[derive(Clone,Debug)]
pub enum PhoneType {
    Mobile,
    Landline,
    Any
}

#[derive(Clone,Debug)]
pub enum DistributionType {
    Normal
}

#[derive(Clone,Debug)]
pub enum PlaceholderType {
    Name(NameType),
    Location(LocationType),
    Phone(PhoneType),
    Distribution(DistributionType),
    Guid,
    Float,
    Int,
    Set
}

// ------------------------------------------

#[derive(Clone,Debug)]
pub enum PlaceholderArgs {
    Float { min: f64, max: f64 },
    Int { min: i64, max: i64 },
    IntRepeated { min: i64, max: i64, repeat: u64 },
    Set { options: Vec<String> },
    Normal { mean: f64, stddev: f64 }
}