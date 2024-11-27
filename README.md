Steps to create a compression tool Unix Command (compression_tool)

- Compile the program
```bash 
cargo build --release
```

- Move the executable to a directory in your PATH
```bash
sudo cp ~/projects/compression_tool/target/release/compression_tool /usr/local/bin/
```

- Verify the installation
```bash
compression_tool test.txt
```

- Optional: Create a symbolic link
```bash
sudo ln -s ~/projects/compression_tool/target/release/compression_tool /usr/local/bin/compression_tool
```

- Check if it is working
```bash
which compression_tool
```