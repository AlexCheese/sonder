use super::*;
pub fn generate_standard_game() -> Game {
    use Pattern::*;
    let mut map = HashMap::with_capacity(6);
    map.insert('r', PieceType {
        patterns: vec![
            Repeat(-1),
            Attack(1),
            Direction(Coord(vec![1,0])),
            Direction(Coord(vec![-1,0])),
            Direction(Coord(vec![0,1])),
            Direction(Coord(vec![0,-1])),
        ]
    });
    map.insert('b', PieceType {
        patterns: vec![
            Repeat(-1),
            Attack(1),
            Direction(Coord(vec![1,1])),
            Direction(Coord(vec![-1,1])),
            Direction(Coord(vec![1,-1])),
            Direction(Coord(vec![-1,-1])),
        ]
    });
    map.insert('q', PieceType {
        patterns: vec![
            Inherit('r'),
            Inherit('b'),
        ]
    });
    map.insert('n', PieceType {
        patterns: vec![
            Repeat(1),
            Attack(1),
            Direction(Coord(vec![1,2])),
            Direction(Coord(vec![-1,2])),
            Direction(Coord(vec![2,1])),
            Direction(Coord(vec![-2,1])),
            Direction(Coord(vec![1,-2])),
            Direction(Coord(vec![-1,-2])),
            Direction(Coord(vec![2,-1])),
            Direction(Coord(vec![-2,-1])),
        ]
    });
    map.insert('p', PieceType {
        patterns: vec![
            Repeat(1),
            Action(MoveAction::SetFlag(1, false), Coord(vec![])),
            // team white
            Branch(vec![
                Condition(MoveCondition::Team(0), Coord(vec![]), true),
                // non-attacks
                Branch(vec![
                    Attack(0),
                    // normal move
                    Branch(vec![
                        Condition(MoveCondition::EdgeOfBoard(1), Coord(vec![0,1]), false),
                        Direction(Coord(vec![0,1])),
                    ]),
                    // first move
                    Branch(vec![
                        Condition(MoveCondition::HasMoved, Coord(vec![]), true),
                        Condition(MoveCondition::Piece, Coord(vec![0,1]), false),
                        Action(MoveAction::SetFlag(1, true), Coord(vec![])),
                        Direction(Coord(vec![0,2])),
                    ]),
                    // fucking en passant idk
                    Branch(vec![
                        Condition(MoveCondition::Enemy, Coord(vec![1,0]), true),
                        Condition(MoveCondition::Flag(1), Coord(vec![1,0]), true),
                        Action(MoveAction::Destroy, Coord(vec![1,0])),
                        Direction(Coord(vec![1,1])),
                    ]),
                    Branch(vec![
                        Condition(MoveCondition::Enemy, Coord(vec![-1,0]), true),
                        Condition(MoveCondition::Flag(1), Coord(vec![-1,0]), true),
                        Action(MoveAction::Destroy, Coord(vec![-1,0])),
                        Direction(Coord(vec![-1,1])),
                    ]),
                    // promotion YAYYY
                    Branch(vec![
                        Condition(MoveCondition::EdgeOfBoard(1), Coord(vec![0,1]), true),
                        Action(MoveAction::Transmute(vec!['q', 'r', 'b', 'n']), Coord(vec![])),
                        Direction(Coord(vec![0,1])),
                    ]),
                ]),
                // attacks
                Branch(vec![
                    Attack(1),
                    Branch(vec![
                        Condition(MoveCondition::Enemy, Coord(vec![1,1]), true),
                        Direction(Coord(vec![1,1])),
                    ]),
                    Branch(vec![
                        Condition(MoveCondition::Enemy, Coord(vec![-1,1]), true),
                        Direction(Coord(vec![-1,1])),
                    ]),
                ]),
            ]),
            // team black
            // TODO: copy paste everything from above lmao kill me
            Branch(vec![]),
        ]
    });
    map.insert('k', PieceType {
        patterns: vec![
            Repeat(1),
            Inherit('q'),
            // TODO: castling and other shit
        ]
    });
    Game {
        board_size: vec![8,8],
        piece_dict: map
    }
}