# Memcached Report

## Project Structure

the project is composed of three folders:

1. server - the memcached server
2. protocol - the memcached protocol. this involves types for requests and
   responses as well as the parsing logic.
3. client - a client library that is able to communicate with the server. the
   main tests are in this folder

### Protocol

this library includes all the types and parsing logic used in the client library
and the server. It includes a Deserializer struct that contains helper methods
to help pass strings. the types that this library defines are:

- `Request`: the request that a client send.
  - Store(StoreRequest),
    - Set(Entry),
    - Add(Entry),
    - Replace(Entry),
    - Append(Entry),
    - Prepend(Entry),
  - Retreive(RetrieveRequest),
    - Get(String)
  - FlushAll,
  - Delete(String),
- `Response`: the response that is sent from the server to after handling a
  request.
  - Store(StoreResponse)
    - Stored,
    - NotStored,
    - Exists,
    - NotFound,
  - Retrieve(Entry)
  - Delete(DeleteResponse)
    - Deleted,
    - NotFound,
  - Ok
  - End
  - Error(MemcachedError)
    - Error: sent the client sends an invalid command
    - ClientError: sent when the key contains invalid charaters such as a space
      on control characters
    - ServerError: sent when the server encounters an unexpected error

- `Entry`: the data that is stored/sent betweeen the server and client e server
  and client. It fields include:
  - key: String,
  - flags: u32,
  - exptime: u32,
  - len: u32,
  - value: String,

### Server

this is the binary executable that listens for incoming requests and sends back
responses. It first looks for a file named data.txt in the current directory.
That file contains the persisted data. If the file is not found, then it will be
created in the current directory. It then attempts reads the contents of the
file (if it existed) and load it in a hashmap. After that, the server creates a
tcp socket and listens for incoming connections at address `127.0.0.1:9889`.
When a client connects, a new thread is spawned to handle its requests. All
clients write and read to the same hashmap. After every store request (set, add,
replace, prepend, append), the contents of the hashmap is written to disk.

The commands that are supported by the server are:

- set
- add
- replace
- append
- prepend
- get
- flush_all
  - deletes all contents of the hashmap
- delete

### Client

this library includes the code to connect to the server and perform the
supported. It includes a `Client` struct that communicats with the server. Its
has methods similar to pymemcache. In addition, the main testing occures in this
library.

---

# Questions

### How many concurrent clients can the server handle?

From the testing I did on my machine, the server can handle somewhere between
250-270 concurrent clients.

### What is the performance of the server

The performance of the server is ok. It can handle 250-270 concurrent
connections. however, there some things that we can do to improve its
performance as mentioned below.

### What are some future improvments?

- One of the things that we can do to dramatically improve the performance of
  this key value is to not write the data to disk on every store request. This
  process tasks a huge amount of time compared to other tasks that the server
  can execute. By eliminating this step, the server will be able to handle store
  requests faster.
- Another thing that we can do to improve the performance is to not spawn OS
  threads when a new connection is established. We can utilize asynchronous
  non-blocking IO. To do this, we can use an asynchronous runtime and spawn
  green threads instead. This will improve the performance of our server because
  it will allow us to spawn more green threads and spawn them faster due to
  their low overhead. This is beneficial to our server because it is mostly IO
  bound. The server spends most of its time doing IO tasks than CPU
  computations.
- Handle exptime if it is longer than 30 days. the current memcached protocol
  states that if the number of seconds is greater than 30 days, then exptime is
  interpreted as real unix time instead of as seconds. Currently the server can
  only handle exptime if it is not greater than 30 days.

### Server Limitations

As I mentioned above in possible future improvements, there are many things that
we can do to improve the performance of this server. The biggest limitation is
that it writes to disk on each store request which slows it down tremendously.
Instead of persisting to file after every store request, we can periodically
store the data to disk after some time interval. Also, from my testing, the
server can handly 250-270 concurrent reqeusts.

### Max Key and Value and Sizes

As per the memcached protocol, the maximum key size that this server accepts is
250 bytes. However, I chose the value max size limit to be 350. Although this
can't be changed, I believe this is a reasonable limit to maintain a good level
of performance for our use case.
