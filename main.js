const BOARD_WIDTH = 10;
const BOARD_HEIGHT = 20;

async function fetchState() {
    const res = await fetch('/api/state');
    return await res.json();
}

async function sendAction(action) {
    await fetch('/api/action', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action })
    });
}

function renderBoard(board) {
    const boardDiv = document.getElementById('board');
    boardDiv.innerHTML = '';
    for (let y = 0; y < BOARD_HEIGHT; y++) {
        const rowDiv = document.createElement('div');
        rowDiv.className = 'row';
        for (let x = 0; x < BOARD_WIDTH; x++) {
            const cellDiv = document.createElement('div');
            cellDiv.className = 'cell ' + (board[y][x] ? 'filled' : 'empty');
            rowDiv.appendChild(cellDiv);
        }
        boardDiv.appendChild(rowDiv);
    }
}

function renderScore(score) {
    document.getElementById('score').textContent = score;
}

async function update() {
    const state = await fetchState();
    renderBoard(state.board);
    renderScore(state.score);
}

// Keyboard controls
window.addEventListener('keydown', async (e) => {
    let action = null;
    if (e.key === 'ArrowLeft') action = 'left';
    if (e.key === 'ArrowRight') action = 'right';
    if (e.key === 'ArrowDown') action = 'down';
    if (e.key === 'ArrowUp') action = 'rotate';
    if (action) {
        await sendAction(action);
        await update();
    }
});

// Gravity loop
setInterval(async () => {
    await sendAction('tick');
    await update();
}, 500);

// Initial render
update();
