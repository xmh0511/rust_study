use async_std::task::sleep;
use futures_lite::{future, Future, FutureExt};
use std::task::Context;
use std::task::Poll;

struct Data {
    i: i32,
}
impl Future for Data {
    type Output = i32;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.i == 0 {
            this.i += 1;
            Poll::Pending
        } else {
            Poll::Ready(this.i)
        }
    }
}

async fn show() -> i32 {
    println!("evaluation");

    let c = async {
        let data = Data { i: 0 };
        let d = data.await;
        d
    }
    .await;
    c
}
fn main() {
    use std::task::Waker;

    use parking::Parker;
    use waker_fn::waker_fn;

    let mut r = show();
    //future::block_on(r);
    let mut d = unsafe { core::pin::Pin::new_unchecked(&mut r) };


    fn parker_and_waker() -> (Parker, Waker) {
        let parker = Parker::new();
        let unparker = parker.unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        (parker, waker)
    }
    let r = parker_and_waker();
    let cx = &mut Context::from_waker(&r.1);
    let r = d.as_mut().poll(cx);
    match r {
        Poll::Pending => {
            println!("pending");
        }
        Poll::Ready(x) => {
            println!("result is {}", x);
        }
    };
    let r = parker_and_waker();
    let cx = &mut Context::from_waker(&r.1);
    let r = d.as_mut().poll(cx);
    match r {
        Poll::Pending => {
            println!("pending");
        }
        Poll::Ready(x) => {
            println!("result is {}", x);
        }
    };
    
}
