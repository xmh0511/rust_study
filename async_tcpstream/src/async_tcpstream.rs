use std::future::Future;
use std::io::{self, BufReader, BufWriter};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
pub struct AsyncThread<R: Send + 'static> {
    f: Option<Box<dyn FnOnce() -> io::Result<R> + Send + 'static>>,
    ret: Arc<Mutex<Option<io::Result<R>>>>,
}

impl<R: Send + 'static> AsyncThread<R> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> io::Result<R>,
        F: Send + 'static,
    {
        Self {
            f: Some(Box::new(f)),
            ret: Arc::new(Mutex::new(None)),
        }
    }
}

impl<R: Send + 'static> Future for AsyncThread<R> {
    type Output = io::Result<R>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let ret = self.ret.lock().unwrap().take();
        match ret {
            Some(t) => std::task::Poll::Ready(t),
            None => {
                let that = self.get_mut();
                let f = that.f.take();
                let setter = that.ret.clone();
                let waker = cx.waker().clone();
                let _r = thread::spawn(move || {
                    let r = f.unwrap()();
                    *setter.lock().unwrap() = Some(r);
                    //println!("prepare to wake");
                    waker.wake();
                });
                std::task::Poll::Pending
            }
        }
    }
}

pub struct AsyncTcp(Arc<TcpStream>);

impl AsyncTcp {
    pub fn new(addr: &str) -> Self {
        AsyncTcp(Arc::new(TcpStream::connect(addr).unwrap()))
    }
    pub async fn write(&self, buff: &[u8]) -> io::Result<usize> {
        let conn = self.0.clone();
        let buff = buff.to_owned();
        let task = AsyncThread::new(move || {
            let mut writer = BufWriter::new(conn.as_ref());
            //println!("evaluate write");
            let r = writer.write(&buff);
            //println!("write ok {r:?}");
            r
        });
        task.await
    }
    pub async fn read(&self) -> io::Result<Vec<u8>> {
        let conn = self.0.clone();
        let task = AsyncThread::new(move || {
            let mut reader = BufReader::new(conn.as_ref());
            let mut vec = Vec::new();
            vec.resize(1024 * 1024, b'\0');
            match reader.read(&mut vec) {
                Ok(size) => {
                    println!("read completely");
                    vec.resize(size, b'\0');
                    Ok(vec)
                }
                Err(e) => Err(e),
            }
        });
        task.await
    }
}