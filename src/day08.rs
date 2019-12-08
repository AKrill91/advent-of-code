#[derive(Eq, PartialEq, Debug)]
struct SpaceImage {
    width: usize,
    height: usize,
    layers: Vec<SpaceImageLayer>
}

#[derive(Eq, PartialEq, Debug)]
struct SpaceImageLayer {
    width: usize,
    height: usize,
    data: Vec<i32>
}

impl SpaceImageLayer {
    pub fn num_occurences(&self, search: i32) -> i32 {
        let mut count = 0;

        for i in &self.data {
            if *i == search {
                count += 1;
            }
        }

        count
    }
}

pub fn run_a(input: &Vec<String>, width: usize, height: usize) -> i32 {
    let image = parse_image(input, width, height);

    let mut min_layer = None;
    let mut min_zeroes = std::i32::MAX;

    for layer in image.layers {
        let num_zeroes = layer.num_occurences(0);
        if num_zeroes < min_zeroes {
            min_layer = Some(layer);
            min_zeroes = num_zeroes;
        }
    }

    assert!(min_layer.is_some());

    let layer = min_layer.unwrap();

    let num_ones = layer.num_occurences(1);
    let num_twos = layer.num_occurences(2);

    num_ones * num_twos
}

pub fn run_b(input: &Vec<String>) -> i32 {
    0
}

fn parse_image(input: &Vec<String>, width: usize, height: usize) -> SpaceImage {
    let size = width * height;
    let data_str: &str = &input[0];
    let num_layers = data_str.len() / size;

    let mut layers = Vec::with_capacity(num_layers);

    let mut position = 0;

    for i in 0..num_layers {
        let mut data = Vec::with_capacity(size);

        for pos in 0..size {
            let c = &data_str[position..position+1];
            data.push(c.parse::<i32>().unwrap());
            position += 1;
        }

        layers.push(SpaceImageLayer {
            width,
            height,
            data
        });
    }

    SpaceImage {
        width,
        height,
        layers
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_parse() {
        let input = vec![String::from("123456789012")];
        let width = 3;
        let height = 2;

        let expected = SpaceImage {
            width,
            height,
            layers: vec![
                SpaceImageLayer {
                    width,
                    height,
                    data: vec![1,2,3,4,5,6]
                },
                SpaceImageLayer {
                    width,
                    height,
                    data: vec![7,8,9,0,1,2]
                }
            ]
        };

        let actual = parse_image(&input, width, height);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("123456789012")];
        let width = 3;
        let height = 2;

        assert_eq!(1, run_a(&input, width, height));
    }

}