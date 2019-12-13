use std::fmt::{Display, Formatter};

const BLOCKS: [&str; 8] = [" ", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];
const FULL_BLOCK: &str = "█";
const EMPTY_BLOCK: &str = " ";

pub enum GaugeError {
    InvalidPercentage
}

impl Display for GaugeError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
	write!(f, "{}",
	       match self {
		   GaugeError::InvalidPercentage => "invalid percentage",
	       }
	)
    }
}

pub fn bar<'a>(percentage: u8, total: usize) -> Result<String, GaugeError> {
    if percentage > 100 {
	return Err(GaugeError::InvalidPercentage);
    }
    
    let fraction = total as f32 * percentage as f32 / 100f32;

    let filled = fraction.floor() as usize;

    let remainder = ((fraction - fraction.floor()) * 8f32) as usize;

    let result;
    
    if percentage == 100 {
    	result = FULL_BLOCK.repeat(filled as usize) + &EMPTY_BLOCK.repeat(total - filled);
    }else {
    	result = FULL_BLOCK.repeat(filled) + BLOCKS[remainder] + &EMPTY_BLOCK.repeat(total - filled - 1);
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::bar;
    
    #[test]
    fn when_empty_and_size_ten_then_ten_empty_chars() -> Result<(), String> {
	let empty_bar = bar(0, 10)?;
	
	assert_eq!("          ", empty_bar);

	Ok(())
    }

    #[test]
    fn when_half_and_size_six_then_three_empty_chars() -> Result<(), String> {
	let half_bar = bar(50, 6)?;

	assert_eq!("███   ", half_bar);

	Ok(())
    }

    #[test]
    fn when_three_quarters_and_size_four_then_three_full_chars() -> Result<(), String> {
	let three_quarter_bar = bar(75, 4)?;

	assert_eq!("███ ", three_quarter_bar);

	Ok(())
    }

    #[test]
    fn when_half_and_size_one_then_left_half_block() -> Result<(), String> {
	let half_bar = bar(50, 1)?;

	assert_eq!("▌", half_bar);

	Ok(())
    }

    #[test]
    fn when_full_and_size_one_then_full_block() -> Result<(), String> {
	let full_bar = bar(100, 1)?;

	assert_eq!("█", full_bar);

	Ok(())
    }

    #[test]
    fn when_full_and_size_two_then_two_full_blocks() -> Result<(), String> {
	let full_bar = bar(100, 2)?;

	assert_eq!("██", full_bar);

	Ok(())
    }
    
    #[test]
    fn when_thirty_two_and_size_four_then_one_full_and_one_quarter() -> Result<(), String> {
	let half_bar = bar(32, 4)?;

	assert_eq!("█▎  ", half_bar);

	Ok(())
    }

    
    #[test]
    fn when_filled_larger_than_hundred_then_is_err() {
	assert!(bar(105, 3).is_err());
    }
}
