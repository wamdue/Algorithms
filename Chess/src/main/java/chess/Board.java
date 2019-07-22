package chess;

import java.util.HashMap;
import java.util.Map;

public class Board {

    private static final int BOARD_HEIGH = 8;
    private static final int AD_INFO = 5;
    private static final String HALF_MOVE_WIPER = "pP";

    private final Map<Integer, String> board;
    private final Map<Character, Integer> horizontalMapper;
    private int halfMove = 0;
    private int move = 0;
    private char side = 'w';

    public Board() {
        board = new HashMap<>();
        horizontalMapper = new HashMap<>();
        horizontalMapper.put('a', 0);
        horizontalMapper.put('b', 1);
        horizontalMapper.put('c', 2);
        horizontalMapper.put('d', 3);
        horizontalMapper.put('e', 4);
        horizontalMapper.put('f', 5);
        horizontalMapper.put('g', 6);
        horizontalMapper.put('h', 7);
    }

    private void initBoard() {
        board.clear();
        board.put(0, "");
        board.put(1, "");
        board.put(2, "");
        board.put(3, "");
        board.put(4, "");
        board.put(5, "");
        board.put(6, "");
        board.put(7, "");
    }

    public void loadFigures(String fenString) {
        initBoard();
        String boardSide = fenString.substring(0, fenString.indexOf(' '));
        String aditionalInfo = fenString.substring(boardSide.length() + 1);
        String[] lines = boardSide.split("/");
        for (int i = 0; i < BOARD_HEIGH; i++) {
            char[] chars = lines[i].toCharArray();
            for (char aChar : chars) {
                if (Character.isDigit(aChar)) {
                    for (int k = 0; k < Integer.parseInt(String.valueOf((aChar))); k++) {
                        board.put(i, board.get(i) + '.');
                    }
                } else {
                    board.put(i, board.get(i) + aChar);
                }
            }
        }

        String[] moves = aditionalInfo.split(" ");
        side = moves[0].charAt(0);
        halfMove = Integer.parseInt(moves[3]);
        move = Integer.parseInt(moves[4]);
    }

    public void drawBoard() {
        System.out.println("  +-----------------+");
        for (int i = 0; i < BOARD_HEIGH; i++) {
            System.out.print(BOARD_HEIGH - i + " | ");
            for (char a : board.get(i).toCharArray()) {
                System.out.print(a + " ");
            }
            System.out.print("|");
            System.out.println();
        }
        System.out.println("  +-----------------+");
        System.out.println("    a b c d e f g h  ");
    }

    public String toFenString() {
        StringBuilder fenString = new StringBuilder();
        int blank = 0;
        for (int i = 0; i < BOARD_HEIGH; i++) {
            for (char c : board.get(i).toCharArray()) {
                if (c == '.') {
                    blank++;
                } else if (blank > 0 && c != '.') {
                    fenString.append(blank).append(c);
                    blank = 0;
                } else {
                    fenString.append(c);
                }
            }
            if (blank > 0) {
                fenString.append(blank);
                blank = 0;
            }
            fenString.append("/");
        }
        fenString.deleteCharAt(fenString.length() - 1);
        fenString.append(" ").append(side);
        fenString.append(" ").append(halfMove).append(" ").append(move);

        return fenString.toString();
    }

    public void move(String moves) {
        String moveFrom = moves.substring(0, 2);
        String moveTo = moves.substring(2, 4);
        int vFrom = BOARD_HEIGH - Integer.parseInt(String.valueOf(moveFrom.charAt(1)));
        int hFrom = horizontalMapper.get(moveFrom.charAt(0));


        int vTo = BOARD_HEIGH - Integer.parseInt(String.valueOf(moveTo.charAt(1)));
        int hTo = horizontalMapper.get(moveTo.charAt(0));

        char from = board.get(vFrom).charAt(hFrom);
        char to = board.get(vTo).charAt(hTo);


        if ((!HALF_MOVE_WIPER.contains(String.valueOf(from)) && !HALF_MOVE_WIPER.contains(String.valueOf(to)))) {
            halfMove++;
        } else {
            halfMove = 0;
        }

        String lineFrom = new StringBuilder(board.get(vFrom)).replace(hFrom, hFrom + 1, ".").toString();
        String lineTo = new StringBuilder(board.get(vTo)).replace(hTo, hTo + 1, String.valueOf(from)).toString();


        board.put(vFrom, lineFrom);
        board.put(vTo, lineTo);

        if (side == 'w') {
            side = 'b';
        } else {
            side = 'w';
            move++;
        }
    }
}
