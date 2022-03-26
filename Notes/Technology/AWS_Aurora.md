# Amazon Aurora Multi-Master: Scaling out database write performance

## Database Architecture (Relational)

### Traditional Stack
```
 ---------------
| Compute Logic |
| ------------- |
| Local Storage |
 ---------------
```

**Monolithic Stack:** the compute and storage are done on the same instances

Compute
* SQL
* Transaction
* Caching
* Logging

Storage
* Local Storage

```
  ---------------
 | Compute Logic |
  ---------------
         |
  ----------------
| Network Storage |
 -----------------
```

**Decoupled Storage:** use network storage instead of local storage

### Existing Multi-Master Solution

**Distributed Lock Manager**
* Heavyweight synchronization: Pessimistic and negative scaling

**Global ordering unit**
* single point of failure
* global entity: scaling bottleneck

**Paxos**
* heavy weight consensus protocol

## AWS Aurora

* Log applicator is done in the storage
  * the compute (database) node never construct the pages
  * the compute node send log entry to the storage
  * the storage node construct the pages
  * reduce write amplification
* 4/6 write quorum and 3/6 read quorum
  * fault tolerance
  * reduce performance jitter
  * read scale out
  * instant database recovery
* Storage
  * distributed to 3 AZ
  * 2 copy per AZ
  * divided into 10 GB chunks

AWS Aurora vs MySQL with Replica
* AWS Aurora
  * write logs to the storage
  * exchange page delta with other master
* MySQL
  * write logs, binlogs, data, double-write, FRM files to storage
  * write logs, binlogs, data, double-write, FRM files to replica


### Multi-Master

* write scale out
* continuous availability

### Scale Out

#### Single Master

```
  ---------------    Page Cache   ----------------
 | Aurora Master |   Update      | Aurora Replica |
 | ------------- | ------------> | -------------- |
 | Read + write  |               | Read Only      |
  ---------------                 ----------------
         |                               ^
         v                               |
  ------------------------------------------------
|               Shared Storage Volume             |
 -------------------------------------------------
```

* Page cache updated using physical delta changes
* No writes on replicas
* Shared storage

#### Multi Master

```
  ---------------    Page Cache   ---------------
 | Aurora Master |   Update      | Aurora Master |
 | ------------- | ------------> | ------------- |
 | Read + write  | <------------ | Read + Write  |
  ---------------                 ---------------
         |                               |
         v                               v
  ------------------------------------------------
|               Shared Storage Volume             |
 -------------------------------------------------
```

* Page cache updated using physical delta changes
* All instances can execute write transaction
* Shared storage with *optimistic conflict detection*

* each master sees its own writes
* write from other nodes are subject to replication delay
* transaction are only committed if there is no conflict