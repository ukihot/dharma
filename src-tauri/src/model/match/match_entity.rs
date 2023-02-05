pub struct Match {
    tournament: Tournament,
    id: MatchId,
    teams: Vec<Team>,
    raid: MatchRaid,
    score: MatchScore,
    started_at: DateTime<Utc>,
    finished_at: Option<DateTime<Utc>>,
}
