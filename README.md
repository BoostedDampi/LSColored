# 📁 LSColored 🌈

## Introduction
Welcome to LSColored, a custom implementation of the traditional `ls` command written in Rust 🦀. This project was developed as a fun and educational journey into Rust programming. It replicates the functionality of the classic directory listing command but also introduces improved colored output 🎨 and several additional features to enhance the user experience.

## Features
- **Enhanced Colored Output**: Utilizes a rich color palette to distinguish file types, permissions, and other attributes.
- **Additional Listing Options**: Includes new flags and parameters to customize the directory listing.
- **User-Friendly Interface**: Improved readability and user interaction.

## Installation
To install LSColored, ensure you have Rust and Cargo 📦 installed on your machine. Follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/BoostedDampi/LSColored.git
   ```
2. Change to the directory:
   ```sh
   cd LSColored
   ```
3. Build and install:
   ```sh
   cargo install --path .
   ```

## Usage
After installation, LSColored can be used similarly to the traditional `ls` command. Here's how you can use it:

- List files in the current directory:
  ```sh
  lsc
  ```
- List files with detailed information:
  ```sh
  lsc -l
  ```
- List files with more detailed information:
  ```sh
  lsc -ll
  ```
- Show hidden files:
  ```sh
  lsc -a
  ```

For more options, refer to the help:
```sh
lsc --help
```

## ToDo 📝
- **Better Color Profile Editing**: User editabele color profiles. [Updated color library, TOML color profile not yet implemented]
- **Subfolders Listing**: Add an option to list files in subfolders recursively. [DONE but slow, needs improvements]
- **Dynamic Directory Traversal**: Enable dynamic navigation through directories. [IN PLANNING]
- **More Metadata in Verbose Mode**: Display additional file metadata when using verbose mode (-ll).
- **Add Tests**: Add a series of tests for simplified development and updating.
- **Better Error Handling**: I need better error handling in the code.

## License
LSColored is released under the GNU GPLv3 License. See the [LICENSE](LICENSE) file for more details.

---

Enjoy your colorful directory listings with LSColored! 🎉📂
