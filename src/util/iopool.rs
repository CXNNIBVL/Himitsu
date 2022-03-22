use std::thread;
use std::ops::FnOnce;
use std::sync::{Arc, Mutex};
use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::sync::ShardedLock;

struct Transmission<T> where T: Send {
    inner: T,
    id: usize
}

impl<T: Send> PartialEq for Transmission<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl<T: Send> Eq for Transmission<T> {}

impl<T: Send> Ord for Transmission<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T: Send> PartialOrd for Transmission<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

pub struct IoPool<I, O> where I: Send + 'static, O: Send + 'static
{
    tx: Sender<Transmission<I>>,
    rx: Receiver<Transmission<O>>,
    count: usize,
    is_ordered: bool,
}

impl<I, O> IoPool<I,O> where I: Send + 'static, O: Send + 'static
{
    pub fn ordered<F>(threads: usize, f: F) -> Self where F: FnOnce(I) -> O, F: Send + Copy + 'static
    {
        let (tx, rx) = Self::spawn(threads, f);
        Self { tx, rx, count: 0, is_ordered: true }
    }

    pub fn unordered<F>(threads: usize, f: F) -> Self where F: FnOnce(I) -> O, F: Send + Copy + 'static
    {
        let (tx, rx) = Self::spawn(threads, f);
        Self { tx, rx, count: 0, is_ordered: false }
    }

    pub fn ordered_with_shared<S, F>(shared: S, threads: usize, f: F) -> Self
    where S: Send + Sync + 'static,
    F: FnOnce(&S, I) -> O, 
    F: Send + Copy + 'static
    {
        let (tx, rx) = Self::spawn_with_shared(threads, shared, f);
        Self { tx, rx, count: 0, is_ordered: true }
    }

    pub fn unordered_with_shared<S, F>(shared: S, threads: usize, f: F) -> Self 
    where S: Send + Sync + 'static,
    F: FnOnce(&S, I) -> O, 
    F: Send + Copy + 'static
    {
        let (tx, rx) = Self::spawn_with_shared(threads, shared, f);
        Self { tx, rx, count: 0, is_ordered: false }
    }

    pub fn ordered_with_shared_mut<S, F>(shared: S, threads: usize, f: F) -> Self
    where S: Send + Sync + 'static,
    F: FnOnce(&Mutex<S>, I) -> O, 
    F: Send + Copy + 'static
    {
        let (tx, rx) = Self::spawn_with_shared_mut(threads, shared, f);
        Self { tx, rx, count: 0, is_ordered: true }
    }

    pub fn unordered_with_shared_mut<S, F>(shared: S, threads: usize, f: F) -> Self 
    where S: Send + Sync + 'static,
    F: FnOnce(&Mutex<S>, I) -> O, 
    F: Send + Copy + 'static
    {
        let (tx, rx) = Self::spawn_with_shared_mut(threads, shared, f);
        Self { tx, rx, count: 0, is_ordered: false }
    }

    fn spawn_with_shared_mut<S, F>(threads: usize, shared: S, f: F) -> (Sender<Transmission<I>>, Receiver<Transmission<O>>)
    where 
    S: Send + Sync + 'static,
    F: FnOnce(&Mutex<S>, I) -> O, 
    F: Send + Copy + 'static
    {
        let (tx_in, rx_in) = unbounded();
        let (tx_out, rx_out) = unbounded();
        let arc_shared = Arc::new(Mutex::new(shared));

        for _ in 0..threads {
            let rx = rx_in.clone();
            let tx = tx_out.clone();
            let arc = Arc::clone(&arc_shared);

            thread::spawn(move || {
                let (arc, rx, tx) = (arc, rx, tx);

                while let Ok(Transmission{inner, id}) = rx.recv()
                {
                    tx.send(Transmission{
                        inner: f(arc.as_ref(), inner), 
                        id 
                    }).unwrap();
                }
            });
        }

        (tx_in, rx_out)
    }

    fn spawn_with_shared<S, F>(threads: usize, shared: S, f: F) -> (Sender<Transmission<I>>, Receiver<Transmission<O>>)
    where 
    S: Send + Sync + 'static,
    F: FnOnce(&S, I) -> O, 
    F: Send + Copy + 'static
    {
        let (tx_in, rx_in) = unbounded();
        let (tx_out, rx_out) = unbounded();
        let arc_shared = Arc::new(ShardedLock::new(shared));

        for _ in 0..threads {
            let rx = rx_in.clone();
            let tx = tx_out.clone();
            let arc = Arc::clone(&arc_shared);

            thread::spawn(move || {
                let (arc, rx, tx) = (arc, rx, tx);

                while let Ok(Transmission{inner, id}) = rx.recv()
                {
                    let shared = arc.as_ref().read().unwrap();
                    tx.send(Transmission{
                        inner: f(&(*shared), inner), 
                        id 
                    }).unwrap();
                }
            });
        }

        (tx_in, rx_out)
    }

    fn spawn<F>(threads: usize, f: F) -> (Sender<Transmission<I>>, Receiver<Transmission<O>>)
    where F: FnOnce(I) -> O, F: Send + Copy + 'static
    {
        let (tx_in, rx_in) = unbounded();
        let (tx_out, rx_out) = unbounded();

        for _ in 0..threads {
            let rx = rx_in.clone();
            let tx = tx_out.clone();

            thread::spawn(move || {
                let (rx, tx) = (rx, tx);

                while let Ok(Transmission{inner, id}) = rx.recv()
                {
                    tx.send(Transmission{
                        inner: f(inner), 
                        id 
                    }).unwrap();
                }
            });
        }

        (tx_in, rx_out)
    }

    pub fn push(&mut self, transmission: I) {

        self.tx.send(Transmission{
            inner: transmission, 
            id: self.count
        }).unwrap();

        self.count += 1;
    }

    pub fn finalize(&mut self) -> Vec<O> {

        let mut transmissions = Vec::with_capacity(self.count);

        for _ in 0..self.count {
            if let Ok(t) = self.rx.recv() {
                transmissions.push(t);
            }
        }

        self.count = 0;

        if self.is_ordered { transmissions.sort() }

        transmissions.into_iter().map(|t| t.inner).collect()
    }
}