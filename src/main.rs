use nom::{
    branch::*, bytes::complete::*, character::complete::multispace0,
    character::complete::u32 as p_u32, combinator::*, sequence::*, Parser,
};
use nom::character::complete::hex_digit1;


fn main() {

    let  hex_u32 = 
        map_opt(
          preceded(alt((tag("0x"), tag("0X"))), recognize(hex_digit1)),
          |x| u32::from_str_radix(x, 16).ok(),
        );

    let mut parse =  all_consuming(hex_u32);
  
    let r = match parse("0x00FF") {
      Ok(x) => x,
      Err::<_, nom::Err<nom::error::Error<&str>>>(_)  => ("FAIL",0)
    };
    println!("Hello, world! {} remainder: '{}''",r.1,r.0);
}
