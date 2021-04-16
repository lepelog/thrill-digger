const rust = import('./pkg');

window.Unspecified = 0;
window.GreenRupee = 1;
window.BlueRupee = 2;
window.RedRupee = 3;
window.SilverRupee = 4;
window.GoldRupee = 5;
window.Rupoor = 6;
window.Bomb = 7;

window.DifficultyBeginner = 0;
window.DifficultyIntermediate = 1;
window.DifficultyExpert = 2;

rust
  .then(m => {
      let solver = m.create_solver(DifficultyExpert);
      const old = Date.now();
      // solver.calculate_probabilities();
      solver.cache_boards();
      const diff = Date.now() - old;
      console.log(`calculated in ${diff}ms`);
      window.thrillDigger = m;
      window.board = solver;
    })
  .catch(console.error);

window.printBoard = function(board) {
    for(let y = 0; y < board.get_height(); y++) {
        let currentLine = "";
        for (let x = 0; x < board.get_width(); x++) {
            currentLine = currentLine + board.get_probability(x + y * board.get_width()).toFixed(2) + "  ";
        }
        console.log(currentLine);
    }
}

window.setHoleToContent = function(hole, content) {
  board.set_hole(hole, content);
  const old = Date.now();
  board.calculate_probabilities_with_pregenerated();
  const diff = Date.now() - old;
  console.log(`calculated probs with cached in ${diff}ms`);
  printBoard(board);
}
