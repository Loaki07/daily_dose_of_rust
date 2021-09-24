use futures::Future;
use futures::task::Context;
use futures::task::Poll;

use std::pin::Pin;

pub struct SimpleFuture {
    n: i32
}

impl Future for SimpleFuture {
    type Output = i32;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(self.n)
    }
}

pub async fn simple_exec_future(p: i32) -> i32 {
    p + 10
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use futures::future::FutureExt;
    use futures::channel::oneshot;
    
    #[test]
    fn test_future_returns_a_value() {
        let f = SimpleFuture{n: 10};
        let (ch_s, ch_r) = oneshot::channel();


        let _v = block_on(f.map(move |n| ch_s.send( n + 5)));
        let res = block_on(ch_r).unwrap();
        assert_eq!(res, 15);
    }

    #[test]
    fn test_simple_exec_future() {
        let f = simple_exec_future(10);
        let (ch_s, ch_r) = oneshot::channel();


        let _v = block_on(f.map(move |n| ch_s.send( n + 5)));
        let res = block_on(ch_r).unwrap();
        assert_eq!(res, 25);
    }

    #[test]
    fn test_async_send() {
        let (ch_s, ch_r) = oneshot::channel();
        block_on(async move {
            let v = simple_exec_future(10).await;
            let _send_to_channel = ch_s.send(v);
        });
        let final_res = block_on(async move {
            let res = ch_r.await.unwrap();
            res + 5
        });
        assert_eq!(final_res, 25);
    }
}
