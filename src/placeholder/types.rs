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