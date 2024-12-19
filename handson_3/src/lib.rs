/// Plans the holiday to maximize the number of actractions that you can visit in a limited number of days.
/// 
/// # Parameters
/// - `cities`: A vector of cities, each represented as a vector of integers indicating the number of actractions for each day of the visit.
/// - `days`: The number of available days for the trip.
/// 
/// # Returns
/// Returns the maximum number of actractions that you can visit during the trip.
pub fn holiday_planning(cities: Vec<Vec<usize>>, days: usize) -> usize {
    // Table to store the maximum number of actractions for each available number of days.
    let mut table = vec![0; days + 1];

    for city in cities.iter() {
        let mut prefix_sum = vec![0; city.len() + 1];

        // Calculate the prefix sum of actractions for the current city.
        for day in 0..city.len() {
            prefix_sum[day + 1] = prefix_sum[day] + city[day];
        }

        // Update the table
        for d in (0..=days).rev() {
            for k in 1..=city.len().min(d) {
                table[d] = table[d].max(table[d - k] + prefix_sum[k]);
            }
        }
    }

    table[days]
}

/// Organizes a course by designing the maximum number of topics to cover such that each subsequent topic has a beauty and a difficulty 
/// that is not lower than the previous one.
/// 
/// # Parameters
/// - `topics`: A vector of tuples, where each tuple contains the beauty and the difficulty of the topic.
/// 
/// # Returns
/// Returns the maximum number of topics that can be covered in a non-decreasing order of beauty and difficulty.
pub fn design_course(mut topics: Vec<(usize, usize)>) -> usize {
    // Sort the topics first by beauty, and then by difficulty if the beauties are the same.
    topics.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    // The table for LIS algorithm
    let mut table: Vec<usize> = Vec::new();

    for (_, difficulty) in topics {
        match table.binary_search(&difficulty) {
            Ok(_) => {}
            Err(pos) => {
                if pos == table.len() {
                    table.push(difficulty);
                } else {
                    table[pos] = difficulty;
                }
            }
        }
    }

    table.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holiday_planning_0() {
        let attractions = holiday_planning(
            vec![
                vec![3, 2, 1, 4, 2, 4, 3, 4],
                vec![3, 3, 1, 2, 3, 5, 5, 3],
                vec![3, 4, 1, 5, 3, 3, 4, 1],
                vec![3, 1, 5, 4, 3, 4, 2, 5],
                vec![2, 5, 4, 4, 4, 5, 3, 4],
                vec![5, 1, 4, 4, 3, 2, 4, 5],
            ],
            8,
        );
        assert_eq!(attractions, 32);
    }

    #[test]
    fn holiday_planning_1() {
        let attractions = holiday_planning(
            vec![
                vec![1, 1, 1, 4, 2],
                vec![3, 3, 5, 3, 5],
                vec![2, 1, 4, 5, 1],
            ],
            5,
        );
        assert_eq!(attractions, 19);
    }

    #[test]
    fn holiday_planning_2() {
        let attractions = holiday_planning(
            vec![
                vec![4, 1, 1, 2],
                vec![1, 1, 0, 5],
                vec![5, 0, 1, 1],
                vec![2, 1, 0, 4],
                vec![3, 1, 0, 3],
            ],
            4,
        );
        assert_eq!(attractions, 14);
    }

    #[test]
    fn holiday_planning_3() {
        let attractions = holiday_planning(
            vec![
                vec![1, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 2, 0, 2, 2],
                vec![0, 1, 1, 0, 0, 1, 2, 2, 1, 1, 1, 1, 2, 2, 1],
            ],
            15,
        );
        assert_eq!(attractions, 16);
    }

    #[test]
    fn holiday_planning_4() {
        let attractions = holiday_planning(
            vec![
                vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 3],
                vec![0, 3, 0, 0, 0, 0, 0, 0, 4, 0],
                vec![0, 0, 2, 0, 0, 0, 0, 5, 0, 0],
                vec![0, 0, 0, 5, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 5, 4, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 4, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 5, 0, 0, 0],
                vec![0, 0, 5, 0, 0, 0, 0, 5, 0, 0],
                vec![0, 5, 0, 0, 0, 0, 0, 0, 5, 0],
                vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            ],
            10,
        );
        assert_eq!(attractions, 21);
    }

    #[test]
    fn design_course_0() {
        let topics = design_course(vec![(0, 3), (99, 1), (11, 20), (1, 2), (10, 5)]);
        assert_eq!(topics, 3);
    }

    #[test]
    fn design_course_1() {
        let topics = design_course(vec![(6, 4), (13, 11), (10, 8)]);
        assert_eq!(topics, 3);
    }

    #[test]
    fn design_course_2() {
        let topics = design_course(vec![(3, 3), (3, 3)]);
        assert_eq!(topics, 1);
    }

    #[test]
    fn design_course_3() {
        let topics = design_course(vec![
            (44, 49),
            (15, 35),
            (38, 21),
            (55, 93),
            (14, 29),
            (50, 52),
            (94, 76),
            (89, 84),
            (30, 96),
            (41, 14),
            (17, 38),
            (30, 14),
            (21, 100),
            (12, 78),
            (86, 87),
        ]);
        assert_eq!(topics, 6);
    }

    #[test]
    fn design_course_4() {
        let topics = design_course(vec![
            (54, 56),
            (66, 50),
            (74, 97),
            (52, 23),
            (62, 74),
            (27, 56),
            (73, 24),
            (11, 47),
            (32, 83),
            (51, 29),
            (12, 74),
            (4, 48),
            (51, 22),
            (82, 82),
            (1, 24),
        ]);
        assert_eq!(topics, 5);
    }

    #[test]
    fn design_course_5() {
        let topics = design_course(vec![
            (56, 90),
            (61, 30),
            (82, 62),
            (60, 44),
            (72, 58),
            (20, 80),
            (46, 79),
            (39, 15),
            (67, 46),
            (64, 63),
            (72, 9),
        ]);
        assert_eq!(topics, 5);
    }

    #[test]
    fn design_course_6() {
        let topics = design_course(vec![
            (64, 56),
            (51, 51),
            (61, 74),
            (88, 53),
            (1, 15),
            (50, 81),
            (43, 24),
            (53, 78),
            (6, 34),
            (33, 46),
            (27, 1),
            (9, 37),
            (18, 47),
            (38, 21),
            (69, 95),
        ]);
        assert_eq!(topics, 7);
    }

    #[test]
    fn design_course_7() {
        let topics = design_course(vec![
            (22, 19),
            (63, 24),
            (30, 16),
            (44, 80),
            (48, 17),
            (29, 66),
            (13, 13),
            (63, 27),
            (49, 9),
            (93, 75),
            (38, 47),
            (33, 34)
        ]);
        assert_eq!(topics, 6);
    }

    #[test]
    fn design_course_8() {
        let topics = design_course(vec![
            (33, 5),
            (52, 5),
            (33, 54),
            (80, 11),
            (12, 78),
            (62, 2),
            (17, 1),
            (66, 79),
            (94, 30),
            (54, 14),
            (28, 17),
            (100, 70)
        ]);
        assert_eq!(topics, 5);
    }

    #[test]
    fn design_course_9() {
        let topics = design_course(vec![
            (80, 88),
            (7, 62),
            (60, 14),
            (27, 60),
            (95, 66),
            (68, 71),
            (10, 76),
            (14, 87),
            (6, 92),
            (81, 81),
            (80, 90)
        ]);
        assert_eq!(topics, 5);
    }

    #[test]
    fn design_course_10() {
        let topics = design_course(vec![
            (30, 73),
            (4, 89),
            (94, 60),
            (61, 22),
            (30, 16),
            (66, 60),
            (27, 87),
            (75, 8),
            (91, 33),
            (69, 78),
            (41, 69),
            (70, 12),
            (88, 76),
            (91, 92)
        ]);
        assert_eq!(topics, 5);
    }
}
