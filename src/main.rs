pub type Currency = u32;
pub type CurrencyType = String;
pub type ExchangeRate = u32;
pub type Rates = std::collections::hash_map::HashMap<CurrencyType, ExchangeRate>;

struct ExchangeRate() {
    base: CurrencyType,
    rate: Rates
}

pub struct ConversionRequest {
    from: CurrencyType,
    to: CurrencyType,
    sum: Currency
}

pub struct SuccessfulResponse {
    status: bool,
    error: String
}

pub struct ErrorResponse {
    status: bool, 
    error: String
}

pub type Response = std::result::Result<SuccessfulResponse, ErrorResponse>;

impl SuccessfulResponse {
    pub fn Construct(sum: u32) -> SuccessfulResponse {
        SuccessfulResponse (status: true, sum)
    }
}

pub struct CurrencyResponse {
    status: bool,
    data: Box<[CurrencyType]>
}

impl CurrencyResponse {
    pub fn construct(data: Box<[CurrencyType]>) -> CurrencyResponse {
        CurrencyResponse (status: true, data)
    }
}

impl ErrorResponse {
    fn construct(error: &str) -> ErrorResponse {
        ErrorResponse {
            status: false,
            error: error.to_string()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
