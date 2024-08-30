## Overview
This Rust program computes and displays the subgroups of the direct product $\mathbb{Z}_n\times\mathbb{Z}_m^\*$, where $\mathbb{Z}_n$ is the additive group of integers modulo $n$, and 
$\mathbb{Z}_m^\*$ is the multiplicative group of integers modulo $m$ that are coprime with $m$. The program can find the additive subgroups of $\mathbb{Z}_n$ , the multiplicative subgroups of 
$\mathbb{Z}_m^\*$ , and their direct product.
## Features
- **GCD Calculation:** Computes the greatest common divisor $\gcd$ of two integers.
- **Additive Subgroups:** Finds all additive subgroups of $\mathbb{Z}_n$  based on the divisors of $n$.
- **Multiplicative Subgroups:** Identifies all subgroups of $\mathbb{Z}_m^\*$ by checking the closure under multiplication modulo $m$.
- Direct Product Subgroups: Combines the additive and multiplicative subgroups to form subgroups of the direct product $\mathbb{Z}_n\times\mathbb{Z}_m^\*$.
## How It Works
- **gcd(a: u128, b: u128) -> u128:** Computes the greatest common divisor $\gcd$ of two integers $a$ and $b$ using the Euclidean algorithm.
- **multiplicative_elements(m: u128) -> Vec<u128>:** Identifies all elements in $\mathbb{Z}_m^\*$, the set of integers from $1$ to $m-1$ that are coprime with $m$.
- **is_subgroup(subset: &Vec<u128>, m: u128) -> bool:** Checks if a subset of $\mathbb{Z}_m^\*$ is a valid subgroup by verifying the presence of the identity element (1) and closure under multiplication modulo m.
- **multiplicative_subgroups(m: u128) -> Vec<Vec<u128>>:** Generates all subgroups of $\mathbb{Z}_m^\*$ by considering all possible subsets and verifying if each subset is a valid subgroup.
- **divisors(n: u128) -> Vec<u128>:** Finds all divisors of $n$ by checking which numbers from $1$ to $n$ divide $n$ evenly.
- **additive_subgroups(n: u128) -> Vec<Vec<u128>>:** Generates all additive subgroups of $\mathbb{Z}_n$ based on the divisors of $n$.
- **main():** Combines the additive subgroups of $\mathbb{Z}_n$ and the multiplicative subgroups of $\mathbb{Z}_m^\*$to form subgroups of $\mathbb{Z}_n\times\mathbb{Z}_m^\*$, and prints the results.
## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- You can change the values of $n$ and $m$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
- This will compile and run the program, printing the subgroups of the specified $\mathbb{Z}_n\times\mathbb{Z}_m\^*$.
## Customizing
- To compute subgroups for a different value of $n$ and $m$, simply modify the value of $n$ and $m$ in the main function.
## Example Output, Given $n=4$ and $m=6$:
>```
>Additive subgroups of Z_4: [[0, 1, 2, 3], [0, 2], [0]]
>Multiplicative subgroups of Z_6^*: [[1], [1, 5]]
>Subgroups of Z_4 x Z_6^*: [[(0, 1), (1, 1), (2, 1), (3, 1)], [(0, 1), (0, 5), (1, 1), (1, 5), (2, 1), (2, 5), (3, 1), (3, 5)], [(0, 1), (2, 1)], [(0, 1), (0, 5), (2, 1), (2, 5)], [(0, 1)], [(0, 1), (0, 5)]]

- This output lists all subgroups of $\mathbb{Z}_4\times\mathbb{Z}_6$^*, with each subgroup represented as a sorted list of elements.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Subgroups_of_a_Product_Mixed_Groups.git
   cd Subgroups_of_a_Product_Mixed_Groups
