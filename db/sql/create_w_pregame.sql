PRAGMA FOREIGN_KEYS = ON;

CREATE TABLE IF NOT EXISTS W_PREGAME (
  PREGAME_ID INTEGER NOT NULL PRIMARY KEY,
  TOURNAMENT_CODE INTEGER NOT NULL REFERENCES D_TOURNAMENT(TOURNAMENT_CODE),
  TEAM_DOG_ID INTEGER NOT NULL REFERENCES D_TEAM(TEAM_ID),
  TEAM_WOLF_ID INTEGER NOT NULL REFERENCES D_TEAM(TEAM_ID),
  HELD_AT TEXT
);
