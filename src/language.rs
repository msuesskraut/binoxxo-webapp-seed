use std::collections::HashMap;
use std::hash::Hash;

struct Translator<L> {
    translations: HashMap<L, HashMap<String, String>>,
}

impl<L: Hash + Eq> Translator<L> {
    pub fn new() -> Translator<L> {
        Translator {
            translations : HashMap::new()
        }
    }

    pub fn add<S1: Into<String>, S2: Into<String>>(&mut self, language: L, id: S1, text: S2) {
        let texts = self.translations.entry(language).or_insert_with(|| HashMap::new());
        texts.insert(id.into(), text.into());
    }

    pub fn get<S: Into<String>>(&self, language: L, id: S) -> &String {
        self.translations.get(&language).unwrap().get(&id.into()).unwrap()
    }
}

mod test {
    use super::*;

    #[derive(PartialEq, Eq, Hash)]
    enum Languages { De, En }
    
    #[test]
    fn simple_translate() {
        use Languages::*;

        let mut langs = Translator::new();
        
        langs.add(En, "hi", "Hello World");
        langs.add(De, "hi", "Hallo Welt");

        assert_eq!("Hello World", langs.get(En, "hi"));
        assert_eq!("Hallo Welt", langs.get(De, "hi"));
    }
}