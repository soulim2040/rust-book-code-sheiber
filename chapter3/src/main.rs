mod collect;
use collect::stack::Stack;
use collect::par_checker;
use collect::queue;
use collect::link;
use collect::list_stack;
use collect::list_vec;
fn main() {
    // println!("Hello, world!");
    // let mut st: Stack<i32> = Stack::new();
    // st.push(11);
    // st.push(22);
    // let tt = st.pop().unwrap();
    // st.push(33);
    // dbg!(st.size(), st.peek().unwrap(), st.is_empty(), tt);

    //let is_balance = par_checker::par_checker3("((1+2)({f[3*2]}))");
    //dbg!(is_balance);
    //dbg!(par_checker::base_conver(3555, 16));

    // let infix = "( A + B * 3 )";
    // let infix = "A * B + C * D";
    // let  result = par_checker::infix_to_postfix(infix);
    // dbg!(result);

    // let mut q = Queue::new(3);
    // q.enqueue(11);
    // q.enqueue(22);
    // q.enqueue(33);

    // let item = q.dequeue().unwrap();
    // println!("{}, {}", q.size(), item);
    // let names = [
    //     "ben",
    //     "ben2",
    //     "ben3",
    //     "ben4",
    //     "ben5",
    //     "ben6"
    // ];

    // let name = queue::hot_potato(names.to_vec(), 8);
    // dbg!(name);

    // let is_pal = queue::pal_check("baocb");
    // dbg!(is_pal);

    // let mut m = Some(22);
    // dbg!(m);
    // let ne = m.take();
    // dbg!(ne, m);

    // let mut list = link::List::new();
    // list.push(1);
    // list.push(2);
    // let d1 = list.pop().unwrap();
    // println!("{}", d1);

    // let mut stack = list_stack::Stack::new();
    // stack.push(1);
    // stack.push(2);
    // let m = stack.pop().unwrap();
    // let n = stack.peek().unwrap();
    // println!("{}, {}", m, n);

    // println!("{}", stack.size());

    let mut lvec = list_vec::LVec::new();

    lvec.push(11);
    lvec.push(22);
    lvec.push(33);

    let m = lvec.pop().unwrap();
    println!("{}, {}", m, lvec.len());
    lvec.insert(1, 44);

    let mut rvec = list_vec::LVec::new();
    rvec.push(88);
    rvec.push(99);
    lvec.append(&mut rvec);

    lvec.remove(0);
    lvec.print_lvec();
}
