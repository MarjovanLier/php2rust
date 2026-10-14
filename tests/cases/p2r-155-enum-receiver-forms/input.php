<?php

declare(strict_types=1);

final class Cell
{
    public int $n = 0;
}

enum Op
{
    case Read;

    public function apply(Cell $cell): int
    {
        return $cell->n;
    }

    public function twice(Cell $cell): int
    {
        return $this->apply($cell) + $this->apply($cell);
    }
}

final class Writer
{
    public function apply(Cell $cell): int
    {
        $cell->n += 1;
        return $cell->n;
    }
}

function run(Cell $cell): int
{
    return Op::Read->apply($cell);
}

$cell = new Cell();
echo (new Writer())->apply($cell), "\n";
echo run($cell), Op::Read->twice($cell), "\n";
