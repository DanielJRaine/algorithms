pub fn two_crystal_balls(breaks: [bool; 10]) -> usize {
    let jump = breaks.len().isqrt();
    let mut i = jump;

    // advance by sqrt(n) until the ball breaks
    while let Ok(b) = breaks.iter().advance_by(i) {
        if breaks[i] { break }
        i += jump;
    };
    
    // then walk back 1 * sqrt(n)
    i -= jump;
    
    while !breaks[i] {
        i += 1;
    }
    
    return i+1;
}
