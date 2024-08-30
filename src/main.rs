// Function to compute the greatest common divisor
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to find the elements of Z_m* (multiplicative group)
fn multiplicative_elements(m: u128) -> Vec<u128> {
    let mut elements = Vec::new();
    for x in 1..m {
        if gcd(x, m) == 1 {
            elements.push(x);
        }
    }
    elements
}

// Function to check if a set is closed under multiplication modulo m
fn is_subgroup(subset: &Vec<u128>, m: u128) -> bool {
    let identity = 1;
    if !subset.contains(&identity) {
        return false;
    }

    for &x in subset {
        for &y in subset {
            if !subset.contains(&(x * y % m)) {
                return false;
            }
        }
    }
    true
}

// Function to find all valid subgroups of Z_m*
fn multiplicative_subgroups(m: u128) -> Vec<Vec<u128>> {
    let elements = multiplicative_elements(m);
    let len = elements.len();
    let mut subgroups = Vec::new();

    for i in 1..(1 << len) {
        let mut subset = Vec::new();
        for j in 0..len {
            if i & (1 << j) != 0 {
                subset.push(elements[j]);
            }
        }
        if is_subgroup(&subset, m) {
            subgroups.push(subset);
        }
    }
    subgroups
}

// Function to find divisors of n
fn divisors(n: u128) -> Vec<u128> {
    (1..=n).filter(|&d| n % d == 0).collect()
}

// Function to find the additive subgroups of Z_n
fn additive_subgroups(n: u128) -> Vec<Vec<u128>> {
    let divs = divisors(n);
    divs.iter().map(|&d| (0..n).step_by(d as usize).collect()).collect()
}

// Main function
fn main() {
    let n: u128 = 4; // Replace with desired value
    let m: u128 = 6; // Replace with desired value

    let add_subgroups = additive_subgroups(n);
    let mult_subgroups = multiplicative_subgroups(m);

    println!("Additive subgroups of Z_{}: {:?}", n, add_subgroups);
    println!("Multiplicative subgroups of Z_{}^*: {:?}", m, mult_subgroups);

    let mut subgroups = Vec::new();

    for add_subgroup in add_subgroups {
        for mult_subgroup in &mult_subgroups {
            let mut combined_subgroup = Vec::new();
            for &a in &add_subgroup {
                for &b in mult_subgroup {
                    combined_subgroup.push((a, b));
                }
            }
            subgroups.push(combined_subgroup);
        }
    }

    println!("Subgroups of Z_{} x Z_{}^*: {:?}", n, m, subgroups);
}
