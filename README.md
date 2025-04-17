# Simple-Rust-File-Manager

A Command Line Interface (CLI)-based file management tool written in **Rust**, designed to perform basic file operations. This project showcases the use of Rust's standard library for file and directory management.

## Features

-   **File Creation**: Create a new file.
-   **File Renaming**: Rename an existing file.
-   **File Deletion**: Delete a specified file.
-   **Directory Listing**: List all files and directories in the current working directory.

## Getting Started

Follow these instructions to get the project set up and running on your local machine.

### Prerequisites

-   **Rust**: You need Rust installed on your machine. If you don't have it, follow the official installation instructions [here](https://www.rust-lang.org/tools/install).

### Installation

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/AnikethJana/Simple-Rust-File-Manager.git
    ```
2.  **Navigate into the directory:**
    ```sh
    cd Simple-Rust-File-Manager
    ```
3.  **Build the application:**
    For development:
    ```sh
    cargo build
    ```
    For an optimized release version:
    ```sh
    cargo build --release
    ```

4. **Run the Application**
     To run the application directly using Cargo, navigate to the project directory and execute:
    ```sh
    cargo run
    ```
### Usage
Upon running the application, you will be presented with the following options in your terminal:

- Create a file: Prompts you to enter the name for the new file to be created in the current directory.
- Rename a file: Asks for the current name of the file and then the new name you want to assign to it.
- Delete a file: Prompts you to enter the name of the file you wish to delete from the current directory.
- List files in directory: Displays a list of all files and directories present in the current working directory.
- Exit: Closes the application.
- The tool provides feedback and error messages for invalid inputs or file operations, such as when files don't exist or permissions are insufficient.
