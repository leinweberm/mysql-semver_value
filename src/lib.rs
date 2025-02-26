use udf::prelude::*;

struct SemverValue;

#[register(name = "semver_value")]
impl BasicUdf for SemverValue {
    type Returns<'a> = Option<String>;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        init_check_args(args)?;
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let input_version = args.get(0).unwrap().value();
        let input_segments = args.get(1).unwrap().value();
        let version = input_version.as_string().unwrap();
        let segments_count = input_segments.as_int().unwrap();
        let mut result_string = String::new();

        if version.len() < 1 || version.len() > 44 || segments_count < 1 || segments_count > 4 {
            return Err(ProcessError);
        }

        let segments = parse_input_args(&version, segments_count);
        let u128_value = process_segments(segments);
        let u128_string = u128_value.to_string();
        let u128_max_string_length = u128::MAX.to_string().len();
        let u128_length_diff = u128_max_string_length - u128_string.len();

        let padded_u128_string = if u128_length_diff > 0 {
            format!("{:0>width$}", u128_string, width = u128_max_string_length)
        } else {
            u128_string
        };

        let u128_digit_chars =padded_u128_string.chars();

        for string_digit in u128_digit_chars {
            let digit = string_digit.to_digit(10);
            if digit.is_some() {
                let mapped_char = number_to_char(digit.unwrap());
                result_string.push(mapped_char);
            } else {
                result_string.push('A');
            }
        }

        result_string = result_string.chars().take(u128_max_string_length).collect::<String>();
        Ok(result_string.try_into().unwrap())
    }
}

fn init_check_args(args: &ArgList<Init>) -> Result<(), String> {
    let invalid_args_count_error = || {
        format!(
            "usage: semver_value(semver: str, segments: int8). Gor {} args",
            args.len()
        )
    };

    let (Some(mut version_arg), Some(mut segments_arg)) = (args.get(0), args.get(1)) else {
        return Err(invalid_args_count_error());
    };

    if args.len() > 2 {
        return Err(invalid_args_count_error());
    }

    version_arg.set_type_coercion(SqlType::String);
    segments_arg.set_type_coercion(SqlType::Int);

    Ok(())
}

fn number_to_char(n: u32) -> char {
    const MAP: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    if n < 10 {
        MAP[n as usize]
    } else {
        'A'
    }
}
fn process_segments(segments: Vec<String>) -> u128 {
    let mut numeric_semver: u128 = 0;

    for (_i, segment) in segments.iter().enumerate() {
        match segment.parse::<u128>() {
            Ok(value) => {
                numeric_semver = (numeric_semver << 32) | value
            }
            Err(_) => {
                numeric_semver = (numeric_semver << 32) | 0
            }
        }
    }

    numeric_semver
}

fn parse_input_args(version: &str, count: i64) -> Vec<String> {
    let mut segments: Vec<String> = version.split('.').map(|s| s.to_string()).collect();
    let length_diff: i64 = count - (segments.len() as i64);

    if length_diff > 0 {
        let zeros = vec!["0".to_string(); length_diff.try_into().unwrap()];
        segments.extend(zeros);
    } else if length_diff < 0 {
        segments.truncate(segments.len() - (length_diff.abs() as usize));
    }

    segments
}
