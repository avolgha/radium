**Warning**: This project is _work in progress_. It is for learning Rust and for some other things coming with Rust so it won't be perfect at all and some code might be really bad as well.
|-----------------------------------------|

# radium

a simple package manager written in Rust for learning purposes.  
Feel free to add anything to the project or to improve the parts currently available.

## Building (simple)

For a simple build task, you can use the install script provided within this repository under `tools/install.sh`.

You can use this script under this comment:

```shell
$ curl https://github.com/avolgha/radium/blob/dev/tools/install.sh?raw=true | bash
```

This will clone the repository into the `/opt` directory and build the executable. It will install the shell completion as well.  
If you like, you can change the directory where the repository is cloned into by providing a `OUTPUT_PATH` variable in front of the script:

```shell
$ curl https://github.com/avolgha/radium/blob/dev/tools/install.sh?raw=true >> install.sh
$ OUTPUT_PATH="~/Downloads/radium" ./install.sh
```

## Building

<details>
  <summary>Requirements</summary>
  
  >1. rustc
  >2. cargo
  >
  >You can install both through [rustup](https://rustup.rs/)
</details>

1. first you need to clone the repository onto your local machine

```shell
$ git clone https://github.com/avolgha/radium.git
$ cd radium
```

2. then you need to build the executable

```shell
$ make radium
```

3. **optionally**, you can now "install" the binary to run it from the path

```shell
$ mv target/release/radium ~/.local/bin/
```

## Shell completion

The shell completion script for bash is already generated.  
It is located under `completions/completion.bash`.

To install it, run `source completions/completion.bash`.

If you want to add it automatically run these commands:

```shell
$ mkdir ~/.completions
$ cp completions/completion.bash ~/.completions/radium.bash
```

Then you have to add this to your bashrc:

```bash
source ~/.completions/radium.bash
```

Now the shell completion is loaded on shell startup.

## Support

You can get help for this project by opening an issue or writing me on Discord: `Marius#0686`.
