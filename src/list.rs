pub struct List {
    list: Vec<i32>,
    min: i32,
    max: i32,
    length: usize,
    max_length: usize
}

impl List {
    pub fn add(&mut self, value: i32) {
    }
}


// pub struct ThreadPool<T: Send> {
//     workers : Arc<Vec<Worker<T>>>,
//     tasks: Arc<Mutex<Queue<Task<T>>>>
// }

// type Runnable<T> = Box<dyn Fn() -> T + Send>;

// pub struct Task<T: Send> {
//     runnable: Runnable<T>,
//     sender_res: Arc<Mutex<Sender<T>>>,
// }

// impl<T: Send> Task<T> {
//     pub fn new(runnable: Runnable<T>, sender: Arc<Mutex<Sender<T>>>) -> Task<T> {
//         Task {
//             runnable,
//             sender
//         }
//     }
// }