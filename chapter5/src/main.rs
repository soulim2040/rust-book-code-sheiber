pub mod hashmap;
fn sequential_search(nums : &[i32], num : i32) -> bool {
    let mut pos = 0;
    let mut found = false;
    
    while pos < nums.len() && !found {
        if nums[pos] == num {
            found = true;
        }else{
            found = false;
            pos += 1;
        }
    }
    return found;
}

fn binary_search1(nums : &[i32], num : i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        let mid : usize = (low + high) >> 1;

        if num == nums[mid] {
            found = true;
        }else if num < nums[mid] {
            high = mid - 1;
        }else{
            low = mid + 1;
        }
    }
    found
}

fn binary_search2(nums : &[i32], num : i32) -> bool {
    if 0 == nums.len() { return false;}

    let mid = nums.len() >> 1;

    if num == nums[mid] {
        return true;
    }else if num < nums[mid] {
        return binary_search2(&nums[..mid], num);
    }else {
        return binary_search2(&nums[mid+1..], num);
    }
}

//这个函数不对
fn interpolation_search(nums : &[i32], num : i32) -> bool {
    if nums.is_empty() { return false;}

    let mut low: usize = 0;
    let mut high = nums.len() - 1;

    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        if high_val <= low_val || num < low_val || num > high_val {
            break;
        }

        let offset = (num - low_val) * (high - low) as i32 
                    / (high_val - low_val);

        let interpolant = low + offset as usize;

        if nums[interpolant] > num {
            high = interpolant -1;
        }else if nums[interpolant] < num {
            low = interpolant + 1;
        }else{
            break;
        }
    }

    if num == nums[high]{
        true
    }else {
        false
    }
}

fn exponential_search(nums : &[i32], num: i32) -> bool {
    let size = nums.len();
    if size == 0 { return false;}

    let mut high : usize = 1;
    while high < size && nums[high] < num {
        high <<= 1;
    }

    let low = high >> 1;

    binary_search1(&nums[low..(size.min(high+1))], num)
}

fn main() {
    //let nums = [11, 22, 33];
    //println!("{}", sequential_search(&nums, 22));

    //let nums = [ 1 , 3 , 8 , 10 , 15 , 32 , 44 , 48 , 50 , 55 , 60 , 62, 64];
    //let ret = exponential_search(&nums, 16);
    //println!("{}", ret);
    let mut hashMap = hashmap::HashMap::new();

    hashMap.insert(11, "dog");
    hashMap.insert(2, "cat");
    hashMap.insert(3, "tiger");
    hashMap.insert(14, "lion");
    hashMap.insert(14, "lion22");
    dbg!(hashMap);
}
