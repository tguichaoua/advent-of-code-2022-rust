use nom::{
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many1, many_m_n},
    sequence::preceded,
    IResult,
};
use num::Num;

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(preceded(
        many_m_n(0, 1, char('-')),
        many1(one_of("0123456789")),
    ))(input)
}

pub fn decimal_value<N: Num>(input: &str) -> IResult<&str, N> {
    map_res(decimal, |x| Num::from_str_radix(x, 10))(input)
}
