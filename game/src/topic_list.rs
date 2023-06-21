use std::collections::{BTreeSet, HashMap};

pub struct TopicList (HashMap::<String, BTreeSet::<String>>);
impl TopicList {
    pub fn get(&self, key: &str) -> Option<&BTreeSet<String>> {
        self.0.get(key)
    }
    pub fn insert_item_in_topic(&mut self, key:&str, value: String) {

        if let Some(topic) = self.0.get_mut(key) {
            topic.insert(value);
            return;
            
        } else {
            let mut topic = BTreeSet::new();
            topic.insert(value);
            self.0.insert(key.to_string(), topic);
            return;
        };
    }
    pub fn add_topic(&mut self, topic: String, items: BTreeSet::<String>) -> Option<BTreeSet::<String>>{
        self.0.insert(topic, items)
    }
    pub fn remove_item(&mut self, topic: &str, item: &str) -> bool {
        let Some(thing) = self.0.get_mut(topic) else {
            return false;
        };
        thing.remove(item)
    }
    pub fn remove_topic(&mut self, topic: &str) -> Option<BTreeSet<String>> {
        self.0.remove(topic)
    }
    pub fn new()->Self {
        Self(HashMap::new())
    }

}