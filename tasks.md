- [ ] Frontend
	- [ ] Finalize framework
	- [ ] Define screens (concrete wireframes)
	- [ ] Develop them
- [ ] Refactor Code
	- [ ] Change the directory structure
	- [ ] Code architecture
		- [ ] Define proper types
		- [ ] Define proper traits
	- [ ] Make it run end to end (this includes)
		- [ ] From the CLI/API/SDK
		- [ ] Call the external vectorizer s.mdervice
		- [ ] Pass embeddings through the processes/services via protobufs
		- [ ] Data should be stored in rocksdb (embedding and original data)
- [ ] Additional Tasks
	- [ ] Implement better indexing algorithms
	- [ ] Endpoints
		- [ ] gRPC
		- [ ] REST
	- [ ] PCA
	- [ ] Final Design and implementation



### Short term tasks
- [ ] Implement Storage Engine trait for RocksdbStorage and also implement RocksdbStorage
- [ ] KD Tree refactor according to new architechure
- [ ] Write a mock API for the project (will be volatile)
- [ ] Implement in memory - LSM tree, ACID
