# Kafka



## Terminologies

**Topics**

```
 ------------------------------------------------------
|  -----  -----  -----  -----  -----                    |
| | K/V || K/V || K/V || K/V || K/V |                   |
|  -----  -----  -----  -----  -----                    |
 ------------------------------------------------------
```

* An event log that stores event (immutable K/V pairs) for a pre-defined lifetime

**Partitions**

```
        PARTITION 0
 ------------------------------------------------------
|  -----  -----  -----  -----  -----                    |
| | K/V || K/V || K/V || K/V || K/V |                   |
|  -----  -----  -----  -----  -----                    |
 ------------------------------------------------------
        PARTITION 1
 -------------------------------------------------------
|  -----  -----  -----  -----  -----                    |
| | K/V || K/V || K/V || K/V || K/V |                   |
|  -----  -----  -----  -----  -----                    |
 ------------------------------------------------------
        PARTITION 2
 -------------------------------------------------------
|  -----  -----  -----  -----  -----                    |
| | K/V || K/V || K/V || K/V || K/V |                   |
|  -----  -----  -----  -----  -----                    |
 ------------------------------------------------------
```

* Use constant hashing to distribute event from the same topic into partitions
* Run the key thought a hash function, and use the result to determine which partition
* Event with the same key will always be produced into the same partition
* For the same key, the ordering will always be in-order

**Replicas**

* A partition can have several replicas
* Each replica is on a different broker (server)
* For each partition, there will be a partition leader (R/W)
* Reading/Writing are only done on the leader

* Leader
  * Writes/Reads always goes to the lead replica
* Follower
  * Follower ask (query) the leader for new writes

* Observer replica
  * async replica from the leader
  * can read from observer replica

**Producer**

* A program that write event to a topic
* Producer can tune what constitute as a successful write

**Consumer**

* A program that retrieve event from a topic
* When an event is been consumed, the event does not get destroyed
* A consumer can have as many instance as the number of partition
* Consumer only sees fully replicated data

**Controller**

* a broker (server) in the kafka cluster (by default its the first leader )
* monitors the health of all other brokers
* coordinates partition leader election on broker failover
* communicates partition leader changes to other brokers

**Controller Election**
* `/controller` path in zookeeper
* when a broker boots up, it write metatdata into `/controller` as an ephemeral node
* when the controller dies, the session expires, and the next broker in-line become the controller

**Replication Protocol**

* in-sync: a replica is in-sync if it is not more than N messages or T seconds behind the leader
  * default: a replica falls out-of-sync after 10 seconds

* For each partition, ZK knows the leader, the epoch, and the In-Sync Replica list
* The high water marks is the highest replicated offset common to all partitions in the In-Sync Replica list
* scale to 10^4 consumer groups and 10^2 consumers per group
* Strongly consistent (client comply once a decision is made)

**Consumer Groups**

* Assign partitions automatically as consumers enter and leave the group
* Clients will decide partition assignment (so brokers remain stupid)

Actors of Consumer Groups:
* Consumers in the group
* the consumer group controller
* every broker acts as a controller, but every group has exactly one controller

Operations:
* `FindCoordinator`
* `JoinGroup`
* `SyncGroup`
* `Heartbeat`
* `LeaveGroup`



## Question?
* Does Kafka use Zookeeper ?
* How does consumer groups works ?
* How does the underlying system work with ZK ?