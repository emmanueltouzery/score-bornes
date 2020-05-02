use std::env;
type Coord = (u32, u32);

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct BoundingBox {
    top_left: Coord,
    bottom_right: Coord,
    points_count: u32,
}

impl BoundingBox {
    fn of_point(point: Coord) -> BoundingBox {
        BoundingBox {
            top_left: point,
            bottom_right: point,
            points_count: 1,
        }
    }

    fn center(&self) -> Coord {
        (
            self.top_left.0 + (self.bottom_right.0 - self.top_left.0) / 2,
            self.top_left.1 + (self.bottom_right.1 - self.top_left.1) / 2,
        )
    }

    // https://gamedev.stackexchange.com/a/44496
    fn sq_distance(&self, (px, py): Coord) -> u32 {
        let (x, y) = self.center();
        let x = x as i32;
        let y = y as i32;
        let width = (self.bottom_right.0 - self.top_left.0) as i32;
        let height = (self.bottom_right.1 - self.top_left.1) as i32;
        let dx = i32::max((px as i32 - x).abs() - width / 2, 0);
        let dy = i32::max((py as i32 - y).abs() - height / 2, 0);
        (dx * dx + dy * dy) as u32
    }

    fn add_point(&mut self, (x, y): Coord) {
        if x < self.top_left.0 {
            self.top_left.0 = x;
        } else if x > self.bottom_right.0 {
            self.bottom_right.0 = x;
        }
        if y > self.bottom_right.1 {
            self.bottom_right.1 = y;
        } else if y < self.top_left.1 {
            self.top_left.1 = y;
        }
        self.points_count += 1;
    }

    fn intersects(&self, other: BoundingBox) -> bool {
        // one rectangle is on left side of other
        if self.bottom_right.0 < other.top_left.0 || self.top_left.0 > other.bottom_right.0 {
            return false;
        }
        // one rectangle is above other
        if self.bottom_right.1 < other.top_left.1 || self.top_left.1 > self.bottom_right.1 {
            return false;
        }
        true
    }

    fn combine(&mut self, other: BoundingBox) {
        self.add_point(other.top_left);
        self.add_point(other.bottom_right);
    }
}

fn main() {
    let fname = env::args().skip(1).next().unwrap();
    let mut blue_pixels = vec![];
    let mut red_pixels = vec![];
    if let image::DynamicImage::ImageRgb8(mut img) =
        image::open(fname)
            .unwrap()
            .resize(300, 300, image::imageops::FilterType::Nearest)
    {
        for x in 0..img.width() {
            for y in 0..img.height() {
                let image::Rgb([r, g, b]) = img.get_pixel(x, y);
                if r < &150 && r + 4 < *g && g + 4 < *b {
                    blue_pixels.push((x, y));
                } else if *r > 140 && *r - 80 > *g && *r - 80 > *b {
                    println!("{} {} {}", r, g, b);
                    red_pixels.push((x, y));
                }
            }
        }
        println!("blue pixels: {}", blue_pixels.len());
        let blue_areas = pixels_find_contiguous_areas(&blue_pixels);
        for area in blue_areas {
            draw_area(&mut img, area, image::Rgb([0, 0, 255]));
        }
        println!("red pixels: {}", red_pixels.len());
        let red_areas = pixels_find_contiguous_areas(&red_pixels);
        for area in red_areas {
            draw_area(&mut img, area, image::Rgb([255, 0, 0]));
        }
        img.save("out.png").unwrap();
    } else {
        eprintln!("Image in wrong format!");
    }
}

fn draw_area(img: &mut image::RgbImage, area: BoundingBox, col: image::Rgb<u8>) {
    for x in area.top_left.0..area.bottom_right.0 {
        img.put_pixel(x, area.top_left.1, col);
        img.put_pixel(x, area.bottom_right.1, col);
    }
    for y in area.top_left.1..area.bottom_right.1 {
        img.put_pixel(area.top_left.0, y, col);
        img.put_pixel(area.bottom_right.0, y, col);
    }
}

// very much ugly brute force
fn pixels_find_contiguous_areas(pixels: &Vec<Coord>) -> Vec<BoundingBox> {
    let mut iter = pixels.iter();
    let fst = BoundingBox::of_point(*iter.next().unwrap());
    let areas = iter.fold(vec![fst], |mut acc, x| {
        match acc.iter_mut().find(|bbox| bbox.sq_distance(*x) < 6) {
            Some(bbox) => {
                bbox.add_point(*x);
                acc
            }
            None => {
                acc.push(BoundingBox::of_point(*x));
                acc
            }
        }
    });

    println!("areas: {}", areas.len());
    let large_areas: Vec<_> = areas.into_iter().filter(|a| a.points_count > 20).collect();
    println!("large blue areas: {:?}", large_areas);
    let lba_fst = *large_areas.first().unwrap();
    large_areas
        .into_iter()
        .skip(1)
        .fold(vec![lba_fst], |mut acc, x| {
            match acc.iter_mut().find(|a| a.intersects(x)) {
                Some(a) => {
                    a.combine(x);
                    acc
                }
                None => {
                    acc.push(x);
                    acc
                }
            }
        })
}

#[test]
fn bbox_inside() {
    let mut bb = BoundingBox::of_point((5, 5));
    bb.add_point((10, 10));
    assert_eq!(0, bb.sq_distance((6, 9)));
}

#[test]
fn bbox_near() {
    let mut bb = BoundingBox::of_point((5, 5));
    bb.add_point((10, 10));
    assert_eq!(8, bb.sq_distance((3, 3)));
}

#[test]
fn bbox_near2() {
    let mut bb = BoundingBox::of_point((5, 5));
    bb.add_point((10, 10));
    assert_eq!(4, bb.sq_distance((11, 5)));
}
