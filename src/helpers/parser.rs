use nom::{
    character::complete::one_of,
    combinator::{map_res, recognize},
    multi::many1,
    IResult,
};
use num::Num;

pub fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}

pub fn decimal_value<N: Num>(input: &str) -> IResult<&str, N> {
    map_res(decimal, |x| Num::from_str_radix(x, 10))(input)
}
