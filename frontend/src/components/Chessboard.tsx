import {GameTypes} from "../machines/game";

interface ChessboardProps {
    chessboard: GameTypes.Chessboard;
}

const RANK_WIDTH = 8;

export const Chessboard = (props: ChessboardProps) => {
    const ranks = Object.values(GameTypes.Rank).filter(
        (item) => typeof item === "number",
    );

    const position = sortPosition(props.chessboard.position);
    // const position = props.chessboard.position.sort((a, b) => a[0] > b[0] ? 1 : -1);
    console.log(position);

    return (
        <div>
            <div
                style={{
                    display: "flex",
                    flexDirection: "column",
                    alignItems: "center",
                }}
            >
                {ranks.map((rank) => {
                    return (
                        <ChessboardRank
                            key={rank}
                            rank={position.slice((rank - 1) * RANK_WIDTH, rank * RANK_WIDTH)}
                        />
                    );
                })}
            </div>
        </div>
    );
};


const sortPosition = (position: GameTypes.Square[]): GameTypes.Square[] => {
    const scoreSquare = (square: GameTypes.Square): number => {
        const fileScores = {A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8};
        return square.rank * 8 + fileScores[square.file]
    }

    return position.sort((a, b) => scoreSquare(a) - scoreSquare(b))
}


interface ChessboardRankProps {
    rank: GameTypes.Square[];
}

const ChessboardRank = (props: ChessboardRankProps) => {
    return (
        <div style={{display: "flex", flexDirection: "row", width: "50%"}}>
            {props.rank.map((square: GameTypes.Square) => {
                return (
                    <ChessboardSquare
                        key={`${square.file}${square.rank}`}
                        square={square}
                    />
                );
            })}
        </div>
    );
};

interface ChessboardSquareProps {
    square: GameTypes.Square;
}

const ChessboardSquare = (props: ChessboardSquareProps) => {
    return (
        <div
            style={{
                display: "flex",
                flexDirection: "column",
                flex: 1,
                alignItems: "center",
                justifyContent: "center",
                aspectRatio: "1 / 1",
                border: "1px solid black",
            }}
        >
            {props.square.piece && <Piece piece={props.square.piece}/>}
        </div>
    );
};

interface PieceProps {
    piece: GameTypes.Piece;
}

const Piece = (props: PieceProps) => {
    const colour =
        props.piece.colour === GameTypes.Colour.White ? "white" : "black";

    return (
        <div
            style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center",
                flex: 1,
            }}
        >
            {colour}
            {props.piece.pieceType}
        </div>
    );
};
