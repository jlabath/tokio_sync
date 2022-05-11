### Sample impact of 100ms sync call on performance of requests in tokio

The numbers will vary by hardware and are meant only for comparison to understand the impact

ulimit -n 10000
cargo run --release


### Test 1 when sync_sleep endpoint is hit

1. ./sync_sleep_loop.sh
2. start another shell
3. ulimit -n 10000
4. siege --time=20s -c 30 -b http://127.0.0.1:3030/hello

	Lifting the server siege...
	Transactions:		        5051 hits
	Availability:		      100.00 %
	Elapsed time:		       19.66 secs
	Data transferred:	        0.06 MB
	Response time:		        0.12 secs
	Transaction rate:	      256.92 trans/sec
	Throughput:		        0.00 MB/sec
	Concurrency:		       29.57
	Successful transactions:        5051
	Failed transactions:	           0
	Longest transaction:	        0.23
	Shortest transaction:	        0.00


### Test 2 when async_sleep endpoint is hit

1. ./async_sleep_loop.sh
2. start another shell
3. ulimit -n 10000
4. siege --time=20s -c 30 -b http://127.0.0.1:3030/hello

	Lifting the server siege...
	Transactions:		       16355 hits
	Availability:		      100.00 %
	Elapsed time:		       19.39 secs
	Data transferred:	        0.20 MB
	Response time:		        0.01 secs
	Transaction rate:	      843.48 trans/sec
	Throughput:		        0.01 MB/sec
	Concurrency:		        5.26
	Successful transactions:       16355
	Failed transactions:	           0
	Longest transaction:	        6.72
	Shortest transaction:	        0.00

### Conclusion

The tiny 100ms sleep was enough to reduce transaction rate from 843.48/s to 256.92/s. 
The /hello transaction rate declined by 70% when traffic to /sync_sleep was present.


### The same test repeated with sleep duration increased to 400ms

### Test 1 sync_sleep

	Lifting the server siege...
	Transactions:		        1412 hits
	Availability:		      100.00 %
	Elapsed time:		       19.85 secs
	Data transferred:	        0.02 MB
	Response time:		        0.41 secs
	Transaction rate:	       71.13 trans/sec
	Throughput:		        0.00 MB/sec
	Concurrency:		       29.42
	Successful transactions:        1412
	Failed transactions:	           0
	Longest transaction:	        0.83
	Shortest transaction:	        0.00

### Test 2 async sleep

	Lifting the server siege...
	Transactions:		       16371 hits
	Availability:		      100.00 %
	Elapsed time:		       19.92 secs
	Data transferred:	        0.20 MB
	Response time:		        0.01 secs
	Transaction rate:	      821.84 trans/sec
	Throughput:		        0.01 MB/sec
	Concurrency:		        5.49
	Successful transactions:       16371
	Failed transactions:	           0
	Longest transaction:	        6.72
	Shortest transaction:	        0.00

### Conclusion

The performance of /hello under async_sleep test was virtually unchanged at rate 821.84/s. 
The sync_sleep traffic caused the /hello performance to drop to a rate of 71.13/s.
The /hello transaction rate dropped by 92% when traffic to /sync_sleep was present.
