const _blocks: [&str; 8] = [" ", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];
const full_block: &str = "█";
const empty_block: &str = " ";

fn bar<'a>(percentage: u8, total: usize) -> Result<String, &'a str> {
    if percentage > 100 {
	return Err("percentage larger than 100");
    }
    
    let fraction = total as f32 * percentage as f32 / 100f32;

    let filled = ((total as f32) * (percentage as f32 / 100f32)) as usize;

    let remainder = ((fraction - (filled as f32)) * 8f32) as usize;

    let result = full_block.repeat(filled) + _blocks[remainder] + &empty_block.repeat(total - filled - 1);

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
    fn when_half_and_size_one_then_left_half_block() -> Result<(), String> {
	let half_bar = bar(50, 1)?;

	assert_eq!("▌", half_bar);

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
