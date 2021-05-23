use reqwest::blocking::get;
use crate::utils::ImportError;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

const API_URL: &str = "https://api.wikitree.com/api.php";
const PERSON_FIELDS: [&str; 15] = ["FirstName", "MiddleName", "LastNameAtBirth", "LastNameCurrent", "Nicknames", "LastNameOther", "RealName", "Prefix", "Suffix", "Gender", "BirthDate", "DeathDate", "BirthLocation", "DeathLocation", "IsLiving"];

pub struct WikiTreeSearchPersonRequest {
    FirstName: String,
    LastName: String,
    BirthDate: String,
    DeathDate: String,
    RealName: String,
    LastNameCurrent: String,
    BirthLocation: String,
    DeathLocation: String,
    Gender: String,
    fatherFirstName: String,
    fatherLastName: String,
    motherFirstName: String,
    motherLastName: String,
}

pub struct WikiTreeSearchPersonResult {
    Id: i64,
    Name: String,
    LastNameAtBirth: String,
    LastNameCurrent: String,
    LastNameOther: String,
    RealName: String,
    Prefix: String,
    Suffix: String,
    ShortName: String,
    Father: i64,
    Mother: i64,
}

#[derive(Serialize, Debug)]
pub struct WikiTreeGetPersonRequest<'a> {
    pub key: i64,
    fields: [&'a str; 15]
}

#[derive(Deserialize, Debug)]
pub struct WikiTreePerson {
    Id: i64,
    FirstName: String,
    MiddleName: String,
    LastNameAtBirth: String,
    LastNameCurrent: String,
    NickNames: String,
    LastNameOther: String,
    RealName: String,
    Prefix: String,
    Suffix: String,
    Gender: String,
    BirthDate: String,
    DeathDate: String,
    BirthLocation: String,
    DeathLocation: String,
    IsLiving: i64,
}

#[derive(Deserialize, Debug)]
struct WikiTreeGetPersonResult {
    user_id: i64,
    person: WikiTreePerson,
    status: i64
}

fn construct_url<T: Serialize>(action: &str, fields: T) -> Result<String, ImportError> {
    let fields_str = serde_urlencoded::to_string(fields)?;
    Ok(format!("{}?action={}&{}", API_URL, action, fields_str))
}

fn call_wikitree_api<T: DeserializeOwned>(url: &str) -> Result<T, ImportError> {
    let resp = get(url)?.error_for_status();
    match resp {
        Ok(body) => {
            let info: T = body.json()?;
            Ok(info)
        }
        Err(err) => {
            Err(ImportError::HTTPStatusError(String::from(url), err.status().unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR)))
        }
    }
}

pub fn search_person(person: &WikiTreeSearchPersonRequest) -> Result<Vec<WikiTreeSearchPersonResult>, ImportError> {

}

pub fn get_person(person_id: i64) -> Result<WikiTreePerson, ImportError> {
    let params = WikiTreeGetPersonRequest {
        key: person_id,
        fields: PERSON_FIELDS
    };
    let url = construct_url("getPerson", params)?;
    let results: Vec<WikiTreeGetPersonResult> = call_wikitree_api(&url)?;
    if results.len() > 0 {
        Ok(results[0].person)
    } else {
        Err(ImportError::NoMatchesError)
    }
}