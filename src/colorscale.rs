use rgb::RGB8;

struct Palette{
    breakpoints: Vec<RGB8>
}

impl Palette {
    fn new(breakpoints: Vec<RGB8>) -> Self {
	Palette {breakpoints: breakpoints}
    }

    fn color(&self, percentage: u8) -> RGB8 {
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
    use super::Palette;

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
}
