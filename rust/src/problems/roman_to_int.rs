use super::solution::Solution;


impl Solution {

    //     Symbol       Value
    //     I             1
    //     V             5
    //     X             10
    //     L             50
    //     C             100
    //     D             500
    //     M             1000
    // I can be placed before V (5) and X (10) to make 4 and 9. 
    // X can be placed before L (50) and C (100) to make 40 and 90. 
    // C can be placed before D (500) and M (1000) to make 400 and 900.
    pub fn roman_to_int(s: String) -> i32 {
        let (mut sum, mut prev) = (0, 0);
        for c in s.chars() {
            match c {
                'I' => {
                    sum += 1;
                    prev = 1;
                },
                'V' => {
                    if prev == 1 {
                        sum +=3
                    }else {
                        sum +=5;
                    }
                    prev = 5;
                }
                'X' => {
                    sum+=10;
                    prev = 10;
                },
                'L' => {
                    if prev == 10{
                        sum += 30;
                    } else {
                        sum += 50;
                    }
                    prev = 50;
                },
                'C' => {
                    if prev == 10 {
                        sum += 80;
                        prev = 50;
                    }else {
                        sum += 100;
                        prev = 100;
                    }
                },

                'D' => {
                    if prev == 100 {
                        sum += 300;
                    }else {
                        sum += 500;
                    }
                    prev = 500;
                },
                'M' => {
                    if prev == 100 {
                        sum += 800;
                    } else {
                        sum += 1000;
                    }
                    prev = 1000;

                },
                _ => {}
            }
        }

        sum
    }
}