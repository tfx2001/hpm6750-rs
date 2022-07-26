svd2rust -i svd/HPM6750.svd --target riscv --const_generic --keep_list --nightly
rm -rf src
form -i lib.rs -o src/ 
rm lib.rs
cargo fmt
