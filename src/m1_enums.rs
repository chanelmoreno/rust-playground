#[derive(Debug)] // by putting this here, CarColour is debuggable?
#[allow(dead_code)]
enum CarColour {
    Red,
    Green,
    Black,
    White,
    Blue,
    Silver,
}

// Generics Enum - i.e. Generic Types
#[derive(Debug)]
#[allow(dead_code)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
#[allow(dead_code)]
enum GivenOption<T> {
    None,
    Some(T),
}

#[allow(dead_code)]
fn create_car_colour_blue() -> CarColour {
    // let my_car_colour: CarColour = CarColour::Blue;
    // my_car_colour

    CarColour::Blue //can also return like this
                    // the above is the sanme as "return Blue", when there is no ; it is a return
}

#[allow(dead_code)]
// Result Enum
// When you want to return ok or error
fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

#[allow(dead_code)]
fn check_under_five_built_in_result_enum(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not under 5!".to_string())
    }
}

#[allow(dead_code)]
// Option Enums
// Option is for when you either want to return something (some) or Nothing (none)
fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

#[allow(dead_code)]
//Option Enum - Built In Function as part of rust
fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour: CarColour = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_result: GivenResult<u8, String> = check_under_five(10);
        dbg!(under_five_result);

        let under_five_result: GivenResult<u8, String> = check_under_five(2);
        dbg!(under_five_result);

        let check_under_five_built_in_result_enum_result: Result<u8, String> =
            check_under_five_built_in_result_enum(10);
        dbg!(check_under_five_built_in_result_enum_result);

        let check_under_five_built_in_result_enum_result: Result<u8, String> =
            check_under_five_built_in_result_enum(2);
        dbg!(check_under_five_built_in_result_enum_result);

        let remainder: GivenOption<f32> = remainder_zero(12.2);
        dbg!(remainder);

        let remainder: GivenOption<f32> = remainder_zero(10.0);
        dbg!(remainder);

        let remainder_built_in_result: Option<f32> = remainder_zero_built_in(12.2);
        dbg!(remainder_built_in_result);

        let remainder_built_in_result: Option<f32> = remainder_zero_built_in(10.0);
        dbg!(remainder_built_in_result);
    }
}
