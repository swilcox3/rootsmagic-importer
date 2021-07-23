use super::*;
use crate::utils::ImportError;
use reqwest::blocking::get;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const API_URL: &str = "https://api.wikitree.com/api.php";
const PERSON_FIELDS: &str = "Id,FirstName,MiddleName,Nicknames,LastNameAtBirth,LastNameCurrent,Nicknames,LastNameOther,RealName,Prefix,Suffix,Gender,BirthDate,DeathDate,BirthLocation,DeathLocation,IsLiving";

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
struct WikiTreeSearchPerson {
    FirstName: Option<String>,
    LastName: Option<String>,
    BirthDate: Option<String>,
    DeathDate: Option<String>,
    BirthLocation: Option<String>,
    DeathLocation: Option<String>,
    Gender: Option<String>,
    fatherFirstName: Option<String>,
    fatherLastName: Option<String>,
    motherFirstName: Option<String>,
    motherLastName: Option<String>,
}

impl std::convert::From<Search> for WikiTreeSearchPerson {
    fn from(search: Search) -> Self {
        let gender = match search.gender {
            Gender::Female => Some(String::from("female")),
            Gender::Male => Some(String::from("male")),
            Gender::Other => Some(String::from("male")),
            Gender::Unknown => None,
        };

        WikiTreeSearchPerson {
            FirstName: search.first_name,
            LastName: search.last_name,
            BirthDate: search.birth_date,
            DeathDate: search.death_date,
            BirthLocation: search.birth_location,
            DeathLocation: search.death_location,
            Gender: gender,
            fatherFirstName: search.father_first_name,
            fatherLastName: search.father_last_name,
            motherFirstName: search.mother_first_name,
            motherLastName: search.mother_last_name,
        }
    }
}

#[derive(Serialize, Debug, Default)]
struct WikiTreeSearchPersonRequest {
    #[serde(flatten)]
    person: WikiTreeSearchPerson,
    fields: &'static str,
}

impl WikiTreeSearchPersonRequest {
    fn new(person: WikiTreeSearchPerson) -> WikiTreeSearchPersonRequest {
        WikiTreeSearchPersonRequest {
            person,
            fields: PERSON_FIELDS,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct WikiTreeSearchPersonResult {
    status: i64,
    matches: Vec<WikiTreePerson>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct WikiTreeGetPersonRequest {
    pub key: i64,
    fields: &'static str,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct WikiTreePerson {
    Id: i64,
    FirstName: Option<String>,
    MiddleName: Option<String>,
    LastNameAtBirth: Option<String>,
    LastNameCurrent: Option<String>,
    Nicknames: Option<String>,
    LastNameOther: Option<String>,
    RealName: Option<String>,
    Prefix: Option<String>,
    Suffix: Option<String>,
    Gender: Option<String>,
    BirthDate: Option<String>,
    DeathDate: Option<String>,
    BirthLocation: Option<String>,
    DeathLocation: Option<String>,
    IsLiving: i64,
}

impl Person for WikiTreePerson {
    fn get_id(&self) -> String {
        format!("{}", self.Id)
    }
    fn first_name(&self) -> &Option<String> {
        &self.FirstName
    }
    fn middle_name(&self) -> &Option<String> {
        &self.MiddleName
    }
    fn last_name_at_birth(&self) -> &Option<String> {
        &self.LastNameAtBirth
    }
    fn last_name_current(&self) -> &Option<String> {
        &self.LastNameCurrent
    }
    fn nicknames(&self) -> &Option<String> {
        &self.Nicknames
    }
    fn last_name_other(&self) -> &Option<String> {
        &self.LastNameOther
    }
    fn real_name(&self) -> &Option<String> {
        &self.RealName
    }
    fn prefix(&self) -> &Option<String> {
        &self.Prefix
    }
    fn suffix(&self) -> &Option<String> {
        &self.Suffix
    }
    fn gender(&self) -> Gender {
        match &self.Gender {
            Some(gender) => {
                if gender == "male" {
                    Gender::Male
                } else if gender == "female" {
                    Gender::Female
                } else {
                    Gender::Other
                }
            }
            None => Gender::Unknown,
        }
    }
    fn birth_date(&self) -> &Option<String> {
        &self.BirthDate
    }
    fn death_date(&self) -> &Option<String> {
        &self.DeathDate
    }
    fn birth_location(&self) -> &Option<String> {
        &self.BirthLocation
    }
    fn death_location(&self) -> &Option<String> {
        &self.DeathLocation
    }
    fn is_living(&self) -> bool {
        if self.IsLiving == 0 {
            false
        } else {
            true
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct WikiTreeGetPersonResult {
    user_id: i64,
    person: WikiTreePerson,
    status: i64,
}

fn construct_url<T: Serialize>(action: &str, fields: T) -> Result<String, ImportError> {
    let fields_str = serde_urlencoded::to_string(fields)?;
    Ok(format!("{}?action={}&{}", API_URL, action, fields_str))
}

fn call_wikitree_api<T: DeserializeOwned>(url: &str) -> Result<T, ImportError> {
    let resp = get(url)?.error_for_status();
    match resp {
        Ok(body) => {
            let text = body.text()?;
            println!("{:?}", text);
            let info: T = serde_json::from_str(&text)?;
            Ok(info)
        }
        Err(err) => Err(ImportError::HTTPStatusError(
            String::from(url),
            err.status()
                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
        )),
    }
}

fn search_person(request: WikiTreeSearchPersonRequest) -> Result<Vec<WikiTreePerson>, ImportError> {
    let url = construct_url("searchPerson", request)?;
    let mut results: Vec<WikiTreeSearchPersonResult> = call_wikitree_api(&url)?;
    Ok(results.remove(0).matches)
}

fn get_person(person_id: i64) -> Result<WikiTreePerson, ImportError> {
    let params = WikiTreeGetPersonRequest {
        key: person_id,
        fields: PERSON_FIELDS,
    };
    let url = construct_url("getPerson", params)?;
    let mut results: Vec<WikiTreeGetPersonResult> = call_wikitree_api(&url)?;
    if results.len() > 0 {
        Ok(results.remove(0).person)
    } else {
        Err(ImportError::NoMatchesError)
    }
}

pub struct WikiTreeImporter;

impl WikiTreeImporter {
    pub fn new() -> WikiTreeImporter {
        WikiTreeImporter {}
    }
}

impl ImportSource for WikiTreeImporter {
    fn search_person(&self, search: Search) -> Result<Vec<Box<dyn Person>>, ImportError> {
        let person = WikiTreeSearchPerson::from(search);
        let request = WikiTreeSearchPersonRequest::new(person);
        let results = search_person(request)?;
        let mut boxed_results = Vec::new();
        for result_person in results {
            boxed_results.push(Box::new(result_person) as Box<dyn Person>);
        }
        Ok(boxed_results)
    }

    fn get_person(&self, id: String) -> Result<Box<dyn Person>, ImportError> {
        let id_num = id.parse().unwrap(); //Safe because we get the id directly off the return for search
        let result = get_person(id_num)?;
        Ok(Box::new(result) as Box<dyn Person>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_pauline() {
        let mut search = WikiTreeSearchPerson::default();
        search.FirstName = Some(String::from("Pauline"));
        search.LastName = Some(String::from("Winkel"));
        let mut results = search_person(WikiTreeSearchPersonRequest::new(search)).unwrap();
        let first = results.remove(0);
        println!("{:?}", first);
    }
}
