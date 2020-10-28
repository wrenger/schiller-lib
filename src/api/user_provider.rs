use gdnative::prelude::*;

use crate::api;
use crate::provider::user::{UserData, CSV};

struct Delimiter(u8);

impl FromVariant for Delimiter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let string = String::from_variant(variant)?;
        if string.chars().count() == 1 && string.is_ascii() {
            Ok(Delimiter(string.chars().next().unwrap() as u32 as u8))
        } else {
            Err(FromVariantError::InvalidLength {
                len: string.len(),
                expected: 1,
            })
        }
    }
}

impl ToVariant for Delimiter {
    #[inline]
    fn to_variant(&self) -> Variant {
        String::from_utf8_lossy(&[self.0]).to_variant()
    }
}

impl nativescript::Export for Delimiter {
    type Hint = nativescript::property::StringHint;
    #[inline]
    fn export_info(_: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::GodotString)
    }
}

/// The UserCSVProvider wrapper "class"
#[derive(NativeClass)]
#[inherit(Reference)]
#[register_with(Self::register)]
pub struct UserCSVProvider {
    provider: CSV,
}

#[methods]
impl UserCSVProvider {
    fn new(_owner: &Reference) -> Self {
        UserCSVProvider {
            provider: CSV::default(),
        }
    }

    #[export]
    fn request(&self, _owner: &Reference, account: String) -> api::Result<UserData> {
        let account = account.trim();
        if !account.is_empty() {
            self.provider.request(account)
        } else {
            Err(api::Error::InvalidArguments)
        }
    }

    #[export]
    fn bulk_request(
        &self,
        _owner: &Reference,
        accounts: Vec<String>,
    ) -> api::Result<Vec<UserData>> {
        let accounts: Vec<&str> = accounts.iter().map(|a| a.trim()).collect();
        self.provider.bulk_request(&accounts)
    }

    // Properties

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<GodotString>("path")
            .with_getter(Self::get_path)
            .with_setter(Self::set_path)
            .done();
        builder
            .add_property::<Delimiter>("delimiter")
            .with_getter(Self::get_delimiter)
            .with_setter(Self::set_delimiter)
            .done();
        builder
            .add_property::<i64>("column_account")
            .with_getter(Self::get_column_account)
            .with_setter(Self::set_column_account)
            .done();
        builder
            .add_property::<i64>("column_forename")
            .with_getter(Self::get_column_forename)
            .with_setter(Self::set_column_forename)
            .done();
        builder
            .add_property::<i64>("column_surname")
            .with_getter(Self::get_column_surname)
            .with_setter(Self::set_column_surname)
            .done();
        builder
            .add_property::<i64>("column_role")
            .with_getter(Self::get_column_role)
            .with_setter(Self::set_column_role)
            .done();
    }

    fn get_path(&self, _owner: TRef<Reference>) -> GodotString {
        GodotString::from_str(&self.provider.path)
    }

    fn set_path(&mut self, _owner: TRef<Reference>, path: GodotString) {
        self.provider.path = path.to_string();
    }

    fn get_delimiter(&self, _owner: TRef<Reference>) -> Delimiter {
        Delimiter(self.provider.delimiter)
    }

    fn set_delimiter(&mut self, _owner: TRef<Reference>, delimiter: Delimiter) {
        self.provider.delimiter = delimiter.0;
    }

    fn get_column_account(&self, _owner: TRef<Reference>) -> i64 {
        self.provider.column_account as _
    }

    fn set_column_account(&mut self, _owner: TRef<Reference>, column_account: i64) {
        self.provider.column_account = column_account as _;
    }

    fn get_column_forename(&self, _owner: TRef<Reference>) -> i64 {
        self.provider.column_forename as _
    }

    fn set_column_forename(&mut self, _owner: TRef<Reference>, column_forename: i64) {
        self.provider.column_forename = column_forename as _;
    }

    fn get_column_surname(&self, _owner: TRef<Reference>) -> i64 {
        self.provider.column_surname as _
    }

    fn set_column_surname(&mut self, _owner: TRef<Reference>, column_surname: i64) {
        self.provider.column_surname = column_surname as _;
    }

    fn get_column_role(&self, _owner: TRef<Reference>) -> i64 {
        self.provider.column_role as _
    }

    fn set_column_role(&mut self, _owner: TRef<Reference>, column_role: i64) {
        self.provider.column_role = column_role as _;
    }
}
