sudo rm -rf ~/sapling
git clone git@github.com:NavinShrinivas/StaticType.git ~/sapling
cd ~/sapling 
cargo build --release
sudo rm -rf /usr/local/src/sapling
sudo mkdir -p /usr/local/src/sapling
sudo cp -r ~/sapling/* /usr/local/src/sapling/
cd /usr/local/src/sapling
sudo rm -rf /usr/bin/sapling
sudo ln -sf /usr/local/src/sapling/target/release/sapling /usr/bin/sapling 
sudo chmod +x /usr/bin/sapling
