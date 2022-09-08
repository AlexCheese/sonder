use super::*;
pub fn generate_standard_game() -> Game {
    use Pattern::*;
    use MoveAction::*;
    use MoveCondition::*;
    let mut map = HashMap::with_capacity(6);
    map.insert('r', super::PieceType {
        patterns: vec![
            Repeat(-1),
            Attack(1),
            Direction(Coord(vec![1,0])),
            Direction(Coord(vec![-1,0])),
            Direction(Coord(vec![0,1])),
            Direction(Coord(vec![0,-1])),
        ]
    });
    map.insert('b', super::PieceType {
        patterns: vec![
            Repeat(-1),
            Attack(1),
            Direction(Coord(vec![1,1])),
            Direction(Coord(vec![-1,1])),
            Direction(Coord(vec![1,-1])),
            Direction(Coord(vec![-1,-1])),
        ]
    });
    map.insert('q', super::PieceType {
        patterns: vec![
            Inherit('r'),
            Inherit('b'),
        ]
    });
    map.insert('n', super::PieceType {
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
    map.insert('p', super::PieceType {
        patterns: vec![
            Repeat(1),
            Action(SetFlag(1, false), Coord(vec![])),
            // team white
            Branch(vec![
                Condition(Team(0), Coord(vec![]), true),
                // non-attacks
                Branch(vec![
                    Attack(0),
                    // normal move
                    Branch(vec![
                        Condition(EdgeOfBoard(1), Coord(vec![0,1]), false),
                        Direction(Coord(vec![0,1])),
                    ]),
                    // first move
                    Branch(vec![
                        Condition(HasMoved, Coord(vec![]), true),
                        Condition(Piece, Coord(vec![0,1]), false),
                        Action(SetFlag(1, true), Coord(vec![])),
                        Direction(Coord(vec![0,2])),
                    ]),
                    // fucking en passant idk
                    Branch(vec![
                        Condition(Enemy, Coord(vec![1,0]), true),
                        Condition(Flag(1), Coord(vec![1,0]), true),
                        Action(Destroy, Coord(vec![1,0])),
                        Direction(Coord(vec![1,1])),
                    ]),
                    Branch(vec![
                        Condition(Enemy, Coord(vec![-1,0]), true),
                        Condition(Flag(1), Coord(vec![-1,0]), true),
                        Action(Destroy, Coord(vec![-1,0])),
                        Direction(Coord(vec![-1,1])),
                    ]),
                    // promotion YAYYY
                    Branch(vec![
                        Condition(EdgeOfBoard(1), Coord(vec![0,1]), true),
                        Action(Transmute(vec!['q', 'r', 'b', 'n']), Coord(vec![])),
                        Direction(Coord(vec![0,1])),
                    ]),
                ]),
                // attacks
                Branch(vec![
                    Attack(1),
                    Branch(vec![
                        Condition(Enemy, Coord(vec![1,1]), true),
                        Direction(Coord(vec![1,1])),
                    ]),
                    Branch(vec![
                        Condition(Enemy, Coord(vec![-1,1]), true),
                        Direction(Coord(vec![-1,1])),
                    ]),
                ]),
            ]),
            // team black
            Branch(vec![]),
        ]
    });
    map.insert('k', super::PieceType {
        patterns: vec![
            Repeat(1),
            Inherit('q'),
            Condition(HasMoved, Coord(vec![]), false),
            Condition(Safe, Coord(vec![]), true),
            Branch(vec![
                Condition(PieceType('r'), Coord(vec![3,0]), true),
                Condition(HasMoved, Coord(vec![3,0]), false),
                Condition(Piece, Coord(vec![1,0]), false),
                Condition(Piece, Coord(vec![2,0]), false),
                Condition(Safe, Coord(vec![1,0]), true),
                Condition(Safe, Coord(vec![2,0]), true),
                Action(Teleport(Coord(vec![3,0])), Coord(vec![1,0])),
                Direction(Coord(vec![2,0])),
            ]),
            Branch(vec![
                Condition(PieceType('r'), Coord(vec![-4,0]), true),
                Condition(HasMoved, Coord(vec![-4,0]), false),
                Condition(Piece, Coord(vec![-1,0]), false),
                Condition(Piece, Coord(vec![-2,0]), false),
                Condition(Piece, Coord(vec![-3   ,0]), false),
                Condition(Safe, Coord(vec![-1,0]), true),
                Condition(Safe, Coord(vec![-2,0]), true),
                Condition(Safe, Coord(vec![-3,0]), true),
                Action(Teleport(Coord(vec![-4,0])), Coord(vec![-1,0])),
                Direction(Coord(vec![-2,0])),
            ]),
        ]
    });
    Game {
        board_size: vec![8,8],
        piece_dict: map
    }
}