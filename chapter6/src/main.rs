fn bubble_sort1(nums : &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    let len = nums.len();
    for i in 0..(len-1) {
        for j in (i+1)..len {
            if nums[j] > nums[i]{
                nums.swap(j, i);
            }
        }
    }
}

fn quick_sort1(nums : &mut [i32], low : usize, high : usize) {
    if low < high {
        let split = partition(nums, low, high);
        if split > 1 {
            quick_sort1(nums, low, split - 1);
        }
        quick_sort1(nums, split + 1, high)
    }
}

fn partition(nums : &mut [i32], low : usize, high : usize) -> usize {
    let mut lm = low;
    let mut rm = high;

    let tmp = nums[low];
    loop {
        while lm <= rm && nums[lm] <= tmp {
            lm += 1;
        }

        while lm <= rm && nums[rm] >= tmp {
            rm -= 1;
        }

        if lm > rm {
            break;
        }else{
            nums.swap(lm, rm);
        }
    }

    nums.swap(low, rm);

    rm
}

fn insertion_sort(nums : &mut [i32]) {
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];

        while pos > 0 && curr < nums[pos -1] {
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }

        nums[pos] = curr;
    }
}

//这个方法不行(判断mid后可以了)
fn bn_inserttion_sort(nums : &mut [i32]) {
    let mut temp;
    let mut left;
    let mut right;
    let mut mid;

    for i in 1..nums.len() {
        left = 0;
        right = i - 1;
        temp = nums[i];

            //加了等号会爆掉
        while left <= right{
            mid = (left + right) >> 1;
            if temp < nums[mid]{
                if mid < 1 {
                    break;
                }else
                {
                    right = mid - 1;
                }
            }else {
                left = mid + 1;
            }
        }

        println!("in left {}", left);
        for j in (left..=i-1).rev(){
            nums.swap(j, j+1);
        }

        if left != i {
            nums[left] = temp;
        }
    }
}

fn shell_sort(nums : &mut [i32]) {

    fn ist_sort(nums : &mut [i32], start : usize, gap : usize) {
        let mut i = start + gap;

        while i < nums.len(){
            let mut pos = i;
            let curr = nums[pos];

            while pos >= gap  && curr < nums[pos - gap]{
                nums[pos] = nums[pos - gap];
                pos -= gap;
            }

            nums[pos] = curr;
            i += gap;
        }
    }

    let mut gap = nums.len() / 2;
    while gap > 0 {
        for start in 0..gap{
            ist_sort(nums, start, gap);
        }
        gap /= 2;
    }
}

fn merge_sort(nums : &mut [i32]){
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[..mid]);
        merge_sort(&mut nums[mid..]);
        merge(nums, mid);
    }
}

fn merge(nums : &mut [i32], mid : usize){
    let mut left = 0;
    let mut right = mid;
    let mut temp_vec = Vec::new();

    for i in 0..nums.len(){
        if right == nums.len() || left == mid {
            break;
        }

        //小的放左，大的放右
        if nums[left] < nums[right] {
            temp_vec.push(nums[left]);
            left += 1;
        }else{
            temp_vec.push(nums[right]);
            right += 1;
        }
    }

    //左边残余放末尾
    if left < mid && right == nums.len(){
        for j in left..mid{
            temp_vec.push(nums[j]);
        }  //右边残余放末尾
    }else if right < nums.len() && left == mid {
        for j in right..nums.len(){
            temp_vec.push(nums[j]);
        }
    }

    for j in 0..nums.len(){
        nums[j] = temp_vec[j];
    }
}

fn select_sort(nums : &mut [i32]){
    let mut left = nums.len() - 1;
    while left > 0{
        let mut pos_max = 0;
        for i in 1..=left{
            if nums[i] > nums[pos_max]{
                pos_max = i;
            }
        }
        nums.swap(left, pos_max);
        left -= 1;
    }
}

macro_rules! parent {
    ($child : ident) => {
        $child >> 1
    };
}

macro_rules! left_child {
    ($parent  : ident) => {
        ($parent << 1) + 1
    };
}

macro_rules! right_child {
    ($parent : ident) => {
        ($parent + 1) << 1
    };
}

fn heap_sort(nums : &mut [i32]){
    if nums.len() <= 1 { return;}

    let len = nums.len();

    let last_parent = parent!(len);

    for i in (0..=last_parent).rev(){
        move_down(nums, i);
    }

    for end in (1..nums.len()).rev(){
        nums.swap(0, end);
        move_down(&mut nums[..end], 0)
    }
}

//大者沉，小者浮
fn move_down(nums : &mut [i32], mut parent : usize){
    let last = nums.len() - 1;
    loop {
        let left = left_child!(parent);
        let right = right_child!(parent);

        if left > last {
            break;
        }

        //取较大者
        let child = if right <= last && nums[left] < nums[right]{
            right
        }else{
            left
        };

        if nums[child] > nums[parent]{
            nums.swap(parent, child);
        }

        parent = child;
    }
}

fn main() {
    let mut nums = [11, 5, 22, 7, 12, 90, 76, 2];
    heap_sort(&mut nums);

    dbg!(nums);   
}
