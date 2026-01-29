#!/bin/bash
find programs -name "Cargo.toml" -exec sed -i 's/anchor-spl = "0.30.1"/anchor-spl = "=0.30.0"/g' {} +
find programs -name "Cargo.toml" -exec sed -i 's/anchor-spl = "0.30.0"/anchor-spl = "=0.30.0"/g' {} +
