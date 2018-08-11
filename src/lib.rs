extern crate chrono;
#[macro_use]
extern crate lazy_static;

//a module containing all the parts of jikan
pub mod prelude {

}

//a module specifically for all the parts of the library focusing on the
//Japanese era calendar
pub mod jpn {
    mod date;
    mod era;
    mod period;
    
    pub use self::period::Period;
    pub use self::date::Date;
    pub use self::era::{Era, Court, EraYear};
}
