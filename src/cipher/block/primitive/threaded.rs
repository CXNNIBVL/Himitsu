use crate::traits::cipher::{
    BlockCipherPrimitiveDecryption as PrimitiveDecryption,
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::sync::ShardedLock;
use std::sync::Arc;
use std::thread;

struct InData<const BLOCKSIZE: usize> {
    mut_block: [u8; BLOCKSIZE],
    xor_pre: Option<[u8; BLOCKSIZE]>,
    xor_post: Option<[u8; BLOCKSIZE]>,
    id: usize,
}

struct OutData<const BLOCKSIZE: usize> {
    block: [u8; BLOCKSIZE],
    id: usize,
}

pub struct ThreadedCipherEncryption<T: PrimitiveEncryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    tx: Sender<InData<BLOCKSIZE>>,
    rx: Receiver<OutData<BLOCKSIZE>>,
    block_count: usize,
    ph: std::marker::PhantomData<T>,
}

impl<T, const B: usize> ThreadedCipherEncryption<T, B>
where
    T: PrimitiveEncryption<B> + Send + Sync + 'static,
{
    fn setup(
        arc: &Arc<ShardedLock<T>>,
        threads: usize,
    ) -> (Sender<InData<B>>, Receiver<OutData<B>>) {
        let (tx_in, rx_in) = unbounded();
        let (tx_out, rx_out) = unbounded();

        for _ in 0..threads {
            let arc = Arc::clone(arc);
            let rx = rx_in.clone();
            let tx = tx_out.clone();

            thread::spawn(move || {
                let (arc, rx, tx) = (arc, rx, tx);

                while let Ok(InData {
                    mut mut_block,
                    xor_pre,
                    xor_post,
                    id,
                }) = rx.recv()
                {
                    let cipher = arc.as_ref().read().unwrap();
                    cipher.encrypt(&mut mut_block, xor_pre.as_ref(), xor_post.as_ref());
                    tx.send(OutData {
                        block: mut_block,
                        id,
                    })
                    .unwrap();
                }
            });
        }

        (tx_in, rx_out)
    }

    pub fn new(primitive: T, threads: usize) -> Self {
        let arc_primitive = Arc::new(ShardedLock::new(primitive));

        let (tx, rx) = Self::setup(&arc_primitive, threads);

        Self {
            tx,
            rx,
            block_count: 0,
            ph: std::marker::PhantomData {},
        }
    }

    pub fn put(&mut self, mut_block: [u8; B], xor_pre: Option<[u8; B]>, xor_post: Option<[u8; B]>) {
        let in_data = InData {
            mut_block,
            xor_pre,
            xor_post,
            id: self.block_count,
        };
        self.tx.send(in_data).unwrap();
        self.block_count += 1;
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        let mut out = vec![0; self.block_count * B];

        while self.block_count != 0 {
            let OutData { block, id } = self.rx.recv().unwrap();
            self.block_count -= 1;

            let start = id * B;
            let end = start + B;

            out.splice(start..end, block);
        }

        out
    }
}

pub struct ThreadedCipherDecryption<T: PrimitiveDecryption<BLOCKSIZE>, const BLOCKSIZE: usize> {
    tx: Sender<InData<BLOCKSIZE>>,
    rx: Receiver<OutData<BLOCKSIZE>>,
    block_count: usize,
    ph: std::marker::PhantomData<T>,
}

impl<T, const B: usize> ThreadedCipherDecryption<T, B>
where
    T: PrimitiveDecryption<B> + Send + Sync + 'static,
{
    fn setup(
        arc: &Arc<ShardedLock<T>>,
        threads: usize,
    ) -> (Sender<InData<B>>, Receiver<OutData<B>>) {
        let (tx_in, rx_in) = unbounded();
        let (tx_out, rx_out) = unbounded();

        for _ in 0..threads {
            let arc = Arc::clone(arc);
            let rx = rx_in.clone();
            let tx = tx_out.clone();

            thread::spawn(move || {
                let (arc, rx, tx) = (arc, rx, tx);

                while let Ok(InData {
                    mut mut_block,
                    xor_pre,
                    xor_post,
                    id,
                }) = rx.recv()
                {
                    let cipher = arc.as_ref().read().unwrap();
                    cipher.decrypt(&mut mut_block, xor_pre.as_ref(), xor_post.as_ref());
                    tx.send(OutData {
                        block: mut_block,
                        id,
                    })
                    .unwrap();
                }
            });
        }

        (tx_in, rx_out)
    }

    pub fn new(primitive: T, threads: usize) -> Self {
        let arc_primitive = Arc::new(ShardedLock::new(primitive));

        let (tx, rx) = Self::setup(&arc_primitive, threads);

        Self {
            tx,
            rx,
            block_count: 0,
            ph: std::marker::PhantomData {},
        }
    }

    pub fn put(&mut self, mut_block: [u8; B], xor_pre: Option<[u8; B]>, xor_post: Option<[u8; B]>) {
        let in_data = InData {
            mut_block,
            xor_pre,
            xor_post,
            id: self.block_count,
        };
        self.tx.send(in_data).unwrap();
        self.block_count += 1;
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        let mut out = vec![0; self.block_count * B];

        while self.block_count != 0 {
            let OutData { block, id } = self.rx.recv().unwrap();
            self.block_count -= 1;

            let start = id * B;
            let end = start + B;

            out.splice(start..end, block);
        }

        out
    }
}
