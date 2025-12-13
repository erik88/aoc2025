Time measured using [hyperfine](https://github.com/sharkdp/hyperfine), running

```shell
cargo build --release
hyperfine --warmup 3 --runs 10 './target/release/aoc2025 [day]'
```

**REQUIREMENTS FOR DAY10**

- Running through a *nix based shell
- Having **glpsol** available on path, in order to solve GMPL-files

**σ** measures the standard-deviation, meaning that 68% of runs are within `mean ± σ`.

|        | Mean time | σ       |
|--------|-----------|---------|
| Day 1  | < 5 ms    | -       |
| Day 2  | 90.1 ms   | 1.0 ms  |
| Day 3  | < 5 ms    | -       |
| Day 4  | 9.1 ms    | 0.1 ms  |
| Day 5  | < 5 ms    | -       |
| Day 6  | < 5 ms    | -       |
| Day 7  | < 5 ms    | -       |
| Day 8  | 95.0 ms   | 1.2 ms  |
| Day 9  | 61.5 ms   | 0.8 ms  |
| Day 10 | 280.4 ms  | 39.7 ms |
| Day 11 | < 5 ms    | -       |
| Day 12 | < 5 ms    | -       |