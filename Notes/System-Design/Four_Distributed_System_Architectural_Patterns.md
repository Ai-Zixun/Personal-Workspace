# Four Distributed Systems Architectural Patterns - by Tim Berglund

https://www.youtube.com/watch?v=BO761Fj6HH8

| Overall Rating | Modern Three Tier | Sharded | Batch + Stream | Event Bus |
| -------------- | ----------------- | ------- | -------------- | --------- |
| Scalability    | 4/5               | 3/5     | 5/5            | 5/5       |
| Coolness       | 2/5               | 1/5     | 1/5            | 5/5       |
| Difficult      | 3/5               | 4/5     | 5/5            | 4/5       |
| Flexibility    | 5/5               | 3/5     | 2/5            | 5/5       |


## Pattern 1 - Modern Three-Tier

```
 -----------------    -------------    ---------
|Presentation Tier|--|Business Tier|--|Data Tier|
 -----------------    -------------    ---------
```

```
 ----------           ---------    -----------
| React JS |-- ELB --| Node JS |--| Cassandra |
 ----------           ---------    -----------
```

1. Presentation Tier - React JS
   * stateless - on client
2. Business Tier - Node JS
   * stateless - on server
3. Data Tier - Cassandra

Cassandra
* all nodes on the Cassandra cluster is the same
* assign each node a token (hash range) - for sharding
* hash the input - write/read the message from the server which contain the hash range
* write replicas to the next X nodes

**strengths** of the modern three tier
* reach front-end frameworks
* scalable middle tier - stateless
* infinitely scalable data tire - with cassandra

**weaknesses** of the modern three tier
* need to keep the middle tier stateless for scalability

## Pattern - Shard

Break up the system into several shard, where each shard is a complete system

Good real-world examples:
* Slack

**Stage 1**
```
 --------      ----------------------      ----------
| Client | -- | Complete Application | -- | Database |
 --------      ----------------------      ----------
```

**Stage 2**
```
 --------      --------      ----------------------      ----------
| Client | -- |        | -- | Complete Application | -- | Database |
 --------     |        |     ----------------------      ----------
 --------     |        |     ----------------------      ----------
| Client | -- | Router | -- | Complete Application | -- | Database |
 --------     |        |     ----------------------      ----------
 --------     |        |     ----------------------      ----------
| Client | -- |        | -- | Complete Application | -- | Database |
 --------      --------      ----------------------      ----------
```

**strengths** of shard
* client isolation is easy (data and deployment)
* known, simple technologies

**weaknesses** of shard
* complexity: monitoring, routing
* no comprehensive view of data (need to merge all data)
* oversized shards -> a shard become a distributed system on itself
* difficult to re-shard; need to design the sharding schema upfront

## Pattern 3 - Batch + Stream

Streaming vs Batch ?
* streaming - data is coming in in real time
* batch - data that is store somewhere

Batch + Stream - assumes unbounded, immutable data

```
 --------      ----------------------      ----------
| Source | -- | batch processing     | -- | Scalable |
| of     |     ----------------------     | Database |
| Event  |     ----------------------     |          |
|        | -- | streaming processing | -- |          |
 --------      ----------------------      ----------
```

batch processing
* long-term storage
* bounded analysis
* high latency

streaming processing
* temporary queueing
* unbounded computation
* low latency

```
 --------      ----------------------      -----------
| Kafka  | -- | Cassandra + Spark    | -- | Cassandra |
|        |     ----------------------     |           |
|        |     ----------------------     |           |
|        | -- | Event frameworks     | -- |           |
 --------      ----------------------      -----------
```

Kafka
* producer
* consumer
* topic
  * named queue
  * can be partitioned

topic partitioning
* the queue become unordered
* because partition does not track order in other partition

**strengths** of batch + stream
* optimized subsystems based on operational requirements
* good at unbounded data

**weaknesses** of batch + stream
* complex to operate and maintain

## Pattern 4 - Event Bus

* integration is a first-class concern
* life is dynamic; database are static
* table are streams and streams are tables
* keep your services close, your computer closer

Storing Data in Message Queue
* Retention policy (e.g. can be forever)
* high I/O performance
* O(1) writes, O(1)reads
* partitioning, replication
* elastic scale

first-class event - event or request ?
* request
  * request
  * response
* event
  * produce
  * consume
