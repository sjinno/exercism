pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    } else if size == 1 {
        return vec![vec![1]];
    }

    let s = size as usize;

    let mut mtx = vec![vec![0; s]; s];
    let mut num = 1..;

    let mut low = 0;
    let mut up = s - 1;

    while low <= up {
        let (mut sr, mut er) = (low, up);
        let (mut sc, mut ec) = (low, up);

        while sc != up {
            mtx[low][sc] = num.next().unwrap();
            sc += 1;
        }

        while sr != up {
            mtx[sr][up] = num.next().unwrap();
            sr += 1;
        }

        while ec != low {
            mtx[up][ec] = num.next().unwrap();
            ec -= 1;
        }

        while er != low {
            mtx[er][low] = num.next().unwrap();
            er -= 1;
        }

        low += 1;
        up -= 1;

        if low == up {
            mtx[low][up] = num.next().unwrap();
        }
    }

    mtx
}
