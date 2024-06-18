use futures::future::{BoxFuture, Future};
use tokio::task::{JoinError, JoinSet};

pub struct IterJoinSet<T: Sized> {
    inner: JoinSet<T>,
    it: Option<Box<dyn Iterator<Item = BoxFuture<'static, T>>>>,
    capacity: usize,
}

impl<T: Send + 'static> IterJoinSet<T> {
    /// Creates empty structure with default capacity of 10
    pub fn new() -> Self {
        IterJoinSet {
            inner: JoinSet::new(),
            it: None,
            capacity: 10,
        }
    }

    /// Creates structure from params
    pub fn build(it: Box<dyn Iterator<Item = BoxFuture<'static, T>>>, capacity: usize) -> Self {
        IterJoinSet {
            inner: JoinSet::new(),
            it: Some(it),
            capacity,
        }
    }

    /// Spawn future directly onto JoinSet ignoring capacity bound logic
    pub fn spawn_inner<F>(&mut self, handle: F)
    where
        F: Future<Output = T>,
        F: Send + 'static,
        T: Send,
    {
        self.inner.spawn(handle);
    }

    /// Convert future into iter::once and adds it as
    pub fn spawn(&mut self, handle: BoxFuture<'static, T>)
    where
        T: Send,
    {
        let it = Box::new(std::iter::once(handle));
        self.spawn_iter(it);
    }

    //
    pub fn spawn_iter(&mut self, it: Box<dyn Iterator<Item = BoxFuture<'static, T>>>) {
        match self.it.take() {
            Some(inner_it) => self.it = Some(Box::new(inner_it.chain(it))),
            None => self.it = Some(it),
        }
    }
    // Return first completed future blocking until returned. Backfill queue
    pub async fn join_next(&mut self) -> Option<Result<T, JoinError>> {
        self.fill_queue();
        let res = self.inner.join_next().await;
        res
    }

    // Return first completed future or None of there are no completed futures. Backfill queue
    pub async fn try_join_next(&mut self) -> Option<Result<T, JoinError>> {
        self.fill_queue();
        let res = self.inner.try_join_next();
        res
    }

    // Fills queue from iterator and returns number of scheduled tasks
    fn fill_queue(&mut self) -> usize {
        let mut total_spawned = 0;
        if self.inner.len() < self.capacity {
            let jobs_num = self.capacity - self.inner.len();
            for _ in 0..jobs_num {
                if let Some(it) = &mut self.it {
                    let task = it.next();
                    match task {
                        Some(task) => {
                            self.inner.spawn(task);
                            total_spawned += 1;
                        }
                        None => break,
                    }
                } else {
                    break;
                }
            }
        }
        total_spawned
    }
}

impl<T: Send + 'static> Default for IterJoinSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
