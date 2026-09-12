use std::thread ;
use std::cell::RefCell ;

// thread_local! оборачивает любое количество статических объявлений и 
// делает их локальными для потока. Допускается публичность и атрибуты 
// для каждого статического объявления.
thread_local! {
    // COUNTER не передаётся между потокоами
    static COUNTER: RefCell<u64> = RefCell::new(0) ;    
}

// Инкремент значение статической переменной локальной для этого потока
// TLS (Thread-Local Storage)
fn my_incr() {
    // Получить ссылку на значение этого TLS (Thread-Local Storage) ключа
    // Это будет лениво инициализировать значение если не ссылки на 
    // этот ключь ещё не было.    
    COUNTER.with(|c| {
        *c.borrow_mut() += 1 ;
    })
}

// Получить значение статической переменной локальной для этого потока
// TLS (Thread-Local Storage)
fn my_get() ->u64 {
    // Получить ссылку на значение этого TLS (Thread-Local Storage) ключа
    // Это будет лениво инициализировать значение если не ссылки на 
    // этот ключь ещё не было.    
    COUNTER.with(|c| {
        *c.borrow()
    })
}

fn main() {
    let handle: Vec<_> = (0..3)
        .map(|ind| {
            thread::spawn(move || {
                my_incr();
                my_incr();
                println!("Thread: {ind}, val: {}", my_get()) ;
            })
        })
        .collect()
        ;

    for h in handle {
        h.join().unwrap() ;
    }

    println!("Local thread, val: {}", my_get()) ;
}
