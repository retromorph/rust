#![forbid(unsafe_code)]

pub fn flatten<const N: usize>(data: Vec<Box<[&mut i32; N]>>) -> Vec<&mut i32> {
    let mut result: Vec<&mut i32> = Vec::new();

    for line in data {
        for el in *line {
            result.push(el);
        }
    }

    result
}

pub fn transform_to_fixed_arr<const N: usize>(data: &mut Vec<Vec<i32>>) -> Vec<Box<[&mut i32; N]>> {
    let mut result: Vec<Box<[&mut i32; N]>> = Vec::new();

    for line in data {
        if line.len() != N {
            panic!("Inner vectors are of different size");
        }

        let mut tmp: Vec<&mut i32> = Vec::new();
        for el in line {
            tmp.push(el)
        }
        result.push(Box::new(tmp.try_into().unwrap()));
    }

    result
}
