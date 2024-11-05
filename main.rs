// Array sorter

fn sort_array(mut array: [i32; 10]) -> [i32; 10] {
    for _ in 0..array.len() - 1 {
        for i in 0..array.len() - 1 {
            let current_number: i32 = array[i];
            let next_number: i32 = array[i + 1];
            if current_number > next_number {
                array[i + 1] = current_number;
                array[i] = next_number;
            }
        }
    }
    array
}

fn main() {
    let array = [10, 4, 1, 7, 9, 2, 5, 3, 6, 8];
    println!("{:?}", sort_array(array));
}