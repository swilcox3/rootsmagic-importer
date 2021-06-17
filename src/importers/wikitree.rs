use crate::utils::ImportError;
use reqwest::blocking::get;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const API_URL: &str = "https://api.wikitree.com/api.php";
const PERSON_FIELDS: &str = "Id,FirstName,MiddleName,Nicknames,LastNameAtBirth,LastNameCurrent,Nicknames,LastNameOther,RealName,Prefix,Suffix,Gender,BirthDate,DeathDate,BirthLocation,DeathLocation,IsLiving";

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Default)]
pub struct WikiTreeSearchPersonParams<'a> {
    pub FirstName: Option<String>,
    pub LastName: Option<String>,
    BirthDate: Option<String>,
    DeathDate: Option<String>,
    RealName: Option<String>,
    LastNameCurrent: Option<String>,
    BirthLocation: Option<String>,
    DeathLocation: Option<String>,
    Gender: Option<String>,
    fatherFirstName: Option<String>,
    fatherLastName: Option<String>,
    motherFirstName: Option<String>,
    motherLastName: Option<String>,
    fields: &'a str,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct WikiTreeSearchPersonFrame {
    status: i64,
    matches: Vec<WikiTreePerson>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct WikiTreeGetPersonRequest<'a> {
    pub key: i64,
    fields: &'a str,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct WikiTreePerson {
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

pub fn search_person(
    mut person: WikiTreeSearchPersonParams,
) -> Result<Vec<WikiTreePerson>, ImportError> {
    person.fields = PERSON_FIELDS;
    let url = construct_url("searchPerson", person)?;
    let mut results: Vec<WikiTreeSearchPersonFrame> = call_wikitree_api(&url)?;
    Ok(results.remove(0).matches)
}

pub fn get_person(person_id: i64) -> Result<WikiTreePerson, ImportError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_pauline() {
		let mut search = WikiTreeSearchPersonParams::default();
        search.FirstName = Some(String::from("Pauline"));
        search.LastName = Some(String::from("Winkel"));
        let mut results = search_person(search).unwrap();
        let first = results.remove(0);
        println!("{:?}", results);
    }
}
