use std::{collections::{HashMap, hash_map::Keys}, fs};

use serde::{Deserialize, Serialize};

use fyrox::{
    core::{reflect::Reflect, visitor::{Visit, VisitResult, Visitor}, reflect::prelude::*,},
};



#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct Scripts(HashMap::<String, Vec::<ScriptItem>>);
impl Scripts {
    pub fn get(&self, key: &str) -> Option<&Vec<ScriptItem>> {
        self.0.get(key)
    }
    pub fn get_blurp(&self, key: &str, index: usize) -> Option<&ScriptItem> {
        self.0.get(key)?.get(index)
    }
    pub fn keys(&self) -> Keys<'_, String, Vec<ScriptItem>> {
        self.0.keys()
    }
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct Action {
    action: String
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct AddQuest {
    add_quest_script: String,
	add_quest_name: String,
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct Animation {
    animation: String,
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct Blurp {
    character: String,
    text: String,
    annotation: Option<String>,
    wait: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct Cue {
    cue: String
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct Choice {
    choice: Vec<ChoiceOption>
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct ChoiceOption {
    jump: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct End {
    end: String
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct Jump {
    jump: String
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect, Default)]
pub struct OfferTopics {
    offer_topics: String,
    exit: bool
}

#[derive(Serialize, Deserialize, Debug, Visit, Reflect)]
#[serde(untagged)]
pub enum ScriptItem {
    Action(Action),
    AddQuest(AddQuest),
    Animation(Animation),
    Blurp(Blurp),
    Choice(Choice),
    Cue(Cue),
    End(End),
    Jump(Jump),
    OfferTopics(OfferTopics),
}
impl Default for ScriptItem {
    fn default() -> Self {
        Self::End(End::default())
    }
}

pub fn load_from_file(file_path : &str) -> Result<Scripts, String>{
    let data = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(err) => {
            return Err(format!("{err:?}"));
        }
    };
    
    match serde_jsonrc::from_str::<Scripts>(&data) {
        Ok(script) => {
            return Ok(script);
        }
        Err(err) => {
            return Err(format!("{err:?}"));
        }
    };
}