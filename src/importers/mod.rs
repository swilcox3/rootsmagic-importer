use crate::utils::*;

pub mod wikitree;

#[derive(Debug, Clone)]
pub enum Gender {
    Unknown,
    Female,
    Male,
    Other,
}

#[derive(Debug)]
pub struct Search {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub birth_location: Option<String>,
    pub death_location: Option<String>,
    pub gender: Gender,
    pub father_first_name: Option<String>,
    pub father_last_name: Option<String>,
    pub mother_first_name: Option<String>,
    pub mother_last_name: Option<String>,
}

impl Search {
    pub fn new(
        first_name: Option<String>,
        last_name: Option<String>,
        birth_date: Option<String>,
        death_date: Option<String>,
        birth_location: Option<String>,
        death_location: Option<String>,
        gender: Gender,
        father_first_name: Option<String>,
        father_last_name: Option<String>,
        mother_first_name: Option<String>,
        mother_last_name: Option<String>,
    ) -> Search {
        Search {
            first_name,
            last_name,
            birth_date,
            death_date,
            birth_location,
            death_location,
            gender,
            father_first_name,
            father_last_name,
            mother_first_name,
            mother_last_name,
        }
    }
}

pub trait Person: std::fmt::Debug {
    fn get_id(&self) -> String;
    fn first_name(&self) -> &Option<String>;
    fn middle_name(&self) -> &Option<String>;
    fn last_name_at_birth(&self) -> &Option<String>;
    fn last_name_current(&self) -> &Option<String>;
    fn nicknames(&self) -> &Option<String>;
    fn last_name_other(&self) -> &Option<String>;
    fn real_name(&self) -> &Option<String>;
    fn prefix(&self) -> &Option<String>;
    fn suffix(&self) -> &Option<String>;
    fn gender(&self) -> Gender;
    fn birth_date(&self) -> &Option<String>;
    fn death_date(&self) -> &Option<String>;
    fn birth_location(&self) -> &Option<String>;
    fn death_location(&self) -> &Option<String>;
    fn is_living(&self) -> bool;
}

pub trait ImportSource {
    fn search_person(&self, search: Search) -> Result<Vec<Box<dyn Person>>, ImportError>;
    fn get_person(&self, id: String) -> Result<Box<dyn Person>, ImportError>;
}
