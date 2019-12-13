use rgb::RGB8;

struct _Palette{
    breakpoints: Vec<RGB8>
}

fn _interval<T>(v: &Vec<T>, percentage: usize) -> Result<(&T, &T), ()> {
    // if v.is_empty() || percentage > 100 {
    // 	return Err(());
    // }
    
    if v.len() == 1 {
	Ok((&v[0], &v[0]))
    } else {
	let _interval_length = 100f64 / (v.len() as f64);
	
	let point = v.len() * 100 / percentage;

	return Ok((&v[point], &v[point + 1]));
    }
}

impl _Palette {
    fn _new(breakpoints: Vec<RGB8>) -> Self {
	_Palette {breakpoints: breakpoints}
    }

    fn _color(&self, _percentage: u8) -> RGB8 {
	if self.breakpoints.len() == 1 {
	    self.breakpoints[0]
	}
	else if self.breakpoints.len() == 2{
	    let rgb0 = self.breakpoints[0];
	    let diff = self.breakpoints[1] - self.breakpoints[0];

	    RGB8{r: rgb0.r + diff.r / 2,
		 g: rgb0.g + diff.g / 2,
		 b: rgb0.b + diff.b / 2}
	} else {
	    RGB8{r: 0, g: 0, b: 0}
	}
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
	assert_eq!(palette.color(50), RGB8{r:1, g:1, b:1});
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
    
}
