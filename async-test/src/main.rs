use std::future::Future;
use std::{task::{Context, Poll}};

struct Data {
    i: i32,
}
impl Future for Data {
    type Output = i32;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.i == 0 {
            println!("invoke Data poll that returns pending");
            this.i += 10;
            Poll::Pending
        } else {
            println!("invoke Data poll that returns Ready");
            Poll::Ready(this.i)
        }
    }
}

async fn show() -> i32 {
    println!("evaluation show");

    let c = async {
        // in the first time, it returns the Pending state, whereas, in the second time, it returns the Ready
        let data = Data { i: 0 };
        println!("precede data.await");
        let d = data.await;  // invoke Data::poll. Suspension if the call returns Pending. Otherwise, the following statement will be invoked for Ready  
        println!("follow data.await");
        d
    }
    .await;
    c
}

fn main() {
    use std::task::Waker;

    use parking::Parker;
    use waker_fn::waker_fn;

    let mut r = show(); // get the future of show

    let mut d = unsafe { core::pin::Pin::new_unchecked(& mut r) }; // construct a Pin from the got future


    fn parker_and_waker() -> (Parker, Waker) {
        // inspired from futures_lite

        let parker = Parker::new();
        let unparker = parker.unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        (parker, waker)
    }
    let r = parker_and_waker();
    let cx = &mut Context::from_waker(&r.1); // construct the context that is the second argument of Future:poll

    println!("the first time try to invoke show");
    // first evaluation of show
    let r = d.as_mut().poll(cx);
    match r {
        Poll::Pending => {
            println!("invoking show gets pending");
        }
        Poll::Ready(x) => {
            println!("invoking show whose result is {}", x);
        }
    };
    println!("----------------------------");

    println!("the second time try to invoke show");
    // second evaluation of show
    let r = d.as_mut().poll(cx);
    match r {
        Poll::Pending => {
            println!("invoking show gets pending");
        }
        Poll::Ready(x) => {
            println!("invoking show whose result is {}", x);
        }
    };

}
