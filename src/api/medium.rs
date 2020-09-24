use gdnative::prelude::*;

/// The Database wrapper "class"
#[derive(NativeClass, Debug)]
#[inherit(Object)]
pub struct Medium {
    #[property]
    pub id: GodotString,
    #[property]
    pub isbn: GodotString,
    #[property]
    pub title: GodotString,
    #[property]
    pub publisher: GodotString,
    #[property]
    pub year: i64,
    #[property]
    pub costs: f64,
    #[property]
    pub note: GodotString,
    #[property]
    pub borrowable: bool,
    #[property]
    pub category: GodotString,
    #[property]
    pub authors: StringArray,
    #[property]
    pub borrower: GodotString,
    #[property]
    pub deadline: GodotString,
    #[property]
    pub reservation: GodotString,
}

#[methods]
impl Medium {
    fn new(_owner: &Object) -> Self {
        Medium {
            id: GodotString::new(),
            isbn: GodotString::new(),
            title: GodotString::new(),
            publisher: GodotString::new(),
            year: 0,
            costs: 0.0,
            note: GodotString::new(),
            borrowable: true,
            category: GodotString::new(),
            authors: StringArray::new(),
            borrower: GodotString::new(),
            deadline: GodotString::new(),
            reservation: GodotString::new(),
        }
    }

    #[export]
    pub fn demo(&mut self, _owner: &Object) {
        self.id = "FANT ABNT 1".into();
        self.isbn = "123456789".into();
        self.title = "Into The Abyss".into();
        self.publisher = "Cruel World".into();
        self.year = 2020;
        self.costs = 9.99;
        self.note = "TEST".into();
        self.borrowable = true;
        self.category = "FANT".into();
        self.authors.push("Lars Wrenger".into());
        self.authors.push("Rek".into());
    }

    #[export]
    fn list_item(&mut self, owner: &Object) -> StringArray {
        StringArray::from_vec(vec![
            self.id.clone(),
            self.title.clone(),
            self.authors
                .read()
                .iter()
                .fold(String::new(), |acc, x| acc + &x.to_string())
                .into(),
            if !self.reservation.is_empty() {
                owner.tr(".medium.reserved")
            } else if !self.borrower.is_empty() {
                owner.tr(".medium.borrowed")
            } else {
                GodotString::new()
            },
        ])
    }
}
