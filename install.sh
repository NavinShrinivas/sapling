sudo rm -rf ~/sapling
git clone git@github.com:NavinShrinivas/sapling.git ~/sapling
cd ~/sapling 
cargo build --release
sudo rm -rf /usr/local/src/sapling
sudo mkdir -p /usr/local/src/sapling
sudo cp -r ~/sapling/* /usr/local/src/sapling/
cd /usr/local/src/sapling
sudo mkdir -p /usr/local/bin/
sudo rm -rf /usr/local/bin/sapling
sudo ln -sf /usr/local/src/sapling/target/release/sapling /usr/local/bin/sapling 
sudo chmod +x /usr/local/bin/sapling
