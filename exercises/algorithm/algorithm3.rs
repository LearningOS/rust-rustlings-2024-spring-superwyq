/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T:std::cmp::PartialOrd>(array: &mut [T]){
	//TODO
    if array.len() <= 1 {
        return;
    }
    let mut now_minimum_index = 0;
    for i in 0..array.len() {
        now_minimum_index = i;
        for j in i+1..array.len() {
            if array[j] < array[now_minimum_index] {
                now_minimum_index = j;
            }
        }
        if now_minimum_index != i {
            array.swap(i, now_minimum_index);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}