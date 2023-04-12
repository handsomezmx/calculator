# Command-Line Calculator

A simple command-line calculator written in Rust that supports addition, subtraction, multiplication, and division.

## Getting Started

These instructions will help you set up and run the command-line calculator on your local machine.

### Prerequisites

To run this project, you'll need Rust and Cargo installed on your system. If you don't have them already, follow the instructions on the official Rust website: https://www.rust-lang.org/tools/install

### Installing

First, clone the repository or download the source code:

git clone 

Change to the project directory:

cd calculator

Build the project:

cargo build

### Running the Calculator

To run the calculator, use the following command format:

cargo run -- --operation <OPERATION> <NUMBER_1> <NUMBER_2>

Replace <OPERATION> with one of the following: add, sub, mul, or div. Replace <NUMBER_1> and <NUMBER_2> with the numbers you want to perform the operation on.

#### Examples

Addition:

cargo run -- --operation add 2 3

Subtraction:

cargo run -- --operation sub 5 2

Multiplication:

cargo run -- --operation mul 3 4

Division:

cargo run -- --operation div 10 2

