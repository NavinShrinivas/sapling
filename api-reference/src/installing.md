# Installing 

There are some pre-requisites to install the project. You need to have the following installed:

- Cargo

To check whether you have cargo installed, run the following command in your terminal:
```bash
cargo --version
```
> You should see something like this:

```bash
cargo 1.84.1
```

> Note : I am trying to distribute the binary in the future, but for now, you need to have cargo installed.

After making sure you have cargo installed, just run this : 
```bash
bash <(curl -Ls https://raw.githubusercontent.com/NavinShrinivas/sapling/refs/heads/main/install.sh)
```
This by default installs the latest version of `sapling` in your system. You can run the same command as above to update your sapling version as well. 

All version of sapling WILL be backwards compatible. So you can use the latest version of sapling without worrying about breaking you existing website.

Obviously, websites generated using sapling will be backwards compatible only with the same `settings.yaml` from before as all new features if breaking will be added behind a settings flag.
