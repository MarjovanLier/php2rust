<?php

declare(strict_types=1);

readonly class Marker
{
    public function size(): int
    {
        return 0;
    }
}

enum Side
{
    case Left;
    case Right;
}

echo (new Point(3, 4))->sum(), "\n";
echo (new Marker())->size(), "\n";

final readonly class Point
{
    public function __construct(public int $x, public int $y)
    {
    }

    public function sum(): int
    {
        return $this->x + $this->y;
    }
}

readonly class Placed
{
    public function __construct(public Side $side, readonly int $at, final int $weight)
    {
    }
}

$p = new Placed(Side::Right, 7, 2);
echo $p->at + $p->weight, "\n";
if ($p->side === Side::Right) {
    echo "right\n";
}
