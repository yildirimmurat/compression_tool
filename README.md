# Compression Tool

This is a simple file compression and decompression tool that uses Huffman coding to compress files. The tool supports two primary operations:

- **Compression**: Compresses an input file and stores the result in an output file.
- **Decompression**: Decompresses a compressed file and stores the result in an output file.

The tool is used through the command line with the following commands:

- `cczip` for compression
- `ccunzip` for decompression

## Steps to Create and Use the Compression Tool

### 1. Compile the Program

First, build the program by running the following command:

```bash
cargo build --release
```

### 2. Move the executables to a directory in your PATH

To make the tool accessible from anywhere in your terminal, move the compiled executables to a directory that is included in your system's PATH.

```bash
sudo cp target/release/cczip /usr/local/bin/
sudo cp target/release/ccunzip /usr/local/bin/
```

### 3. Verify the installation

To verify the installation, check if the executables are accessible from anywhere in your terminal:

```bash
which cczip
which ccunzip
```


### 4. Compress a file using cczip

```bash
cczip test.txt [test.zip]
```

### 5. Decompress a file using ccunzip

```bash
cczip test.dat [test.txt]
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests to improve the tool. To contribute:

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Submit a pull request.


