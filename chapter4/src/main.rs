use core::num;

fn nums_sum1(nums : &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    }else{
        let first = nums[0];
        first + nums_sum1(&nums[1..])
    }
}

fn nums_sum3(sum : i32, nums : &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    }else{
        let first = nums[0];
        nums_sum3(sum + nums[0], &nums[1..])
    }
}

fn nums_sum2(nums : &[i32]) -> i32 {
    if nums.len() == 1 {
        nums[0]
    }else{
        let lastIdx = nums.len() - 1;
        let last = nums[lastIdx];
        last + nums_sum2(&nums[0..lastIdx])
    }
}

fn nums_sum4(sum : i32, nums : &[i32]) -> i32 {
    if nums.len() == 1 {
        sum + nums[0]
    }else{
        let lastIdx = nums.len() - 1;
        nums_sum4(sum + nums[lastIdx], &nums[0..lastIdx])
    }
}

const BASESTR : [&str; 16] = ["0", "1", "2", "3","4", "5","6", "7","8", "9","A", "B","C", "D","E", "F"];

fn num2str_rec(num : i32, base : i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    }else{
        num2str_rec(num / base, base) + BASESTR[(num % base) as usize]
    } 
}

fn move2tower(height : u32, src_p : &str, des_p : &str, mid_p : &str){
    if height >= 1{
        move2tower(height - 1, src_p, mid_p, des_p);
        println!("move disk fom {src_p} to {des_p}");
        move2tower(height - 1, mid_p, des_p, src_p);
    }
}

fn rec_mc1(caches : &[u32], amount : u32) -> u32 {
    let mut min_caches = amount;

    if caches.contains(&amount) {
        return 1;
    }

    for c in caches.iter()
        .filter(|&c| *c <= amount)
        .collect::<Vec<&u32>>(){

            let num_caches = 1 + rec_mc1(&caches, amount - c);
            if num_caches < min_caches {
                min_caches = num_caches;
            }
        }

    min_caches
}

fn rec_mc2(caches : &[u32], amount : u32, min_caches : &mut [u32]) -> u32 {
    let mut min_num = amount;

    if caches.contains(&amount){
        min_caches[amount as usize] = amount;
        return 1;
    }
    
    if min_caches[amount as usize] > 0 {
        return min_caches[amount as usize];
    }else{
        for c in caches.iter()
                .filter(|&c| *c <= amount)
                .collect::<Vec<&u32>>(){
            let cache_num = 1 + rec_mc2(caches, amount - c, min_caches);
            if cache_num < min_num {
                min_num = cache_num;
                min_caches[amount as usize] = min_num;
            }
        }
        return min_num;
    }
}

fn dp_rec_mc(caches : &[u32], amount : u32, min_caches : &mut [u32]) -> u32 {
    for denm in 1..=amount {
        let mut min_num = denm;
        for c in caches.iter()
            .filter(|&c| *c <= denm)
            .collect::<Vec<&u32>>(){
            let index = (denm - c) as usize;

            let cache_num = 1 + min_caches[index];
            if cache_num < min_num {
                min_num = cache_num;
            }
        }
        min_caches[denm as usize] = min_num;
    }
    
    min_caches[amount as usize]
}

fn dp_rec_mc_show(caches : &[u32], amount : u32, min_caches : &mut [u32],
    caches_used : &mut [u32]) -> u32 {
    for denm in 1..=amount {
        let mut min_num = denm;
        let mut used_cache = 1;
        for c in caches.iter()
            .filter(|&c| *c <= denm)
            .collect::<Vec<&u32>>(){
            let index = (denm - c) as usize;

            let cache_num = 1 + min_caches[index];
            if cache_num < min_num {
                min_num = cache_num;
                used_cache = *c;
            }
        }
        min_caches[denm as usize] = min_num;
        caches_used[denm as usize] = used_cache;
    }
    
    min_caches[amount as usize]
}

fn fib_rec(n : u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }else {
        fib_rec(n -1) + fib_rec(n -2)
    }
}

fn fib_dp(n : u32) -> u32 {
    let mut dp = [1, 1];

    for i in 2..=n{
        let idx1 = (i %2) as usize;
        let idx2 = ((i - 1) %2) as usize;
        let idx3 = ((i - 2) %2) as usize;
        dp[idx1] = dp[idx2] + dp[idx3];
    }
    dp[((n-1) %2) as usize]
}
fn main() {
    //let nums = [11, 22, 33, 44];
    //println!("{}", nums_sum4(0, &nums));
    //let ret = num2str_rec(345, 16);
    //println!("{}", ret);
    //move2tower(2, "A", "B" , "C");

    // let caches = [1, 5, 10, 20, 50];

    // let amount = 18;

    // let mut min_caches : [u32; 19]= [0; 19];
    // let mut caches_used : [u32;19] = [0; 19];
    // let cache_nums = dp_rec_mc_show(&caches, amount, &mut min_caches, &mut caches_used);
    // println!("{}, {:?}, {:?}", cache_nums, min_caches, caches_used);

    let ret = fib_dp(4);
    println!("{}", ret);
}
