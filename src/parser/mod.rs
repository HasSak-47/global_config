pub mod preprocess;

#[derive(Debug)]
pub struct ParseError;

type Result<T> = core::result::Result<T, ParseError>;

impl From<std::io::Error> for ParseError{
    fn from(_: std::io::Error) -> Self {ParseError}
}

impl From<std::string::FromUtf8Error> for ParseError{
    fn from(_: std::string::FromUtf8Error) -> Self {ParseError}
}

#[derive(Debug)]
pub struct WorkPlace{
    pub data  : String,
    pub childs: Vec<usize>,
}

// wacky wrapper
#[derive(Debug)]
pub struct WorkPlaces(Vec<WorkPlace>);

pub fn load_worksplaces<S: AsRef<std::path::Path>>(path: S) -> Result<WorkPlaces>{
    let mut result = WorkPlaces{0: Vec::new()};

    let stri = preprocess::load_and_preprocess(path)?;

    let section_header = "-- [[section:";
    let headers = stri.matches(section_header);


    Ok(result)
}



