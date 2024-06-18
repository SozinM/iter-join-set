## IterJoinSet
`IterJoinSet` is a small Rust library that provides a structure to manage a set of asynchronous tasks with the capability to spawn futures directly or through an iterator, 
ensuring a bounded number of concurrent tasks.

### Features

- Create with Default Capacity: Instantiate with a default capacity of 10 tasks.
- Custom Capacity: Build with custom capacity and iterator.
- Spawn Futures: Spawn futures directly onto the JoinSet or through an iterator.
- Backfill Queue: Automatically refill the task queue to maintain the specified capacity.
- Retrieve Completed Tasks: Fetch the next completed task, either blocking or non-blocking.

### Spawning Futures:

- Use spawn_inner to directly spawn a future onto the JoinSet ignoring capacity limits.
- Use spawn to convert a future into an iterator of one and add it to the JoinSet.
- Use spawn_iter to add multiple futures via an iterator to the JoinSet.
- 
### Managing the Queue:

- join_next returns the next completed future, blocking until one is available and refilling the queue as necessary.
- try_join_next attempts to return the next completed future without blocking and refills the queue.


### License
This project is licensed under the MIT License.