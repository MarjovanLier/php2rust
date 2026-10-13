<?php

declare(strict_types=1);

readonly class Point
{
    public function __construct(public int $x)
    {
    }
}

function build(): int
{
    $p = new Point(1);
    $p->__construct(4);
    $e = new ArrayObject();
    $c = 'Point';
    $q = new $c(2);
    return $p->x + count($e) + $q->x;
}
