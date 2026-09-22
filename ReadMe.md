# Design

- thread pool to avoid exhausting resources

# [TODO] Async-Await (stackless coroutines)

- should support async await (stackless coroutines) in conjuction with Thread pool to avoid starvation that can be cause by slow requests (exhausting the thread pool)

- since the server is IO-bound, so the threads in thread pool spend their time waiting blocked, doing nothing

# Refs

- https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html
