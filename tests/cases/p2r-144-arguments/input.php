<?php

declare(strict_types=1);

final class Cell
{
    public int $v = 0;

    public function bump(): void
    {
        $this->v++;
    }
}

final class Grid
{
    /** @var list<Cell> */
    public array $cells = [];

    public function __construct(public Cell $corner)
    {
    }

    public function poke(): void
    {
        nudge($this->corner);
        nudgeAll($this);
    }

    public function make(): Cell
    {
        return new Cell();
    }
}

function nudge(Cell $cell): void
{
    $cell->v++;
}

function nudgeAll(Grid $grid): void
{
    nudge($grid->corner);
    foreach ($grid->cells as $cell) {
        nudge($cell);
    }
}

function again(Cell $cell): void
{
    nudge($cell);
}

function peek(Cell $cell): void
{
    echo $cell->v, "\n";
}

function keep(Cell $cell): Grid
{
    peek($cell);
    return new Grid($cell);
}

$grid = new Grid(new Cell());
$grid->cells[] = new Cell();
$grid->poke();
nudge($grid->cells[0]);
nudge(new Cell());
again($grid->corner);
$grid->make()->bump();
echo $grid->corner->v, " ", $grid->cells[0]->v, "\n";
$kept = keep(new Cell());
peek($kept->corner);
