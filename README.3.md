### The story of three tokio sleeps

The numbers will vary by hardware and are meant for only.

Sleep duration was 100ms

    ulimit -n 30000
    cargo run --release


### Test 1 when async_sleep endpoint is hit

1. ulimit -n 30000
2. siege --time=20s -c 100 -b http://127.0.0.1:3030/async_sleep

```
Lifting the server siege...
Transactions:		       16385 hits
Availability:		      100.00 %
Elapsed time:		       19.37 secs
Data transferred:	        0.03 MB
Response time:		        0.10 secs
Transaction rate:	      845.90 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		       88.72
Successful transactions:       16385
Failed transactions:	           0
Longest transaction:	        0.13
Shortest transaction:	        0.10
```

### Test 2 the sync_sleep_nb endpoint is hit

1. ulimit -n 30000
2. siege --time=20s -c 100 -b http://127.0.0.1:3030/sync_sleep_nb

```
Lifting the server siege...
Transactions:		       16400 hits
Availability:		      100.00 %
Elapsed time:		       19.08 secs
Data transferred:	        0.03 MB
Response time:		        0.11 secs
Transaction rate:	      859.54 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		       97.29
Successful transactions:       16400
Failed transactions:	           0
Longest transaction:	        0.15
Shortest transaction:	        0.10
```

### Test 3 the sync_sleep endpoint is hit

1. ulimit -n 30000
2. siege --time=20s -c 100 -b http://127.0.0.1:3030/sync_sleep

```
Lifting the server siege...
Transactions:		         179 hits
Availability:		      100.00 %
Elapsed time:		       19.37 secs
Data transferred:	        0.00 MB
Response time:		        7.81 secs
Transaction rate:	        9.24 trans/sec
Throughput:		        0.00 MB/sec
Concurrency:		       72.19
Successful transactions:         179
Failed transactions:	           0
Longest transaction:	       11.14
Shortest transaction:	        0.12
```

### Conclusion

There was no meaningful difference between the async version and tokio::spawn_blocking version. In fact the spawn_blocking version performed little better, which is strange given that it has the overhead of spawning threads.
If I had to speculate tokio used a thread pool of some kind.

The sync_sleep was a failure as was expected. Where the other two endpoints managed to serve 16K+ requests. sync sleep only managed 179.
