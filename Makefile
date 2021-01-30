
all:

bench:
	@rm -f z.bench.log
	cargo bench --no-run
	cargo bench --bench=bench-trait-obj -- -n | tee -a z.bench.log
	cargo bench --bench=bench-enum-obj -- -n | tee -a z.bench.log

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results
