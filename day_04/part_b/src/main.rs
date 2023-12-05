pub fn main() {
    // Read the content of the file "../input.txt" into a byte array
    let input = include_bytes!("../input.txt");

    // Find the position of the first colon ':' and the first pipe '|' in the input
    let col = input.iter().position(|&b| b == b':').unwrap();
    let sep = input.iter().position(|&b| b == b'|').unwrap();

    // Create an array 'factors' with 256 elements, each initialized to 1usize
    let mut factors = [1usize; 256];

    // Print the result of the computation explained below
    println!(
        "{}",
        // Split the input byte array by newline characters '\n'
        input
            .split(|&b| b == b'\n')
            // Enumerate over the split lines with their index 'i'
            .enumerate()
            // For each enumerated line ('game'), perform the following operations
            .map(|(i, game)| {
                // Retrieve the factor for the current line 'i'
                let factor = factors[i];
                // Extract the winning sequence from the game (from col+1 to sep)
                let win_seq = &game[col + 1..sep];
                // Extract the remaining part of the game after the pipe '|' character
                let win_count = game[sep + 1..]
                    // Split the remaining part into chunks of 3 bytes
                    .chunks_exact(3)
                    // For each chunk, take the sub-slice starting from the second byte
                    .map(|n| &n[1..])
                    // Filter chunks where there is at least one winning sequence
                    .filter(|n| win_seq.chunks_exact(3).map(|n| &n[1..]).any(|c| &c == n))
                    // Count the number of winning sequences
                    .count();

                // Update the factors array for the subsequent lines
                (i..i + win_count).for_each(|i| factors[i + 1] += factor);

                // Calculate the score for the current line and return it
                factor * win_count + 1
            })
            // Sum up all the scores to get the final result
            .sum::<usize>()
    );
}