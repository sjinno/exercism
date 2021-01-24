pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let (mut low, mut up) = (0, array.len() - 1);

    while low < up {
        let mid = (low + up) / 2;
        let num = array[mid];

        if num == key {
            return Some(mid);
        }

        if num < key {
            low = mid + 1;
        } else if num > key {
            if mid == 0 {
                break;
            }
            up = mid - 1;
        }
    }

    if array[low] == key {
        return Some(low);
    }

    None
}
