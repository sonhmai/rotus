# rotus
Distributed Key Value Store In Rust

Mainly for learning.

Roadmap:
- [ ] Key Value Store API in memory on a single server: get, put
- [ ] Distributed Design
- [ ] Distributed Implementation
- [ ] Productionize

Distributed Core
- partitioning
- replication
- versioning
- membership and failure detection
- failure handling
- scaling

Productionize
- load balancing
- replica synchronization
- state transfer
- concurrency and job scheduling
- request marshalling
- request routing
- system monitoring and alarming
- configuration management

DynamoDB
- partitioning: consistent hashing
- HA for writes: vector clocks with reconciliation during reads
- membership and failure detection: gossip-based membership protocol and failure detection

# References
- [DynamoDB paper](http://www.cs.cornell.edu/courses/cs5414/2017fa/papers/dynamo.pdf)
- BigTable paper
- [TiKV](https://developpaper.com/building-a-distributed-key-value-store-using-rust/)
- [Noria](https://github.com/mit-pdos/noria)

