use rgb::RGB8;

struct Palette{
    breakpoints: Vec<RGB8>
}

impl Palette {
    fn new(breakpoints: Vec<RGB8>) -> Self {
	Palette {breakpoints: breakpoints}
    }

    fn color(&self, _percentage: usize) -> RGB8 {
	if self.breakpoints.len() == 1 {
	    return self.breakpoints[0];
	};

	let (a, b) = interval(&self.breakpoints, _percentage).unwrap();

	RGB8{r: (a.r + b.r) / 2,
	     g: (a.g + b.g) / 2,
	     b: (a.b + b.b) / 2}
    }
}

fn interval<T>(v: &Vec<T>, mut percentage: usize) -> Result<(&T, &T), ()> {
    if v.is_empty() || percentage > 100 {
    	return Err(());
    }
    
    if v.len() == 1 {
	Ok((&v[0], &v[0]))
    } else {
	// Little trick to close the last interval on the right
	if percentage == 100 {
	    percentage -= 1;
	}
	let intervals = v.len() - 1;

	let point = intervals * percentage / 100;
	dbg!(point);

	return Ok((&v[point], &v[point + 1]));
    }
}


#[cfg(test)]
mod test {
    use rgb::RGB8;
    use super::{Palette, interval};

    #[test]
    fn when_one_target_color_then_50_equal_target_color() {
	let target_color = RGB8{r:1, g:1, b:1};
	
	let palette = Palette::new(vec![target_color]);
	assert_eq!(palette.color(50), target_color);
    }

    #[test]
    fn when_two_equal_target_colors_then_50_equal_target_color() {
	let target_color = RGB8{r:1, g:1, b:1};
	
	let palette = Palette::new(vec![target_color, target_color]);
	assert_eq!(palette.color(50), target_color);
    }

    #[test]
    fn when_two_different_target_colors_then_50_equal_mean_color() {
	let target_color0 = RGB8{r:0, g:1, b:2};
	let target_color2 = RGB8{r:2, g:3, b:4};
	
	let palette = Palette::new(vec![target_color0, target_color2]);
	assert_eq!(palette.color(50), RGB8{r:1, g:2, b:3});
    }

    #[test]
    fn when_three_different_target_colors_then_50_equal_second_color() {
	let target_color0 = RGB8{r:0, g:0, b:0};
	let target_color1 = RGB8{r:1, g:1, b:1};
	let target_color2 = RGB8{r:2, g:2, b:2};
	
	let palette = Palette::new(vec![target_color0, target_color1, target_color2]);
	assert_eq!(palette.color(50), target_color1);
    }

    #[test]
    fn interval_when_one_point_then_50_equal_point() {
	let v = vec![0];
	
	assert_eq!(interval(&v, 50), Ok((&v[0], &v[0])));
    }

    #[test]
    fn interval_when_two_points_then_50_equal_two_points() {
	let v = vec![0, 1];
	
	assert_eq!(interval(&v, 50), Ok((&v[0], &v[1])));
    }

    #[test]
    fn interval_when_three_points_then_50_equal_second_interval() {
	let v = vec![0, 1, 2];
	
	assert_eq!(interval(&v, 50), Ok((&v[1], &v[2])));
    }

    #[test]
    // All intervals are [a, b) except the last one [n, 100]
    fn interval_when_three_points_then_100_equal_second_interval() {
	let v = vec![0, 1, 2];
	
	assert_eq!(interval(&v, 100), Ok((&v[1], &v[2])));
    }
    
    #[test]
    fn interval_when_four_points_then_75_equal_third_interval() {
	let v = vec![0, 1, 2, 3];
	
	assert_eq!(interval(&v, 75), Ok((&v[2], &v[3])));
    }
}
