TASKSET = taskset -c 2

all:

bench:
	@rm -f z.bench.log
	cargo bench --no-run
	$(TASKSET) cargo bench --bench=bench-trait-obj -- -n | tee -a z.bench.log
	$(TASKSET) cargo bench --bench=bench-enum-obj -- -n | tee -a z.bench.log

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results
