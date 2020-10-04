use std::iter::FromIterator;

use gdnative::prelude::*;

use crate::db::DBMedium;

/// The Medium wrapper "class"
#[derive(NativeClass, Debug)]
#[inherit(Reference)]
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
    fn new(_owner: &Reference) -> Self {
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

    pub fn fill(&mut self, medium: DBMedium) {
        self.id = medium.id.into();
        self.isbn = medium.isbn.into();
        self.title = medium.title.into();
        self.publisher = medium.publisher.into();
        self.year = medium.year;
        self.costs = medium.costs;
        self.note = medium.note.into();
        self.borrowable = medium.borrowable;
        self.category = medium.category.into();
        self.authors = StringArray::from_iter(medium.authors.iter().map(|a| a.into()));
        self.borrower = medium.borrower.into();
        self.deadline = medium.deadline.into();
        self.reservation = medium.reservation.into();
    }

    pub fn db(&self) -> DBMedium {
        let mut authors = Vec::with_capacity(self.authors.len() as _);
        for i in 0..self.authors.len() {
            authors.push(self.authors.get(i).to_string());
        }
        DBMedium {
            id: self.id.to_string(),
            isbn: self.isbn.to_string(),
            title: self.title.to_string(),
            publisher: self.publisher.to_string(),
            year: self.year,
            costs: self.costs,
            note: self.note.to_string(),
            borrowable: self.borrowable,
            category: self.category.to_string(),
            authors,
            borrower: self.borrower.to_string(),
            deadline: self.deadline.to_string(),
            reservation: self.reservation.to_string(),
        }
    }

    #[export]
    pub fn list_item(&mut self, owner: &Reference) -> StringArray {
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
