cargo build --release
cd parser
cargo build --release
cd ..
mv target/release/qtcompress qtcompress
mv parser/target/release/qtcompress_parse qtcompress_parse
