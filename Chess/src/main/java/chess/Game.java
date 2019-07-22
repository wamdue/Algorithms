package chess;


public class Game {
    public static void main(String[] args) {
        Board board = new Board();
        System.out.println("rnbqkb1r/pppppppp/8/4N3/6n1/8/PPPPPPPP/RNBQKB1R w KQkq - 4 3");
        board.loadFigures("rnbqkb1r/pppppppp/8/4N3/6n1/8/PPPPPPPP/RNBQKB1R w KQkq - 4 3");
        board.drawBoard();
        System.out.println(board.toFenString());
        board.move("e5g4");
        board.drawBoard();
        System.out.println(board.toFenString());
    }
}
