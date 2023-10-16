/// Représente un morceau de musique avec une note et un titre.
pub struct Song {
    rank: u32,
    title: String,
}

/// Retourne la note moyenne d'un ensemble de morceaux.
pub fn average_rank(songs: &Vec<Song>) -> f64 {
    let total_rank: u32 = songs.iter().map(|song| song.rank).sum();
    let num_songs = songs.len() as f64;
    total_rank as f64 / num_songs
}


/// Filtre les morceaux dans `songs` et ne garde que ceux dont la note est
/// strictement supérieure à `rank_min`.
pub fn filter_songs(songs: Vec<Song>, rank_min: u32) -> Vec<Song> {
    songs.into_iter().filter(|song| song.rank > rank_min).collect()
}

/// Filtre les morceaux dans `songs` pour ne conserver que ceux dont la note
/// est strictement supérieure à la moyenne.
pub fn good_songs(songs: Vec<Song>) -> Vec<Song> {
    let avg_rank = average_rank(&songs);
    songs.into_iter().filter(|song| song.rank > avg_rank).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<Song> {
        vec![
            Song {
                rank: 4,
                title: String::from("Stairway to Heaven"),
            },
            Song {
                rank: 2,
                title: String::from("Never Gonna Give You Up"),
            },
            Song {
                rank: 5,
                title: String::from("Nigerian Marketplace"),
            },
        ]
    }

    #[test]
    #[should_panic]
    fn empty() {
        let sgs = Vec::new();
        let _m = average_rank(&sgs);
    }

    #[test]
    fn moyenne_un_seul_morceau() {
        let sgs = vec![Song {
            rank: 4,
            title: String::from("Stairway to Heaven"),
        }];
        let m = average_rank(&sgs);
        assert_eq!(m, 4.0);
    }

    #[test]
    fn moyenne_plusieurs_morceau() {
        let sgs = example();
        let m = average_rank(&sgs);
        assert!((m - 3.66666).abs() < 0.001);
    }

    #[test]
    fn filtre() {
        let sgs = example();
        let f = filter_songs(sgs, 4);
        assert!(f.len() == 1);
        assert_eq!(f[0].rank, 5);
        assert_eq!(f[0].title, "Nigerian Marketplace");
    }

    #[test]
    fn meilleurs() {
        let sgs = example();
        let f = good_songs(sgs);
        assert!(f.len() == 2);
        for m in &f {
            assert!(m.rank >= 4);
        }
    }
}
